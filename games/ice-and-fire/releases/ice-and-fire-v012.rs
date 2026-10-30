// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
mod 乛{type C=usize;use crate::且::l::乢;use crate::且::{G,J,I,T,SIZE};use crate::九::chain::{亷,亗};use crate::九::丒::亙;use crate::乧::亐;pub const 佪:bool=true;fn
価(l:&I,q:C,休:i64,chain:i64)->bool{let 一=l.与();let 丛=1-一;let(x,y)=丸(q);let mut 丳=l.clone();if!丳.两(J::予{x,y}){return false;}if 亐(&丳,一)<休{return true;}佪&&chain<=l.丅(丛)+2*l.上(丛).max(0)&&亗(&丳,丛,false,T[一])>chain}pub
const 佯:C=4;pub const 亱:C=5;pub enum 乐{亵(J),仭(Vec<J>),}impl 乐{pub fn 丁(&self)->&[J]{match self{乐::亵(三)=>std::slice::from_ref(三),乐::仭(丁)=>丁,}}}pub fn 丸(q:C)->(u8,u8){((q%SIZE)as
u8,(q/SIZE)as u8)}pub fn 伷(l:&I,丆:&mut Vec<乐>){丆.clear();if l.下().is_some(){return;}let 一=l.与();let max=l.世().乨();let mut 东=[G::Z;4];for level in 1..=max{东[C::from(level)]=l.丹(一,level);}for
from in l.亀().iter(){let Some((id,level))=l.仇(from)else{continue;};let 乃=G::q(from).丕()&东[C::from(level)];for 乬 in 乃.iter(){let(x,y)=丸(乬);丆.push(乐::亵(J::乶{id,x,y}));}}let
丅=l.丅(一);let 仟=l.二(一);for level in 1..=max{let 万=10*i64::from(level);if 丅<万{break;}let mut 丌=仟&东[C::from(level)];for 侥 in 1..level{丌=丌.丣(东[C::from(侥)]);}for
q in 丌.iter(){let(x,y)=丸(q);丆.push(乐::亵(J::书{level,x,y}));}}if 丅>=乢{let free=l.丙(一)&仟&l.亼();if l.世().仑(){for q in(free&l.map().丱).iter(){let(x,y)=丸(q);丆.push(乐::亵(J::乴{x,y}));}}if
l.世().仳(){let 侐=l.map().valid.丣(l.丙(一)).丕();let 丌=(free&侐).丣(l.map().丱);let 休=亐(l,一);let chain=亗(l,1-一,false,T[一]);for q in 丌.iter().filter(|&q|価(l,q,休,chain)){let(x,y)=丸(q);丆.push(乐::亵(J::予{x,y}));}}}for
丒 in 亙(l,亱,丅,佯){if 丒.丁.len()>1{丆.push(乐::仭(丒.丁));}}if let Some(chain)=亷(l,T[1-一],丅){if chain.丁.len()>1{丆.push(乐::仭(chain.丁));}}}}mod 乧{type C=usize;type
F=i64;use crate::且::I;use crate::両::乫::伍;use crate::九::chain::佉;use crate::九::乧::伴;pub fn 亐(l:&I,z:C)->F{let 丛=1-z;let 丩=l.丅(丛)+l.上(丛);F::from((佉(l,丛,false,丩)&l.七(z)).count())}const
侂:F=2;fn 佝(l:&I,z:C)->F{let 丒=伍(l,z);let mut 乽=0;let mut 伅=0;for q in l.七(z).iter(){if 丒[q]>0{乽+=1;伅=伅.max(丒[q]);}}乽+伅}pub fn 伎(l:&I,z:C)->F{侂*(佝(l,1-z)-佝(l,z))}fn
亸(l:&I,z:C)->F{if l.丅(z)+l.上(z)>=0{return 0;}(l.丂(z)&l.七(z)).iter().filter_map(|q|l.云(q)).map(|level|10*F::from(level)).sum()}pub const 佮:F=15;fn 余(l:&I,z:C)->F{(l.丂(z)&l.七(z)).iter().filter(|&q|l.云(q)==Some(2)).count()as
F}pub fn 乧(l:&I,z:C)->F{let 不=伴(l,z);if l.下().is_some(){return 不;}不-2*亸(l,z)+2*亸(l,1-z)+伎(l,z)+佮*(余(l,1-z)-余(l,z))+7*(亐(l,1-z)-亐(l,z))/4}}mod 义{type C=usize;type
Y=u64;type F=i64;type U=bool;type 举=Instant;type K<T>=Vec<T>;type Q<T>=Option<T>;use std::collections::hash_map::DefaultHasher;use std::collections::HashMap;use
std::hash::BuildHasherDefault;use std::time::Instant;use crate::且::{J,丠,I,T};use crate::両::伇::乚;use crate::伽::伹;use crate::九::chain::亷;use crate::九::丒::亙;use
crate::九::乧::亶;use crate::乛::{伷,丸,乐,亱};use crate::乧::乧;const 佷:C=8;const 佽:C=3;const 佰:C=32;const 佾:Y=300;const 佣:f64=0.75;#[derive(Clone,Copy)]pub struct
乣{pub 乳:Q<举>,pub 事:Q<举>,pub 亿:Y,}#[derive(Default)]pub struct 伝{pub 丁:K<J>,pub 仹:C,pub 乺:Y,pub 丗:C,pub 丧:C,pub 伪:C,}const 侊:U=true;#[derive(Clone)]struct
乥{丁:K<J>,任:F,}struct 从{丁:K<J>,end:I,first:Q<Y>,丑:Q<K<乥>>,}type 亳=HashMap<Y,F,BuildHasherDefault<DefaultHasher>>;struct 亓{一:C,limit:乣,乺:Y,仗:亳,}fn 伤(l:&mut
I,丁:&[J])->U{丁.iter().all(|&三|l.两(三))}fn key(丁:&[J])->Y{let mut hash:Y=0xcbf29ce484222325;for 三 in 丁{let(kind,a,b,c)=match*三{J::乶{id,x,y}=>(1,Y::from(id),x,y),J::书{level,x,y}=>(2,Y::from(level),x,y),J::乴{x,y}=>(3,0,x,y),J::予{x,y}=>(4,0,x,y),J::仐=>(5,0,0,0),};let
word=kind|a<<8|Y::from(b)<<40|Y::from(c)<<48;hash=(hash^word).wrapping_mul(0x0100000001b3);}hash}fn 伢(end:&I,乪:&[J])->I{let mut next=end.clone();next.亚();for&三
in 乪{next.两(三);}next.亚();next}fn 仜(l:&I)->Q<K<J>>{let 一=l.与();let 丐=T[1-一];let 亰=|业:&I|业.下()==Some(丠::乓(一));for q in l.亀().iter(){if l.map().丿(q,丐)!=1{continue;}let
Some((id,_))=l.仇(q)else{continue;};let(x,y)=丸(丐);let 三=J::乶{id,x,y};let mut 业=l.clone();if 业.两(三)&&亰(&业){return Some(vec![三]);}}let chain=亷(l,丐,l.丅(一))?;let
mut 业=l.clone();(伤(&mut 业,&chain.丁)&&亰(&业)).then_some(chain.丁)}impl 亓{fn 习(&self)->U{self.乺>=self.limit.亿||self.limit.乳.is_some_and(|t|举::now()>=t)}fn
不(&mut self,end:&I,丗:&[乥],件:F)->F{self.乺+=1;self.伉(end,丗,件)}fn 伉(&self,end:&I,丗:&[乥],件:F)->F{if end.下()==Some(丠::乓(self.一)){return 亶;}if 丗.is_empty(){return
乧(end,self.一);}let mut 亇=F::MAX;for 乪 in 丗{亇=亇.min(乧(&伢(end,&乪.丁),self.一));if 亇<=件{break;}}亇}#[allow(clippy::too_many_arguments)]fn 侯(&mut self,乹:&I,亘:F,乛:&[乐],亨:&[(F,C)],个:&mut[U],丗:&[乥],些:&mut
亳,伀:U,)->(Q<(C,I,F)>,U){let mut 丈=None;let mut 仓=亘;for&(仠,index)in 亨{if 个[index]{continue;}if 伀&&仠!=F::MAX&&仠<=仓-亘{return(丈,true);}if self.习(){return(丈,false);}个[index]=true;let
丁=乛[index].丁();let mut next=乹.clone();if!伤(&mut next,丁){些.insert(key(丁),F::MIN/4);continue;}let 不=self.不(&next,丗,仓);些.insert(key(丁),不-亘);if 不>仓{仓=不;丈=Some((index,next,不));}}(丈,false)}fn
伬(&mut self,仄:&I,丗:&[乥],仒:&[Y])->从{let j=self;let mut first=None;let mut 乹=仄.clone();let mut 丁=K::new();let mut 亘=j.不(&乹,丗,F::MIN);let mut 些=亳::default();let
mut 乛=K::new();let mut 亨=K::new();while 乹.下().is_none()&&!j.习(){伷(&乹,&mut 乛);亨.clear();for(index,仲)in 乛.iter().enumerate(){let k=key(仲.丁());let 仠=些.get(&k).or(j.仗.get(&k)).copied();亨.push((仠.unwrap_or(F::MAX),index));}亨.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 个=vec![false;乛.len()];if 丁.is_empty(){for(index,仲)in 乛.iter().enumerate(){个[index]=仒.contains(&key(仲.丁()));}}let
mut 丈=None;for 伀 in[true,false]{let(並,便)=j.侯(&乹,亘,&乛,&亨,&mut 个,丗,&mut 些,伀,);丈=並;if 丈.is_some()||!便{break;}}let Some((index,next,不))=丈 else{break;};if 丁.is_empty(){first=Some(key(乛[index].丁()));}丁.extend_from_slice(乛[index].丁());乹=next;亘=不;}for(k,乼)in
些{let 伺=j.仗.entry(k).or_insert(乼);*伺=(*伺).max(乼);}从{丁,end:乹,first,丑:None,}}fn 侶(&self,end:&I)->K<K<J>>{let mut next=end.clone();next.亚();let mut 丆=K::new();if
next.下().is_some(){return 丆;}let 也=next.与();let 丅=next.丅(也);if let Some(chain)=亷(&next,T[self.一],丅){丆.push(chain.丁);}for 丒 in 亙(&next,亱,丅,佽){丆.push(丒.丁);}let
mut 伸=K::new();伹(&next,&mut 伸);丆.push(伸);let mut theirs=亓{一:也,limit:乣{亿:佾,..self.limit},乺:0,仗:亳::default(),};丆.push(theirs.伬(&next,&[],&[]).丁);丆.retain(|丁|!丁.is_empty());丆}fn
伣(&self,end:&I)->K<乥>{self.侶(end).into_iter().map(|丁|{let 任=乧(&伢(end,&丁),self.一);乥{丁,任}}).collect()}fn 伵(&self,丄:&从,丗:&[乥])->F{let 不=self.伉(&丄.end,丗,F::MIN);match&丄.丑{Some(丑)if!丑.is_empty()=>不.min(self.伉(&丄.end,丑,F::MIN)),_=>不,}}}fn
侵(並:&[乥],丗:&mut Vec<乥>)->U{let 严:K<_> =丗.iter().map(|r|r.丁.clone()).collect();for 乪 in 並{if 丗.iter().any(|r|r.丁==乪.丁){continue;}丗.push(乪.clone());}丗.sort_by_key(|乪|乪.任);丗.truncate(佷);丗.iter().map(|r|&r.丁).ne(&严)}pub
fn 佖(l:&I,limit:乣,产:&mut 乚)->伝{let 一=l.与();let mut 串=伝::default();if l.下().is_some(){return 串;}if let Some(丁)=仜(l){串.丁=丁;return 串;}let 丨=|侹:f64|match(limit.乳,limit.事){(Some(乳),Some(事))=>Some(乳+(事-乳).mul_f64(侹)),_=>None,};if
产.侰(l){串.丁=产.turn(l,丨(0.4));return 串;}let mut 义=亓{一,limit:乣{乳:丨(佣),..limit},乺:0,仗:亳::default(),};let mut 丗=K::new();let mut 丧:K<从> =K::new();let mut 仒=K::new();while
串.仹<佰&&(丧.is_empty()||!义.习()){let 丄=义.伬(l,&丗,&仒);串.仹+=1;let 住=丧.iter().any(|p|p.丁==丄.丁);let first=丄.first;let 体=义.习();let mut 丄=丄;let mut 佂=false;if!体&&!住{let
並=义.伣(&丄.end);佂=侵(&並,&mut 丗);丄.丑=Some(並);}if!住{丧.push(丄);}if 体{break;}if!佂{match first{Some(first)=>仒.push(first),None=>break,}}}串.乺=义.乺;串.丗=丗.len();串.丧=丧.len();let
伾=||limit.事.is_some_and(|t|举::now()>=t);let mut 享:K<Q<F>> =vec![None;丧.len()];for index in(0..丧.len()).rev(){if index+1<丧.len()&&伾(){break;}享[index]=Some(义.伵(&丧[index],&丗));}let
亩=|享:&[Q<F>]|{let mut 丈:Q<(F,C)> =None;for(index,不)in 享.iter().enumerate().rev(){if let Some(不)=*不{if 丈.is_none_or(|(乏,_)|不>乏){丈=Some((不,index));}}}丈};let
mut 丈=亩(&享);义.limit.乳=limit.事;while let Some((_,index))=丈{if!侊||丧[index].丑.is_some()||伾(){break;}let 並=义.伣(&丧[index].end);丧[index].丑=Some(並);串.伪+=1;享[index]=Some(义.伵(&丧[index],&丗));丈=亩(&享);}if
let Some((_,index))=丈{let mut 丫=l.clone();for&三 in&丧[index].丁{if 丫.两(三){串.丁.push(三);}}}串}}mod 侸{use std::io::{BufRead,Write};use std::time::Instant;use
crate::且::{仴,久,I};use crate::両::伇::乚;use crate::伽::io::丟;use crate::义::{佖,乣};pub fn 乂(limit:&dyn Fn(Instant,u32)->乣,mut 产:乚,仚:impl BufRead,仡:impl Write,)->Result<(),String>{丟(仚,仡,|亟,乎,turn,亏|match
I::read(亟,乎,久::仨,turn){Ok(l)=>{let 串=佖(&l,limit(亏,turn),&mut 产);eprintln!("turn {turn}: climbs {} tries {} memory {} checks {} {:?}",串.仹,串.乺,串.丗,串.伪,亏.elapsed());仴(&串.丁)}Err(err)=>{eprintln!("plan: cannot read the input: {err}");"WAIT".to_string()}},)}}use
std::io;use std::time::{Duration,Instant};use crate::両::乕::P;use crate::両::伇::{乚,乒};use crate::両::乫::{self,乷};use crate::义::乣;const 你:f64=40.0;const 佧:f64=800.0;const
侃:f64=0.6;fn limit(亏:Instant,turn:u32)->乣{if let Some(亿)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
乣{乳:None,事:None,亿,};}let 代=crate::侓::亭::俄().min(100.0);let 侪=if turn==1{佧}else{你};let 丩=Duration::from_secs_f64(侪/1000.0*代);乣{乳:Some(亏+丩.mul_f64(侃)),事:Some(亏+丩),亿:u64::MAX,}}const
佻:usize=7;const 佺:乷=乷{丌:4,上:3,丅:1,丒:2,乽:2,仆:2,乗:6,了:1,};fn 产()->乚{侭(64)}fn 侭(丽:usize)->乚{let 乱=乒::仏{丟:佻,丽,乍:佺,};乚::new(乱,乫::侁.to_vec(),P::仰)}fn main(){let
stdin=io::stdin();let stdout=io::stdout();if let Err(err)=侸::乂(&limit,产(),stdin.lock(),stdout.lock()){eprintln!("plan: {err}");std::process::exit(1);}}mod
侓{pub mod 仚{type 丢=String;use std::fmt;#[derive(Clone,Debug,PartialEq,Eq)]pub enum 亴{佦,佬(丢),佼{line:丢,expected:丢},}impl fmt::Display for 亴{fn fmt(&self,f:&mut
fmt::Formatter<'_>)->fmt::Result{match self{亴::佦=>write!(f,"input ended"),亴::佬(err)=>write!(f,"cannot read input: {err}"),亴::佼{line,expected}=>{write!(f,"expected {expected}, got {line:?}")}}}}impl
std::error::Error for 亴{}}pub mod 亭{pub const 來:&str="CG_TIME_SCALE";pub fn 俄()->f64{侮(std::env::var(來).ok().as_deref())}fn 侮(value:Option<&str>)->f64{value.and_then(|乎|乎.trim().parse::<f64>().ok()).filter(|代|代.is_finite()&&*代>0.0).unwrap_or(1.0)}}}mod
且{pub mod bits{type C=usize;type Y=u64;use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,Not};pub const SIZE:C=12;pub const M:C=SIZE*SIZE;const 乑:C=4*SIZE;const
乤:Y=(1<<乑)-1;const 今:Y=1|1<<12|1<<24|1<<36;const 仩:Y=今<<11;const 伜:Y=(1<<SIZE)-1;#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct G([Y;3]);impl
G{pub const Z:G=G([0;3]);pub const 仉:G=G([乤;3]);#[inline]pub fn q(q:C)->G{let mut bits=G::Z;bits.set(q);bits}#[inline]pub fn test(self,q:C)->bool{(self.0[q/乑]>>(q%乑))&1!=0}#[inline]pub
fn set(&mut self,q:C){self.0[q/乑]|=1<<(q%乑);}#[inline]pub fn clear(&mut self,q:C){self.0[q/乑]&=!(1<<(q%乑));}#[inline]pub fn is_empty(self)->bool{self.0[0]|self.0[1]|self.0[2]==0}#[inline]pub
fn count(self)->u32{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]pub fn 丣(self,丞:G)->G{G([self.0[0]&!丞.0[0],self.0[1]&!丞.0[1],self.0[2]&!丞.0[2],])}#[inline]pub
fn iter(self)->仧{仧{伃:self.0,丨:0,}}#[inline]pub fn 侾(self)->G{let[a,b,c]=self.0;G([(a>>SIZE)|((b&伜)<<(乑-SIZE)),(b>>SIZE)|((c&伜)<<(乑-SIZE)),c>>SIZE,])}#[inline]pub
fn 侻(self)->G{let[a,b,c]=self.0;G([(a<<SIZE)&乤,((b<<SIZE)&乤)|(a>>(乑-SIZE)),((c<<SIZE)&乤)|(b>>(乑-SIZE)),])}#[inline]pub fn 侼(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仩&乤,(b>>1)&!仩&乤,(c>>1)&!仩&乤,])}#[inline]pub
fn 侽(self)->G{let[a,b,c]=self.0;G([(a<<1)&!今&乤,(b<<1)&!今&乤,(c<<1)&!今&乤,])}#[inline]pub fn 丕(self)->G{let j=self;j.侾()|j.侻()|j.侼()|j.侽()}}impl BitAnd for
G{type Output=G;#[inline]fn bitand(self,丞:G)->G{G([self.0[0]&丞.0[0],self.0[1]&丞.0[1],self.0[2]&丞.0[2],])}}impl BitAndAssign for G{#[inline]fn bitand_assign(&mut
self,丞:G){*self=*self&丞;}}impl BitOr for G{type Output=G;#[inline]fn bitor(self,丞:G)->G{G([self.0[0]|丞.0[0],self.0[1]|丞.0[1],self.0[2]|丞.0[2],])}}impl
BitOrAssign for G{#[inline]fn bitor_assign(&mut self,丞:G){*self=*self|丞;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::仉.丣(self)}}#[derive(Clone,Debug)]pub
struct 仧{伃:[Y;3],丨:C,}impl Iterator for 仧{type Item=C;#[inline]fn next(&mut self)->Option<C>{let j=self;while j.丨<3{let 仝=j.伃[j.丨];if 仝!=0{j.伃[j.丨]=仝&(仝-1);return
Some(j.丨*乑+仝.trailing_zeros()as C);}j.丨+=1;}None}}}pub mod 三{type 丢=String;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum J{乶{id:u32,x:u8,y:u8},书{level:u8,x:u8,y:u8},乴{x:u8,y:u8},予{x:u8,y:u8},仐,}impl
J{pub fn 俆(&self)->丢{match*self{J::乶{id,x,y}=>format!("MOVE {id} {x} {y}"),J::书{level,x,y}=>format!("TRAIN {level} {x} {y}"),J::乴{x,y}=>format!("BUILD MINE {x} {y}"),J::予{x,y}=>format!("BUILD TOWER {x} {y}"),J::仐=>"WAIT".to_string(),}}}pub
fn 仴(丁:&[J])->丢{if 丁.is_empty(){return "WAIT".to_string();}let mut line=丢::with_capacity(16*丁.len());for(index,三)in 丁.iter().enumerate(){if index>0{line.push(';');}line.push_str(&三.俆());}line}}pub
mod 世{#[derive(Clone,Copy,Debug,PartialEq,Eq,Default)]pub enum 久{伟,俓,例,#[default]仨,}impl 久{pub fn 乨(self)->u8{match self{久::伟=>1,_=>3,}}pub fn 仑(self)->bool{matches!(self,久::例|久::仨)}pub
fn 仳(self)->bool{self==久::仨}}}pub mod map{type C=usize;type 仈=i32;use std::cell::RefCell;use std::sync::Arc;use crate::且::bits::{G,M,SIZE};pub const 乆:u8=u8::MAX;pub
struct 乙{pub 丷:G,pub valid:G,pub 丱:G,丿:Box<[u8]>,pub 丕:[[[u8;4];M];2],}thread_local!{static 佭:RefCell<Option<Arc<乙>>> =const{RefCell::new(None)};}impl
乙{pub fn 侺(丷:G,丱:G)->Arc<乙>{佭.with(|乯|{let mut 乯=乯.borrow_mut();if let Some(map)=乯.as_ref(){if map.丷==丷&&map.丱==丱{return Arc::clone(map);}}let map=Arc::new(乙::build(丷,丱));*乯=Some(Arc::clone(&map));map})}fn
build(丷:G,丱:G)->乙{let valid=!丷;const 侄:[[(仈,仈);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丕=[[[乆;4];M];2];for(z,乃)in 侄.iter().enumerate(){for
q in valid.iter(){let(x,y)=((q%SIZE)as 仈,(q/SIZE)as 仈);let mut count=0;for&(侙,侚)in 乃{let(佃,佄)=(x+侙,y+侚);if(0..SIZE as 仈).contains(&佃)&&(0..SIZE as 仈).contains(&佄){let
next=佄 as C*SIZE+佃 as C;if valid.test(next){丕[z][q][count]=next as u8;count+=1;}}}}}let mut 丿=vec![乆;M*M].into_boxed_slice();let mut 伈=[0u8;M];for from
in valid.iter(){let row=&mut 丿[from*M..(from+1)*M];row[from]=0;伈[0]=from as u8;let(mut 份,mut 伐)=(0,1);while 份<伐{let q=伈[份]as C;份+=1;for&next in&丕[0][q]{if
next!=乆&&row[next as C]==乆{row[next as C]=row[q]+1;伈[伐]=next;伐+=1;}}}}乙{丷,valid,丱,丿,丕,}}#[inline]pub fn 丿(&self,from:C,乬:C)->u8{self.丿[from*M+乬]}}}pub
mod l{type C=usize;type L=u8;type 丬=u32;type F=i64;type Q<T>=Option<T>;use std::fmt;use std::sync::Arc;use crate::且::bits::{G,M,SIZE};use crate::且::三::J;use
crate::且::世::久;use crate::且::map::{乙,乆};pub const 佶:丬=100;pub const 乢:F=15;const 侉:[F;3]=[1,4,20];pub const T:[C;2]=[0,M-1];#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub
enum 丠{乓(C),众,}#[derive(Clone,Copy,PartialEq,Eq)]enum 乘{仮,会,伛,}#[derive(Clone)]pub struct I{pub(crate)map:Arc<乙>,pub(crate)世:久,pub(crate)丑:[G;2],pub(crate)七:[G;2],pub(crate)乀:G,pub(crate)乄:G,pub(crate)丂:[G;2],pub(crate)level:[G;3],pub(crate)乿:G,pub(crate)亯:[丬;M],pub(crate)丅:[F;2],pub(crate)上:[F;2],pub
(crate)与:C,pub(crate)乜:丬,pub(crate)交:丬,pub(crate)下:Q<丠>,}fn 乻(丑:G,亞:C)->G{if!丑.test(亞){return G::Z;}let mut 仃=G::q(亞);loop{let next=(仃|仃.丕())&丑;if next==仃{return
仃;}仃=next;}}impl I{pub fn 与(&self)->C{self.与}pub fn 乜(&self)->丬{self.乜}pub fn 下(&self)->Q<丠>{self.下}pub fn 世(&self)->久{self.世}pub fn 亣(&self,z:C)->F{let
j=self;let 丂=j.丂[z];j.丅[z]+10*F::from((丂&j.level[0]).count())+20*F::from((丂&j.level[1]).count())+30*F::from((丂&j.level[2]).count())}pub fn 丅(&self,z:C)->F{self.丅[z]}pub
fn 上(&self,z:C)->F{self.上[z]}pub fn 伓(&self,z:C)->C{self.丂[z].count()as C}pub fn map(&self)->&乙{&self.map}pub fn 丙(&self,z:C)->G{self.丑[z]}pub fn 七(&self,z:C)->G{self.七[z]}pub
fn 丂(&self,z:C)->G{self.丂[z]}pub fn 仇(&self,q:C)->Q<(丬,L)>{let j=self;(j.丂[0]|j.丂[1]).test(q).then(||(j.亯[q],j.伂(q)))}pub fn 亀(&self)->G{self.丂[self.与]&self.乿}pub
fn 丝(&self)->G{self.乀}pub fn 乡(&self)->G{self.乄}pub fn 云(&self,q:C)->Q<L>{(q<M&&(self.丂[0]|self.丂[1]).test(q)).then(||self.伂(q))}fn 俁(&mut self,z:C){let
j=self;j.七=[乻(j.丑[0],T[0]),乻(j.丑[1],T[1])];let 侕=j.丂[z].丣(j.七[z]);j.伊(侕);j.伭();j.丅[z]+=j.上[z];if j.丅[z]<0{j.丅[z]=0;j.伊(j.丂[z]);}j.乿|=j.丂[z];}fn 伭(&mut
self){let j=self;for z in 0..2{let 七=j.七[z];let 丂=七&j.丂[z];let mut 上=F::from(七.count())+4*F::from((七&j.乀).count());for(level,俍)in 侉.iter().enumerate(){上-=俍*F::from((丂&j.level[level]).count());}j.上[z]=上;}}pub
fn 亚(&mut self){let j=self;if j.下.is_some(){return;}if j.与==1{if j.乜>=佶{j.下=Some(match j.亣(0).cmp(&j.亣(1)){std::cmp::Ordering::Greater=>丠::乓(0),std::cmp::Ordering::Less=>丠::乓(1),std::cmp::Ordering::Equal=>丠::众,});return;}j.乜+=1;}j.与=1-j.与;j.俁(j.与);}pub
fn 仁(&mut self,丁:&[J]){let j=self;if j.下.is_some(){return;}let 仼=j.交;for&三 in 丁{if j.侘(三,仼){continue;}j.两(三);if j.下.is_some(){return;}}j.亚();}fn 侘(&self,三:J,仼:丬)->bool{let
j=self;match 三{J::仐=>true,J::乶{id,x,y}=>{j.井(x,y).is_none()||id>=仼||j.伶(j.与,id).is_none()}J::书{level,x,y}=>{j.井(x,y).is_none()||level==0||level>j.世.乨()}J::乴{x,y}|J::予{x,y}=>{j.井(x,y).is_none()}}}#[inline]pub(crate)fn
井(&self,x:L,y:L)->Q<C>{let(x,y)=(C::from(x),C::from(y));(x<SIZE&&y<SIZE&&!self.map.丷.test(y*SIZE+x)).then_some(y*SIZE+x)}pub(crate)fn 伶(&self,z:C,id:丬)->Q<C>{self.丂[z].iter().find(|&q|self.亯[q]==id)}#[inline]pub(crate)fn
伂(&self,q:C)->L{if self.level[0].test(q){1}else if self.level[1].test(q){2}else{3}}pub(crate)fn 侏(&self,也:C)->G{let 乡=self.乄&self.丑[也];let 七=self.七[也];乡|(七&(七&乡).丕())}pub
fn 丹(&self,z:C,level:L)->G{let j=self;debug_assert!((1..=3).contains(&level),"unit level {level} is not 1 to 3");let 也=1-z;let 丂=j.丂[0]|j.丂[1];let 供=j.亼();let
俊=match level{1 if j.世==久::伟=>j.level[1]|j.level[2],1=>丂,2=>j.level[1]|j.level[2],_=>G::Z,};let 丹=供|j.丑[也].丣(俊);if level<3{丹.丣(j.侏(也))}else{丹}}pub fn 二(&self,z:C)->G{self.七[z]|self.七[z].丕()}pub
fn 亼(&self)->G{let j=self;let 丂=j.丂[0]|j.丂[1];let 侣=G::q(T[0])|G::q(T[1]);j.map.valid.丣(丂|j.乀|j.乄|侣)}pub fn 两(&mut self,三:J)->bool{let j=self;if j.下.is_some(){return
false;}let z=j.与;let 亊=match 三{J::仐=>return false,J::书{level,x,y}=>j.俈(z,level,x,y),J::乶{id,x,y}=>j.侩(z,id,x,y),J::乴{x,y}=>j.build(z,true,x,y),J::予{x,y}=>j.build(z,false,x,y),};let
Some(亊)=亊 else{return false;};match 亊{乘::仮=>{}乘::会=>j.七[z]=乻(j.丑[z],T[z]),乘::伛=>{j.七=[乻(j.丑[0],T[0]),乻(j.丑[1],T[1])];}}j.伭();if!j.丑[0].test(T[0]){j.下=Some(丠::乓(1));}else
if!j.丑[1].test(T[1]){j.下=Some(丠::乓(0));}true}fn 伊(&mut self,丌:G){let j=self;j.丂[0]=j.丂[0].丣(丌);j.丂[1]=j.丂[1].丣(丌);for level in&mut j.level{*level=level.丣(丌);}j.乿=j.乿.丣(丌);}fn
佔(&mut self,z:C,q:C)->乘{let j=self;j.伊(G::q(q));j.乀.clear(q);j.乄.clear(q);let 也=1-z;if j.丑[也].test(q){j.丑[也].clear(q);j.丑[z].set(q);乘::伛}else if j.丑[z].test(q){乘::仮}else{j.丑[z].set(q);乘::会}}fn
俈(&mut self,z:C,level:L,x:L,y:L)->Q<乘>{let j=self;if level==0||level>j.世.乨(){return None;}let q=j.井(x,y)?;let 万=10*F::from(level);if j.丅[z]<万||!j.二(z).test(q)||!j.丹(z,level).test(q){return
None;}let 亊=j.佔(z,q);j.佇(z,level,q,j.交);j.交=j.交.saturating_add(1);j.丅[z]-=万;Some(亊)}fn 佇(&mut self,z:C,level:L,q:C,id:丬){self.丂[z].set(q);self.level[C::from(level)-1].set(q);self.亯[q]=id;}fn
侩(&mut self,z:C,id:丬,x:L,y:L)->Q<乘>{let j=self;let 丐=j.井(x,y)?;let from=j.伶(z,id)?;if!j.乿.test(from){return None;}let level=j.伂(from);let 东=j.丹(z,level);let
主=j.map.丕[z][from];if 主.contains(&(丐 as L))&&!东.test(丐){return None;}let mut 丈=j.map.丿(from,丐);let mut 亹=None;for&next in 主.iter().take_while(|&&next|next!=乆){let
next=C::from(next);let 丿=j.map.丿(next,丐);if 丿<丈&&东.test(next){丈=丿;亹=Some(next);}}let 亹=亹?;let 亊=j.佔(z,亹);j.丂[z].clear(from);j.level[C::from(level)-1].clear(from);j.乿.clear(from);j.佇(z,level,亹,id);Some(亊)}fn
build(&mut self,z:C,乀:bool,x:L,y:L)->Q<乘>{let j=self;let 侎=if 乀{j.世.仑()}else{j.世.仳()};let q=j.井(x,y)?;let 侫=j.map.丱.test(q);if!侎||j.丅[z]<乢||侫!=乀||!j.丑[z].test(q)||!j.二(z).test(q)||!j.亼().test(q){return
None;}if 乀{j.乀.set(q);}else{j.乄.set(q);}j.丅[z]-=乢;Some(乘::仮)}}impl fmt::Debug for I{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{let j=self;f.debug_struct("State").field("league",&j.世).field("to_act",&j.与).field("game_turn",&j.乜).field("gold",&j.丅).field("income",&j.上).field("outcome",&j.下).
finish_non_exhaustive()}}}pub mod 乎{type C=usize;type F=i64;type 丢=String;use std::str::Lines;use crate::且::bits::{G,M,SIZE};use crate::且::世::久;use crate::且::map::乙;use
crate::且::l::{I,T};const 佲:F=1000000000;const 仪:F=0;const 佸:F=1;const 侇:F=2;impl I{pub fn read(亟:&str,turn:&str,世:久,乜:u32,)->Result<I,丢>{let mut 丱=Vec::new();let
mut lines=亟.lines();let count=仺(&mut lines,"mine spot count")?;for _ in 0..count{let[x,y]=买(亥(&mut lines,"a mine spot")?,"a mine spot")?;丱.push((仕(x)?,仕(y)?));}伲(lines,"the initial input")?;let
mut lines=turn.lines();let mut 仾=[0i64;4];for(value,what)in 仾.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[number]=买(亥(&mut
lines,what)?,what)?;if number.abs()>佲{return Err(format!("{what}: {number} is out of range"));}*value=number;}let mut 丷=G::Z;let mut 丌=[G::Z;2];let mut
七=[G::Z;2];for y in 0..SIZE{let row=亥(&mut lines,"a map row")?.trim();if row.len()!=SIZE{return Err(format!("map row {y} is not {SIZE} characters: {row:?}"));}for(x,仅)in
row.bytes().enumerate(){let q=y*SIZE+x;match 仅{b'#'=>丷.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let 丼=C::from(仅==b'X'||仅==b'x');丌[丼].set(q);if 仅.is_ascii_uppercase(){七[丼].set(q);}}_=>{return
Err(format!("map row {y} has {:?} at {x}",仅 as char))}}}}let 侑=仺(&mut lines,"building count")?;let mut 之=Vec::new();for _ in 0..侑{let 买=买(亥(&mut lines,"a building")?,"a building")?;之.push(买);}let
伓=仺(&mut lines,"unit count")?;let mut 丂=Vec::new();for _ in 0..伓{let 买=买(亥(&mut lines,"a unit")?,"a unit")?;丂.push(买);}伲(lines,"the turn's input")?;let
但=之.iter().find(|伧|伧[0]==0&&伧[1]==仪).ok_or("the bot's own HQ is not among the buildings")?;let hq_cell=价(但[2],但[3],丷)?;let z=T.iter().position(|&亞|亞==hq_cell).ok_or_else(||format!("the bot's HQ is at cell {hq_cell}, not in a corner"))?;let
mut 佑=G::Z;for(x,y)in 丱{if 丷.test(y*SIZE+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佑.set(y*SIZE+x);}let 仱=|丼:C|if 丼==0{z}else{1-z};let
mut l=I{map:乙::侺(丷,佑),世,丑:[G::Z;2],七:[G::Z;2],乀:G::Z,乄:G::Z,丂:[G::Z;2],level:[G::Z;3],乿:G::Z,亯:[0;M],丅:[0;2],上:[0;2],与:z,乜,交:1,下:None,};for 丼 in 0..2{let
owner=仱(丼);l.丑[owner]=丌[丼];l.七[owner]=七[丼];l.丅[owner]=仾[2*丼];l.上[owner]=仾[2*丼+1];}let mut 佅=G::Z;for[owner,kind,x,y]in 之{let 丼=佋(owner)?;let q=价(x,y,丷)?;if!丌[丼].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佅.test(q){return Err(format!("two buildings at ({x}, {y})"));}佅.set(q);match kind{仪
if q==T[仱(丼)]=>{}仪=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}佸=>l.乀.set(q),侇=>l.乄.set(q),_=>return Err(format!("unknown building type {kind}")),}}let
mut 丯=G::Z;let mut largest_id=0u32;for[owner,id,level,x,y]in 丂{let 丼=佋(owner)?;let q=价(x,y,丷)?;if!丌[丼].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
丯.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&level){return Err(format!("unit level {level} is not 1, 2 or 3"));}let id=u32::try_from(id).map_err(|_|format!("unit id {id} is out of range"))?;if
丯.iter().any(|丞|l.亯[丞]==id){return Err(format!("two units have id {id}"));}丯.set(q);l.丂[仱(丼)].set(q);l.level[level as C-1].set(q);l.亯[q]=id;largest_id=largest_id.max(id);}if!丯.is_empty(){l.交=largest_id.checked_add(1).ok_or_else(||format!("unit id {largest_id} is out of range"))?;}l.乿=l.丂[z];Ok(l)}}fn
亥<'a>(lines:&mut Lines<'a>,what:&str)->Result<&'a str,丢>{lines.next().ok_or_else(||format!("the text ends before {what}"))}fn 伲(mut lines:Lines<'_>,what:&str)->Result<(),丢>{match
lines.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {what}: {line:?}")),None=>Ok(()),}}fn 买<const N:C>(line:&str,what:&str)->Result<[F;N],丢>{let
mut 佞=line.split_whitespace();let mut 佛=[0i64;N];for value in&mut 佛{let word=佞.next().ok_or_else(||format!("{what}: expected {N} numbers in {line:?}"))?;*value=word.parse().map_err(|_|format!("{what}: {word:?} is not an integer"))?;}if
佞.next().is_some(){return Err(format!("{what}: expected {N} numbers in {line:?}"));}Ok(佛)}fn 仺(lines:&mut Lines<'_>,what:&str)->Result<C,丢>{let[count]=买(亥(lines,what)?,what)?;C::try_from(count).ok().filter(|&count|count<=M).ok_or_else(||format!("{what}: {count} is out of range"))}fn
仕(value:F)->Result<C,丢>{C::try_from(value).ok().filter(|&value|value<SIZE).ok_or_else(||format!("coordinate {value} is off the map"))}fn 价(x:F,y:F,丷:G)->Result<C,丢>{let
q=仕(y)?*SIZE+仕(x)?;if 丷.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佋(owner:F)->Result<C,丢>{match owner{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {owner} is not 0 or 1")),}}}pub
use bits::{G,M,SIZE};pub use 三::{仴,J};pub use 世::久;pub use l::{丠,I,T};}mod 両{pub mod 乕{type C=usize;type L=u8;type F=i64;use crate::且::l::乢;use crate::且::{G,J,I,M,T,SIZE};pub
const 佩:F=15;pub const 佹:F=6;const 伞:F=10;const 侀:L=3;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct P{pub open:F,pub 仔:F,pub 估:L,pub 仿:F,pub 似:F,pub
伿:F,pub 低:L,pub 伯:F,pub 伮:F,pub 伡:F,pub 亝:F,pub 京:F,pub 丶:F,pub 亠:F,pub 亅:F,}impl P{pub const 仰:P=P{open:3,仔:2,估:12,仿:4,似:佩,伿:佹,低:侀,伯:2,伮:2,伡:0,亝:0,京:0,丶:0,亠:0,亅:0,};pub
const fn 俑(self,亝:F,京:F,丶:F)->P{P{亝,京,丶,..self}}pub const fn 俐(self,亠:F,亅:F)->P{P{亠,亅,..self}}}fn 丸(q:C)->(L,L){((q%SIZE)as L,(q/SIZE)as L)}pub fn 亻(l:&I,from:G)->[L;M]{let
mut 么=[L::MAX;M];let valid=l.map().valid;let mut 丵=from&valid;let mut 丯=丵;let mut d=0u8;while!丵.is_empty(){for q in 丵.iter(){么[q]=d;}let next=(丵.丕()&valid).丣(丯);丯|=next;丵=next;d=d.saturating_add(1);}么}pub
fn 侞(l:&I)->L{let 么=亻(l,l.丙(0));l.丙(1).iter().map(|q|么[q]).min().unwrap_or(L::MAX)}struct 乸<'a>{乁:&'a P,一:C,theirs:[L;M],亮:[L;M],free:G,乲:G,ours:G,丱:G,}impl
乸<'_>{fn new<'a>(l:&I,乁:&'a P)->乸<'a>{let 一=l.与();let 乀=亻(l,l.丙(一));let theirs=亻(l,l.丙(1-一));let 亮=亻(l,G::q(T[1-一]));let free=l.map().valid.丣(l.丙(0)|l.丙(1));let
mut 乲=G::Z;for q in free.iter(){if 乀[q]<=theirs[q]{乲.set(q);}}乸{乁,一,theirs,亮,free,乲,ours:l.丙(一),丱:l.map().丱,}}fn value(&self,q:C)->F{let j=self;let p=j.乁;let
open=F::from((G::q(q).丕()&j.free).count());let 伩=p.估;let 仔=F::from(伩)-F::from(j.theirs[q].min(伩));let 亞=if p.仿>0{F::from(j.亮[q].min(30))/p.仿}else{0};let
丑=F::from((G::q(q).丕()&j.ours).count());let line=if 丑<=1{p.京}else{0};let 丶=if j.丱.test(q){p.丶}else{0};let 係=if open==0{p.亠}else{0};let 亅=p.亅*(22-F::from(j.亮[q].min(22)));p.open*open+p.仔*仔-亞+p.亝*丑-line+丶-係+亅}fn
丈(&self,丌:G)->Option<C>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 仸(&mut self,q:C){self.free.clear(q);self.乲.clear(q);self.ours.set(q);}}fn
乂(l:&mut I,三:J,丆:&mut Vec<J>)->bool{let 传=l.两(三);if 传{丆.push(三);}传}fn 丏(l:&mut I,from:C,乬:C,丆:&mut Vec<J>)->bool{let Some((id,_))=l.仇(from)else{return
false;};let(x,y)=丸(乬);乂(l,J::乶{id,x,y},丆)}pub fn 俋(h:F,k:F)->F{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伞}pub fn 伌(l:&I,乁:&P)->Vec<J>{伒(l,乁,中::争)}#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]pub
enum 中{争,乵{丝:L,丂:L},}pub fn 伒(l:&I,乁:&P,乔:中)->Vec<J>{let mut 丫=l.clone();let mut turn=乸::new(l,乁);let mut 丆=Vec::new();乞(&mut 丫,&mut turn,&mut 丆);match
乔{中::争=>侳(&mut 丫,&mut turn,&mut 丆),中::乵{丝,丂}=>{依(&mut 丫,&mut turn,丝,丂,&mut 丆)}}丆}fn 乞(l:&mut I,turn:&mut 乸<'_>,丆:&mut Vec<J>){let 一=turn.一;let mut 仙:Vec<C>
=Vec::new();loop{let 东=l.丹(一,1);let mut 亩:Option<(u32,C)> =None;for from in l.亀().iter(){if 仙.contains(&from){continue;}let 乖=(G::q(from).丕()&turn.free&东).count();if
乖==0{仙.push(from);}else if 亩.is_none_or(|(n,_)|乖<n){亩=Some((乖,from));}}let Some((_,from))=亩 else{break;};match turn.丈(G::q(from).丕()&turn.free&东){Some(乬)if
丏(l,from,乬,丆)=>turn.仸(乬),_=>仙.push(from),}}for from in 仙{if!l.亀().test(from){continue;}let 丐=[turn.乲,turn.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().丿(from,q),q))});if
let Some(丐)=丐{丏(l,from,丐,丆);}}}fn 侳(l:&mut I,turn:&mut 乸<'_>,丆:&mut Vec<J>){let 一=turn.一;let p=turn.乁;let h=(p.似-F::from(l.乜())).max(p.伿);let 俅=(F::from(侞(l))-p.伯).max(0)/p.伮.max(1)+p.伡;loop{let
丅=l.丅(一);let 丂=l.伓(一)as F;let 侷=F::from(turn.乲.count());let q=turn.丈(l.二(一)&turn.free&l.丹(一,1));let 佚=俋(h,(侷/(丂+1)).min(俅));let 伨=q.is_some()&&丅>=伞&&佚>0;let
佀=4*h-乢;let 丶=佁(l,turn);let 侒=丶.is_some()&&丅>=乢&&佀>0;if 侒&&(!伨||佀>=佚){let(x,y)=丸(丶.unwrap_or_default());if 乂(l,J::乴{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|伨)else{break;};let(x,y)=丸(q);if!乂(l,J::书{level:1,x,y},丆){break;}turn.仸(q);}}fn
佁(l:&I,turn:&乸<'_>)->Option<C>{let 一=turn.一;(l.map().丱&l.七(一)&l.亼()).iter().filter(|&丶|turn.theirs[丶]>turn.乁.低).min_by_key(|&丶|(l.map().丿(T[一],丶),丶))}fn
依(l:&mut I,turn:&mut 乸<'_>,丝:L,丂:L,丆:&mut Vec<J>,){let 一=turn.一;for _ in 0..丝{let Some(丶)=佁(l,turn)else{break;};let(x,y)=丸(丶);if!乂(l,J::乴{x,y},丆){break;}}for
_ in 0..丂{let Some(q)=turn.丈(l.二(一)&turn.free&l.丹(一,1))else{break;};let(x,y)=丸(q);if!乂(l,J::书{level:1,x,y},丆){break;}turn.仸(q);}}}pub mod 亢{use crate::且::I;use
crate::九::chain::乭;pub fn 于(l:&I)->bool{if l.下().is_some(){return false;}let 一=l.与();let 丅=l.丅(一);let 乗=乭::new(l,一,true);l.丙(1-一).iter().any(|q|乗.万[q]<=丅)}}pub
mod 丄{type C=usize;type Y=u64;type K<T>=Vec<T>;use std::collections::HashMap;use std::time::Instant;use crate::且::{J,I};use crate::両::乕::{self,中,P};use
crate::両::亢;use crate::両::伋::{self,互};use crate::両::不::{self,乇};#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]pub struct O{pub 乊:u8,pub 乔:中,}const 仉:u8=u8::MAX;pub
const 仦:[中;8]=[中::争,中::乵{丝:0,丂:0},中::乵{丝:0,丂:1},中::乵{丝:0,丂:2},中::乵{丝:0,丂:仉,},中::乵{丝:1,丂:0},中::乵{丝:1,丂:1},中::乵{丝:1,丂:仉,},];pub struct 亲<'a>{pub 专:&'a[P],pub
为:P,pub rest:O,pub 乌:Option<Instant>,pub 丰:乇,}impl 亲<'_>{fn 亍(&self)->bool{self.乌.is_some_and(|t|Instant::now()>=t)}fn 乁(&self,丏:O)->&P{&self.专[C::from(丏.乊)%self.专.len()]}pub
fn 丁(&self,l:&I,丏:O)->K<J>{乕::伒(l,self.乁(丏),丏.乔)}pub fn 乂(&self,l:&I,丄:&[O])->I{伋::乂(l,&self.为,|i,s|{self.丁(s,丄.get(i).copied().unwrap_or(self.rest))})}pub
fn value(&self,l:&I,丄:&[O],丰:乇)->i64{不::不(&self.乂(l,丄),l.与(),丰)}fn finish(&self,丧:K<(i64,K<O>)>)->K<O>{let mut 丈:Option<(i64,K<O>)> =None;for(value,丄)in
丧{if 丈.as_ref().is_none_or(|(乏,_)|value>*乏){丈=Some((value,丄));}}丈.map(|(_,丄)|丄).unwrap_or_default()}fn 乃(&self,l:&I)->K<(O,K<J>)>{let mut 丆:K<(O,K<J>)>
=K::new();let mut 乖:K<O> =仦.iter().map(|&乔|O{乊:0,乔}).collect();for 乊 in 1..self.专.len().min(C::from(u8::MAX)){乖.push(O{乊:乊 as u8,乔:中::争,});}for 丏 in 乖{let
丁=self.丁(l,丏);if!丆.iter().any(|(_,c)|*c==丁){丆.push((丏,丁));}}丆}}fn advance(l:&I,丁:&[J],为:&P)->(I,bool){let mut next=l.clone();next.仁(丁);if next.下().is_some()||亢::于(&next){return(next,true);}let
乪=乕::伌(&next,为);next.仁(&乪);let 乩=next.下().is_some()||亢::于(&next)||next.乜()>互;(next,乩)}pub fn 侲(l:&I,丄:&[O],丮:&亲<'_>)->K<Y>{let 一=l.与();let mut 亽=vec![亃(l,一)];let
mut 丘=l.clone();for turn in 0..互 as C{let 丏=丄.get(turn).copied().unwrap_or(丮.rest);let 丁=丮.丁(&丘,丏);let(next,乩)=advance(&丘,&丁,&丮.为);if 乩{break;}丘=next;亽.push(亃(&丘,一));}亽}pub
fn 亃(l:&I,一:C)->Y{let mut hash:Y=0xcbf29ce484222325;let mut 伄=|word:Y|hash=(hash^word).wrapping_mul(0x0100000001b3);for(促,bits)in[l.丙(一),l.丂(一),l.丝()].into_iter().enumerate(){for
q in bits.iter(){伄(((促 as Y)<<8)|q as Y);}伄(0xff);}伄(l.丅(一)as Y);hash}pub fn 仵(l:&I,丮:&亲<'_>,丽:C)->K<O>{let 一=l.与();struct 仍{l:I,丄:K<O>,}let mut 丵=vec![仍{l:l.clone(),丄:K::new(),}];let
mut 个:K<(i64,K<O>)> =vec![(丮.value(l,&[],丮.丰),K::new())];'search:while!丵.is_empty(){let mut 临:K<(i64,仍)> =K::new();let mut 丯:HashMap<Y,C> =HashMap::new();for
亁 in&丵{for(丏,丁)in 丮.乃(&亁.l){if 丮.亍(){if let Some((_,亖))=临.iter().max_by_key(|(v,_)|*v){个.push((丮.value(l,&亖.丄,丮.丰),亖.丄.clone(),));}break 'search;}let(next,乩)=advance(&亁.l,&丁,&丮.为);let
mut 丄=亁.丄.clone();丄.push(丏);let value=不::不(&next,一,丮.丰);if 乩{个.push((value,丄));continue;}let key=亃(&next,一);match 丯.get(&key){Some(&丨)if 临[丨].0>=value=>{}Some(&丨)=>临[丨]=(value,仍{l:next,丄}),None=>{丯.insert(key,临.len());临.push((value,仍{l:next,丄}));}}}}临.sort_by(|a,b|b.0
.cmp(&a.0));临.truncate(丽);丵=临.into_iter().map(|(_,亁)|亁).collect();}丮.finish(个)}}pub mod 伇{type C=usize;type K<T>=Vec<T>;use std::time::Instant;use crate::且::{J,I};use
crate::両::乕::{中,P};use crate::両::丄::{self,亃,亲,O};use crate::両::伋::{伫,互};use crate::両::不::乇;use crate::両::乫::{self,乷};#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub
enum 乒{佡{丽:C,仢:C},佥,仏{丟:C,丽:C,乍:乷,},}pub struct 乚{pub 乱:乒,pub 专:K<P>,pub 为:P,丄:K<O>,expected:K<u64>,亪:C,pub 丧:u32,乩:bool,}impl 乚{pub fn new(乱:乒,专:K<P>,为:P)->乚{乚{乱,专,为,丄:K::new(),expected:K::new(),亪:0,丧:0,乩:false,}}pub
fn 侰(&mut self,l:&I)->bool{let j=self;let 乫=matches!(j.乱,乒::仏{..});let 侱=!乫&&j.丧>0&&j.亪>=j.expected.len();if!j.乩&&(侱||crate::両::亢::于(l)){j.乩=true;}!j.乩}pub
fn turn(&mut self,l:&I,乌:Option<Instant>)->K<J>{let j=self;let 丮=亲{专:&j.专,为:j.为,rest:O{乊:0,乔:中::争,},乌,丰:乇::DEFAULT,};let 一=l.与();let 侬=j.expected.get(j.亪)==Some(&亃(l,一));if!侬{let
first=j.丧==0;j.丄=match j.乱{乒::佡{丽,仢}=>{丄::仵(l,&丮,if first{丽}else{仢})}乒::佥=>{let 专=&j.专;let 乊=伫(l,专,&j.为,丮.丰,乌);let 丏=O{乊:乊 as u8,乔:中::争,};vec![丏;互 as C]}乒::仏{丟,丽,乍,}=>{let
乍=&乍;let rest=乫::使;let mut 乃=乫::丄(l,&j.专,rest,丟,丽,乌,乍,);乃.resize(丟,rest);乃}};j.expected=match j.乱{乒::仏{..}=>俀(l,&j.专,&j.丄),_=>丄::侲(l,&j.丄,&丮),};j.亪=0;j.丧+=1;}let
丏=j.丄.get(j.亪).copied().unwrap_or(丮.rest);j.亪+=1;丮.丁(l,丏)}}fn 俀(l:&I,专:&[P],丄:&[O])->K<u64>{let 一=l.与();let mut 亽=K::with_capacity(丄.len());let mut 丘=l.clone();for&丏
in 丄{if 丘.下().is_some(){break;}亽.push(亃(&丘,一));丘=乫::advance(&丘,&乫::丁(&丘,专,丏));}亽}}pub mod 伋{use std::time::Instant;use crate::且::{J,I};use crate::両::乕::{self,P};use
crate::両::亢;use crate::両::不::{self,乇};pub const 互:u32=20;pub fn 乂(l:&I,为:&P,mut ours:impl FnMut(usize,&I)->Vec<J>,)->I{let 一=l.与();let mut 丘=l.clone();let
mut 丟=0;loop{if 丘.下().is_some()||丘.乜()>互{return 丘;}if(丟>0||丘.与()!=一)&&亢::于(&丘){return 丘;}let 丁=if 丘.与()==一{丟+=1;ours(丟-1,&丘)}else{乕::伌(&丘,为)};丘.仁(&丁);}}pub
fn 俎(l:&I,乁:&P,为:&P,丰:乇)->i64{let 一=l.与();let end=乂(l,为,|_,s|乕::伌(s,乁));不::不(&end,一,丰)}pub fn 伫(l:&I,专:&[P],为:&P,丰:乇,乌:Option<Instant>,)->usize{let mut
丈:Option<(i64,usize)> =None;for(index,乁)in 专.iter().enumerate(){if 丈.is_some()&&乌.is_some_and(|t|Instant::now()>=t){break;}let value=俎(l,乁,为,丰);if 丈.is_none_or(|(乏,_)|value>乏){丈=Some((value,index));}}丈.map_or(0,|(_,index)|index)}}pub
mod 不{type C=usize;type F=i64;use crate::且::{G,丠,I,M,T};use crate::九::chain::乭;use crate::九::丒::亙;pub const 佢:F=1;pub const 侈:F=3;pub const 们:F=1000;const
亱:C=5;const 佤:C=8;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct 乇{pub 丟:F,}impl 乇{pub const DEFAULT:乇=乇{丟:侈};}#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub
struct 五{pub 了:F,pub 丅:F,pub 上:F,pub 他:F,pub 亸:F,pub 亾:F,}impl 五{pub fn value(&self)->F{let j=self;j.了+j.丅+j.上-j.他-j.亸-j.亾}}pub fn 俌(l:&I,z:C)->F{(l.丂(z)&l.七(z)).iter().filter_map(|q|l.云(q)).map(|level|10*F::from(level)).sum()}pub
fn 不(l:&I,z:C,丰:乇)->F{何(l,z,丰).value()-何(l,1-z,丰).value()}pub fn 何(l:&I,z:C,丰:乇)->五{match l.下(){Some(丠::乓(伕))if 伕!=z=>{return 五{亾:们,..五::default()}}Some(_)=>return
五::default(),None=>{}}let 丅=l.丅(z);let 上=l.上(z);let 仞=亐(l,z);五{了:佢*作(l,z)/2,丅,上:丰.丟*上,他:if 仞{0}else{他(l,z,丰)},亸:if 丅+上<0{俌(l,z)}else{0},亾:if 仞{们}else{0},}}fn
乃(l:&I,from:G)->[u8;M]{let valid=l.map().valid;let mut 么=[u8::MAX;M];let mut 丵=from&valid;let mut 丯=丵;let mut d=0u8;while!丵.is_empty(){for q in 丵.iter(){么[q]=d;}let
next=(丵.丕()&valid).丣(丯);丯|=next;丵=next;d=d.saturating_add(1);}么}pub fn 作(l:&I,z:C)->F{let ours=乃(l,l.七(z));let theirs=乃(l,l.七(1-z));l.map().valid.iter().map(|q|match
ours[q].cmp(&theirs[q]){_ if ours[q]==u8::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}fn 侜(l:&I,丛:C)->F{l.丅(丛)+if
l.与()==丛{0}else{l.上(丛)}}pub fn 亐(l:&I,z:C)->bool{let 丛=1-z;let 与=l.与()==丛;乭::new(l,丛,与).万[T[z]]<=侜(l,丛)}pub fn 他(l:&I,伔:C,丰:乇)->F{if l.下().is_some(){return
0;}let mut next=l.clone();if next.与()==伔{next.亚();if next.下().is_some(){return 0;}}let 丛=1-伔;let 丝=|s:&I|F::from((s.丝()&s.七(伔)).count());let 侨=丝(&next);let
mut 亇=0;for 丒 in 亙(&next,亱,next.丅(丛),佤){let mut 丳=next.clone();for&三 in&丒.丁{丳.两(三);}let 亾=if 丳.下()==Some(丠::乓(丛)){们}else{丰.丟*(丒.丌+4*(侨-丝(&丳)))+丒.丂};亇=亇.max(亾);}亇}}pub
mod 乫{type C=usize;type F=i64;type 举=Instant;type K<T>=Vec<T>;use std::collections::HashMap;use std::time::Instant;use crate::且::{G,J,I,M,T};use crate::両::乕::{self,P};use
crate::両::丄::{亃,O,仦};pub const 侁:[P;27]=专();pub const 使:O=O{乊:16,乔:crate::両::乕::中::争,};const fn 专()->[P;27]{let 亝=[0,2,4];let 京=[0,3,6];let 亅=[0,1,2];let
mut 丆=[P::仰;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let 亠=if g>0||t>0{4}else{0};丆[i]=P::仰.俑(亝[g],京[t],0).俐(亠,亅[w]);i+=1;}丆}#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub
struct 仯{pub 丌:F,pub 上:F,pub 丅:F,pub 乽:F,pub 丒:F,pub 仆:F,pub 乗:F,pub 了:F,}#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct 乷{pub 丌:F,pub 上:F,pub 丅:F,pub
丒:F,pub 乽:F,pub 仆:F,pub 乗:F,pub 了:F,}pub fn 伍(l:&I,z:C)->[F;M]{struct 伖{七:G,人:[u16;M],乾:[u16;M],size:[F;M],丒:[F;M],亭:u16,}fn 以(d:&mut 伖,v:C){d.亭+=1;d.人[v]=d.亭;d.乾[v]=d.亭;d.size[v]=1;for
w in(G::q(v).丕()&d.七).iter(){if d.人[w]==0{以(d,w);d.size[v]+=d.size[w];d.乾[v]=d.乾[v].min(d.乾[w]);if d.乾[w]>=d.人[v]{d.丒[v]+=d.size[w];}}else{d.乾[v]=d.乾[v].min(d.人[w]);}}}let
mut d=伖{七:l.七(z),人:[0;M],乾:[0;M],size:[0;M],丒:[0;M],亭:0,};let 仄=T[z];if d.七.test(仄){以(&mut d,仄);}d.丒[仄]=0;d.丒}pub fn 伎(l:&I,z:C)->仯{let 七=l.七(z);let 丒=伍(l,z);let
亮=乕::亻(l,G::q(T[1-z]));let mut 丆=仯{丌:F::from(七.count()),上:l.上(z),丅:l.丅(z),乗:F::MAX,了:crate::両::不::作(l,z)/2,..仯::default()};for q in 七.iter(){if 丒[q]>0{丆.乽+=1;丆.丒=丆.丒.max(丒[q]);}if
q!=T[z]&&(G::q(q).丕()&七).count()<=1{丆.仆+=1;}丆.乗=丆.乗.min(F::from(亮[q]));}丆}pub fn value(l:&I,z:C,乍:&乷)->F{let s=伎(l,z);let o=乍;o.丌*s.丌+o.上*s.上+o.丅*s.丅-o.丒*s.丒-o.乽*s.乽-o.仆*s.仆-o.乗*s.乗.min(30)+o.了*s.了}pub
fn advance(l:&I,丁:&[J])->I{let mut next=l.clone();next.仁(丁);if next.下().is_none(){next.仁(&[]);}next}pub fn 丁(l:&I,专:&[P],丏:O)->K<J>{乕::伒(l,&专[C::from(丏.乊)%专.len()],丏.乔,)}pub
fn 乂(l:&I,专:&[P],丄:&[O],rest:O,丟:C,)->I{let mut 丘=l.clone();for turn in 0..丟{if 丘.下().is_some(){break;}let 丏=丄.get(turn).copied().unwrap_or(rest);丘=advance(&丘,&丁(&丘,专,丏));}丘}fn
乃(l:&I,专:&[P])->K<(O,K<J>)>{let mut 丆:K<(O,K<J>)> =K::new();for 乊 in 0..专.len().min(C::from(u8::MAX)){for&乔 in&仦{let 丏=O{乊:乊 as u8,乔,};let c=丁(l,专,丏);if!丆.iter().any(|(_,丞)|*丞==c){丆.push((丏,c));}}}丆}pub
fn 仵(l:&I,专:&[P],rest:O,丟:C,丽:C,乌:Option<举>,乍:&乷,)->K<O>{let 一=l.与();let 亍=||乌.is_some_and(|t|举::now()>=t);let 仂=|丄:&[O]|value(&乂(l,专,丄,rest,丟),一,乍);let
mut 丈:(F,K<O>)=(仂(&[]),K::new());let mut 丵:K<(I,K<O>)> =vec![(l.clone(),K::new())];for 侗 in 0..丟{let mut 临:K<(F,I,K<O>)> =K::new();let mut 丯:HashMap<u64,C>
=HashMap::new();for(亁,丄)in&丵{if 亍(){break;}for(丏,c)in 乃(亁,专){let next=advance(亁,&c);let mut 亖=丄.clone();亖.push(丏);let v=value(&next,一,乍);let key=亃(&next,一);match
丯.get(&key){Some(&丨)if 临[丨].0>=v=>{}Some(&丨)=>临[丨]=(v,next,亖),None=>{丯.insert(key,临.len());临.push((v,next,亖));}}}}if 临.is_empty(){break;}临.sort_by(|a,b|b.0
.cmp(&a.0));临.truncate(丽);let 乯=侗+1==丟||亍();for(i,(_,_,丄))in 临.iter().take(if 乯{丽}else{1}).enumerate(){if i>0&&亍(){break;}let v=仂(丄);if v>丈.0{丈=(v,丄.clone());}}if
乯{break;}丵=临.into_iter().map(|(_,s,p)|(s,p)).collect();}丈.1}pub fn 丄(l:&I,专:&[P],rest:O,丟:C,侧:C,乌:Option<举>,乍:&乷,)->K<O>{let 一=l.与();let 仂=|丄:&[O]|value(&乂(l,专,丄,rest,丟),一,乍);let
mut 丈:(F,K<O>)=(仂(&[]),K::new());let mut 丽=2;loop{let 並=仵(l,专,rest,丟,丽,乌,乍);let v=仂(&並);if v>丈.0{丈=(v,並);}let 亍=乌.is_some_and(|t|举::now()>=t);if 亍||(乌.is_none()&&丽>=侧){break;}丽*=2;}丈.1}}}mod
伽{pub mod io{type C=usize;type 丢=String;use std::io::{BufRead,Write};use std::time::Instant;const 佨:C=4+12;pub fn 丟(仚:impl BufRead,mut 仡:impl Write,mut
answer:impl FnMut(&str,&str,u32,Instant)->丢,)->Result<(),丢>{let mut lines=Lines{佊:仚};let Some(first)=lines.line()?else{return Ok(());};let mut 亟=format!("{first}\n");lines.take(count(&first,"mine spot count")?,&mut
亟)?;let mut turn=0;loop{let Some(first)=lines.line()?else{return Ok(());};let 亏=Instant::now();turn+=1;let mut 乎=format!("{first}\n");lines.take(佨-1,&mut
乎)?;for what in["building count","unit count"]{let line=lines.佌()?;let n=count(&line,what)?;乎.push_str(&line);乎.push('\n');lines.take(n,&mut 乎)?;}let answer=answer(&亟,&乎,turn,亏);writeln!(仡,"{answer}").and_then(|()|仡.flush()).map_err(|err|format!("turn {turn}: cannot write the answer: {err}"))?;}}struct
Lines<R>{佊:R,}impl<R:BufRead>Lines<R>{fn line(&mut self)->Result<Option<丢>,丢>{let mut line=丢::new();match self.佊.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(err)=>Err(format!("cannot read input: {err}")),}}fn
佌(&mut self)->Result<丢,丢>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:C,乎:&mut String)->Result<(),丢>{for
_ in 0..n{乎.push_str(&self.佌()?);乎.push('\n');}Ok(())}}fn count(line:&str,what:&str)->Result<C,丢>{match line.trim().parse::<C>(){Ok(n)if n<=佱=>Ok(n),_=>Err(format!("bad {what} {line:?}")),}}const
佱:C=144;}mod 丄{type C=usize;type F=i64;type Q<T>=Option<T>;use crate::且::map::乆;use crate::且::l::乢;use crate::且::{G,J,丠,I,M,T,SIZE};const 侅:F=3;const 佫:F=2;const
併:C=16;pub fn 丄(l:&I,丆:&mut Vec<J>){let z=l.与();let mut 乱=乒{s:l.clone(),z,丛:1-z,丐:T[1-z],付:T[z],turn:l.乜(),丆,};乱.run();}struct 乒<'a>{s:I,z:C,丛:C,丐:C,付:C,turn:u32,丆:&'a
mut Vec<J>,}fn 丸(q:C)->(u8,u8){((q%SIZE)as u8,(q/SIZE)as u8)}fn 伆(id:u32,q:C)->J{let(x,y)=丸(q);J::乶{id,x,y}}fn 佘(level:u8,q:C)->J{let(x,y)=丸(q);J::书{level,x,y}}impl
乒<'_>{fn run(&mut self){let j=self;if j.仜(){return;}j.乞();if j.仜(){return;}j.之();j.俉();}fn 么(&self,from:C,乬:C)->F{F::from(self.s.map().丿(from,乬))}fn 两(&mut
self,三:J)->bool{let 个=self.s.两(三);if 个{self.丆.push(三);}个}fn 亰(&self,业:&I)->bool{业.下()==Some(丠::乓(self.z))}fn 仜(&mut self)->bool{let j=self;if j.s.下().is_some(){return
false;}for q in j.s.亀().iter(){if j.么(q,j.丐)!=1{continue;}let Some((id,_))=j.s.仇(q)else{continue;};let 三=伆(id,j.丐);let mut 业=j.s.clone();if 业.两(三)&&j.亰(&业){return
j.两(三);}}let Some(chain)=j.侔()else{return false;};let mut 业=j.s.clone();if!chain.iter().all(|&三|业.两(三))||!j.亰(&业){return false;}for 三 in chain{j.两(三);}true}fn
侔(&self)->Q<Vec<J>>{let j=self;const 优:F=F::MAX;let max=j.s.世().乨();let 东:Vec<G> =(1..=max).map(|level|j.s.丹(j.z,level)).collect();let entry=|q:C|{东.iter().position(|bits|bits.test(q)).map(|index|(index
as u8+1,10*(index as F+1)))};let 七=j.s.七(j.z);let mut 万=[优;M];let mut 严=[C::MAX;M];let mut 个=[false;M];for q in 七.iter(){万[q]=0;}loop{let mut next:Q<C>
=None;for q in 0..M{if!个[q]&&万[q]<优&&next.is_none_or(|n|万[q]<万[n]){next=Some(q);}}let Some(q)=next else{break;};个[q]=true;if q==j.丐{break;}for&主 in&j.s.map().丕[0][q]{if
主==乆{break;}let 主=C::from(主);if 个[主]||七.test(主){continue;}let Some((_,亂))=entry(主)else{continue;};if 万[q]+亂<万[主]{万[主]=万[q]+亂;严[主]=q;}}}if 万[j.丐]>j.s.丅(j.z){return
None;}let mut chain=Vec::new();let mut q=j.丐;while!七.test(q){let(level,_)=entry(q)?;chain.push(佘(level,q));q=严[q];}chain.reverse();Some(chain)}fn 乼(&self,丳:&I)->F{let(一,亄)=(self.z,self.丛);let
严=&self.s;let count=|bits:G|F::from(bits.count());let 丑=count(丳.丙(一))-count(严.丙(一));let 令=count(严.丙(亄))-count(丳.丙(亄));let 仞=count(严.七(亄))-count(丳.七(亄));let
仛=严.亣(亄)-丳.亣(亄);let 丝=count(严.丝()&严.丙(亄))-count(丳.丝()&丳.丙(亄));12*丑+20*令+8*(仞-令).max(0)+5*仛+25*丝}fn 佒(&self,q:C)->F{let j=self;let 丶=j.s.map().丱.test(q);if
丶&&j.s.世().仑()&&!j.s.丙(j.z).test(q){20}else{0}}fn 伳(&self,limit:F)->bool{let j=self;j.s.丂(j.丛).iter().any(|q|j.么(q,j.付)<=limit)}fn 乞(&mut self){let j=self;let
mut 丂:Vec<(F,C)> =j.s.亀().iter().map(|q|(j.么(q,j.丐),q)).collect();丂.sort_unstable();for(侢,q)in 丂{if j.s.下().is_some(){return;}let Some((id,level))=j.s.仇(q)else{continue;};let
东=j.s.丹(j.z,level);let 主=j.s.map().丕[j.z][q];let mut 丈:Q<(F,C)> =None;for&next in 主.iter().take_while(|&&next|next!=乆){let next=C::from(next);if!东.test(next){continue;}let
mut 业=j.s.clone();if!业.两(伆(id,next)){continue;}let value=if j.亰(&业){100000}else{j.乼(&业)+10*(侢-j.么(next,j.丐))+j.佒(next)};if 丈.is_none_or(|(乏,_)|value>乏){丈=Some((value,next));}}if
let Some((value,next))=丈{if value>0{j.两(伆(id,next));}}}}fn 之(&mut self){let j=self;if j.s.下().is_some(){return;}if j.伳(侅){j.俇();}else{j.丝();}}fn 俇(&mut
self){let j=self;if!j.s.世().仳()||j.s.丅(j.z)<乢{return;}let 主=j.s.map().丕[j.z][j.付];let 丌:Vec<C> =主.iter().take_while(|&&q|q!=乆).map(|&q|C::from(q)).collect();let
乀=j.s.丙(j.z);if 丌.iter().any(|&q|乀.test(q)&&j.s.乡().test(q)){return;}for q in 丌{let(x,y)=丸(q);if j.两(J::予{x,y}){return;}}}fn 丝(&mut self){let j=self;if!j.s.世().仑(){return;}while
j.s.丅(j.z)>=乢{let free=j.s.map().丱&j.s.丙(j.z)&j.s.七(j.z)&j.s.亼();let Some(q)=free.iter().min_by_key(|&q|(j.么(q,j.付),q))else{return;};let(x,y)=丸(q);if!j.两(J::乴{x,y}){return;}}}fn
俉(&mut self){let j=self;let 位=if j.turn<=3{0}else{5};for _ in 0..併{if j.s.下().is_some(){return;}let 丅=j.s.丅(j.z);if 丅<10{return;}let 侤=j.伳(佫);let 仟=j.s.二(j.z);let
mut 丈:Q<(F,J)> =None;let mut 伏:Q<(F,J)> =None;let mut 伦=G::Z;for level in 1..=j.s.世().乨(){let 东=j.s.丹(j.z,level);let 乖=(仟&东).丣(伦);伦=东;let 亂=10*F::from(level);if
亂>丅{continue;}for q in 乖.iter(){let 三=佘(level,q);let mut 业=j.s.clone();if!业.两(三){continue;}let 企=业.丅(j.z);let 上=业.上(j.z);let 仛=j.s.亣(j.丛)-业.亣(j.丛);let
侌=if 仛>0&&侤{企+上>=0}else{上>=0&&(企>=位||仛>0)};if!侌{continue;}let 乼=j.乼(&业);let value=乼+j.佒(q)-6*j.么(q,j.丐)-10*(F::from(level)-1);if 乼>0{if 丈.is_none_or(|(乏,_)|value>乏){丈=Some((value,三));}}else
if 上>=3&&企>=位+10&&level==1{if 伏.is_none_or(|(乏,_)|value>乏){伏=Some((value,三));}}}}let Some((_,三))=丈.or(伏)else{return;};if!j.两(三){return;}}}}}use crate::且::{J,I};pub
fn 伹(l:&I,丆:&mut Vec<J>){丆.clear();丄::丄(l,丆);}}mod 九{pub mod chain{type C=usize;type F=i64;type U=bool;use crate::且::map::乆;use crate::且::{G,J,丠,I,M,SIZE};pub
const 乮:F=F::MAX;const 佴:C=8;pub struct 乭{pub 万:[F;M],pub 严:[u8;M],}pub fn 亗(l:&I,z:C,乞:U,丐:C)->F{let mut 万=乮;乻(l,z,乞,|丏,new|{if new.test(丐){万=10*丏 as
F;}万!=乮});万}pub fn 佉(l:&I,z:C,乞:U,丩:F)->G{let mut 于=G::Z;乻(l,z,乞,|丏,new|{if 10*丏 as F>丩{return true;}于|=new;false});于.丣(l.七(z))}fn 乻(l:&I,z:C,乞:U,mut 以:impl
FnMut(C,G)->U,){let max=l.世().乨();let mut entry=[G::Z;3];let mut 丯=G::Z;for level in 1..=max{let bits=l.丹(z,level);entry[C::from(level)-1]=bits.丣(丯);丯|=bits;}let
mut open=[l.七(z),G::Z,G::Z,G::Z];if 乞&&z==l.与(){for q in l.亀().iter(){if let Some(level)=l.云(q).filter(|&level|level<=max){open[0]|=G::q(q).丕()&l.丹(z,level);}}}let
mut 个=G::Z;for 丏 in 0..{let new=open[丏%4].丣(个);open[丏%4]=G::Z;if 以(丏,new){return;}个|=new;let next=new.丕().丣(个);for(侍,丌)in entry.iter().enumerate(){open[(丏+侍+1)%4]|=next&*丌;}if
open.iter().all(|bits|bits.丣(个).is_empty()){break;}}}impl 乭{pub fn new(l:&I,z:C,乞:U)->乭{let max=C::from(l.世().乨());let mut 东=[G::Z;3];for(index,bits)in
东.iter_mut().enumerate().take(max){*bits=l.丹(z,index as u8+1);}let 七=l.七(z);let 丕=&l.map().丕[0];let mut 万=[乮;M];let mut 严=[乆;M];let mut open=[0u8;M];let
mut count=0;let mut 佐=G::Z;for q in 七.iter(){万[q]=0;open[count]=q as u8;count+=1;}if 乞&&z==l.与(){for from in l.亀().iter(){let Some(level)=l.云(from)else{continue;};let
乃=G::q(from).丕()&东[C::from(level)-1];for q in 乃.iter(){if 万[q]!=0{万[q]=0;open[count]=q as u8;count+=1;}}}}while count>0{let mut 丈=0;for index in 1..count{let(a,b)=(C::from(open[index]),C::from(open[丈]));if(万[a],a)<(万[b],b){丈=index;}}let
q=C::from(open[丈]);count-=1;open[丈]=open[count];佐.set(q);for&next in&丕[q]{if next==乆{break;}let next=C::from(next);if 佐.test(next)||七.test(next){continue;}let
Some(level)=东[..max].iter().position(|bits|bits.test(next))else{continue;};let 佗=万[q]+10*(level as F+1);if 佗<万[next]{if 万[next]==乮{open[count]=next as
u8;count+=1;}万[next]=佗;严[next]=q as u8;}}}乭{万,严}}pub fn path(&self,丐:C)->Vec<C>{let mut path=Vec::new();if 丐>=M||self.万[丐]==乮{return path;}let mut q=丐;while
self.严[q]!=乆{path.push(q);q=C::from(self.严[q]);}path.reverse();path}}#[derive(Clone,Debug,PartialEq,Eq)]pub struct 介{pub 丁:Vec<J>,pub 万:F,}fn 侦(l:&I,z:C,q:C)->Option<u8>{(1..=l.世().乨()).find(|&level|l.丹(z,level).test(q))}fn
仢(丫:&mut I,一:C,亜:C,丩:F,丁:&mut Vec<J>,习:&mut i64,)->U{loop{if 丫.下()==Some(丠::乓(一)){return true;}if 丫.丙(一).test(亜)&&丫.七(一).test(亜){return true;}let 乗=乭::new(丫,一,false);let
万=乗.万[亜];if 万==乮||*习+万>丩{return false;}let Some(&q)=乗.path(亜).first()else{return false;};let Some(level)=侦(丫,一,q)else{return false;};let 三=J::书{level,x:(q%SIZE)as
u8,y:(q/SIZE)as u8,};if!丫.两(三){return false;}*习+=10*F::from(level);丁.push(三);}}fn 伥(l:&I,一:C,侠:&[C],丩:F)->Option<介>{let mut 丫=l.clone();let mut 丁=Vec::new();let
mut 习=0;for&亜 in 侠{if!仢(&mut 丫,一,亜,丩,&mut 丁,&mut 习){return None;}}(习<=丩).then_some(介{丁,万:习,})}pub fn 亷(l:&I,丐:C,丩:F)->Option<介>{if 丩<0||l.下().is_some()||丐>=M||l.map().丷.test(丐){return
None;}let 一=l.与();if l.丙(一).test(丐)&&l.七(一).test(丐){return Some(介{丁:Vec::new(),万:0,});}let mut 丈=伥(l,一,&[丐],丩);let first=乭::new(l,一,false);let mut 乡:Vec<C>
=(l.乡()&l.丙(1-一)).iter().filter(|&乄|乄!=丐&&first.万[乄]!=乮).collect();乡.sort_by_key(|&乄|(l.map().丿(乄,丐),乄));乡.truncate(佴);for 乄 in 乡{if let Some(chain)=伥(l,一,&[乄,丐],丩){if
丈.as_ref().is_none_or(|乏|chain.万<乏.万){丈=Some(chain);}}}丈}}pub mod 丒{type C=usize;type F=i64;use std::collections::hash_map::DefaultHasher;use std::collections::HashMap;use
std::hash::BuildHasherDefault;use crate::且::{G,J,丠,I,SIZE};pub const 佳:C=20000;#[derive(Clone,Debug,PartialEq,Eq)]pub struct 仌{pub 丁:Vec<J>,pub 万:F,pub
丌:F,pub 丂:F,pub 之:F,pub 上:F,pub 不:F,}#[derive(Clone,Copy)]struct 仫{丌:F,丂:F,之:F,}fn 伻(l:&I,z:C)->仫{let 七=l.七(z);let 丂=l.丂(z)&七;let value=丂.iter().filter_map(|q|l.云(q)).map(|level|10*F::from(level)).sum();仫{丌:F::from(七.count()),丂:value,之:F::from(((l.丝()|l.乡())&七).count()),}}struct
亓{一:C,佟:G,严:仫,上:F,亡:C,丩:F,亦:C,path:Vec<J>,並:Vec<仌>,index:HashMap<Key,C,BuildHasherDefault<DefaultHasher>>,}type Key=[u64;3];fn 俏(mut key:Key,q:C)->Key{key[q/64]|=1<<(q%64);key}impl
亓{fn 伱(&mut self,丫:&I,乯:Option<C>,令:Key,万:F,)->bool{let j=self;let 一=j.一;let 乖=match 乯{None=>丫.二(一),Some(q)=>G::q(q).丕(),}&j.佟;let 乖=乖.丣(丫.丙(一));let mut
东=[G::Z;3];for level in 1..=丫.世().乨(){东[C::from(level)-1]=丫.丹(一,level);}for q in 乖.iter(){let max=C::from(丫.世().乨());let Some(index)=东[..max].iter().position(|bits|bits.test(q))else{continue;};let
level=index as u8+1;let 亂=10*F::from(level);if 万+亂>j.丩{continue;}if j.亦>=佳{return false;}j.亦+=1;let mut next=丫.clone();let 三=J::书{level,x:(q%SIZE)as u8,y:(q/SIZE)as
u8,};if!next.两(三){continue;}j.path.push(三);let set=俏(令,q);j.侴(&next,set,万+亂);let 侖=j.path.len()<j.亡&&next.下().is_none();let 侟=!侖||j.伱(&next,Some(q),set,万+亂);j.path.pop();if!侟{return
false;}}true}fn 侴(&mut self,丳:&I,set:Key,万:F){let j=self;let 什=1-j.一;let now=伻(丳,什);let 丌=j.严.丌-now.丌;let 丂=j.严.丂-now.丂;if 丌<=0&&丂<=0{return;}let 之=j.严.之-now.之;let
上=丳.上(j.一)-j.上;let 俒=丳.下()==Some(丠::乓(j.一));let 不=if 俒{F::MAX/2}else{3*丌+丂+10*之+2*上-万};if let Some(&丨)=j.index.get(&set){if 万>=j.並[丨].万{return;}}let 丒=仌{丁:j.path.clone(),万,丌,丂,之,上,不,};match
j.index.get(&set){Some(&丨)=>j.並[丨]=丒,None=>{j.index.insert(set,j.並.len());j.並.push(丒);}}}}pub fn 亙(l:&I,亡:C,丩:F,limit:C)->Vec<仌>{义(l,亡,丩,limit).0}fn 义(l:&I,亡:C,丩:F,limit:C)->(Vec<仌>,C){if
l.下().is_some()||亡==0||limit==0{return(Vec::new(),0);}let 一=l.与();let 什=1-一;let mut run=亓{一,佟:l.丙(什)|l.丙(什).丕(),严:伻(l,什),上:l.上(一),亡,丩,亦:0,path:Vec::new(),並:Vec::new(),index:HashMap::default(),};run.伱(l,None,[0;3],0);let
亦=run.亦;let mut 並=run.並;並.sort_by(|a,b|b.不.cmp(&a.不).then(a.万.cmp(&b.万)));並.truncate(limit);(並,亦)}}pub mod 乧{type F=i64;use crate::且::{丠,I,T};use crate::九::chain::{亗,乮};pub
const 亶:F=1000000;const 伙:F=200;fn 乲(l:&I,z:usize)->F{let 七=l.七(z);let 丂:F=(l.丂(z)&七).iter().filter_map(|q|l.云(q)).map(|level|10*F::from(level)).sum();l.丅(z)+3*l.上(z)+4*F::from(七.count())+丂+10*F::from((l.乡()&七).count())}fn
佈(l:&I,z:usize)->F{let 万=亗(l,z,false,T[1-z]);if 万==乮{return-伙;}(l.丅(z)+l.上(z)-万).clamp(-伙,0)}pub fn 伴(l:&I,z:usize)->F{let 丞=1-z;match l.下(){Some(丠::乓(伕))=>return
if 伕==z{亶}else{-亶},Some(丠::众)=>return 0,None=>{}}let mut 不=乲(l,z)-乲(l,丞);if z==l.与(){let mut next=l.clone();next.亚();if next.下().is_none()&&亗(&next,丞,true,T[z])<=next.丅(丞){不-=亶/2;}}不+佈(l,z)-佈(l,丞)}}}
