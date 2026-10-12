# 0031. Arena games: one archive per game on GitHub, shown read-only in the studio

- Status: accepted (owner, 2026-10-12)
- Date: 2026-10-12
- Completes: [0027](0027-game-records.md), decisions 2 and 3; [0030](0030-studio-on-github-pages.md), decision 6
- Scope: framework

## Context

The SPRT and the league upload a sample of their games as workflow artifacts ([ADR 0027](0027-game-records.md), decision 2), which GitHub deletes after their retention period. To see them in the studio, the owner had to download each file and import it, and the studio on GitHub Pages ([ADR 0030](0030-studio-on-github-pages.md)) showed only the games saved in that browser.

On 2026-10-11 the owner asked for the games played on GitHub to appear in the site's history, bundled from the artifacts, with workflows that keep them before they expire. On 2026-10-12 the owner set how: re-upload the same games, and when a new release is published, keep the current games and add the new ones in a new artifact that replaces the current one, "only one artifact per game". The same day the owner chose that the local server shows them too, that games are added only when a release is published, that the archive starts from the league games already on GitHub, that it has no size limit, and that the studio calls them "Arena games", a name it already used, rather than a new one.

GitHub keeps an artifact for at most 90 days in a public repository, and turns off scheduled workflows in a public repository after 60 days without activity. A record is a few kilobytes; the sample of one league run of Ultimate Tic-Tac-Toe is about 0.5 MB compressed.

## Decision

1. **Each game keeps one archive of its arena games, as a single workflow artifact.** It holds every kept record, each checked by replaying it with the game's referee and stored once under its record id, with an index the studio lists. When a newer archive is uploaded, the older ones are deleted, so one remains per game.
2. **Games are added only when a release is published.** When a push to `main` adds a release, the sample of the league that rates it and the SPRT sample of its pull request are added to the current archive, and the result is uploaded as the new archive. The first archive starts from the league samples still on GitHub.
3. **A scheduled workflow keeps the archive alive without playing games.** It uploads the same records again, with the longest retention GitHub allows, before the current archive expires.
4. **The archive has no size limit.** Nothing is pruned; a limit would be a new decision.
5. **The studio shows the archive as "Arena games", read-only, beside the visitor's own games.** The site on GitHub Pages publishes the archive's records with each build and rebuilds when the archive changes; the local server's front end reads the same published files from the site. Arena games can be filtered, reviewed, exported, and copied into one's own games; they are never changed or deleted from the studio.

## Consequences

- Games played on GitHub can be reviewed by anyone with the link, without downloading or importing a file.
- The archive lives only in GitHub's artifact storage, not in the repository: if every scheduled run fails or is turned off for longer than the retention period, it is lost, and starts again from the next release. The daily report should warn when the archive is close to expiring.
- A workflow gets permission to delete artifacts, and the Pages build to read them.
- With the local server, the history's Arena games need a network connection to the published site; the visitor's own games do not.
- Each release adds its samples to the site, which grows by about the size of a league sample per release.
