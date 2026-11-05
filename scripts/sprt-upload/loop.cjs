// The upload loop of one shard of the SPRT in blocks (ADR 0049, decision 1;
// scripts/sprt-blocks.sh and .github/workflows/sprt-parallel.yml). The arena
// plays the shard's tournament in the background and writes block-N.jsonl into
// $SPRT_WORK/blocks as each block is whole. This loop, every UPLOAD_SECONDS
// (120): looks for the verdict artifact of the run and, if it is there, stops
// the arena and ends; otherwise uploads the blocks finished since the last
// upload as one artifact, sprt-blocks-GAME-ATTEMPT-SHARD-K (1 day), a folder
// of files SHARD-N.jsonl. It ends after the upload that holds the last block
// (BLOCKS blocks, from $SPRT_WORK/blocks-total). A failed upload is retried
// 3 times, each under a new number K, then the loop fails. An arena that died before its last block
// fails it too, after uploading what exists, and so does one that is still
// running but has finished no block for HUNG_SECONDS (900): it is killed, so
// that the coordinator sees the shard end instead of waiting for the limits.
//
// The artifact client (@actions/artifact, installed by the workflow at an
// exact version into a folder named by SPRT_ARTIFACT_LIB) reads
// ACTIONS_RUNTIME_TOKEN and ACTIONS_RESULTS_URL, which the runner gives to
// actions only: that is why the workflow runs this file through
// actions/github-script, a JavaScript action. The verdict is looked for with
// `gh api` (GH_TOKEN, REPO, RUN_ID): one request. Tokens are never printed.
//
// For tests, SPRT_DRY_RUN_DIR copies each batch to a folder of that name
// instead of uploading, and the existence of the file SPRT_DRY_VERDICT_FILE
// stands for the verdict artifact. `node scripts/sprt-upload/loop.cjs` runs
// the loop from the environment: GAME, RUN_ATTEMPT, SHARD, SPRT_WORK, and
// optionally UPLOAD_SECONDS, POLL_MS (1000, how often it looks at the clock
// and the arena).
'use strict';

const fs = require('node:fs');
const path = require('node:path');
const { execFileSync } = require('node:child_process');

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

function alive(pid) {
  try {
    process.kill(pid, 0);
  } catch {
    return false;
  }
  try {
    // A process that ended but is not reaped yet is a zombie, not running.
    const stat = fs.readFileSync(`/proc/${pid}/stat`, 'utf8');
    return stat.slice(stat.lastIndexOf(')') + 2, stat.lastIndexOf(')') + 3) !== 'Z';
  } catch {
    return true;
  }
}

function stopPlayer(pid) {
  // The arena leads a session of its own: its bots die with it.
  for (const target of [-pid, pid]) {
    try {
      process.kill(target, 'SIGTERM');
      return;
    } catch {
      // Try the process alone, or it is gone.
    }
  }
}

function verdictExists(env) {
  if (env.SPRT_DRY_RUN_DIR) {
    return fs.existsSync(env.SPRT_DRY_VERDICT_FILE || '');
  }
  const name = `sprt-verdict-${env.GAME}-${env.RUN_ATTEMPT}`;
  try {
    const out = execFileSync(
      'gh',
      ['api', `repos/${env.REPO}/actions/runs/${env.RUN_ID}/artifacts?name=${name}&per_page=1`, '--jq', '.total_count'],
      { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'] },
    );
    return /^[1-9]/.test(out.trim());
  } catch {
    // No answer is no verdict: the next look tries again.
    return false;
  }
}

// Uploads the files as artifact `name`. In a dry run, copies them instead;
// SPRT_DRY_FAIL (a number) makes that many uploads fail first, to test retries.
let dryFailures = 0;
async function upload(env, name, dir, files) {
  if (env.SPRT_DRY_RUN_DIR) {
    if (dryFailures < Number(env.SPRT_DRY_FAIL ?? 0)) {
      dryFailures++;
      throw new Error('a failure for the test');
    }
    const target = path.join(env.SPRT_DRY_RUN_DIR, name);
    fs.mkdirSync(target, { recursive: true });
    for (const file of files) fs.copyFileSync(file, path.join(target, path.basename(file)));
    return;
  }
  const { DefaultArtifactClient } = require(env.SPRT_ARTIFACT_LIB);
  await new DefaultArtifactClient().uploadArtifact(name, files, dir, { retentionDays: 1 });
}

// Uploads a batch, trying 3 times. Each try has a name of its own (the next
// batch number): an artifact that was created but not finished may refuse the
// same name again. The coordinator names blocks by their files, so a block in
// two batches counts once. Returns the number of the batch that went through.
async function uploadBatch(env, batch, dir, files) {
  let last;
  for (let attempt = 1; attempt <= 3; attempt++, batch++) {
    const name = `sprt-blocks-${env.GAME}-${env.RUN_ATTEMPT}-${env.SHARD}-${batch}`;
    try {
      await upload(env, name, dir, files);
      return batch;
    } catch (error) {
      last = error;
      console.log(`Upload of ${name} failed (try ${attempt} of 3): ${error.message}`);
      await sleep(Number(env.RETRY_MS ?? 5000) * attempt);
    }
  }
  throw new Error(`the upload failed 3 times: ${last.message}`);
}

async function run(env = process.env) {
  const work = env.SPRT_WORK || 'target/sprt-parallel';
  const interval = Number(env.UPLOAD_SECONDS ?? 120) * 1000;
  const poll = Number(env.POLL_MS ?? 1000);
  const total = Number(fs.readFileSync(path.join(work, 'blocks-total'), 'utf8'));
  const pid = Number(fs.readFileSync(path.join(work, 'player.pid'), 'utf8'));
  const shard = env.SHARD;
  let next = 0;
  let batches = 0;
  const hung = Number(env.HUNG_SECONDS ?? 900) * 1000;
  let last = Date.now();
  let progress = last;
  for (;;) {
    while (Date.now() - last < interval && alive(pid)) await sleep(poll);
    last = Date.now();
    if (verdictExists(env)) {
      console.log(`The verdict is in: shard ${shard} stops.`);
      stopPlayer(pid);
      return;
    }
    const dir = path.join(work, 'upload', `batch-${batches}`);
    fs.rmSync(dir, { recursive: true, force: true });
    fs.mkdirSync(dir, { recursive: true });
    const files = [];
    while (next < total && fs.existsSync(path.join(work, 'blocks', `block-${next}.jsonl`))) {
      const file = path.join(dir, `${shard}-${next}.jsonl`);
      fs.copyFileSync(path.join(work, 'blocks', `block-${next}.jsonl`), file);
      files.push(file);
      next++;
    }
    if (files.length > 0) {
      progress = Date.now();
      const sent = await uploadBatch(env, batches, dir, files);
      console.log(`Shard ${shard}: uploaded sprt-blocks-${env.GAME}-${env.RUN_ATTEMPT}-${shard}-${sent}, ${files.length} blocks, ${next} of ${total} in all.`);
      batches = sent + 1;
    }
    if (next >= total) {
      console.log(`Shard ${shard} played its ${total} blocks.`);
      return;
    }
    if (!alive(pid)) {
      throw new Error(`the tournament of shard ${shard} ended before block ${next}`);
    }
    if (Date.now() - progress > hung) {
      stopPlayer(pid);
      throw new Error(`the tournament of shard ${shard} finished no block in ${hung / 60000} minutes: stopped`);
    }
  }
}

module.exports = run;

if (require.main === module) {
  run().catch((error) => {
    console.error(`error: ${error.message}`);
    process.exit(1);
  });
}
