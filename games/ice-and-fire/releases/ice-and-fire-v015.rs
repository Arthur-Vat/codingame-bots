// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
mod 乍{type C=usize;use crate::丕::l::乥;use crate::丕::{G,K,I,T,SIZE};use crate::习::chain::{什,亝,仩};use crate::习::丒::亟;use crate::乩::亴;pub const 佲:bool=true;fn
侬(l:&I,q:C,伜:i64,丼:G,chain:i64)->bool{let 一=l.与();let 丛=1-一;let 佅=佲&&chain<=l.丅(丛)+2*l.上(丛).max(0);let 仯=G::q(q)|G::q(q).且();if!佅&&(仯&丼).is_empty(){return
false;}let(x,y)=丹(q);let mut 个=l.clone();if!个.严(K::亍{x,y}){return false;}if 亴(&个,一)<伜{return true;}佅&&亝(&个,丛,false,T[一])>chain}pub const 佷:C=4;pub const
亸:C=5;#[derive(Clone)]pub enum 丿{亽(K),价(Vec<K>),}impl 丿{pub fn 丁(&self)->&[K]{match self{丿::亽(三)=>std::slice::from_ref(三),丿::价(丁)=>丁,}}}pub fn 丹(q:C)->(u8,u8){((q%SIZE)as
u8,(q/SIZE)as u8)}pub fn 伉(l:&I,丆:&mut Vec<丿>){丆.clear();if l.下().is_some(){return;}let 一=l.与();let max=l.丗().乫();let mut 东=[G::Z;4];for level in 1..=max{东[C::from(level)]=l.为(一,level);}for
from in l.亅().iter(){let Some((id,level))=l.仏(from)else{continue;};let 么=G::q(from).且()&东[C::from(level)];for 买 in 么.iter(){let(x,y)=丹(买);丆.push(丿::亽(K::亀{id,x,y}));}}let
丅=l.丅(一);let 仦=l.亐(一);for level in 1..=max{let 万=10*i64::from(level);if 丅<万{break;}let mut 丌=仦&东[C::from(level)];for 侱 in 1..level{丌=丌.两(东[C::from(侱)]);}for
q in 丌.iter(){let(x,y)=丹(q);丆.push(丿::亽(K::乨{level,x,y}));}}if 丅>=乥{let free=l.丙(一)&仦&l.仄();if l.丗().付(){for q in(free&l.map().临).iter(){let(x,y)=丹(q);丆.push(丿::亽(K::乾{x,y}));}}if
l.丗().仼(){let 侙=l.map().valid.两(l.丙(一)).且();let 丌=(free&侙).两(l.map().临);let 丢=l.丅(1-一)+l.上(1-一);let 丼=仩(l,1-一,false,丢);let 伜=i64::from((丼&l.丂(一)).count());let
chain=亝(l,1-一,false,T[一]);for q in 丌.iter().filter(|&q|侬(l,q,伜,丼,chain)){let(x,y)=丹(q);丆.push(丿::亽(K::亍{x,y}));}}}for 丒 in 亟(l,亸,丅,佷){if 丒.丁.len()>1{丆.push(丿::价(丒.丁));}}if
let Some(chain)=什(l,T[1-一],丅){if chain.丁.len()>1{丆.push(丿::价(chain.丁));}}}}mod 乩{type C=usize;type F=i64;use crate::丕::{G,I,M,T};use crate::习::chain::仩;use
crate::习::乩::伽;pub fn 亴(l:&I,z:C)->F{let 丛=1-z;let 丢=l.丅(丛)+l.上(丛);F::from((仩(l,丛,false,丢)&l.丂(z)).count())}const 侊:F=2;fn 佥(l:&I,z:C)->F{let 丂=l.丂(z);let[a,b,c]=丂.仮();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as u64).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let index=(hash as C)&(伩-1);侓.with(|企|{let mut
企=企.borrow_mut();let 们=&mut 企[index];if 们.0==丂&&们.1==z as u8+1{return 们.2;}let value=保(丂,T[z]);*们=(丂,z as u8+1,value);value})}const 伩:C=1<<12;thread_local!{static
侓:std::cell::RefCell<Vec<(G,u8,F)>> =std::cell::RefCell::new(vec![(G::Z,0,0);伩]);}pub fn 保(丂:G,乮:C)->F{struct 人{丂:G,乃:[u8;M],丱:[u8;M],size:[u8;M],乯:u8,乷:F,以:F,}fn
亖(d:&mut 人,v:C)->u8{d.乯+=1;let 乃=d.乯;d.乃[v]=乃;let mut 丱=乃;let mut size=1u8;let mut 丒=0u8;for w in(G::q(v).且()&d.丂).iter(){if d.乃[w]==0{let 佛=亖(d,w);size+=佛;丱=丱.min(d.丱[w]);if
d.丱[w]>=乃{丒+=佛;}}else{丱=丱.min(d.乃[w]);}}d.丱[v]=丱;d.size[v]=size;if 丒>0&&d.乃[v]>1{d.乷+=1;d.以=d.以.max(F::from(丒));}size}if!丂.test(乮){return 0;}let mut d=人{丂,乃:[0;M],丱:[0;M],size:[0;M],乯:0,乷:0,以:0,};亖(&mut
d,乮);d.乷+d.以}pub fn 伙(l:&I,z:C)->F{侊*(佥(l,1-z)-佥(l,z))}fn 仁(l:&I,z:C)->F{if l.丅(z)+l.上(z)>=0{return 0;}(l.七(z)&l.丂(z)).iter().filter_map(|q|l.井(q)).map(|level|10*F::from(level)).sum()}pub
const 佶:F=15;fn 佡(l:&I,z:C)->F{(l.七(z)&l.丂(z)).iter().filter(|&q|l.井(q)==Some(2)).count()as F}pub fn 乩(l:&I,z:C)->F{let 不=伽(l,z);if l.下().is_some(){return
不;}不-2*仁(l,z)+2*仁(l,1-z)+伙(l,z)+佶*(佡(l,1-z)-佡(l,z))+7*(亴(l,1-z)-亴(l,z))/4}}mod 丽{type C=usize;type Y=u64;type F=i64;type U=bool;type 举=Instant;type J<T>=Vec<T>;type
O<T>=Option<T>;use std::collections::hash_map::DefaultHasher;use std::collections::HashMap;use std::hash::BuildHasherDefault;use std::time::Instant;use
crate::丕::{K,丠,I,T};use crate::丨::伓::九;use crate::但::佁;use crate::习::chain::什;use crate::习::丒::亟;use crate::习::乩::亾;use crate::乍::{伉,丹,丿,亸};use crate::乩::乩;const
使:C=8;const 侅:C=3;const 佸:C=1000;const 來:Y=300;const 佫:f64=0.79;const 佪:f64=0.84;const 佩:f64=0.95;#[derive(Clone,Copy)]pub struct 书{pub 乼:O<举>,pub 交:O<举>,pub
仈:Y,}#[derive(Default)]pub struct 伧{pub 丁:J<K>,pub 伅:C,pub 乵:Y,pub 世:C,pub 丩:C,pub 伴:C,pub 乶:C,}const 侒:U=true;#[derive(Clone)]struct 乞{丁:J<K>,伇:F,}struct
亘{丁:J<K>,end:I,first:O<Y>,丑:O<J<乞>>,}type 亻=HashMap<Y,F,BuildHasherDefault<DefaultHasher>>;struct 亙{一:C,limit:书,乵:Y,仅:亻,亓:O<J<丿>>,}fn 仾(l:&mut I,丁:&[K])->U{丁.iter().all(|&三|l.严(三))}fn
key(丁:&[K])->Y{let mut hash:Y=0xcbf29ce484222325;for 三 in 丁{let(kind,a,b,c)=match*三{K::亀{id,x,y}=>(1,Y::from(id),x,y),K::乨{level,x,y}=>(2,Y::from(level),x,y),K::乾{x,y}=>(3,0,x,y),K::亍{x,y}=>(4,0,x,y),K::仗=>(5,0,0,0),};let
word=kind|a<<8|Y::from(b)<<40|Y::from(c)<<48;hash=(hash^word).wrapping_mul(0x0100000001b3);}hash}fn 伭(end:&I,乭:&[K])->I{let mut next=end.clone();next.亠();for&三
in 乭{next.严(三);}next.亠();next}fn 仢(l:&I)->O<J<K>>{let 一=l.与();let 丐=T[1-一];let 亷=|业:&I|业.下()==Some(丠::乗(一));for q in l.亅().iter(){if l.map().乄(q,丐)!=1{continue;}let
Some((id,_))=l.仏(q)else{continue;};let(x,y)=丹(丐);let 三=K::亀{id,x,y};let mut 业=l.clone();if 业.严(三)&&亷(&业){return Some(vec![三]);}}let chain=什(l,丐,l.丅(一))?;let
mut 业=l.clone();(仾(&mut 业,&chain.丁)&&亷(&业)).then_some(chain.丁)}impl 亙{fn 乣(&self)->U{self.乵>=self.limit.仈||self.limit.乼.is_some_and(|t|举::now()>=t)}fn
不(&mut self,end:&I,世:&[乞],伀:F)->F{self.乵+=1;self.伕(end,世,伀)}fn 伕(&self,end:&I,世:&[乞],伀:F)->F{if end.下()==Some(丠::乗(self.一)){return 亾;}if 世.is_empty(){return
乩(end,self.一);}let mut 二=F::MAX;for 乭 in 世{二=二.min(乩(&伭(end,&乭.丁),self.一));if 二<=伀{break;}}二}#[allow(clippy::too_many_arguments)]fn 侻(&mut self,乴:&I,亞:F,乍:&[丿],亯:&[(F,C)],丫:&mut[U],世:&[乞],亡:&mut
亻,伍:U,)->(O<(C,I,F)>,U){let mut 丈=None;let mut 仙=亞;for&(仧,index)in 亯{if 丫[index]{continue;}if 伍&&仧!=F::MAX&&仧<=仙-亞{return(丈,true);}if self.乣(){return(丈,false);}丫[index]=true;let
丁=乍[index].丁();let mut next=乴.clone();if!仾(&mut next,丁){亡.insert(key(丁),F::MIN/4);continue;}let 不=self.不(&next,世,仙);亡.insert(key(丁),不-亞);if 不>仙{仙=不;丈=Some((index,next,不));}}(丈,false)}fn
伄(&mut self,乮:&I,世:&[乞],亿:&[Y],供:U)->亘{let j=self;let mut first=None;let mut 乴=乮.clone();let mut 丁=J::new();let mut 亞=j.不(&乴,世,F::MIN);let mut 亡=亻::default();let
mut 乍=J::new();let mut 亯=J::new();while 乴.下().is_none()&&!j.乣(){if 丁.is_empty()&&供{match&j.亓{Some(亓)=>乍.clone_from(亓),None=>{伉(&乴,&mut 乍);j.亓=Some(乍.clone());}}}else{伉(&乴,&mut
乍);}亯.clear();for(index,乳)in 乍.iter().enumerate(){let k=key(乳.丁());let 仧=亡.get(&k).or(j.仅.get(&k)).copied();亯.push((仧.unwrap_or(F::MAX),index));}亯.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丫=vec![false;乍.len()];if 丁.is_empty(){for(index,乳)in 乍.iter().enumerate(){丫[index]=亿.contains(&key(乳.丁()));}}let
mut 丈=None;for 伍 in[true,false]{let(丧,俌)=j.侻(&乴,亞,&乍,&亯,&mut 丫,世,&mut 亡,伍,);丈=丧;if 丈.is_some()||!俌{break;}}let Some((index,next,不))=丈 else{break;};if 丁.is_empty(){first=Some(key(乍[index].丁()));}丁.extend_from_slice(乍[index].丁());乴=next;亞=不;}for(k,乪)in
亡{let 佂=j.仅.entry(k).or_insert(乪);*佂=(*佂).max(乪);}亘{丁,end:乴,first,丑:None,}}fn 係(&self,end:&I)->J<J<K>>{let mut next=end.clone();next.亠();let mut 丆=J::new();if
next.下().is_some(){return 丆;}let 乢=next.与();let 丅=next.丅(乢);if let Some(chain)=什(&next,T[self.一],丅){丆.push(chain.丁);}for 丒 in 亟(&next,亸,丅,侅){丆.push(丒.丁);}let
mut 佀=J::new();佁(&next,&mut 佀);丆.push(佀);let mut theirs=亙{一:乢,limit:书{仈:來,..self.limit},乵:0,仅:亻::default(),亓:None,};丆.push(theirs.伄(&next,&[],&[],true).丁);丆.retain(|丁|!丁.is_empty());丆}fn
伮(&self,end:&I)->J<乞>{self.係(end).into_iter().map(|丁|{let 伇=乩(&伭(end,&丁),self.一);乞{丁,伇}}).collect()}fn 乶(&mut self,乮:&I,乳:&丿,世:&[乞])->O<亘>{let mut 个=乮.clone();if!仾(&mut
个,乳.丁()){return None;}self.乵+=1;let rest=self.伄(&个,世,&[],false);let mut 丁=乳.丁().to_vec();丁.extend_from_slice(&rest.丁);Some(亘{丁,end:rest.end,first:Some(key(乳.丁())),丑:None,})}fn
伾(&self,丄:&亘,世:&[乞])->F{let 不=self.伕(&丄.end,世,F::MIN);match&丄.丑{Some(丑)if!丑.is_empty()=>不.min(self.伕(&丄.end,丑,F::MIN)),_=>不,}}}fn 俁(丧:&[乞],世:&mut Vec<乞>)->U{let
並:J<_> =世.iter().map(|r|r.丁.clone()).collect();for 乭 in 丧{if 世.iter().any(|r|r.丁==乭.丁){continue;}世.push(乭.clone());}世.sort_by_key(|乭|乭.伇);世.truncate(使);世.iter().map(|r|&r.丁).ne(&並)}pub
fn 佞(l:&I,limit:书,亮:&mut 九)->伧{let 一=l.与();let mut 中=伧::default();if l.下().is_some(){return 中;}if let Some(丁)=仢(l){中.丁=丁;return 中;}let 両=|俆:f64|match(limit.乼,limit.交){(Some(乼),Some(交))=>Some(乼+(交-乼).mul_f64(俆)),_=>None,};if
亮.侼(l){中.丁=亮.turn(l,両(0.4));return 中;}let mut 丽=亙{一,limit:书{乼:両(佫),..limit},乵:0,仅:亻::default(),亓:None,};let mut 世=J::new();let mut 丩:J<亘> =J::new();let
mut 亿=J::new();let mut 仜=false;let mut 乶:J<丿> =J::new();while 中.伅<佸&&(丩.is_empty()||!丽.乣()){let 丄=if 仜{let Some(乳)=乶.pop()else{break;};match 丽.乶(l,&乳,&世){Some(丄)=>丄,None=>continue,}}else{丽.伄(l,&世,&亿,true)};中.伅+=1;let
佖=丩.iter().any(|p|p.丁==丄.丁);let first=丄.first;let 佚=丽.乣();let mut 丄=丄;let mut 佊=false;if!佚&&!佖{let 丧=丽.伮(&丄.end);佊=俁(&丧,&mut 世);丄.丑=Some(丧);}if!佖{丩.push(丄);}if
佚{break;}if!佊&&!仜{match first{Some(first)=>亿.push(first),None=>{仜=true;乶=丽.亓.clone().unwrap_or_default();乶.retain(|a|!亿.contains(&key(a.丁())));let 乪=|a:&丿|{丽.仅.get(&key(a.丁())).copied().unwrap_or(F::MIN)};乶.sort_by_key(乪);}}}}中.乶=C::from(仜);中.乵=丽.乵;中.世=世.len();中.丩=丩.len();let
侰=||limit.交.is_some_and(|t|举::now()>=t);let mut 亲:J<O<F>> =vec![None;丩.len()];for index in(0..丩.len()).rev(){if index+1<丩.len()&&侰(){break;}亲[index]=Some(丽.伾(&丩[index],&世));}let
亰=|亲:&[O<F>]|{let mut 丈:O<(F,C)> =None;for(index,不)in 亲.iter().enumerate().rev(){if let Some(不)=*不{if 丈.is_none_or(|(乔,_)|不>乔){丈=Some((不,index));}}}丈};let
mut 丈=亰(&亲);let 侟=両(佪);let 俔=||侟.is_some_and(|t|举::now()>=t);丽.limit.乼=両(佩);while let Some((_,index))=丈{if!侒||丩[index].丑.is_some()||俔(){break;}let 丧=丽.伮(&丩[index].end);丩[index].丑=Some(丧);中.伴+=1;亲[index]=Some(丽.伾(&丩[index],&世));丈=亰(&亲);}if
let Some((_,index))=丈{let mut 丬=l.clone();for&三 in&丩[index].丁{if 丬.严(三){中.丁.push(三);}}}中}}mod 俄{use std::io::{BufRead,Write};use std::time::Instant;use
crate::丕::{份,乊,I};use crate::丨::伓::九;use crate::但::io::丟;use crate::丽::{佞,书};pub fn 乇(limit:&dyn Fn(Instant,u32)->书,mut 亮:九,仠:impl BufRead,仨:impl Write,)->Result<(),String>{丟(仠,仨,|亦,乓,turn,五|match
I::read(亦,乓,乊::仲,turn){Ok(l)=>{let 中=佞(&l,limit(五,turn),&mut 亮);eprintln!("turn {turn}: climbs {} tries {} memory {} checks {} forced {} {:?}",中.伅,中.乵,中.世,中.伴,中.乶,五.elapsed());份(&中.丁)}Err(err)=>{eprintln!("plan: cannot read the input: {err}");"WAIT".to_string()}},)}}use
std::io;use std::time::{Duration,Instant};use crate::丨::乙::Q;use crate::丨::伓::{九,乖};use crate::丨::乽::{self,亁};use crate::丽::书;const 佦:f64=48.0;const 佯:f64=800.0;const
例:f64=0.6;fn limit(五:Instant,turn:u32)->书{if let Some(仈)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
书{乼:None,交:None,仈,};}let 仫=crate::依::乯::俑().min(100.0);let 侶=if turn==1{佯}else{佦};let 丢=Duration::from_secs_f64(侶/1000.0*仫);书{乼:Some(五+丢.mul_f64(例)),交:Some(五+丢),仈:u64::MAX,}}const
侃:usize=7;const 侂:亁=亁{丌:4,上:3,丅:1,丒:2,乷:2,从:2,丼:6,事:1,};fn 亮()->九{侹(64)}fn 侹(乂:usize)->九{let 乺=乖::他{丟:侃,乂,乒:侂,};九::new(乺,乽::侉.to_vec(),Q::仺)}fn main(){let
stdin=io::stdin();let stdout=io::stdout();if let Err(err)=俄::乇(&limit,亮(),stdin.lock(),stdout.lock()){eprintln!("plan: {err}");std::process::exit(1);}}mod
依{pub mod 仠{type 丣=String;use std::fmt;#[derive(Clone,Debug,PartialEq,Eq)]pub enum 亼{佮,佴(丣),侄{line:丣,expected:丣},}impl fmt::Display for 亼{fn fmt(&self,f:&mut
fmt::Formatter<'_>)->fmt::Result{match self{亼::佮=>write!(f,"input ended"),亼::佴(err)=>write!(f,"cannot read input: {err}"),亼::侄{line,expected}=>{write!(f,"expected {expected}, got {line:?}")}}}}impl
std::error::Error for 亼{}}pub mod 乯{pub const 侎:&str="CG_TIME_SCALE";pub fn 俑()->f64{侺(std::env::var(侎).ok().as_deref())}fn 侺(value:Option<&str>)->f64{value.and_then(|乓|乓.trim().parse::<f64>().ok()).filter(|仫|仫.is_finite()&&*仫>0.0).unwrap_or(1.0)}}}mod
丕{pub mod bits{type C=usize;type Y=u64;use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,Not};pub const SIZE:C=12;pub const M:C=SIZE*SIZE;const 乕:C=4*SIZE;const
乧:Y=(1<<乕)-1;const 仒:Y=1|1<<12|1<<24|1<<36;const 仳:Y=仒<<11;const 伦:Y=(1<<SIZE)-1;#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct G([Y;3]);impl
G{pub const Z:G=G([0;3]);pub const 仑:G=G([乧;3]);#[inline]pub fn q(q:C)->G{let mut bits=G::Z;bits.set(q);bits}#[inline]pub fn test(self,q:C)->bool{(self.0[q/乕]>>(q%乕))&1!=0}#[inline]pub
fn set(&mut self,q:C){self.0[q/乕]|=1<<(q%乕);}#[inline]pub fn clear(&mut self,q:C){self.0[q/乕]&=!(1<<(q%乕));}#[inline]pub fn is_empty(self)->bool{self.0[0]|self.0[1]|self.0[2]==0}#[inline]pub
fn 仮(self)->[Y;3]{self.0}#[inline]pub fn count(self)->u32{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]pub fn 两(self,丞:G)->G{G([self.0[0]&!丞.0[0],self.0[1]&!丞.0[1],self.0[2]&!丞.0[2],])}#[inline]pub
fn iter(self)->仱{仱{伐:self.0,両:0,}}#[inline]pub fn 俋(self)->G{let[a,b,c]=self.0;G([(a>>SIZE)|((b&伦)<<(乕-SIZE)),(b>>SIZE)|((c&伦)<<(乕-SIZE)),c>>SIZE,])}#[inline]pub
fn 俈(self)->G{let[a,b,c]=self.0;G([(a<<SIZE)&乧,((b<<SIZE)&乧)|(a>>(乕-SIZE)),((c<<SIZE)&乧)|(b>>(乕-SIZE)),])}#[inline]pub fn 俉(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仳&乧,(b>>1)&!仳&乧,(c>>1)&!仳&乧,])}#[inline]pub
fn 俊(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仒&乧,(b<<1)&!仒&乧,(c<<1)&!仒&乧,])}#[inline]pub fn 且(self)->G{let j=self;j.俋()|j.俈()|j.俉()|j.俊()}}impl BitAnd for
G{type Output=G;#[inline]fn bitand(self,丞:G)->G{G([self.0[0]&丞.0[0],self.0[1]&丞.0[1],self.0[2]&丞.0[2],])}}impl BitAndAssign for G{#[inline]fn bitand_assign(&mut
self,丞:G){*self=*self&丞;}}impl BitOr for G{type Output=G;#[inline]fn bitor(self,丞:G)->G{G([self.0[0]|丞.0[0],self.0[1]|丞.0[1],self.0[2]|丞.0[2],])}}impl
BitOrAssign for G{#[inline]fn bitor_assign(&mut self,丞:G){*self=*self|丞;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::仑.两(self)}}#[derive(Clone,Debug)]pub
struct 仱{伐:[Y;3],両:C,}impl Iterator for 仱{type Item=C;#[inline]fn next(&mut self)->Option<C>{let j=self;while j.両<3{let 代=j.伐[j.両];if 代!=0{j.伐[j.両]=代&(代-1);return
Some(j.両*乕+代.trailing_zeros()as C);}j.両+=1;}None}}}pub mod 三{type 丣=String;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum K{亀{id:u32,x:u8,y:u8},乨{level:u8,x:u8,y:u8},乾{x:u8,y:u8},亍{x:u8,y:u8},仗,}impl
K{pub fn 俓(&self)->丣{match*self{K::亀{id,x,y}=>format!("MOVE {id} {x} {y}"),K::乨{level,x,y}=>format!("TRAIN {level} {x} {y}"),K::乾{x,y}=>format!("BUILD MINE {x} {y}"),K::亍{x,y}=>format!("BUILD TOWER {x} {y}"),K::仗=>"WAIT".to_string(),}}}pub
fn 份(丁:&[K])->丣{if 丁.is_empty(){return "WAIT".to_string();}let mut line=丣::with_capacity(16*丁.len());for(index,三)in 丁.iter().enumerate(){if index>0{line.push(';');}line.push_str(&三.俓());}line}}pub
mod 丗{#[derive(Clone,Copy,Debug,PartialEq,Eq,Default)]pub enum 乊{伪,俢,侔,#[default]仲,}impl 乊{pub fn 乫(self)->u8{match self{乊::伪=>1,_=>3,}}pub fn 付(self)->bool{matches!(self,乊::侔|乊::仲)}pub
fn 仼(self)->bool{self==乊::仲}}}pub mod map{type C=usize;type 仐=i32;use std::cell::RefCell;use std::sync::Arc;use crate::丕::bits::{G,M,SIZE};pub const 之:u8=u8::MAX;pub
struct 乜{pub 丸:G,pub valid:G,pub 临:G,乄:Box<[u8]>,pub 且:[[[u8;4];M];2],}thread_local!{static 併:RefCell<Option<Arc<乜>>> =const{RefCell::new(None)};}impl
乜{pub fn 俇(丸:G,临:G)->Arc<乜>{併.with(|乸|{let mut 乸=乸.borrow_mut();if let Some(map)=乸.as_ref(){if map.丸==丸&&map.临==临{return Arc::clone(map);}}let map=Arc::new(乜::build(丸,临));*乸=Some(Arc::clone(&map));map})}fn
build(丸:G,临:G)->乜{let valid=!丸;const 侌:[[(仐,仐);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 且=[[[之;4];M];2];for(z,么)in 侌.iter().enumerate(){for
q in valid.iter(){let(x,y)=((q%SIZE)as 仐,(q/SIZE)as 仐);let mut count=0;for&(侤,侥)in 么{let(佋,佌)=(x+侤,y+侥);if(0..SIZE as 仐).contains(&佋)&&(0..SIZE as 仐).contains(&佌){let
next=佌 as C*SIZE+佋 as C;if valid.test(next){且[z][q][count]=next as u8;count+=1;}}}}}let mut 乄=vec![之;M*M].into_boxed_slice();let mut 伔=[0u8;M];for from
in valid.iter(){let row=&mut 乄[from*M..(from+1)*M];row[from]=0;伔[0]=from as u8;let(mut 伊,mut 伛)=(0,1);while 伊<伛{let q=伔[伊]as C;伊+=1;for&next in&且[0][q]{if
next!=之&&row[next as C]==之{row[next as C]=row[q]+1;伔[伛]=next;伛+=1;}}}}乜{丸,valid,临,乄,且,}}#[inline]pub fn 乄(&self,from:C,买:C)->u8{self.乄[from*M+买]}}}pub
mod l{type C=usize;type L=u8;type 丮=u32;type F=i64;type O<T>=Option<T>;use std::fmt;use std::sync::Arc;use crate::丕::bits::{G,M,SIZE};use crate::丕::三::K;use
crate::丕::丗::乊;use crate::丕::map::{乜,之};pub const 佾:丮=100;pub const 乥:F=15;const 侑:[F;3]=[1,4,20];pub const T:[C;2]=[0,M-1];#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub
enum 丠{乗(C),伡,}#[derive(Clone,Copy,PartialEq,Eq)]enum 乛{仸,伤,伥,}#[derive(Clone)]pub struct I{pub(crate)map:Arc<乜>,pub(crate)丗:乊,pub(crate)丑:[G;2],pub(crate)丂:[G;2],pub(crate)久:G,pub(crate)义:G,pub(crate)七:[G;2],pub(crate)level:[G;3],pub(crate)亄:G,pub(crate)亶:[丮;M],pub(crate)丅:[F;2],pub(crate)上:[F;2],pub
(crate)与:C,pub(crate)也:丮,pub(crate)享:丮,pub(crate)下:O<丠>,}fn 亃(丑:G,亥:C)->G{if!丑.test(亥){return G::Z;}let mut 仌=G::q(亥);loop{let next=(仌|仌.且())&丑;if next==仌{return
仌;}仌=next;}}impl I{pub fn 与(&self)->C{self.与}pub fn 也(&self)->丮{self.也}pub fn 下(&self)->O<丠>{self.下}pub fn 丗(&self)->乊{self.丗}pub fn 亪(&self,z:C)->F{let
j=self;let 七=j.七[z];j.丅[z]+10*F::from((七&j.level[0]).count())+20*F::from((七&j.level[1]).count())+30*F::from((七&j.level[2]).count())}pub fn 丅(&self,z:C)->F{self.丅[z]}pub
fn 上(&self,z:C)->F{self.上[z]}pub fn 伞(&self,z:C)->C{self.七[z].count()as C}pub fn map(&self)->&乜{&self.map}pub fn 丙(&self,z:C)->G{self.丑[z]}pub fn 丂(&self,z:C)->G{self.丂[z]}pub
fn 七(&self,z:C)->G{self.七[z]}pub fn 仏(&self,q:C)->O<(丮,L)>{let j=self;(j.七[0]|j.七[1]).test(q).then(||(j.亶[q],j.伏(q)))}pub fn 亅(&self)->G{self.七[self.与]&self.亄}pub
fn 丝(&self)->G{self.久}pub fn 乤(&self)->G{self.义}pub fn 井(&self,q:C)->O<L>{(q<M&&(self.七[0]|self.七[1]).test(q)).then(||self.伏(q))}fn 俎(&mut self,z:C){let
j=self;j.丂=[亃(j.丑[0],T[0]),亃(j.丑[1],T[1])];let 侠=j.七[z].两(j.丂[z]);j.伖(侠);j.伶();j.丅[z]+=j.上[z];if j.丅[z]<0{j.丅[z]=0;j.伖(j.七[z]);}j.亄|=j.七[z];}fn 伶(&mut
self){let j=self;for z in 0..2{let 丂=j.丂[z];let 七=丂&j.七[z];let mut 上=F::from(丂.count())+4*F::from((丂&j.久).count());for(level,俛)in 侑.iter().enumerate(){上-=俛*F::from((七&j.level[level]).count());}j.上[z]=上;}}pub
fn 亠(&mut self){let j=self;if j.下.is_some(){return;}if j.与==1{if j.也>=佾{j.下=Some(match j.亪(0).cmp(&j.亪(1)){std::cmp::Ordering::Greater=>丠::乗(0),std::cmp::Ordering::Less=>丠::乗(1),std::cmp::Ordering::Equal=>丠::伡,});return;}j.也+=1;}j.与=1-j.与;j.俎(j.与);}pub
fn 今(&mut self,丁:&[K]){let j=self;if j.下.is_some(){return;}let 伈=j.享;for&三 in 丁{if j.侣(三,伈){continue;}j.严(三);if j.下.is_some(){return;}}j.亠();}fn 侣(&self,三:K,伈:丮)->bool{let
j=self;match 三{K::仗=>true,K::亀{id,x,y}=>{j.些(x,y).is_none()||id>=伈||j.伿(j.与,id).is_none()}K::乨{level,x,y}=>{j.些(x,y).is_none()||level==0||level>j.丗.乫()}K::乾{x,y}|K::亍{x,y}=>{j.些(x,y).is_none()}}}#[inline]pub(crate)fn
些(&self,x:L,y:L)->O<C>{let(x,y)=(C::from(x),C::from(y));(x<SIZE&&y<SIZE&&!self.map.丸.test(y*SIZE+x)).then_some(y*SIZE+x)}pub(crate)fn 伿(&self,z:C,id:丮)->O<C>{self.七[z].iter().find(|&q|self.亶[q]==id)}#[inline]pub(crate)fn
伏(&self,q:C)->L{if self.level[0].test(q){1}else if self.level[1].test(q){2}else{3}}pub(crate)fn 侘(&self,乢:C)->G{let 乤=self.义&self.丑[乢];let 丂=self.丂[乢];乤|(丂&(丂&乤).且())}pub
fn 为(&self,z:C,level:L)->G{let j=self;debug_assert!((1..=3).contains(&level),"unit level {level} is not 1 to 3");let 乢=1-z;let 七=j.七[0]|j.七[1];let 侦=j.仄();let
俘=match level{1 if j.丗==乊::伪=>j.level[1]|j.level[2],1=>七,2=>j.level[1]|j.level[2],_=>G::Z,};let 为=侦|j.丑[乢].两(俘);if level<3{为.两(j.侘(乢))}else{为}}pub fn 亐(&self,z:C)->G{self.丂[z]|self.丂[z].且()}pub
fn 仄(&self)->G{let j=self;let 七=j.七[0]|j.七[1];let 侮=G::q(T[0])|G::q(T[1]);j.map.valid.两(七|j.久|j.义|侮)}pub fn 严(&mut self,三:K)->bool{let j=self;if j.下.is_some(){return
false;}let z=j.与;let 亏=match 三{K::仗=>return false,K::乨{level,x,y}=>j.俖(z,level,x,y),K::亀{id,x,y}=>j.侵(z,id,x,y),K::乾{x,y}=>j.build(z,true,x,y),K::亍{x,y}=>j.build(z,false,x,y),};let
Some(亏)=亏 else{return false;};match 亏{乛::仸=>{}乛::伤=>j.丂[z]=亃(j.丑[z],T[z]),乛::伥=>{j.丂=[亃(j.丑[0],T[0]),亃(j.丑[1],T[1])];}}j.伶();if!j.丑[0].test(T[0]){j.下=Some(丠::乗(1));}else
if!j.丑[1].test(T[1]){j.下=Some(丠::乗(0));}true}fn 伖(&mut self,丌:G){let j=self;j.七[0]=j.七[0].两(丌);j.七[1]=j.七[1].两(丌);for level in&mut j.level{*level=level.两(丌);}j.亄=j.亄.两(丌);}fn
作(&mut self,z:C,q:C)->乛{let j=self;j.伖(G::q(q));j.久.clear(q);j.义.clear(q);let 乢=1-z;if j.丑[乢].test(q){j.丑[乢].clear(q);j.丑[z].set(q);乛::伥}else if j.丑[z].test(q){乛::仸}else{j.丑[z].set(q);乛::伤}}fn
俖(&mut self,z:C,level:L,x:L,y:L)->O<乛>{let j=self;if level==0||level>j.丗.乫(){return None;}let q=j.些(x,y)?;let 万=10*F::from(level);if j.丅[z]<万||!j.亐(z).test(q)||!j.为(z,level).test(q){return
None;}let 亏=j.作(z,q);j.住(z,level,q,j.享);j.享=j.享.saturating_add(1);j.丅[z]-=万;Some(亏)}fn 住(&mut self,z:C,level:L,q:C,id:丮){self.七[z].set(q);self.level[C::from(level)-1].set(q);self.亶[q]=id;}fn
侵(&mut self,z:C,id:丮,x:L,y:L)->O<乛>{let j=self;let 丐=j.些(x,y)?;let from=j.伿(z,id)?;if!j.亄.test(from){return None;}let level=j.伏(from);let 东=j.为(z,level);let
乀=j.map.且[z][from];if 乀.contains(&(丐 as L))&&!东.test(丐){return None;}let mut 丈=j.map.乄(from,丐);let mut 仂=None;for&next in 乀.iter().take_while(|&&next|next!=之){let
next=C::from(next);let 乄=j.map.乄(next,丐);if 乄<丈&&东.test(next){丈=乄;仂=Some(next);}}let 仂=仂?;let 亏=j.作(z,仂);j.七[z].clear(from);j.level[C::from(level)-1].clear(from);j.亄.clear(from);j.住(z,level,仂,id);Some(亏)}fn
build(&mut self,z:C,久:bool,x:L,y:L)->O<乛>{let j=self;let 侗=if 久{j.丗.付()}else{j.丗.仼()};let q=j.些(x,y)?;let 侷=j.map.临.test(q);if!侗||j.丅[z]<乥||侷!=久||!j.丑[z].test(q)||!j.亐(z).test(q)||!j.仄().test(q){return
None;}if 久{j.久.set(q);}else{j.义.set(q);}j.丅[z]-=乥;Some(乛::仸)}}impl fmt::Debug for I{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{let j=self;f.debug_struct("State").field("league",&j.丗).field("to_act",&j.与).field("game_turn",&j.也).field("gold",&j.丅).field("income",&j.上).field("outcome",&j.下).
finish_non_exhaustive()}}}pub mod 乓{type C=usize;type F=i64;type 丣=String;use std::str::Lines;use crate::丕::bits::{G,M,SIZE};use crate::丕::丗::乊;use crate::丕::map::乜;use
crate::丕::l::{I,T};const 佺:F=1000000000;const 仴:F=0;const 侀:F=1;const 侏:F=2;impl I{pub fn read(亦:&str,turn:&str,丗:乊,也:u32,)->Result<I,丣>{let mut 临=Vec::new();let
mut lines=亦.lines();let count=伆(&mut lines,"mine spot count")?;for _ in 0..count{let[x,y]=乹(京(&mut lines,"a mine spot")?,"a mine spot")?;临.push((仛(x)?,仛(y)?));}伻(lines,"the initial input")?;let
mut lines=turn.lines();let mut 伋=[0i64;4];for(value,what)in 伋.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[number]=乹(京(&mut
lines,what)?,what)?;if number.abs()>佺{return Err(format!("{what}: {number} is out of range"));}*value=number;}let mut 丸=G::Z;let mut 丌=[G::Z;2];let mut
丂=[G::Z;2];for y in 0..SIZE{let row=京(&mut lines,"a map row")?.trim();if row.len()!=SIZE{return Err(format!("map row {y} is not {SIZE} characters: {row:?}"));}for(x,仍)in
row.bytes().enumerate(){let q=y*SIZE+x;match 仍{b'#'=>丸.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let 乁=C::from(仍==b'X'||仍==b'x');丌[乁].set(q);if 仍.is_ascii_uppercase(){丂[乁].set(q);}}_=>{return
Err(format!("map row {y} has {:?} at {x}",仍 as char))}}}}let 侚=伆(&mut lines,"building count")?;let mut 乐=Vec::new();for _ in 0..侚{let 乹=乹(京(&mut lines,"a building")?,"a building")?;乐.push(乹);}let
伞=伆(&mut lines,"unit count")?;let mut 七=Vec::new();for _ in 0..伞{let 乹=乹(京(&mut lines,"a unit")?,"a unit")?;七.push(乹);}伻(lines,"the turn's input")?;let
低=乐.iter().find(|伱|伱[0]==0&&伱[1]==仴).ok_or("the bot's own HQ is not among the buildings")?;let hq_cell=伂(低[2],低[3],丸)?;let z=T.iter().position(|&亥|亥==hq_cell).ok_or_else(||format!("the bot's HQ is at cell {hq_cell}, not in a corner"))?;let
mut 佘=G::Z;for(x,y)in 临{if 丸.test(y*SIZE+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佘.set(y*SIZE+x);}let 任=|乁:C|if 乁==0{z}else{1-z};let
mut l=I{map:乜::俇(丸,佘),丗,丑:[G::Z;2],丂:[G::Z;2],久:G::Z,义:G::Z,七:[G::Z;2],level:[G::Z;3],亄:G::Z,亶:[0;M],丅:[0;2],上:[0;2],与:z,也,享:1,下:None,};for 乁 in 0..2{let
owner=任(乁);l.丑[owner]=丌[乁];l.丂[owner]=丂[乁];l.丅[owner]=伋[2*乁];l.上[owner]=伋[2*乁+1];}let mut 位=G::Z;for[owner,kind,x,y]in 乐{let 乁=佒(owner)?;let q=伂(x,y,丸)?;if!丌[乁].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 位.test(q){return Err(format!("two buildings at ({x}, {y})"));}位.set(q);match kind{仴
if q==T[任(乁)]=>{}仴=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侀=>l.久.set(q),侏=>l.义.set(q),_=>return Err(format!("unknown building type {kind}")),}}let
mut 串=G::Z;let mut largest_id=0u32;for[owner,id,level,x,y]in 七{let 乁=佒(owner)?;let q=伂(x,y,丸)?;if!丌[乁].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
串.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&level){return Err(format!("unit level {level} is not 1, 2 or 3"));}let id=u32::try_from(id).map_err(|_|format!("unit id {id} is out of range"))?;if
串.iter().any(|丞|l.亶[丞]==id){return Err(format!("two units have id {id}"));}串.set(q);l.七[任(乁)].set(q);l.level[level as C-1].set(q);l.亶[q]=id;largest_id=largest_id.max(id);}if!串.is_empty(){l.享=largest_id.checked_add(1).ok_or_else(||format!("unit id {largest_id} is out of range"))?;}l.亄=l.七[z];Ok(l)}}fn
京<'a>(lines:&mut Lines<'a>,what:&str)->Result<&'a str,丣>{lines.next().ok_or_else(||format!("the text ends before {what}"))}fn 伻(mut lines:Lines<'_>,what:&str)->Result<(),丣>{match
lines.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {what}: {line:?}")),None=>Ok(()),}}fn 乹<const N:C>(line:&str,what:&str)->Result<[F;N],丣>{let
mut 仮=line.split_whitespace();let mut 佣=[0i64;N];for value in&mut 佣{let word=仮.next().ok_or_else(||format!("{what}: expected {N} numbers in {line:?}"))?;*value=word.parse().map_err(|_|format!("{what}: {word:?} is not an integer"))?;}if
仮.next().is_some(){return Err(format!("{what}: expected {N} numbers in {line:?}"));}Ok(佣)}fn 伆(lines:&mut Lines<'_>,what:&str)->Result<C,丣>{let[count]=乹(京(lines,what)?,what)?;C::try_from(count).ok().filter(|&count|count<=M).ok_or_else(||format!("{what}: {count} is out of range"))}fn
仛(value:F)->Result<C,丣>{C::try_from(value).ok().filter(|&value|value<SIZE).ok_or_else(||format!("coordinate {value} is off the map"))}fn 伂(x:F,y:F,丸:G)->Result<C,丣>{let
q=仛(y)?*SIZE+仛(x)?;if 丸.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佒(owner:F)->Result<C,丣>{match owner{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {owner} is not 0 or 1")),}}}pub
use bits::{G,M,SIZE};pub use 三::{份,K};pub use 丗::乊;pub use l::{丠,I,T};}mod 丨{pub mod 乙{type C=usize;type L=u8;type F=i64;use crate::丕::l::乥;use crate::丕::{G,K,I,M,T,SIZE};pub
const 佱:F=15;pub const 侁:F=6;const 伨:F=10;const 侈:L=3;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct Q{pub open:F,pub 仚:F,pub 伹:L,pub 伌:F,pub 佄:F,pub
佇:F,pub 何:L,pub 伸:F,pub 伷:F,pub 伬:F,pub 亣:F,pub 亳:F,pub 丷:F,pub 产:F,pub 亊:F,}impl Q{pub const 仺:Q=Q{open:3,仚:2,伹:12,伌:4,佄:佱,佇:侁,何:侈,伸:2,伷:2,伬:0,亣:0,亳:0,丷:0,产:0,亊:0,};pub
const fn 俠(self,亣:F,亳:F,丷:F)->Q{Q{亣,亳,丷,..self}}pub const fn 俟(self,产:F,亊:F)->Q{Q{产,亊,..self}}}fn 丹(q:C)->(L,L){((q%SIZE)as L,(q/SIZE)as L)}pub fn 仃(l:&I,from:G)->[L;M]{let
mut 乎=[L::MAX;M];let valid=l.map().valid;let mut 丶=from&valid;let mut 串=丶;let mut d=0u8;while!丶.is_empty(){for q in 丶.iter(){乎[q]=d;}let next=(丶.且()&valid).两(串);串|=next;丶=next;d=d.saturating_add(1);}乎}pub
fn 侩(l:&I)->L{let 乎=仃(l,l.丙(0));l.丙(1).iter().map(|q|乎[q]).min().unwrap_or(L::MAX)}struct 亂<'a>{乆:&'a Q,一:C,theirs:[L;M],亵:[L;M],free:G,乻:G,ours:G,临:G,}impl
亂<'_>{fn new<'a>(l:&I,乆:&'a Q)->亂<'a>{let 一=l.与();let 久=仃(l,l.丙(一));let theirs=仃(l,l.丙(1-一));let 亵=仃(l,G::q(T[1-一]));let free=l.map().valid.两(l.丙(0)|l.丙(1));let
mut 乻=G::Z;for q in free.iter(){if 久[q]<=theirs[q]{乻.set(q);}}亂{乆,一,theirs,亵,free,乻,ours:l.丙(一),临:l.map().临,}}fn value(&self,q:C)->F{let j=self;let p=j.乆;let
open=F::from((G::q(q).且()&j.free).count());let 伳=p.伹;let 仚=F::from(伳)-F::from(j.theirs[q].min(伳));let 亥=if p.伌>0{F::from(j.亵[q].min(30))/p.伌}else{0};let
丑=F::from((G::q(q).且()&j.ours).count());let line=if 丑<=1{p.亳}else{0};let 丷=if j.临.test(q){p.丷}else{0};let 俏=if open==0{p.产}else{0};let 亊=p.亊*(22-F::from(j.亵[q].min(22)));p.open*open+p.仚*仚-亥+p.亣*丑-line+丷-俏+亊}fn
丈(&self,丌:G)->Option<C>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伃(&mut self,q:C){self.free.clear(q);self.乻.clear(q);self.ours.set(q);}}fn
乇(l:&mut I,三:K,丆:&mut Vec<K>)->bool{let 伫=l.严(三);if 伫{丆.push(三);}伫}fn 丏(l:&mut I,from:C,买:C,丆:&mut Vec<K>)->bool{let Some((id,_))=l.仏(from)else{return
false;};let(x,y)=丹(买);乇(l,K::亀{id,x,y},丆)}pub fn 俙(h:F,k:F)->F{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伨}pub fn 优(l:&I,乆:&Q)->Vec<K>{伝(l,乆,丯::于)}#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]pub
enum 丯{于,乿{丝:L,七:L},}pub fn 伝(l:&I,乆:&Q,乘:丯)->Vec<K>{let mut 丬=l.clone();let mut turn=亂::new(l,乆);let mut 丆=Vec::new();乡(&mut 丬,&mut turn,&mut 丆);match
乘{丯::于=>便(&mut 丬,&mut turn,&mut 丆),丯::乿{丝,七}=>{侨(&mut 丬,&mut turn,丝,七,&mut 丆)}}丆}fn 乡(l:&mut I,turn:&mut 亂<'_>,丆:&mut Vec<K>){let 一=turn.一;let mut 仟:Vec<C>
=Vec::new();loop{let 东=l.为(一,1);let mut 亰:Option<(u32,C)> =None;for from in l.亅().iter(){if 仟.contains(&from){continue;}let 乚=(G::q(from).且()&turn.free&东).count();if
乚==0{仟.push(from);}else if 亰.is_none_or(|(n,_)|乚<n){亰=Some((乚,from));}}let Some((_,from))=亰 else{break;};match turn.丈(G::q(from).且()&turn.free&东){Some(买)if
丏(l,from,买,丆)=>turn.伃(买),_=>仟.push(from),}}for from in 仟{if!l.亅().test(from){continue;}let 丐=[turn.乻,turn.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().乄(from,q),q))});if
let Some(丐)=丐{丏(l,from,丐,丆);}}}fn 便(l:&mut I,turn:&mut 亂<'_>,丆:&mut Vec<K>){let 一=turn.一;let p=turn.乆;let h=(p.佄-F::from(l.也())).max(p.佇);let 俒=(F::from(侩(l))-p.伸).max(0)/p.伷.max(1)+p.伬;loop{let
丅=l.丅(一);let 七=l.伞(一)as F;let 促=F::from(turn.乻.count());let q=turn.丈(l.亐(一)&turn.free&l.为(一,1));let 佢=俙(h,(促/(七+1)).min(俒));let 伲=q.is_some()&&丅>=伨&&佢>0;let
佈=4*h-乥;let 丷=佉(l,turn);let 侜=丷.is_some()&&丅>=乥&&佈>0;if 侜&&(!伲||佈>=佢){let(x,y)=丹(丷.unwrap_or_default());if 乇(l,K::乾{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|伲)else{break;};let(x,y)=丹(q);if!乇(l,K::乨{level:1,x,y},丆){break;}turn.伃(q);}}fn
佉(l:&I,turn:&亂<'_>)->Option<C>{let 一=turn.一;(l.map().临&l.丂(一)&l.仄()).iter().filter(|&丷|turn.theirs[丷]>turn.乆.何).min_by_key(|&丷|(l.map().乄(T[一],丷),丷))}fn
侨(l:&mut I,turn:&mut 亂<'_>,丝:L,七:L,丆:&mut Vec<K>,){let 一=turn.一;for _ in 0..丝{let Some(丷)=佉(l,turn)else{break;};let(x,y)=丹(丷);if!乇(l,K::乾{x,y},丆){break;}}for
_ in 0..七{let Some(q)=turn.丈(l.亐(一)&turn.free&l.为(一,1))else{break;};let(x,y)=丹(q);if!乇(l,K::乨{level:1,x,y},丆){break;}turn.伃(q);}}}pub mod 亩{use crate::丕::I;use
crate::习::chain::乱;pub fn 互(l:&I)->bool{if l.下().is_some(){return false;}let 一=l.与();let 丅=l.丅(一);let 丼=乱::new(l,一,true);l.丙(1-一).iter().any(|q|丼.万[q]<=丅)}}pub
mod 丄{type C=usize;type Y=u64;type J<T>=Vec<T>;use std::collections::HashMap;use std::time::Instant;use crate::丕::{K,I};use crate::丨::乙::{self,丯,Q};use
crate::丨::亩;use crate::丨::众::{self,亗};use crate::丨::不::{self,乌};#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]pub struct P{pub 乏:u8,pub 乘:丯,}const 仑:u8=u8::MAX;pub
const 仰:[丯;8]=[丯::于,丯::乿{丝:0,七:0},丯::乿{丝:0,七:1},丯::乿{丝:0,七:2},丯::乿{丝:0,七:仑,},丯::乿{丝:1,七:0},丯::乿{丝:1,七:1},丯::乿{丝:1,七:仑,},];pub struct 亹<'a>{pub 专:&'a[Q],pub
主:Q,pub rest:P,pub 乑:Option<Instant>,pub 丳:乌,}impl 亹<'_>{fn 云(&self)->bool{self.乑.is_some_and(|t|Instant::now()>=t)}fn 乆(&self,丏:P)->&Q{&self.专[C::from(丏.乏)%self.专.len()]}pub
fn 丁(&self,l:&I,丏:P)->J<K>{乙::伝(l,self.乆(丏),丏.乘)}pub fn 乇(&self,l:&I,丄:&[P])->I{众::乇(l,&self.主,|i,s|{self.丁(s,丄.get(i).copied().unwrap_or(self.rest))})}pub
fn value(&self,l:&I,丄:&[P],丳:乌)->i64{不::不(&self.乇(l,丄),l.与(),丳)}fn finish(&self,丩:J<(i64,J<P>)>)->J<P>{let mut 丈:Option<(i64,J<P>)> =None;for(value,丄)in
丩{if 丈.as_ref().is_none_or(|(乔,_)|value>*乔){丈=Some((value,丄));}}丈.map(|(_,丄)|丄).unwrap_or_default()}fn 么(&self,l:&I)->J<(P,J<K>)>{let mut 丆:J<(P,J<K>)>
=J::new();let mut 乚:J<P> =仰.iter().map(|&乘|P{乏:0,乘}).collect();for 乏 in 1..self.专.len().min(C::from(u8::MAX)){乚.push(P{乏:乏 as u8,乘:丯::于,});}for 丏 in 乚{let
丁=self.丁(l,丏);if!丆.iter().any(|(_,c)|*c==丁){丆.push((丏,丁));}}丆}}fn advance(l:&I,丁:&[K],主:&Q)->(I,bool){let mut next=l.clone();next.今(丁);if next.下().is_some()||亩::互(&next){return(next,true);}let
乭=乙::优(&next,主);next.今(&乭);let 乬=next.下().is_some()||亩::互(&next)||next.也()>亗;(next,乬)}pub fn 侾(l:&I,丄:&[P],丰:&亹<'_>)->J<Y>{let 一=l.与();let mut 仆=vec![予(l,一)];let
mut 丘=l.clone();for turn in 0..亗 as C{let 丏=丄.get(turn).copied().unwrap_or(丰.rest);let 丁=丰.丁(&丘,丏);let(next,乬)=advance(&丘,&丁,&丰.主);if 乬{break;}丘=next;仆.push(予(&丘,一));}仆}pub
fn 予(l:&I,一:C)->Y{let mut hash:Y=0xcbf29ce484222325;let mut 休=|word:Y|hash=(hash^word).wrapping_mul(0x0100000001b3);for(俐,bits)in[l.丙(一),l.七(一),l.丝()].into_iter().enumerate(){for
q in bits.iter(){休(((俐 as Y)<<8)|q as Y);}休(0xff);}休(l.丅(一)as Y);hash}pub fn 仿(l:&I,丰:&亹<'_>,乂:C)->J<P>{let 一=l.与();struct 仕{l:I,丄:J<P>,}let mut 丶=vec![仕{l:l.clone(),丄:J::new(),}];let
mut 丫:J<(i64,J<P>)> =vec![(丰.value(l,&[],丰.丳),J::new())];'search:while!丶.is_empty(){let mut 丵:J<(i64,仕)> =J::new();let mut 串:HashMap<Y,C> =HashMap::new();for
了 in&丶{for(丏,丁)in 丰.么(&了.l){if 丰.云(){if let Some((_,亜))=丵.iter().max_by_key(|(v,_)|*v){丫.push((丰.value(l,&亜.丄,丰.丳),亜.丄.clone(),));}break 'search;}let(next,乬)=advance(&了.l,&丁,&丰.主);let
mut 丄=了.丄.clone();丄.push(丏);let value=不::不(&next,一,丰.丳);if 乬{丫.push((value,丄));continue;}let key=予(&next,一);match 串.get(&key){Some(&両)if 丵[両].0>=value=>{}Some(&両)=>丵[両]=(value,仕{l:next,丄}),None=>{串.insert(key,丵.len());丵.push((value,仕{l:next,丄}));}}}}丵.sort_by(|a,b|b.0
.cmp(&a.0));丵.truncate(乂);丶=丵.into_iter().map(|(_,了)|了).collect();}丰.finish(丫)}}pub mod 伓{type C=usize;type J<T>=Vec<T>;use std::time::Instant;use crate::丕::{K,I};use
crate::丨::乙::{丯,Q};use crate::丨::丄::{self,予,亹,P};use crate::丨::众::{伵,亗};use crate::丨::不::乌;use crate::丨::乽::{self,亁};#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub
enum 乖{佧{乂:C,仪:C},佭,他{丟:C,乂:C,乒:亁,},}pub struct 九{pub 乺:乖,pub 专:J<Q>,pub 主:Q,丄:J<P>,expected:J<u64>,亱:C,pub 丩:u32,乬:bool,}impl 九{pub fn new(乺:乖,专:J<Q>,主:Q)->九{九{乺,专,主,丄:J::new(),expected:J::new(),亱:0,丩:0,乬:false,}}pub
fn 侼(&mut self,l:&I)->bool{let j=self;let 乽=matches!(j.乺,乖::他{..});let 侽=!乽&&j.丩>0&&j.亱>=j.expected.len();if!j.乬&&(侽||crate::丨::亩::互(l)){j.乬=true;}!j.乬}pub
fn turn(&mut self,l:&I,乑:Option<Instant>)->J<K>{let j=self;let 丰=亹{专:&j.专,主:j.主,rest:P{乏:0,乘:丯::于,},乑,丳:乌::DEFAULT,};let 一=l.与();let 侸=j.expected.get(j.亱)==Some(&予(l,一));if!侸{let
first=j.丩==0;j.丄=match j.乺{乖::佧{乂,仪}=>{丄::仿(l,&丰,if first{乂}else{仪})}乖::佭=>{let 专=&j.专;let 乏=伵(l,专,&j.主,丰.丳,乑);let 丏=P{乏:乏 as u8,乘:丯::于,};vec![丏;亗 as C]}乖::他{丟,乂,乒,}=>{let
乒=&乒;let rest=乽::侇;let mut 么=乽::丄(l,&j.专,rest,丟,乂,乑,乒,);么.resize(丟,rest);么}};j.expected=match j.乺{乖::他{..}=>俍(l,&j.专,&j.丄),_=>丄::侾(l,&j.丄,&丰),};j.亱=0;j.丩+=1;}let
丏=j.丄.get(j.亱).copied().unwrap_or(丰.rest);j.亱+=1;丰.丁(l,丏)}}fn 俍(l:&I,专:&[Q],丄:&[P])->J<u64>{let 一=l.与();let mut 仆=J::with_capacity(丄.len());let mut 丘=l.clone();for&丏
in 丄{if 丘.下().is_some(){break;}仆.push(予(&丘,一));丘=乽::advance(&丘,&乽::丁(&丘,专,丏));}仆}}pub mod 众{use std::time::Instant;use crate::丕::{K,I};use crate::丨::乙::{self,Q};use
crate::丨::亩;use crate::丨::不::{self,乌};pub const 亗:u32=20;pub fn 乇(l:&I,主:&Q,mut ours:impl FnMut(usize,&I)->Vec<K>,)->I{let 一=l.与();let mut 丘=l.clone();let
mut 丟=0;loop{if 丘.下().is_some()||丘.也()>亗{return 丘;}if(丟>0||丘.与()!=一)&&亩::互(&丘){return 丘;}let 丁=if 丘.与()==一{丟+=1;ours(丟-1,&丘)}else{乙::优(&丘,主)};丘.今(&丁);}}pub
fn 俜(l:&I,乆:&Q,主:&Q,丳:乌)->i64{let 一=l.与();let end=乇(l,主,|_,s|乙::优(s,乆));不::不(&end,一,丳)}pub fn 伵(l:&I,专:&[Q],主:&Q,丳:乌,乑:Option<Instant>,)->usize{let mut
丈:Option<(i64,usize)> =None;for(index,乆)in 专.iter().enumerate(){if 丈.is_some()&&乑.is_some_and(|t|Instant::now()>=t){break;}let value=俜(l,乆,主,丳);if 丈.is_none_or(|(乔,_)|value>乔){丈=Some((value,index));}}丈.map_or(0,|(_,index)|index)}}pub
mod 不{type C=usize;type F=i64;use crate::丕::{G,丠,I,M,T};use crate::习::chain::乱;use crate::习::丒::亟;pub const 佨:F=1;pub const 侐:F=3;pub const 件:F=1000;const
亸:C=5;const 佬:C=8;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct 乌{pub 丟:F,}impl 乌{pub const DEFAULT:乌=乌{丟:侐};}#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub
struct 亚{pub 事:F,pub 丅:F,pub 上:F,pub 仝:F,pub 仁:F,pub 仇:F,}impl 亚{pub fn value(&self)->F{let j=self;j.事+j.丅+j.上-j.仝-j.仁-j.仇}}pub fn 俚(l:&I,z:C)->F{(l.七(z)&l.丂(z)).iter().filter_map(|q|l.井(q)).map(|level|10*F::from(level)).sum()}pub
fn 不(l:&I,z:C,丳:乌)->F{佝(l,z,丳).value()-佝(l,1-z,丳).value()}pub fn 佝(l:&I,z:C,丳:乌)->亚{match l.下(){Some(丠::乗(传))if 传!=z=>{return 亚{仇:件,..亚::default()}}Some(_)=>return
亚::default(),None=>{}}let 丅=l.丅(z);let 上=l.上(z);let 令=亴(l,z);亚{事:佨*佤(l,z)/2,丅,上:丳.丟*上,仝:if 令{0}else{仝(l,z,丳)},仁:if 丅+上<0{俚(l,z)}else{0},仇:if 令{件}else{0},}}fn
么(l:&I,from:G)->[u8;M]{let valid=l.map().valid;let mut 乎=[u8::MAX;M];let mut 丶=from&valid;let mut 串=丶;let mut d=0u8;while!丶.is_empty(){for q in 丶.iter(){乎[q]=d;}let
next=(丶.且()&valid).两(串);串|=next;丶=next;d=d.saturating_add(1);}乎}pub fn 佤(l:&I,z:C)->F{let ours=么(l,l.丂(z));let theirs=么(l,l.丂(1-z));l.map().valid.iter().map(|q|match
ours[q].cmp(&theirs[q]){_ if ours[q]==u8::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}fn 侧(l:&I,丛:C)->F{l.丅(丛)+if
l.与()==丛{0}else{l.上(丛)}}pub fn 亴(l:&I,z:C)->bool{let 丛=1-z;let 与=l.与()==丛;乱::new(l,丛,与).万[T[z]]<=侧(l,丛)}pub fn 仝(l:&I,伟:C,丳:乌)->F{if l.下().is_some(){return
0;}let mut next=l.clone();if next.与()==伟{next.亠();if next.下().is_some(){return 0;}}let 丛=1-伟;let 丝=|s:&I|F::from((s.丝()&s.丂(伟)).count());let 侴=丝(&next);let
mut 二=0;for 丒 in 亟(&next,亸,next.丅(丛),佬){let mut 个=next.clone();for&三 in&丒.丁{个.严(三);}let 仇=if 个.下()==Some(丠::乗(丛)){件}else{丳.丟*(丒.丌+4*(侴-丝(&个)))+丒.七};二=二.max(仇);}二}}pub
mod 乽{type C=usize;type F=i64;type 举=Instant;type J<T>=Vec<T>;use std::collections::HashMap;use std::time::Instant;use crate::丕::{G,K,I,M,T};use crate::丨::乙::{self,Q};use
crate::丨::丄::{予,P,仰};pub const 侉:[Q;27]=专();pub const 侇:P=P{乏:16,乘:crate::丨::乙::丯::于,};const fn 专()->[Q;27]{let 亣=[0,2,4];let 亳=[0,3,6];let 亊=[0,1,2];let
mut 丆=[Q::仺;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let 产=if g>0||t>0{4}else{0};丆[i]=Q::仺.俠(亣[g],亳[t],0).俟(产,亊[w]);i+=1;}丆}#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub
struct 仹{pub 丌:F,pub 上:F,pub 丅:F,pub 乷:F,pub 丒:F,pub 从:F,pub 丼:F,pub 事:F,}#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct 亁{pub 丌:F,pub 上:F,pub 丅:F,pub
丒:F,pub 乷:F,pub 从:F,pub 丼:F,pub 事:F,}pub fn 俅(l:&I,z:C)->[F;M]{struct 人{丂:G,乃:[u16;M],丱:[u16;M],size:[F;M],丒:[F;M],乯:u16,}fn 亖(d:&mut 人,v:C){d.乯+=1;d.乃[v]=d.乯;d.丱[v]=d.乯;d.size[v]=1;for
w in(G::q(v).且()&d.丂).iter(){if d.乃[w]==0{亖(d,w);d.size[v]+=d.size[w];d.丱[v]=d.丱[v].min(d.丱[w]);if d.丱[w]>=d.乃[v]{d.丒[v]+=d.size[w];}}else{d.丱[v]=d.丱[v].min(d.乃[w]);}}}let
mut d=人{丂:l.丂(z),乃:[0;M],丱:[0;M],size:[0;M],丒:[0;M],乯:0,};let 乮=T[z];if d.丂.test(乮){亖(&mut d,乮);}d.丒[乮]=0;d.丒}pub fn 伙(l:&I,z:C)->仹{let 丂=l.丂(z);let 丒=俅(l,z);let
亵=乙::仃(l,G::q(T[1-z]));let mut 丆=仹{丌:F::from(丂.count()),上:l.上(z),丅:l.丅(z),丼:F::MAX,事:crate::丨::不::佤(l,z)/2,..仹::default()};for q in 丂.iter(){if 丒[q]>0{丆.乷+=1;丆.丒=丆.丒.max(丒[q]);}if
q!=T[z]&&(G::q(q).且()&丂).count()<=1{丆.从+=1;}丆.丼=丆.丼.min(F::from(亵[q]));}丆}pub fn value(l:&I,z:C,乒:&亁)->F{let s=伙(l,z);let o=乒;o.丌*s.丌+o.上*s.上+o.丅*s.丅-o.丒*s.丒-o.乷*s.乷-o.从*s.从-o.丼*s.丼.min(30)+o.事*s.事}pub
fn advance(l:&I,丁:&[K])->I{let mut next=l.clone();next.今(丁);if next.下().is_none(){next.今(&[]);}next}pub fn 丁(l:&I,专:&[Q],丏:P)->J<K>{乙::伝(l,&专[C::from(丏.乏)%专.len()],丏.乘,)}pub
fn 乇(l:&I,专:&[Q],丄:&[P],rest:P,丟:C,)->I{let mut 丘=l.clone();for turn in 0..丟{if 丘.下().is_some(){break;}let 丏=丄.get(turn).copied().unwrap_or(rest);丘=advance(&丘,&丁(&丘,专,丏));}丘}fn
么(l:&I,专:&[Q])->J<(P,J<K>)>{let mut 丆:J<(P,J<K>)> =J::new();for 乏 in 0..专.len().min(C::from(u8::MAX)){for&乘 in&仰{let 丏=P{乏:乏 as u8,乘,};let c=丁(l,专,丏);if!丆.iter().any(|(_,丞)|*丞==c){丆.push((丏,c));}}}丆}pub
fn 仿(l:&I,专:&[Q],rest:P,丟:C,乂:C,乑:Option<举>,乒:&亁,)->J<P>{let 一=l.与();let 云=||乑.is_some_and(|t|举::now()>=t);let 介=|丄:&[P]|value(&乇(l,专,丄,rest,丟),一,乒);let
mut 丈:(F,J<P>)=(介(&[]),J::new());let mut 丶:J<(I,J<P>)> =vec![(l.clone(),J::new())];for 侢 in 0..丟{let mut 丵:J<(F,I,J<P>)> =J::new();let mut 串:HashMap<u64,C>
=HashMap::new();for(了,丄)in&丶{if 云(){break;}for(丏,c)in 么(了,专){let next=advance(了,&c);let mut 亜=丄.clone();亜.push(丏);let v=value(&next,一,乒);let key=予(&next,一);match
串.get(&key){Some(&両)if 丵[両].0>=v=>{}Some(&両)=>丵[両]=(v,next,亜),None=>{串.insert(key,丵.len());丵.push((v,next,亜));}}}}if 丵.is_empty(){break;}丵.sort_by(|a,b|b.0
.cmp(&a.0));丵.truncate(乂);let 乸=侢+1==丟||云();for(i,(_,_,丄))in 丵.iter().take(if 乸{乂}else{1}).enumerate(){if i>0&&云(){break;}let v=介(丄);if v>丈.0{丈=(v,丄.clone());}}if
乸{break;}丶=丵.into_iter().map(|(_,s,p)|(s,p)).collect();}丈.1}pub fn 丄(l:&I,专:&[Q],rest:P,丟:C,侳:C,乑:Option<举>,乒:&亁,)->J<P>{let 一=l.与();let 介=|丄:&[P]|value(&乇(l,专,丄,rest,丟),一,乒);let
mut 丈:(F,J<P>)=(介(&[]),J::new());let mut 乂=2;loop{let 丧=仿(l,专,rest,丟,乂,乑,乒);let v=介(&丧);if v>丈.0{丈=(v,丧);}let 云=乑.is_some_and(|t|举::now()>=t);if 云||(乑.is_none()&&乂>=侳){break;}乂*=2;}丈.1}}}mod
但{pub mod io{type C=usize;type 丣=String;use std::io::{BufRead,Write};use std::time::Instant;const 佰:C=4+12;pub fn 丟(仠:impl BufRead,mut 仨:impl Write,mut
answer:impl FnMut(&str,&str,u32,Instant)->丣,)->Result<(),丣>{let mut lines=Lines{佑:仠};let Some(first)=lines.line()?else{return Ok(());};let mut 亦=format!("{first}\n");lines.take(count(&first,"mine spot count")?,&mut
亦)?;let mut turn=0;loop{let Some(first)=lines.line()?else{return Ok(());};let 五=Instant::now();turn+=1;let mut 乓=format!("{first}\n");lines.take(佰-1,&mut
乓)?;for what in["building count","unit count"]{let line=lines.体()?;let n=count(&line,what)?;乓.push_str(&line);乓.push('\n');lines.take(n,&mut 乓)?;}let answer=answer(&亦,&乓,turn,五);writeln!(仨,"{answer}").and_then(|()|仨.flush()).map_err(|err|format!("turn {turn}: cannot write the answer: {err}"))?;}}struct
Lines<R>{佑:R,}impl<R:BufRead>Lines<R>{fn line(&mut self)->Result<Option<丣>,丣>{let mut line=丣::new();match self.佑.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(err)=>Err(format!("cannot read input: {err}")),}}fn
体(&mut self)->Result<丣,丣>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:C,乓:&mut String)->Result<(),丣>{for
_ in 0..n{乓.push_str(&self.体()?);乓.push('\n');}Ok(())}}fn count(line:&str,what:&str)->Result<C,丣>{match line.trim().parse::<C>(){Ok(n)if n<=佹=>Ok(n),_=>Err(format!("bad {what} {line:?}")),}}const
佹:C=144;}mod 丄{type C=usize;type F=i64;type O<T>=Option<T>;use crate::丕::map::之;use crate::丕::l::乥;use crate::丕::{G,K,丠,I,M,T,SIZE};const 侍:F=3;const 佳:F=2;const
佽:C=16;pub fn 丄(l:&I,丆:&mut Vec<K>){let z=l.与();let mut 乺=乖{s:l.clone(),z,丛:1-z,丐:T[1-z],仞:T[z],turn:l.也(),丆,};乺.run();}struct 乖<'a>{s:I,z:C,丛:C,丐:C,仞:C,turn:u32,丆:&'a
mut Vec<K>,}fn 丹(q:C)->(u8,u8){((q%SIZE)as u8,(q/SIZE)as u8)}fn 伒(id:u32,q:C)->K{let(x,y)=丹(q);K::亀{id,x,y}}fn 你(level:u8,q:C)->K{let(x,y)=丹(q);K::乨{level,x,y}}impl
乖<'_>{fn run(&mut self){let j=self;if j.仢(){return;}j.乡();if j.仢(){return;}j.乐();j.俗();}fn 乎(&self,from:C,买:C)->F{F::from(self.s.map().乄(from,买))}fn 严(&mut
self,三:K)->bool{let 丫=self.s.严(三);if 丫{self.丆.push(三);}丫}fn 亷(&self,业:&I)->bool{业.下()==Some(丠::乗(self.z))}fn 仢(&mut self)->bool{let j=self;if j.s.下().is_some(){return
false;}for q in j.s.亅().iter(){if j.乎(q,j.丐)!=1{continue;}let Some((id,_))=j.s.仏(q)else{continue;};let 三=伒(id,j.丐);let mut 业=j.s.clone();if 业.严(三)&&j.亷(&业){return
j.严(三);}}let Some(chain)=j.侞()else{return false;};let mut 业=j.s.clone();if!chain.iter().all(|&三|业.严(三))||!j.亷(&业){return false;}for 三 in chain{j.严(三);}true}fn
侞(&self)->O<Vec<K>>{let j=self;const 伢:F=F::MAX;let max=j.s.丗().乫();let 东:Vec<G> =(1..=max).map(|level|j.s.为(j.z,level)).collect();let entry=|q:C|{东.iter().position(|bits|bits.test(q)).map(|index|(index
as u8+1,10*(index as F+1)))};let 丂=j.s.丂(j.z);let mut 万=[伢;M];let mut 並=[C::MAX;M];let mut 丫=[false;M];for q in 丂.iter(){万[q]=0;}loop{let mut next:O<C>
=None;for q in 0..M{if!丫[q]&&万[q]<伢&&next.is_none_or(|n|万[q]<万[n]){next=Some(q);}}let Some(q)=next else{break;};丫[q]=true;if q==j.丐{break;}for&乀 in&j.s.map().且[0][q]{if
乀==之{break;}let 乀=C::from(乀);if 丫[乀]||丂.test(乀){continue;}let Some((_,亇))=entry(乀)else{continue;};if 万[q]+亇<万[乀]{万[乀]=万[q]+亇;並[乀]=q;}}}if 万[j.丐]>j.s.丅(j.z){return
None;}let mut chain=Vec::new();let mut q=j.丐;while!丂.test(q){let(level,_)=entry(q)?;chain.push(你(level,q));q=並[q];}chain.reverse();Some(chain)}fn 乪(&self,个:&I)->F{let(一,争)=(self.z,self.丛);let
並=&self.s;let count=|bits:G|F::from(bits.count());let 丑=count(个.丙(一))-count(並.丙(一));let 仭=count(並.丙(争))-count(个.丙(争));let 令=count(並.丂(争))-count(个.丂(争));let
仡=並.亪(争)-个.亪(争);let 丝=count(並.丝()&並.丙(争))-count(个.丝()&个.丙(争));12*丑+20*仭+8*(令-仭).max(0)+5*仡+25*丝}fn 余(&self,q:C)->F{let j=self;let 丷=j.s.map().临.test(q);if
丷&&j.s.丗().付()&&!j.s.丙(j.z).test(q){20}else{0}}fn 似(&self,limit:F)->bool{let j=self;j.s.七(j.丛).iter().any(|q|j.乎(q,j.仞)<=limit)}fn 乡(&mut self){let j=self;let
mut 七:Vec<(F,C)> =j.s.亅().iter().map(|q|(j.乎(q,j.丐),q)).collect();七.sort_unstable();for(侭,q)in 七{if j.s.下().is_some(){return;}let Some((id,level))=j.s.仏(q)else{continue;};let
东=j.s.为(j.z,level);let 乀=j.s.map().且[j.z][q];let mut 丈:O<(F,C)> =None;for&next in 乀.iter().take_while(|&&next|next!=之){let next=C::from(next);if!东.test(next){continue;}let
mut 业=j.s.clone();if!业.严(伒(id,next)){continue;}let value=if j.亷(&业){100000}else{j.乪(&业)+10*(侭-j.乎(next,j.丐))+j.余(next)};if 丈.is_none_or(|(乔,_)|value>乔){丈=Some((value,next));}}if
let Some((value,next))=丈{if value>0{j.严(伒(id,next));}}}}fn 乐(&mut self){let j=self;if j.s.下().is_some(){return;}if j.似(侍){j.俕();}else{j.丝();}}fn 俕(&mut
self){let j=self;if!j.s.丗().仼()||j.s.丅(j.z)<乥{return;}let 乀=j.s.map().且[j.z][j.仞];let 丌:Vec<C> =乀.iter().take_while(|&&q|q!=之).map(|&q|C::from(q)).collect();let
久=j.s.丙(j.z);if 丌.iter().any(|&q|久.test(q)&&j.s.乤().test(q)){return;}for q in 丌{let(x,y)=丹(q);if j.严(K::亍{x,y}){return;}}}fn 丝(&mut self){let j=self;if!j.s.丗().付(){return;}while
j.s.丅(j.z)>=乥{let free=j.s.map().临&j.s.丙(j.z)&j.s.丂(j.z)&j.s.仄();let Some(q)=free.iter().min_by_key(|&q|(j.乎(q,j.仞),q))else{return;};let(x,y)=丹(q);if!j.严(K::乾{x,y}){return;}}}fn
俗(&mut self){let j=self;let 佔=if j.turn<=3{0}else{5};for _ in 0..佽{if j.s.下().is_some(){return;}let 丅=j.s.丅(j.z);if 丅<10{return;}let 侯=j.似(佳);let 仦=j.s.亐(j.z);let
mut 丈:O<(F,K)> =None;let mut 会:O<(F,K)> =None;let mut 估=G::Z;for level in 1..=j.s.丗().乫(){let 东=j.s.为(j.z,level);let 乚=(仦&东).两(估);估=东;let 亇=10*F::from(level);if
亇>丅{continue;}for q in 乚.iter(){let 三=你(level,q);let mut 业=j.s.clone();if!业.严(三){continue;}let 伎=业.丅(j.z);let 上=业.上(j.z);let 仡=j.s.亪(j.丛)-业.亪(j.丛);let
侕=if 仡>0&&侯{伎+上>=0}else{上>=0&&(伎>=佔||仡>0)};if!侕{continue;}let 乪=j.乪(&业);let value=乪+j.余(q)-6*j.乎(q,j.丐)-10*(F::from(level)-1);if 乪>0{if 丈.is_none_or(|(乔,_)|value>乔){丈=Some((value,三));}}else
if 上>=3&&伎>=佔+10&&level==1{if 会.is_none_or(|(乔,_)|value>乔){会=Some((value,三));}}}}let Some((_,三))=丈.or(会)else{return;};if!j.严(三){return;}}}}}use crate::丕::{K,I};pub
fn 佁(l:&I,丆:&mut Vec<K>){丆.clear();丄::丄(l,丆);}}mod 习{pub mod chain{type C=usize;type F=i64;type U=bool;use crate::丕::map::之;use crate::丕::{G,K,丠,I,M,SIZE};pub
const 乲:F=F::MAX;const 佼:C=8;pub struct 乱{pub 万:[F;M],pub 並:[u8;M],}pub fn 亝(l:&I,z:C,乡:U,丐:C)->F{let mut 万=乲;亃(l,z,乡,|丏,new|{if new.test(丐){万=10*丏 as
F;}万!=乲});万}pub fn 仩(l:&I,z:C,乡:U,丢:F)->G{let mut 互=G::Z;亃(l,z,乡,|丏,new|{if 10*丏 as F>丢{return true;}互|=new;false});互.两(l.丂(z))}fn 亃(l:&I,z:C,乡:U,mut 亖:impl
FnMut(C,G)->U,){let max=l.丗().乫();let mut entry=[G::Z;3];let mut 串=G::Z;for level in 1..=max{let bits=l.为(z,level);entry[C::from(level)-1]=bits.两(串);串|=bits;}let
mut open=[l.丂(z),G::Z,G::Z,G::Z];if 乡&&z==l.与(){for q in l.亅().iter(){if let Some(level)=l.井(q).filter(|&level|level<=max){open[0]|=G::q(q).且()&l.为(z,level);}}}let
mut 丫=G::Z;for 丏 in 0..{let new=open[丏%4].两(丫);open[丏%4]=G::Z;if 亖(丏,new){return;}丫|=new;let next=new.且().两(丫);for(侖,丌)in entry.iter().enumerate(){open[(丏+侖+1)%4]|=next&*丌;}if
open.iter().all(|bits|bits.两(丫).is_empty()){break;}}}impl 乱{pub fn new(l:&I,z:C,乡:U)->乱{let max=C::from(l.丗().乫());let mut 东=[G::Z;3];for(index,bits)in
东.iter_mut().enumerate().take(max){*bits=l.为(z,index as u8+1);}let 丂=l.丂(z);let 且=&l.map().且[0];let mut 万=[乲;M];let mut 並=[之;M];let mut open=[0u8;M];let
mut count=0;let mut 佗=G::Z;for q in 丂.iter(){万[q]=0;open[count]=q as u8;count+=1;}if 乡&&z==l.与(){for from in l.亅().iter(){let Some(level)=l.井(from)else{continue;};let
么=G::q(from).且()&东[C::from(level)-1];for q in 么.iter(){if 万[q]!=0{万[q]=0;open[count]=q as u8;count+=1;}}}}while count>0{let mut 丈=0;for index in 1..count{let(a,b)=(C::from(open[index]),C::from(open[丈]));if(万[a],a)<(万[b],b){丈=index;}}let
q=C::from(open[丈]);count-=1;open[丈]=open[count];佗.set(q);for&next in&且[q]{if next==之{break;}let next=C::from(next);if 佗.test(next)||丂.test(next){continue;}let
Some(level)=东[..max].iter().position(|bits|bits.test(next))else{continue;};let 佟=万[q]+10*(level as F+1);if 佟<万[next]{if 万[next]==乲{open[count]=next as
u8;count+=1;}万[next]=佟;並[next]=q as u8;}}}乱{万,並}}pub fn path(&self,丐:C)->Vec<C>{let mut path=Vec::new();if 丐>=M||self.万[丐]==乲{return path;}let mut q=丐;while
self.並[q]!=之{path.push(q);q=C::from(self.並[q]);}path.reverse();path}}#[derive(Clone,Debug,PartialEq,Eq)]pub struct 仓{pub 丁:Vec<K>,pub 万:F,}fn 侲(l:&I,z:C,q:C)->Option<u8>{(1..=l.丗().乫()).find(|&level|l.为(z,level).test(q))}fn
仪(丬:&mut I,一:C,亢:C,丢:F,丁:&mut Vec<K>,乣:&mut i64,)->U{loop{if 丬.下()==Some(丠::乗(一)){return true;}if 丬.丙(一).test(亢)&&丬.丂(一).test(亢){return true;}let 丼=乱::new(丬,一,false);let
万=丼.万[亢];if 万==乲||*乣+万>丢{return false;}let Some(&q)=丼.path(亢).first()else{return false;};let Some(level)=侲(丬,一,q)else{return false;};let 三=K::乨{level,x:(q%SIZE)as
u8,y:(q/SIZE)as u8,};if!丬.严(三){return false;}*乣+=10*F::from(level);丁.push(三);}}fn 伯(l:&I,一:C,侫:&[C],丢:F)->Option<仓>{let mut 丬=l.clone();let mut 丁=Vec::new();let
mut 乣=0;for&亢 in 侫{if!仪(&mut 丬,一,亢,丢,&mut 丁,&mut 乣){return None;}}(乣<=丢).then_some(仓{丁,万:乣,})}pub fn 什(l:&I,丐:C,丢:F)->Option<仓>{if 丢<0||l.下().is_some()||丐>=M||l.map().丸.test(丐){return
None;}let 一=l.与();if l.丙(一).test(丐)&&l.丂(一).test(丐){return Some(仓{丁:Vec::new(),万:0,});}let mut 丈=伯(l,一,&[丐],丢);let first=乱::new(l,一,false);let mut 乤:Vec<C>
=(l.乤()&l.丙(1-一)).iter().filter(|&义|义!=丐&&first.万[义]!=乲).collect();乤.sort_by_key(|&义|(l.map().乄(义,丐),义));乤.truncate(佼);for 义 in 乤{if let Some(chain)=伯(l,一,&[义,丐],丢){if
丈.as_ref().is_none_or(|乔|chain.万<乔.万){丈=Some(chain);}}}丈}}pub mod 丒{type C=usize;type F=i64;use std::collections::hash_map::DefaultHasher;use std::collections::HashMap;use
std::hash::BuildHasherDefault;use crate::丕::{G,K,丠,I,SIZE};pub const 佻:C=20000;#[derive(Clone,Debug,PartialEq,Eq)]pub struct 仔{pub 丁:Vec<K>,pub 万:F,pub
丌:F,pub 七:F,pub 乐:F,pub 上:F,pub 不:F,}#[derive(Clone,Copy)]struct 仵{丌:F,七:F,乐:F,}fn 佃(l:&I,z:C)->仵{let 丂=l.丂(z);let 七=l.七(z)&丂;let value=七.iter().filter_map(|q|l.井(q)).map(|level|10*F::from(level)).sum();仵{丌:F::from(丂.count()),七:value,乐:F::from(((l.丝()|l.乤())&丂).count()),}}struct
亙{一:C,仯:G,並:仵,上:F,亨:C,丢:F,亭:C,path:Vec<K>,丧:Vec<仔>,index:HashMap<Key,C,BuildHasherDefault<DefaultHasher>>,}type Key=[u64;3];fn 俞(mut key:Key,q:C)->Key{key[q/64]|=1<<(q%64);key}impl
亙{fn 伺(&mut self,丬:&I,乸:Option<C>,仭:Key,万:F,)->bool{let j=self;let 一=j.一;let 乚=match 乸{None=>丬.亐(一),Some(q)=>G::q(q).且(),}&j.仯;let 乚=乚.两(丬.丙(一));let mut
东=[G::Z;3];for level in 1..=丬.丗().乫(){东[C::from(level)-1]=丬.为(一,level);}for q in 乚.iter(){let max=C::from(丬.丗().乫());let Some(index)=东[..max].iter().position(|bits|bits.test(q))else{continue;};let
level=index as u8+1;let 亇=10*F::from(level);if 万+亇>j.丢{continue;}if j.亭>=佻{return false;}j.亭+=1;let mut next=丬.clone();let 三=K::乨{level,x:(q%SIZE)as u8,y:(q/SIZE)as
u8,};if!next.严(三){continue;}j.path.push(三);let set=俞(仭,q);j.俀(&next,set,万+亇);let 価=j.path.len()<j.亨&&next.下().is_none();let 侪=!価||j.伺(&next,Some(q),set,万+亇);j.path.pop();if!侪{return
false;}}true}fn 俀(&mut self,个:&I,set:Key,万:F){let j=self;let 仉=1-j.一;let now=佃(个,仉);let 丌=j.並.丌-now.丌;let 七=j.並.七-now.七;if 丌<=0&&七<=0{return;}let 乐=j.並.乐-now.乐;let
上=个.上(j.一)-j.上;let 信=个.下()==Some(丠::乗(j.一));let 不=if 信{F::MAX/2}else{3*丌+七+10*乐+2*上-万};if let Some(&両)=j.index.get(&set){if 万>=j.丧[両].万{return;}}let 丒=仔{丁:j.path.clone(),万,丌,七,乐,上,不,};match
j.index.get(&set){Some(&両)=>j.丧[両]=丒,None=>{j.index.insert(set,j.丧.len());j.丧.push(丒);}}}}pub fn 亟(l:&I,亨:C,丢:F,limit:C)->Vec<仔>{丽(l,亨,丢,limit).0}fn 丽(l:&I,亨:C,丢:F,limit:C)->(Vec<仔>,C){if
l.下().is_some()||亨==0||limit==0{return(Vec::new(),0);}let 一=l.与();let 仉=1-一;let mut run=亙{一,仯:l.丙(仉)|l.丙(仉).且(),並:佃(l,仉),上:l.上(一),亨,丢,亭:0,path:Vec::new(),丧:Vec::new(),index:HashMap::default(),};run.伺(l,None,[0;3],0);let
亭=run.亭;let mut 丧=run.丧;丧.sort_by(|a,b|b.不.cmp(&a.不).then(a.万.cmp(&b.万)));丧.truncate(limit);(丧,亭)}}pub mod 乩{type F=i64;use crate::丕::{丠,I,T};use crate::习::chain::{亝,乲};pub
const 亾:F=1000000;const 伣:F=200;fn 乻(l:&I,z:usize)->F{let 丂=l.丂(z);let 七:F=(l.七(z)&丂).iter().filter_map(|q|l.井(q)).map(|level|10*F::from(level)).sum();l.丅(z)+3*l.上(z)+4*F::from(丂.count())+七+10*F::from((l.乤()&丂).count())}fn
佐(l:&I,z:usize)->F{let 万=亝(l,z,false,T[1-z]);if 万==乲{return-伣;}(l.丅(z)+l.上(z)-万).clamp(-伣,0)}pub fn 伽(l:&I,z:usize)->F{let 丞=1-z;match l.下(){Some(丠::乗(传))=>return
if 传==z{亾}else{-亾},Some(丠::伡)=>return 0,None=>{}}let mut 不=乻(l,z)-乻(l,丞);if z==l.与(){let mut next=l.clone();next.亠();if next.下().is_none()&&亝(&next,丞,true,T[z])<=next.丅(丞){不-=亾/2;}}不+佐(l,z)-佐(l,丞)}}}
