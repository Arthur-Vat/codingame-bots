// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
mod 之{type C=usize;use crate::丕::l::乤;use crate::丕::{G,J,I,T,SIZE};use crate::也::chain::{亼,亚,仦};use crate::也::丒::亜;use crate::乩::亱;pub const 佯:bool=true;fn
侧(l:&I,q:C,优:i64,丼:G,chain:i64)->bool{let 一=l.与();let 丛=1-一;let 佃=佯&&chain<=l.丄(丛)+2*l.上(丛).max(0);let 们=G::q(q)|G::q(q).且();if!佃&&(们&丼).is_empty(){return
false;}let(x,y)=丹(q);let mut 临=l.clone();if!临.两(J::事{x,y}){return false;}if 亱(&临,一)<优{return true;}佃&&亚(&临,丛,false,T[一])>chain}pub const 佴:C=4;pub const
亵:C=5;#[derive(Clone)]pub enum 乏{人(J),仴(Vec<J>),}impl 乏{pub fn 丁(&self)->&[J]{match self{乏::人(三)=>std::slice::from_ref(三),乏::仴(丁)=>丁,}}}pub fn 丹(q:C)->(u8,u8){((q%SIZE)as
u8,(q/SIZE)as u8)}pub fn 伅(l:&I,丆:&mut Vec<乏>){丆.clear();if l.下().is_some(){return;}let 一=l.与();let max=l.世().乪();let mut 东=[G::Z;4];for level in 1..=max{东[C::from(level)]=l.为(一,level);}for
from in l.亂().iter(){let Some((id,level))=l.今(from)else{continue;};let 乆=G::q(from).且()&东[C::from(level)];for 乮 in 乆.iter(){let(x,y)=丹(乮);丆.push(乏::人(J::乻{id,x,y}));}}let
丄=l.丄(一);let 代=l.亏(一);for level in 1..=max{let 万=10*i64::from(level);if 丄<万{break;}let mut 丌=代&东[C::from(level)];for 侫 in 1..level{丌=丌.丣(东[C::from(侫)]);}for
q in 丌.iter(){let(x,y)=丹(q);丆.push(乏::人(J::乨{level,x,y}));}}if 丄>=乤{let free=l.丙(一)&代&l.什();if l.世().仔(){for q in(free&l.map().串).iter(){let(x,y)=丹(q);丆.push(乏::人(J::乹{x,y}));}}if
l.世().仺(){let 侖=l.map().valid.丣(l.丙(一)).且();let 丌=(free&侖).丣(l.map().串);let 両=l.丄(1-一)+l.上(1-一);let 丼=仦(l,1-一,false,両);let 优=i64::from((丼&l.丂(一)).count());let
chain=亚(l,1-一,false,T[一]);for q in 丌.iter().filter(|&q|侧(l,q,优,丼,chain)){let(x,y)=丹(q);丆.push(乏::人(J::事{x,y}));}}}for 丒 in 亜(l,亵,丄,佴){if 丒.丁.len()>1{丆.push(乏::仴(丒.丁));}}if
let Some(chain)=亼(l,T[1-一],丄){if chain.丁.len()>1{丆.push(乏::仴(chain.丁));}}}}mod 乩{type C=usize;type F=i64;use crate::丕::{G,I,M,T};use crate::也::chain::仦;use
crate::也::乩::伻;pub fn 亱(l:&I,z:C)->F{let 丛=1-z;let 両=l.丄(丛)+l.上(丛);F::from((仦(l,丛,false,両)&l.丂(z)).count())}const 侇:F=2;fn 佤(l:&I,z:C)->F{let 丂=l.丂(z);let[a,b,c]=丂.仫();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as u64).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let index=(hash as C)&(伥-1);侐.with(|仾|{let mut
仾=仾.borrow_mut();let 仩=&mut 仾[index];if 仩.0==丂&&仩.1==z as u8+1{return 仩.2;}let value=俖(丂,T[z]);*仩=(丂,z as u8+1,value);value})}const 伥:C=1<<12;thread_local!{static
侐:std::cell::RefCell<Vec<(G,u8,F)>> =std::cell::RefCell::new(vec![(G::Z,0,0);伥]);}pub fn 俖(丂:G,亅:C)->F{struct 亷{丂:G,乁:[u8;M],丯:[u8;M],size:[u8;M],乭:u8,乲:F,仢:F,}fn
五(d:&mut 亷,v:C)->u8{d.乭+=1;let 乁=d.乭;d.乁[v]=乁;let mut 丯=乁;let mut size=1u8;let mut 丒=0u8;for w in(G::q(v).且()&d.丂).iter(){if d.乁[w]==0{let 佚=五(d,w);size+=佚;丯=丯.min(d.丯[w]);if
d.丯[w]>=乁{丒+=佚;}}else{丯=丯.min(d.乁[w]);}}d.丯[v]=丯;d.size[v]=size;if 丒>0&&d.乁[v]>1{d.乲+=1;d.仢=d.仢.max(F::from(丒));}size}if!丂.test(亅){return 0;}let mut d=亷{丂,乁:[0;M],丯:[0;M],size:[0;M],乭:0,乲:0,仢:0,};五(&mut
d,亅);d.乲+d.仢}pub fn 伕(l:&I,z:C)->F{侇*(佤(l,1-z)-佤(l,z))}fn 亽(l:&I,z:C)->F{if l.丄(z)+l.上(z)>=0{return 0;}(l.七(z)&l.丂(z)).iter().filter_map(|q|l.亓(q)).map(|level|10*F::from(level)).sum()}pub
const 佳:F=15;fn 你(l:&I,z:C)->F{(l.七(z)&l.丂(z)).iter().filter(|&q|l.亓(q)==Some(2)).count()as F}pub fn 乩(l:&I,z:C)->F{let 不=伻(l,z);if l.下().is_some(){return
不;}不-2*亽(l,z)+2*亽(l,1-z)+伕(l,z)+佳*(你(l,1-z)-你(l,z))+7*(亱(l,1-z)-亱(l,z))/4}}mod 乍{type C=usize;type Y=u64;type F=i64;type U=bool;type 乀=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::collections::hash_map::DefaultHasher;use std::collections::HashMap;use std::hash::BuildHasherDefault;use std::time::Instant;use
crate::丕::{J,丠,I,T};use crate::丧::伏::九;use crate::佄::伿;use crate::也::chain::亼;use crate::也::丒::亜;use crate::也::乩::亻;use crate::之::{伅,丹,乏,亵};use crate::乩::乩;const
佼:C=8;const 侂:C=3;const 併:C=32;const 侃:Y=300;const 佨:f64=0.75;#[derive(Clone,Copy)]pub struct 乥{pub 乷:P<乀>,pub 于:P<乀>,pub 仃:Y,}#[derive(Default)]pub struct
伣{pub 丁:K<J>,pub 企:C,pub 乾:Y,pub 丗:C,pub 丨:C,pub 伱:C,}const 侏:U=true;#[derive(Clone)]struct 乧{丁:K<J>,伃:F,}struct 仑{丁:K<J>,end:I,first:P<Y>,丑:P<K<乧>>,}type
亸=HashMap<Y,F,BuildHasherDefault<DefaultHasher>>;struct 亖{一:C,limit:乥,乾:Y,仚:亸,亮:P<K<乏>>,}fn 伫(l:&mut I,丁:&[J])->U{丁.iter().all(|&三|l.两(三))}fn key(丁:&[J])->Y{let
mut hash:Y=0xcbf29ce484222325;for 三 in 丁{let(kind,a,b,c)=match*三{J::乻{id,x,y}=>(1,Y::from(id),x,y),J::乨{level,x,y}=>(2,Y::from(level),x,y),J::乹{x,y}=>(3,0,x,y),J::事{x,y}=>(4,0,x,y),J::仓=>(5,0,0,0),};let
word=kind|a<<8|Y::from(b)<<40|Y::from(c)<<48;hash=(hash^word).wrapping_mul(0x0100000001b3);}hash}fn 伩(end:&I,乬:&[J])->I{let mut next=end.clone();next.亝();for&三
in 乬{next.两(三);}next.亝();next}fn 仟(l:&I)->P<K<J>>{let 一=l.与();let 丐=T[1-一];let 亴=|业:&I|业.下()==Some(丠::乗(一));for q in l.亂().iter(){if l.map().乂(q,丐)!=1{continue;}let
Some((id,_))=l.今(q)else{continue;};let(x,y)=丹(丐);let 三=J::乻{id,x,y};let mut 业=l.clone();if 业.两(三)&&亴(&业){return Some(vec![三]);}}let chain=亼(l,丐,l.丄(一))?;let
mut 业=l.clone();(伫(&mut 业,&chain.丁)&&亴(&业)).then_some(chain.丁)}impl 亖{fn 乢(&self)->U{self.乾>=self.limit.仃||self.limit.乷.is_some_and(|t|乀::now()>=t)}fn
不(&mut self,end:&I,丗:&[乧],份:F)->F{self.乾+=1;self.休(end,丗,份)}fn 休(&self,end:&I,丗:&[乧],份:F)->F{if end.下()==Some(丠::乗(self.一)){return 亻;}if 丗.is_empty(){return
乩(end,self.一);}let mut 亊=F::MAX;for 乬 in 丗{亊=亊.min(乩(&伩(end,&乬.丁),self.一));if 亊<=份{break;}}亊}#[allow(clippy::too_many_arguments)]fn 侵(&mut self,乱:&I,些:F,之:&[乏],享:&[(F,C)],个:&mut[U],丗:&[乧],亞:&mut
亸,伉:U,)->(P<(C,I,F)>,U){let mut 丈=None;let mut 他=些;for&(令,index)in 享{if 个[index]{continue;}if 伉&&令!=F::MAX&&令<=他-些{return(丈,true);}if self.乢(){return(丈,false);}个[index]=true;let
丁=之[index].丁();let mut next=乱.clone();if!伫(&mut next,丁){亞.insert(key(丁),F::MIN/4);continue;}let 不=self.不(&next,丗,他);亞.insert(key(丁),不-些);if 不>他{他=不;丈=Some((index,next,不));}}(丈,false)}fn
伳(&mut self,亅:&I,丗:&[乧],仕:&[Y])->仑{let j=self;let mut first=None;let mut 乱=亅.clone();let mut 丁=K::new();let mut 些=j.不(&乱,丗,F::MIN);let mut 亞=亸::default();let
mut 之=K::new();let mut 享=K::new();while 乱.下().is_none()&&!j.乢(){if 丁.is_empty(){match&j.亮{Some(亮)=>之.clone_from(亮),None=>{伅(&乱,&mut 之);j.亮=Some(之.clone());}}}else{伅(&乱,&mut
之);}享.clear();for(index,仹)in 之.iter().enumerate(){let k=key(仹.丁());let 令=亞.get(&k).or(j.仚.get(&k)).copied();享.push((令.unwrap_or(F::MAX),index));}享.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 个=vec![false;之.len()];if 丁.is_empty(){for(index,仹)in 之.iter().enumerate(){个[index]=仕.contains(&key(仹.丁()));}}let
mut 丈=None;for 伉 in[true,false]{let(並,俆)=j.侵(&乱,些,&之,&享,&mut 个,丗,&mut 亞,伉,);丈=並;if 丈.is_some()||!俆{break;}}let Some((index,next,不))=丈 else{break;};if 丁.is_empty(){first=Some(key(之[index].丁()));}丁.extend_from_slice(之[index].丁());乱=next;些=不;}for(k,亀)in
亞{let 佀=j.仚.entry(k).or_insert(亀);*佀=(*佀).max(亀);}仑{丁,end:乱,first,丑:None,}}fn 侼(&self,end:&I)->K<K<J>>{let mut next=end.clone();next.亝();let mut 丆=K::new();if
next.下().is_some(){return 丆;}let 乡=next.与();let 丄=next.丄(乡);if let Some(chain)=亼(&next,T[self.一],丄){丆.push(chain.丁);}for 丒 in 亜(&next,亵,丄,侂){丆.push(丒.丁);}let
mut 伾=K::new();伿(&next,&mut 伾);丆.push(伾);let mut theirs=亖{一:乡,limit:乥{仃:侃,..self.limit},乾:0,仚:亸::default(),亮:None,};丆.push(theirs.伳(&next,&[],&[]).丁);丆.retain(|丁|!丁.is_empty());丆}fn
伪(&self,end:&I)->K<乧>{self.侼(end).into_iter().map(|丁|{let 伃=乩(&伩(end,&丁),self.一);乧{丁,伃}}).collect()}fn 似(&self,丅:&仑,丗:&[乧])->F{let 不=self.休(&丅.end,丗,F::MIN);match&丅.丑{Some(丑)if!丑.is_empty()=>不.min(self.休(&丅.end,丑,F::MIN)),_=>不,}}}fn
侻(並:&[乧],丗:&mut Vec<乧>)->U{let 严:K<_> =丗.iter().map(|r|r.丁.clone()).collect();for 乬 in 並{if 丗.iter().any(|r|r.丁==乬.丁){continue;}丗.push(乬.clone());}丗.sort_by_key(|乬|乬.伃);丗.truncate(佼);丗.iter().map(|r|&r.丁).ne(&严)}pub
fn 佝(l:&I,limit:乥,亪:&mut 九)->伣{let 一=l.与();let mut 丳=伣::default();if l.下().is_some(){return 丳;}if let Some(丁)=仟(l){丳.丁=丁;return 丳;}let 丩=|俀:f64|match(limit.乷,limit.于){(Some(乷),Some(于))=>Some(乷+(于-乷).mul_f64(俀)),_=>None,};if
亪.侶(l){丳.丁=亪.turn(l,丩(0.4));return 丳;}let mut 乍=亖{一,limit:乥{乷:丩(佨),..limit},乾:0,仚:亸::default(),亮:None,};let mut 丗=K::new();let mut 丨:K<仑> =K::new();let
mut 仕=K::new();while 丳.企<併&&(丨.is_empty()||!乍.乢()){let 丅=乍.伳(l,&丗,&仕);丳.企+=1;let 何=丨.iter().any(|p|p.丁==丅.丁);let first=丅.first;let 余=乍.乢();let mut 丅=丅;let
mut 佉=false;if!余&&!何{let 並=乍.伪(&丅.end);佉=侻(&並,&mut 丗);丅.丑=Some(並);}if!何{丨.push(丅);}if 余{break;}if!佉{match first{Some(first)=>仕.push(first),None=>break,}}}丳.乾=乍.乾;丳.丗=丗.len();丳.丨=丨.len();let
佅=||limit.于.is_some_and(|t|乀::now()>=t);let mut 亯:K<P<F>> =vec![None;丨.len()];for index in(0..丨.len()).rev(){if index+1<丨.len()&&佅(){break;}亯[index]=Some(乍.似(&丨[index],&丗));}let
京=|亯:&[P<F>]|{let mut 丈:P<(F,C)> =None;for(index,不)in 亯.iter().enumerate().rev(){if let Some(不)=*不{if 丈.is_none_or(|(乔,_)|不>乔){丈=Some((不,index));}}}丈};let
mut 丈=京(&亯);乍.limit.乷=limit.于;while let Some((_,index))=丈{if!侏||丨[index].丑.is_some()||佅(){break;}let 並=乍.伪(&丨[index].end);丨[index].丑=Some(並);丳.伱+=1;亯[index]=Some(乍.似(&丨[index],&丗));丈=京(&亯);}if
let Some((_,index))=丈{let mut 丫=l.clone();for&三 in&丨[index].丁{if 丫.两(三){丳.丁.push(三);}}}丳}}mod 侾{use std::io::{BufRead,Write};use std::time::Instant;use
crate::丕::{任,么,I};use crate::丧::伏::九;use crate::佄::io::丟;use crate::乍::{佝,乥};pub fn 久(limit:&dyn Fn(Instant,u32)->乥,mut 亪:九,仝:impl BufRead,以:impl Write,)->Result<(),String>{丟(仝,以,|亢,乓,turn,互|match
I::read(亢,乓,么::仯,turn){Ok(l)=>{let 丳=佝(&l,limit(互,turn),&mut 亪);eprintln!("turn {turn}: climbs {} tries {} memory {} checks {} {:?}",丳.企,丳.乾,丳.丗,丳.伱,互.elapsed());任(&丳.丁)}Err(err)=>{eprintln!("plan: cannot read the input: {err}");"WAIT".to_string()}},)}}use
std::io;use std::time::{Duration,Instant};use crate::丧::乙::Q;use crate::丧::伏::{九,乖};use crate::丧::乸::{self,乼};use crate::乍::乥;const 佥:f64=40.0;const 佬:f64=800.0;const
侈:f64=0.6;fn limit(互:Instant,turn:u32)->乥{if let Some(仃)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
乥{乷:None,于:None,仃,};}let 仨=crate::侙::乭::俋().min(100.0);let 侰=if turn==1{佬}else{佥};let 両=Duration::from_secs_f64(侰/1000.0*仨);乥{乷:Some(互+両.mul_f64(侈)),于:Some(互+両),仃:u64::MAX,}}const
侀:usize=7;const 使:乼=乼{丌:4,上:3,丄:1,丒:2,乲:2,仉:2,丼:6,争:1,};fn 亪()->九{侳(64)}fn 侳(丿:usize)->九{let 乵=乖::仒{丟:侀,丿,乒:使,};九::new(乵,乸::來.to_vec(),Q::价)}fn main(){let
stdin=io::stdin();let stdout=io::stdout();if let Err(err)=侾::久(&limit,亪(),stdin.lock(),stdout.lock()){eprintln!("plan: {err}");std::process::exit(1);}}mod
侙{pub mod 仝{type 丢=String;use std::fmt;#[derive(Clone,Debug,PartialEq,Eq)]pub enum 亹{佫,佱(丢),侁{line:丢,expected:丢},}impl fmt::Display for 亹{fn fmt(&self,f:&mut
fmt::Formatter<'_>)->fmt::Result{match self{亹::佫=>write!(f,"input ended"),亹::佱(err)=>write!(f,"cannot read input: {err}"),亹::侁{line,expected}=>{write!(f,"expected {expected}, got {line:?}")}}}}impl
std::error::Error for 亹{}}pub mod 乭{pub const 例:&str="CG_TIME_SCALE";pub fn 俋()->f64{侴(std::env::var(例).ok().as_deref())}fn 侴(value:Option<&str>)->f64{value.and_then(|乓|乓.trim().parse::<f64>().ok()).filter(|仨|仨.is_finite()&&*仨>0.0).unwrap_or(1.0)}}}mod
丕{pub mod bits{type C=usize;type Y=u64;use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,Not};pub const SIZE:C=12;pub const M:C=SIZE*SIZE;const 乕:C=4*SIZE;const
书:Y=(1<<乕)-1;const 仍:Y=1|1<<12|1<<24|1<<36;const 仰:Y=仍<<11;const 伢:Y=(1<<SIZE)-1;#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct G([Y;3]);impl
G{pub const Z:G=G([0;3]);pub const 仌:G=G([书;3]);#[inline]pub fn q(q:C)->G{let mut bits=G::Z;bits.set(q);bits}#[inline]pub fn test(self,q:C)->bool{(self.0[q/乕]>>(q%乕))&1!=0}#[inline]pub
fn set(&mut self,q:C){self.0[q/乕]|=1<<(q%乕);}#[inline]pub fn clear(&mut self,q:C){self.0[q/乕]&=!(1<<(q%乕));}#[inline]pub fn is_empty(self)->bool{self.0[0]|self.0[1]|self.0[2]==0}#[inline]pub
fn 仫(self)->[Y;3]{self.0}#[inline]pub fn count(self)->u32{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]pub fn 丣(self,丞:G)->G{G([self.0[0]&!丞.0[0],self.0[1]&!丞.0[1],self.0[2]&!丞.0[2],])}#[inline]pub
fn iter(self)->仮{仮{伌:self.0,丩:0,}}#[inline]pub fn 俅(self)->G{let[a,b,c]=self.0;G([(a>>SIZE)|((b&伢)<<(乕-SIZE)),(b>>SIZE)|((c&伢)<<(乕-SIZE)),c>>SIZE,])}#[inline]pub
fn 係(self)->G{let[a,b,c]=self.0;G([(a<<SIZE)&书,((b<<SIZE)&书)|(a>>(乕-SIZE)),((c<<SIZE)&书)|(b>>(乕-SIZE)),])}#[inline]pub fn 促(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仰&书,(b>>1)&!仰&书,(c>>1)&!仰&书,])}#[inline]pub
fn 俄(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仍&书,(b<<1)&!仍&书,(c<<1)&!仍&书,])}#[inline]pub fn 且(self)->G{let j=self;j.俅()|j.係()|j.促()|j.俄()}}impl BitAnd for
G{type Output=G;#[inline]fn bitand(self,丞:G)->G{G([self.0[0]&丞.0[0],self.0[1]&丞.0[1],self.0[2]&丞.0[2],])}}impl BitAndAssign for G{#[inline]fn bitand_assign(&mut
self,丞:G){*self=*self&丞;}}impl BitOr for G{type Output=G;#[inline]fn bitor(self,丞:G)->G{G([self.0[0]|丞.0[0],self.0[1]|丞.0[1],self.0[2]|丞.0[2],])}}impl
BitOrAssign for G{#[inline]fn bitor_assign(&mut self,丞:G){*self=*self|丞;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::仌.丣(self)}}#[derive(Clone,Debug)]pub
struct 仮{伌:[Y;3],丩:C,}impl Iterator for 仮{type Item=C;#[inline]fn next(&mut self)->Option<C>{let j=self;while j.丩<3{let 仠=j.伌[j.丩];if 仠!=0{j.伌[j.丩]=仠&(仠-1);return
Some(j.丩*乕+仠.trailing_zeros()as C);}j.丩+=1;}None}}}pub mod 三{type 丢=String;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum J{乻{id:u32,x:u8,y:u8},乨{level:u8,x:u8,y:u8},乹{x:u8,y:u8},事{x:u8,y:u8},仓,}impl
J{pub fn 俍(&self)->丢{match*self{J::乻{id,x,y}=>format!("MOVE {id} {x} {y}"),J::乨{level,x,y}=>format!("TRAIN {level} {x} {y}"),J::乹{x,y}=>format!("BUILD MINE {x} {y}"),J::事{x,y}=>format!("BUILD TOWER {x} {y}"),J::仓=>"WAIT".to_string(),}}}pub
fn 任(丁:&[J])->丢{if 丁.is_empty(){return "WAIT".to_string();}let mut line=丢::with_capacity(16*丁.len());for(index,三)in 丁.iter().enumerate(){if index>0{line.push(';');}line.push_str(&三.俍());}line}}pub
mod 世{#[derive(Clone,Copy,Debug,PartialEq,Eq,Default)]pub enum 么{伦,俛,侑,#[default]仯,}impl 么{pub fn 乪(self)->u8{match self{么::伦=>1,_=>3,}}pub fn 仔(self)->bool{matches!(self,么::侑|么::仯)}pub
fn 仺(self)->bool{self==么::仯}}}pub mod map{type C=usize;type 介=i32;use std::cell::RefCell;use std::sync::Arc;use crate::丕::bits::{G,M,SIZE};pub const 义:u8=u8::MAX;pub
struct 乜{pub 丸:G,pub valid:G,pub 串:G,乂:Box<[u8]>,pub 且:[[[u8;4];M];2],}thread_local!{static 佲:RefCell<Option<Arc<乜>>> =const{RefCell::new(None)};}impl
乜{pub fn 俁(丸:G,串:G)->Arc<乜>{佲.with(|乳|{let mut 乳=乳.borrow_mut();if let Some(map)=乳.as_ref(){if map.丸==丸&&map.串==串{return Arc::clone(map);}}let map=Arc::new(乜::build(丸,串));*乳=Some(Arc::clone(&map));map})}fn
build(丸:G,串:G)->乜{let valid=!丸;const 侉:[[(介,介);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 且=[[[义;4];M];2];for(z,乆)in 侉.iter().enumerate(){for
q in valid.iter(){let(x,y)=((q%SIZE)as 介,(q/SIZE)as 介);let mut count=0;for&(侟,侠)in 乆{let(佊,佋)=(x+侟,y+侠);if(0..SIZE as 介).contains(&佊)&&(0..SIZE as 介).contains(&佋){let
next=佋 as C*SIZE+佊 as C;if valid.test(next){且[z][q][count]=next as u8;count+=1;}}}}}let mut 乂=vec![义;M*M].into_boxed_slice();let mut 伐=[0u8;M];for from
in valid.iter(){let row=&mut 乂[from*M..(from+1)*M];row[from]=0;伐[0]=from as u8;let(mut 伆,mut 众)=(0,1);while 伆<众{let q=伐[伆]as C;伆+=1;for&next in&且[0][q]{if
next!=义&&row[next as C]==义{row[next as C]=row[q]+1;伐[众]=next;众+=1;}}}}乜{丸,valid,串,乂,且,}}#[inline]pub fn 乂(&self,from:C,乮:C)->u8{self.乂[from*M+乮]}}}pub
mod l{type C=usize;type L=u8;type 丬=u32;type F=i64;type P<T>=Option<T>;use std::fmt;use std::sync::Arc;use crate::丕::bits::{G,M,SIZE};use crate::丕::三::J;use
crate::丕::世::么;use crate::丕::map::{乜,义};pub const 佻:丬=100;pub const 乤:F=15;const 侎:[F;3]=[1,4,20];pub const T:[C;2]=[0,M-1];#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub
enum 丠{乗(C),伝,}#[derive(Clone,Copy,PartialEq,Eq)]enum 乛{仵,传,伡,}#[derive(Clone)]pub struct I{pub(crate)map:Arc<乜>,pub(crate)世:么,pub(crate)丑:[G;2],pub(crate)丂:[G;2],pub(crate)乃:G,pub(crate)乇:G,pub(crate)七:[G;2],pub(crate)level:[G;3],pub(crate)亁:G,pub(crate)亳:[丬;M],pub(crate)丄:[F;2],pub(crate)上:[F;2],pub
(crate)与:C,pub(crate)乞:丬,pub(crate)产:丬,pub(crate)下:P<丠>,}fn 乿(丑:G,亡:C)->G{if!丑.test(亡){return G::Z;}let mut 仇=G::q(亡);loop{let next=(仇|仇.且())&丑;if next==仇{return
仇;}仇=next;}}impl I{pub fn 与(&self)->C{self.与}pub fn 乞(&self)->丬{self.乞}pub fn 下(&self)->P<丠>{self.下}pub fn 世(&self)->么{self.世}pub fn 亦(&self,z:C)->F{let
j=self;let 七=j.七[z];j.丄[z]+10*F::from((七&j.level[0]).count())+20*F::from((七&j.level[1]).count())+30*F::from((七&j.level[2]).count())}pub fn 丄(&self,z:C)->F{self.丄[z]}pub
fn 上(&self,z:C)->F{self.上[z]}pub fn 会(&self,z:C)->C{self.七[z].count()as C}pub fn map(&self)->&乜{&self.map}pub fn 丙(&self,z:C)->G{self.丑[z]}pub fn 丂(&self,z:C)->G{self.丂[z]}pub
fn 七(&self,z:C)->G{self.七[z]}pub fn 今(&self,q:C)->P<(丬,L)>{let j=self;(j.七[0]|j.七[1]).test(q).then(||(j.亳[q],j.伋(q)))}pub fn 亂(&self)->G{self.七[self.与]&self.亁}pub
fn 丝(&self)->G{self.乃}pub fn 乣(&self)->G{self.乇}pub fn 亓(&self,q:C)->P<L>{(q<M&&(self.七[0]|self.七[1]).test(q)).then(||self.伋(q))}fn 俈(&mut self,z:C){let
j=self;j.丂=[乿(j.丑[0],T[0]),乿(j.丑[1],T[1])];let 供=j.七[z].丣(j.丂[z]);j.伒(供);j.伴();j.丄[z]+=j.上[z];if j.丄[z]<0{j.丄[z]=0;j.伒(j.七[z]);}j.亁|=j.七[z];}fn 伴(&mut
self){let j=self;for z in 0..2{let 丂=j.丂[z];let 七=丂&j.七[z];let mut 上=F::from(丂.count())+4*F::from((丂&j.乃).count());for(level,俔)in 侎.iter().enumerate(){上-=俔*F::from((七&j.level[level]).count());}j.上[z]=上;}}pub
fn 亝(&mut self){let j=self;if j.下.is_some(){return;}if j.与==1{if j.乞>=佻{j.下=Some(match j.亦(0).cmp(&j.亦(1)){std::cmp::Ordering::Greater=>丠::乗(0),std::cmp::Ordering::Less=>丠::乗(1),std::cmp::Ordering::Equal=>丠::伝,});return;}j.乞+=1;}j.与=1-j.与;j.俈(j.与);}pub
fn 仅(&mut self,丁:&[J]){let j=self;if j.下.is_some(){return;}let 伄=j.产;for&三 in 丁{if j.侞(三,伄){continue;}j.两(三);if j.下.is_some(){return;}}j.亝();}fn 侞(&self,三:J,伄:丬)->bool{let
j=self;match 三{J::仓=>true,J::乻{id,x,y}=>{j.亘(x,y).is_none()||id>=伄||j.伽(j.与,id).is_none()}J::乨{level,x,y}=>{j.亘(x,y).is_none()||level==0||level>j.世.乪()}J::乹{x,y}|J::事{x,y}=>{j.亘(x,y).is_none()}}}#[inline]pub(crate)fn
亘(&self,x:L,y:L)->P<C>{let(x,y)=(C::from(x),C::from(y));(x<SIZE&&y<SIZE&&!self.map.丸.test(y*SIZE+x)).then_some(y*SIZE+x)}pub(crate)fn 伽(&self,z:C,id:丬)->P<C>{self.七[z].iter().find(|&q|self.亳[q]==id)}#[inline]pub(crate)fn
伋(&self,q:C)->L{if self.level[0].test(q){1}else if self.level[1].test(q){2}else{3}}pub(crate)fn 侕(&self,乡:C)->G{let 乣=self.乇&self.丑[乡];let 丂=self.丂[乡];乣|(丂&(丂&乣).且())}pub
fn 为(&self,z:C,level:L)->G{let j=self;debug_assert!((1..=3).contains(&level),"unit level {level} is not 1 to 3");let 乡=1-z;let 七=j.七[0]|j.七[1];let 価=j.什();let
俑=match level{1 if j.世==么::伦=>j.level[1]|j.level[2],1=>七,2=>j.level[1]|j.level[2],_=>G::Z,};let 为=価|j.丑[乡].丣(俑);if level<3{为.丣(j.侕(乡))}else{为}}pub fn 亏(&self,z:C)->G{self.丂[z]|self.丂[z].且()}pub
fn 什(&self)->G{let j=self;let 七=j.七[0]|j.七[1];let 侩=G::q(T[0])|G::q(T[1]);j.map.valid.丣(七|j.乃|j.乇|侩)}pub fn 两(&mut self,三:J)->bool{let j=self;if j.下.is_some(){return
false;}let z=j.与;let 亍=match 三{J::仓=>return false,J::乨{level,x,y}=>j.俏(z,level,x,y),J::乻{id,x,y}=>j.侯(z,id,x,y),J::乹{x,y}=>j.build(z,true,x,y),J::事{x,y}=>j.build(z,false,x,y),};let
Some(亍)=亍 else{return false;};match 亍{乛::仵=>{}乛::传=>j.丂[z]=乿(j.丑[z],T[z]),乛::伡=>{j.丂=[乿(j.丑[0],T[0]),乿(j.丑[1],T[1])];}}j.伴();if!j.丑[0].test(T[0]){j.下=Some(丠::乗(1));}else
if!j.丑[1].test(T[1]){j.下=Some(丠::乗(0));}true}fn 伒(&mut self,丌:G){let j=self;j.七[0]=j.七[0].丣(丌);j.七[1]=j.七[1].丣(丌);for level in&mut j.level{*level=level.丣(丌);}j.亁=j.亁.丣(丌);}fn
佛(&mut self,z:C,q:C)->乛{let j=self;j.伒(G::q(q));j.乃.clear(q);j.乇.clear(q);let 乡=1-z;if j.丑[乡].test(q){j.丑[乡].clear(q);j.丑[z].set(q);乛::伡}else if j.丑[z].test(q){乛::仵}else{j.丑[z].set(q);乛::传}}fn
俏(&mut self,z:C,level:L,x:L,y:L)->P<乛>{let j=self;if level==0||level>j.世.乪(){return None;}let q=j.亘(x,y)?;let 万=10*F::from(level);if j.丄[z]<万||!j.亏(z).test(q)||!j.为(z,level).test(q){return
None;}let 亍=j.佛(z,q);j.低(z,level,q,j.产);j.产=j.产.saturating_add(1);j.丄[z]-=万;Some(亍)}fn 低(&mut self,z:C,level:L,q:C,id:丬){self.七[z].set(q);self.level[C::from(level)-1].set(q);self.亳[q]=id;}fn
侯(&mut self,z:C,id:丬,x:L,y:L)->P<乛>{let j=self;let 丐=j.亘(x,y)?;let from=j.伽(z,id)?;if!j.亁.test(from){return None;}let level=j.伋(from);let 东=j.为(z,level);let
丽=j.map.且[z][from];if 丽.contains(&(丐 as L))&&!东.test(丐){return None;}let mut 丈=j.map.乂(from,丐);let mut 亾=None;for&next in 丽.iter().take_while(|&&next|next!=义){let
next=C::from(next);let 乂=j.map.乂(next,丐);if 乂<丈&&东.test(next){丈=乂;亾=Some(next);}}let 亾=亾?;let 亍=j.佛(z,亾);j.七[z].clear(from);j.level[C::from(level)-1].clear(from);j.亁.clear(from);j.低(z,level,亾,id);Some(亍)}fn
build(&mut self,z:C,乃:bool,x:L,y:L)->P<乛>{let j=self;let 侔=if 乃{j.世.仔()}else{j.世.仺()};let q=j.亘(x,y)?;let 侱=j.map.串.test(q);if!侔||j.丄[z]<乤||侱!=乃||!j.丑[z].test(q)||!j.亏(z).test(q)||!j.什().test(q){return
None;}if 乃{j.乃.set(q);}else{j.乇.set(q);}j.丄[z]-=乤;Some(乛::仵)}}impl fmt::Debug for I{fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{let j=self;f.debug_struct("State").field("league",&j.世).field("to_act",&j.与).field("game_turn",&j.乞).field("gold",&j.丄).field("income",&j.上).field("outcome",&j.下).
finish_non_exhaustive()}}}pub mod 乓{type C=usize;type F=i64;type 丢=String;use std::str::Lines;use crate::丕::bits::{G,M,SIZE};use crate::丕::世::么;use crate::丕::map::乜;use
crate::丕::l::{I,T};const 佷:F=1000000000;const 仱:F=0;const 佽:F=1;const 侌:F=2;impl I{pub fn read(亢:&str,turn:&str,世:么,乞:u32,)->Result<I,丢>{let mut 串=Vec::new();let
mut lines=亢.lines();let count=伂(&mut lines,"mine spot count")?;for _ in 0..count{let[x,y]=乴(亨(&mut lines,"a mine spot")?,"a mine spot")?;串.push((付(x)?,付(y)?));}伹(lines,"the initial input")?;let
mut lines=turn.lines();let mut 伇=[0i64;4];for(value,what)in 伇.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[number]=乴(亨(&mut
lines,what)?,what)?;if number.abs()>佷{return Err(format!("{what}: {number} is out of range"));}*value=number;}let mut 丸=G::Z;let mut 丌=[G::Z;2];let mut
丂=[G::Z;2];for y in 0..SIZE{let row=亨(&mut lines,"a map row")?.trim();if row.len()!=SIZE{return Err(format!("map row {y} is not {SIZE} characters: {row:?}"));}for(x,仈)in
row.bytes().enumerate(){let q=y*SIZE+x;match 仈{b'#'=>丸.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let 举=C::from(仈==b'X'||仈==b'x');丌[举].set(q);if 仈.is_ascii_uppercase(){丂[举].set(q);}}_=>{return
Err(format!("map row {y} has {:?} at {x}",仈 as char))}}}}let 侗=伂(&mut lines,"building count")?;let mut 乐=Vec::new();for _ in 0..侗{let 乴=乴(亨(&mut lines,"a building")?,"a building")?;乐.push(乴);}let
会=伂(&mut lines,"unit count")?;let mut 七=Vec::new();for _ in 0..会{let 乴=乴(亨(&mut lines,"a unit")?,"a unit")?;七.push(乴);}伹(lines,"the turn's input")?;let
位=乐.iter().find(|伮|伮[0]==0&&伮[1]==仱).ok_or("the bot's own HQ is not among the buildings")?;let hq_cell=仿(位[2],位[3],丸)?;let z=T.iter().position(|&亡|亡==hq_cell).ok_or_else(||format!("the bot's HQ is at cell {hq_cell}, not in a corner"))?;let
mut 佗=G::Z;for(x,y)in 串{if 丸.test(y*SIZE+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佗.set(y*SIZE+x);}let 仸=|举:C|if 举==0{z}else{1-z};let
mut l=I{map:乜::俁(丸,佗),世,丑:[G::Z;2],丂:[G::Z;2],乃:G::Z,乇:G::Z,七:[G::Z;2],level:[G::Z;3],亁:G::Z,亳:[0;M],丄:[0;2],上:[0;2],与:z,乞,产:1,下:None,};for 举 in 0..2{let
owner=仸(举);l.丑[owner]=丌[举];l.丂[owner]=丂[举];l.丄[owner]=伇[2*举];l.上[owner]=伇[2*举+1];}let mut 佌=G::Z;for[owner,kind,x,y]in 乐{let 举=佑(owner)?;let q=仿(x,y,丸)?;if!丌[举].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佌.test(q){return Err(format!("two buildings at ({x}, {y})"));}佌.set(q);match kind{仱
if q==T[仸(举)]=>{}仱=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}佽=>l.乃.set(q),侌=>l.乇.set(q),_=>return Err(format!("unknown building type {kind}")),}}let
mut 丰=G::Z;let mut largest_id=0u32;for[owner,id,level,x,y]in 七{let 举=佑(owner)?;let q=仿(x,y,丸)?;if!丌[举].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
丰.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&level){return Err(format!("unit level {level} is not 1, 2 or 3"));}let id=u32::try_from(id).map_err(|_|format!("unit id {id} is out of range"))?;if
丰.iter().any(|丞|l.亳[丞]==id){return Err(format!("two units have id {id}"));}丰.set(q);l.七[仸(举)].set(q);l.level[level as C-1].set(q);l.亳[q]=id;largest_id=largest_id.max(id);}if!丰.is_empty(){l.产=largest_id.checked_add(1).ok_or_else(||format!("unit id {largest_id} is out of range"))?;}l.亁=l.七[z];Ok(l)}}fn
亨<'a>(lines:&mut Lines<'a>,what:&str)->Result<&'a str,丢>{lines.next().ok_or_else(||format!("the text ends before {what}"))}fn 伹(mut lines:Lines<'_>,what:&str)->Result<(),丢>{match
lines.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {what}: {line:?}")),None=>Ok(()),}}fn 乴<const N:C>(line:&str,what:&str)->Result<[F;N],丢>{let
mut 仫=line.split_whitespace();let mut 佢=[0i64;N];for value in&mut 佢{let word=仫.next().ok_or_else(||format!("{what}: expected {N} numbers in {line:?}"))?;*value=word.parse().map_err(|_|format!("{what}: {word:?} is not an integer"))?;}if
仫.next().is_some(){return Err(format!("{what}: expected {N} numbers in {line:?}"));}Ok(佢)}fn 伂(lines:&mut Lines<'_>,what:&str)->Result<C,丢>{let[count]=乴(亨(lines,what)?,what)?;C::try_from(count).ok().filter(|&count|count<=M).ok_or_else(||format!("{what}: {count} is out of range"))}fn
付(value:F)->Result<C,丢>{C::try_from(value).ok().filter(|&value|value<SIZE).ok_or_else(||format!("coordinate {value} is off the map"))}fn 仿(x:F,y:F,丸:G)->Result<C,丢>{let
q=付(y)?*SIZE+付(x)?;if 丸.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佑(owner:F)->Result<C,丢>{match owner{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {owner} is not 0 or 1")),}}}pub
use bits::{G,M,SIZE};pub use 三::{任,J};pub use 世::么;pub use l::{丠,I,T};}mod 丧{pub mod 乙{type C=usize;type L=u8;type F=i64;use crate::丕::l::乤;use crate::丕::{G,J,I,M,T,SIZE};pub
const 佮:F=15;pub const 佾:F=6;const 伤:F=10;const 侅:L=3;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct Q{pub open:F,pub 仗:F,pub 伷:L,pub 伈:F,pub 佂:F,pub
但:F,pub 佔:L,pub 伶:F,pub 伵:F,pub 伨:F,pub 亠:F,pub 亰:F,pub 丷:F,pub 亣:F,pub 予:F,}impl Q{pub const 价:Q=Q{open:3,仗:2,伷:12,伈:4,佂:佮,但:佾,佔:侅,伶:2,伵:2,伨:0,亠:0,亰:0,丷:0,亣:0,予:0,};pub
const fn 俙(self,亠:F,亰:F,丷:F)->Q{Q{亠,亰,丷,..self}}pub const fn 俘(self,亣:F,予:F)->Q{Q{亣,予,..self}}}fn 丹(q:C)->(L,L){((q%SIZE)as L,(q/SIZE)as L)}pub fn 亿(l:&I,from:G)->[L;M]{let
mut 乌=[L::MAX;M];let valid=l.map().valid;let mut 丶=from&valid;let mut 丰=丶;let mut d=0u8;while!丶.is_empty(){for q in 丶.iter(){乌[q]=d;}let next=(丶.且()&valid).丣(丰);丰|=next;丶=next;d=d.saturating_add(1);}乌}pub
fn 侤(l:&I)->L{let 乌=亿(l,l.丙(0));l.丙(1).iter().map(|q|乌[q]).min().unwrap_or(L::MAX)}struct 乽<'a>{乄:&'a Q,一:C,theirs:[L;M],亲:[L;M],free:G,乶:G,ours:G,串:G,}impl
乽<'_>{fn new<'a>(l:&I,乄:&'a Q)->乽<'a>{let 一=l.与();let 乃=亿(l,l.丙(一));let theirs=亿(l,l.丙(1-一));let 亲=亿(l,G::q(T[1-一]));let free=l.map().valid.丣(l.丙(0)|l.丙(1));let
mut 乶=G::Z;for q in free.iter(){if 乃[q]<=theirs[q]{乶.set(q);}}乽{乄,一,theirs,亲,free,乶,ours:l.丙(一),串:l.map().串,}}fn value(&self,q:C)->F{let j=self;let p=j.乄;let
open=F::from((G::q(q).且()&j.free).count());let 估=p.伷;let 仗=F::from(估)-F::from(j.theirs[q].min(估));let 亡=if p.伈>0{F::from(j.亲[q].min(30))/p.伈}else{0};let
丑=F::from((G::q(q).且()&j.ours).count());let line=if 丑<=1{p.亰}else{0};let 丷=if j.串.test(q){p.丷}else{0};let 俉=if open==0{p.亣}else{0};let 予=p.予*(22-F::from(j.亲[q].min(22)));p.open*open+p.仗*仗-亡+p.亠*丑-line+丷-俉+予}fn
丈(&self,丌:G)->Option<C>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伀(&mut self,q:C){self.free.clear(q);self.乶.clear(q);self.ours.set(q);}}fn
久(l:&mut I,三:J,丆:&mut Vec<J>)->bool{let 伧=l.两(三);if 伧{丆.push(三);}伧}fn 丏(l:&mut I,from:C,乮:C,丆:&mut Vec<J>)->bool{let Some((id,_))=l.今(from)else{return
false;};let(x,y)=丹(乮);久(l,J::乻{id,x,y},丆)}pub fn 俒(h:F,k:F)->F{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伤}pub fn 伔(l:&I,乄:&Q)->Vec<J>{伙(l,乄,中::二)}#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]pub
enum 中{二,乺{丝:L,七:L},}pub fn 伙(l:&I,乄:&Q,乘:中)->Vec<J>{let mut 丫=l.clone();let mut turn=乽::new(l,乄);let mut 丆=Vec::new();习(&mut 丫,&mut turn,&mut 丆);match
乘{中::二=>侹(&mut 丫,&mut turn,&mut 丆),中::乺{丝,七}=>{侣(&mut 丫,&mut turn,丝,七,&mut 丆)}}丆}fn 习(l:&mut I,turn:&mut 乽<'_>,丆:&mut Vec<J>){let 一=turn.一;let mut 仜:Vec<C>
=Vec::new();loop{let 东=l.为(一,1);let mut 京:Option<(u32,C)> =None;for from in l.亂().iter(){if 仜.contains(&from){continue;}let 乚=(G::q(from).且()&turn.free&东).count();if
乚==0{仜.push(from);}else if 京.is_none_or(|(n,_)|乚<n){京=Some((乚,from));}}let Some((_,from))=京 else{break;};match turn.丈(G::q(from).且()&turn.free&东){Some(乮)if
丏(l,from,乮,丆)=>turn.伀(乮),_=>仜.push(from),}}for from in 仜{if!l.亂().test(from){continue;}let 丐=[turn.乶,turn.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().乂(from,q),q))});if
let Some(丐)=丐{丏(l,from,丐,丆);}}}fn 侹(l:&mut I,turn:&mut 乽<'_>,丆:&mut Vec<J>){let 一=turn.一;let p=turn.乄;let h=(p.佂-F::from(l.乞())).max(p.但);let 俌=(F::from(侤(l))-p.伶).max(0)/p.伵.max(1)+p.伨;loop{let
丄=l.丄(一);let 七=l.会(一)as F;let 侽=F::from(turn.乶.count());let q=turn.丈(l.亏(一)&turn.free&l.为(一,1));let 佡=俒(h,(侽/(七+1)).min(俌));let 伯=q.is_some()&&丄>=伤&&佡>0;let
佇=4*h-乤;let 丷=佈(l,turn);let 侘=丷.is_some()&&丄>=乤&&佇>0;if 侘&&(!伯||佇>=佡){let(x,y)=丹(丷.unwrap_or_default());if 久(l,J::乹{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|伯)else{break;};let(x,y)=丹(q);if!久(l,J::乨{level:1,x,y},丆){break;}turn.伀(q);}}fn
佈(l:&I,turn:&乽<'_>)->Option<C>{let 一=turn.一;(l.map().串&l.丂(一)&l.什()).iter().filter(|&丷|turn.theirs[丷]>turn.乄.佔).min_by_key(|&丷|(l.map().乂(T[一],丷),丷))}fn
侣(l:&mut I,turn:&mut 乽<'_>,丝:L,七:L,丆:&mut Vec<J>,){let 一=turn.一;for _ in 0..丝{let Some(丷)=佈(l,turn)else{break;};let(x,y)=丹(丷);if!久(l,J::乹{x,y},丆){break;}}for
_ in 0..七{let Some(q)=turn.丈(l.亏(一)&turn.free&l.为(一,1))else{break;};let(x,y)=丹(q);if!久(l,J::乨{level:1,x,y},丆){break;}turn.伀(q);}}}pub mod 亥{use crate::丕::I;use
crate::也::chain::乯;pub fn 云(l:&I)->bool{if l.下().is_some(){return false;}let 一=l.与();let 丄=l.丄(一);let 丼=乯::new(l,一,true);l.丙(1-一).iter().any(|q|丼.万[q]<=丄)}}pub
mod 丅{type C=usize;type Y=u64;type K<T>=Vec<T>;use std::collections::HashMap;use std::time::Instant;use crate::丕::{J,I};use crate::丧::乙::{self,中,Q};use
crate::丧::亥;use crate::丧::伓::{self,井};use crate::丧::不::{self,乊};#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]pub struct O{pub 乎:u8,pub 乘:中,}const 仌:u8=u8::MAX;pub
const 仭:[中;8]=[中::二,中::乺{丝:0,七:0},中::乺{丝:0,七:1},中::乺{丝:0,七:2},中::乺{丝:0,七:仌,},中::乺{丝:1,七:0},中::乺{丝:1,七:1},中::乺{丝:1,七:仌,},];pub struct 亶<'a>{pub 专:&'a[Q],pub
主:Q,pub rest:O,pub 乑:Option<Instant>,pub 丱:乊,}impl 亶<'_>{fn 亐(&self)->bool{self.乑.is_some_and(|t|Instant::now()>=t)}fn 乄(&self,丏:O)->&Q{&self.专[C::from(丏.乎)%self.专.len()]}pub
fn 丁(&self,l:&I,丏:O)->K<J>{乙::伙(l,self.乄(丏),丏.乘)}pub fn 久(&self,l:&I,丅:&[O])->I{伓::久(l,&self.主,|i,s|{self.丁(s,丅.get(i).copied().unwrap_or(self.rest))})}pub
fn value(&self,l:&I,丅:&[O],丱:乊)->i64{不::不(&self.久(l,丅),l.与(),丱)}fn finish(&self,丨:K<(i64,K<O>)>)->K<O>{let mut 丈:Option<(i64,K<O>)> =None;for(value,丅)in
丨{if 丈.as_ref().is_none_or(|(乔,_)|value>*乔){丈=Some((value,丅));}}丈.map(|(_,丅)|丅).unwrap_or_default()}fn 乆(&self,l:&I)->K<(O,K<J>)>{let mut 丆:K<(O,K<J>)>
=K::new();let mut 乚:K<O> =仭.iter().map(|&乘|O{乎:0,乘}).collect();for 乎 in 1..self.专.len().min(C::from(u8::MAX)){乚.push(O{乎:乎 as u8,乘:中::二,});}for 丏 in 乚{let
丁=self.丁(l,丏);if!丆.iter().any(|(_,c)|*c==丁){丆.push((丏,丁));}}丆}}fn advance(l:&I,丁:&[J],主:&Q)->(I,bool){let mut next=l.clone();next.仅(丁);if next.下().is_some()||亥::云(&next){return(next,true);}let
乬=乙::伔(&next,主);next.仅(&乬);let 乫=next.下().is_some()||亥::云(&next)||next.乞()>井;(next,乫)}pub fn 侸(l:&I,丅:&[O],丮:&亶<'_>)->K<Y>{let 一=l.与();let mut 仁=vec![了(l,一)];let
mut 丘=l.clone();for turn in 0..井 as C{let 丏=丅.get(turn).copied().unwrap_or(丮.rest);let 丁=丮.丁(&丘,丏);let(next,乫)=advance(&丘,&丁,&丮.主);if 乫{break;}丘=next;仁.push(了(&丘,一));}仁}pub
fn 了(l:&I,一:C)->Y{let mut hash:Y=0xcbf29ce484222325;let mut 伍=|word:Y|hash=(hash^word).wrapping_mul(0x0100000001b3);for(俊,bits)in[l.丙(一),l.七(一),l.丝()].into_iter().enumerate(){for
q in bits.iter(){伍(((俊 as Y)<<8)|q as Y);}伍(0xff);}伍(l.丄(一)as Y);hash}pub fn 仼(l:&I,丮:&亶<'_>,丿:C)->K<O>{let 一=l.与();struct 仐{l:I,丅:K<O>,}let mut 丶=vec![仐{l:l.clone(),丅:K::new(),}];let
mut 个:K<(i64,K<O>)> =vec![(丮.value(l,&[],丮.丱),K::new())];'search:while!丶.is_empty(){let mut 丵:K<(i64,仐)> =K::new();let mut 丰:HashMap<Y,C> =HashMap::new();for
亃 in&丶{for(丏,丁)in 丮.乆(&亃.l){if 丮.亐(){if let Some((_,亙))=丵.iter().max_by_key(|(v,_)|*v){个.push((丮.value(l,&亙.丅,丮.丱),亙.丅.clone(),));}break 'search;}let(next,乫)=advance(&亃.l,&丁,&丮.主);let
mut 丅=亃.丅.clone();丅.push(丏);let value=不::不(&next,一,丮.丱);if 乫{个.push((value,丅));continue;}let key=了(&next,一);match 丰.get(&key){Some(&丩)if 丵[丩].0>=value=>{}Some(&丩)=>丵[丩]=(value,仐{l:next,丅}),None=>{丰.insert(key,丵.len());丵.push((value,仐{l:next,丅}));}}}}丵.sort_by(|a,b|b.0
.cmp(&a.0));丵.truncate(丿);丶=丵.into_iter().map(|(_,亃)|亃).collect();}丮.finish(个)}}pub mod 伏{type C=usize;type K<T>=Vec<T>;use std::time::Instant;use crate::丕::{J,I};use
crate::丧::乙::{中,Q};use crate::丧::丅::{self,了,亶,O};use crate::丧::伓::{伲,井};use crate::丧::不::乊;use crate::丧::乸::{self,乼};#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub
enum 乖{佦{丿:C,仧:C},佪,仒{丟:C,丿:C,乒:乼,},}pub struct 九{pub 乵:乖,pub 专:K<Q>,pub 主:Q,丅:K<O>,expected:K<u64>,亭:C,pub 丨:u32,乫:bool,}impl 九{pub fn new(乵:乖,专:K<Q>,主:Q)->九{九{乵,专,主,丅:K::new(),expected:K::new(),亭:0,丨:0,乫:false,}}pub
fn 侶(&mut self,l:&I)->bool{let j=self;let 乸=matches!(j.乵,乖::仒{..});let 侷=!乸&&j.丨>0&&j.亭>=j.expected.len();if!j.乫&&(侷||crate::丧::亥::云(l)){j.乫=true;}!j.乫}pub
fn turn(&mut self,l:&I,乑:Option<Instant>)->K<J>{let j=self;let 丮=亶{专:&j.专,主:j.主,rest:O{乎:0,乘:中::二,},乑,丱:乊::DEFAULT,};let 一=l.与();let 侲=j.expected.get(j.亭)==Some(&了(l,一));if!侲{let
first=j.丨==0;j.丅=match j.乵{乖::佦{丿,仧}=>{丅::仼(l,&丮,if first{丿}else{仧})}乖::佪=>{let 专=&j.专;let 乎=伲(l,专,&j.主,丮.丱,乑);let 丏=O{乎:乎 as u8,乘:中::二,};vec![丏;井 as C]}乖::仒{丟,丿,乒,}=>{let
乒=&乒;let rest=乸::侄;let mut 乆=乸::丅(l,&j.专,rest,丟,丿,乑,乒,);乆.resize(丟,rest);乆}};j.expected=match j.乵{乖::仒{..}=>俇(l,&j.专,&j.丅),_=>丅::侸(l,&j.丅,&丮),};j.亭=0;j.丨+=1;}let
丏=j.丅.get(j.亭).copied().unwrap_or(丮.rest);j.亭+=1;丮.丁(l,丏)}}fn 俇(l:&I,专:&[Q],丅:&[O])->K<u64>{let 一=l.与();let mut 仁=K::with_capacity(丅.len());let mut 丘=l.clone();for&丏
in 丅{if 丘.下().is_some(){break;}仁.push(了(&丘,一));丘=乸::advance(&丘,&乸::丁(&丘,专,丏));}仁}}pub mod 伓{use std::time::Instant;use crate::丕::{J,I};use crate::丧::乙::{self,Q};use
crate::丧::亥;use crate::丧::不::{self,乊};pub const 井:u32=20;pub fn 久(l:&I,主:&Q,mut ours:impl FnMut(usize,&I)->Vec<J>,)->I{let 一=l.与();let mut 丘=l.clone();let
mut 丟=0;loop{if 丘.下().is_some()||丘.乞()>井{return 丘;}if(丟>0||丘.与()!=一)&&亥::云(&丘){return 丘;}let 丁=if 丘.与()==一{丟+=1;ours(丟-1,&丘)}else{乙::伔(&丘,主)};丘.仅(&丁);}}pub
fn 俕(l:&I,乄:&Q,主:&Q,丱:乊)->i64{let 一=l.与();let end=久(l,主,|_,s|乙::伔(s,乄));不::不(&end,一,丱)}pub fn 伲(l:&I,专:&[Q],主:&Q,丱:乊,乑:Option<Instant>,)->usize{let mut
丈:Option<(i64,usize)> =None;for(index,乄)in 专.iter().enumerate(){if 丈.is_some()&&乑.is_some_and(|t|Instant::now()>=t){break;}let value=俕(l,乄,主,丱);if 丈.is_none_or(|(乔,_)|value>乔){丈=Some((value,index));}}丈.map_or(0,|(_,index)|index)}}pub
mod 不{type C=usize;type F=i64;use crate::丕::{G,丠,I,M,T};use crate::也::chain::乯;use crate::也::丒::亜;pub const 佧:F=1;pub const 侍:F=3;pub const 仳:F=1000;const
亵:C=5;const 佩:C=8;#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct 乊{pub 丟:F,}impl 乊{pub const DEFAULT:乊=乊{丟:侍};}#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub
struct 亗{pub 争:F,pub 丄:F,pub 上:F,pub 仙:F,pub 亽:F,pub 仂:F,}impl 亗{pub fn value(&self)->F{let j=self;j.争+j.丄+j.上-j.仙-j.亽-j.仂}}pub fn 俓(l:&I,z:C)->F{(l.七(z)&l.丂(z)).iter().filter_map(|q|l.亓(q)).map(|level|10*F::from(level)).sum()}pub
fn 不(l:&I,z:C,丱:乊)->F{作(l,z,丱).value()-作(l,1-z,丱).value()}pub fn 作(l:&I,z:C,丱:乊)->亗{match l.下(){Some(丠::乗(伜))if 伜!=z=>{return 亗{仂:仳,..亗::default()}}Some(_)=>return
亗::default(),None=>{}}let 丄=l.丄(z);let 上=l.上(z);let 仡=亱(l,z);亗{争:佧*佣(l,z)/2,丄,上:丱.丟*上,仙:if 仡{0}else{仙(l,z,丱)},亽:if 丄+上<0{俓(l,z)}else{0},仂:if 仡{仳}else{0},}}fn
乆(l:&I,from:G)->[u8;M]{let valid=l.map().valid;let mut 乌=[u8::MAX;M];let mut 丶=from&valid;let mut 丰=丶;let mut d=0u8;while!丶.is_empty(){for q in 丶.iter(){乌[q]=d;}let
next=(丶.且()&valid).丣(丰);丰|=next;丶=next;d=d.saturating_add(1);}乌}pub fn 佣(l:&I,z:C)->F{let ours=乆(l,l.丂(z));let theirs=乆(l,l.丂(1-z));l.map().valid.iter().map(|q|match
ours[q].cmp(&theirs[q]){_ if ours[q]==u8::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}fn 侢(l:&I,丛:C)->F{l.丄(丛)+if
l.与()==丛{0}else{l.上(丛)}}pub fn 亱(l:&I,z:C)->bool{let 丛=1-z;let 与=l.与()==丛;乯::new(l,丛,与).万[T[z]]<=侢(l,丛)}pub fn 仙(l:&I,伛:C,丱:乊)->F{if l.下().is_some(){return
0;}let mut next=l.clone();if next.与()==伛{next.亝();if next.下().is_some(){return 0;}}let 丛=1-伛;let 丝=|s:&I|F::from((s.丝()&s.丂(伛)).count());let 侮=丝(&next);let
mut 亊=0;for 丒 in 亜(&next,亵,next.丄(丛),佩){let mut 临=next.clone();for&三 in&丒.丁{临.两(三);}let 仂=if 临.下()==Some(丠::乗(丛)){仳}else{丱.丟*(丒.丌+4*(侮-丝(&临)))+丒.七};亊=亊.max(仂);}亊}}pub
mod 乸{type C=usize;type F=i64;type 乀=Instant;type K<T>=Vec<T>;use std::collections::HashMap;use std::time::Instant;use crate::丕::{G,J,I,M,T};use crate::丧::乙::{self,Q};use
crate::丧::丅::{了,O,仭};pub const 來:[Q;27]=专();pub const 侄:O=O{乎:16,乘:crate::丧::乙::中::二,};const fn 专()->[Q;27]{let 亠=[0,2,4];let 亰=[0,3,6];let 予=[0,1,2];let
mut 丆=[Q::价;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let 亣=if g>0||t>0{4}else{0};丆[i]=Q::价.俙(亠[g],亰[t],0).俘(亣,予[w]);i+=1;}丆}#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub
struct 件{pub 丌:F,pub 上:F,pub 丄:F,pub 乲:F,pub 丒:F,pub 仉:F,pub 丼:F,pub 争:F,}#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct 乼{pub 丌:F,pub 上:F,pub 丄:F,pub
丒:F,pub 乲:F,pub 仉:F,pub 丼:F,pub 争:F,}pub fn 便(l:&I,z:C)->[F;M]{struct 亷{丂:G,乁:[u16;M],丯:[u16;M],size:[F;M],丒:[F;M],乭:u16,}fn 五(d:&mut 亷,v:C){d.乭+=1;d.乁[v]=d.乭;d.丯[v]=d.乭;d.size[v]=1;for
w in(G::q(v).且()&d.丂).iter(){if d.乁[w]==0{五(d,w);d.size[v]+=d.size[w];d.丯[v]=d.丯[v].min(d.丯[w]);if d.丯[w]>=d.乁[v]{d.丒[v]+=d.size[w];}}else{d.丯[v]=d.丯[v].min(d.乁[w]);}}}let
mut d=亷{丂:l.丂(z),乁:[0;M],丯:[0;M],size:[0;M],丒:[0;M],乭:0,};let 亅=T[z];if d.丂.test(亅){五(&mut d,亅);}d.丒[亅]=0;d.丒}pub fn 伕(l:&I,z:C)->件{let 丂=l.丂(z);let 丒=便(l,z);let
亲=乙::亿(l,G::q(T[1-z]));let mut 丆=件{丌:F::from(丂.count()),上:l.上(z),丄:l.丄(z),丼:F::MAX,争:crate::丧::不::佣(l,z)/2,..件::default()};for q in 丂.iter(){if 丒[q]>0{丆.乲+=1;丆.丒=丆.丒.max(丒[q]);}if
q!=T[z]&&(G::q(q).且()&丂).count()<=1{丆.仉+=1;}丆.丼=丆.丼.min(F::from(亲[q]));}丆}pub fn value(l:&I,z:C,乒:&乼)->F{let s=伕(l,z);let o=乒;o.丌*s.丌+o.上*s.上+o.丄*s.丄-o.丒*s.丒-o.乲*s.乲-o.仉*s.仉-o.丼*s.丼.min(30)+o.争*s.争}pub
fn advance(l:&I,丁:&[J])->I{let mut next=l.clone();next.仅(丁);if next.下().is_none(){next.仅(&[]);}next}pub fn 丁(l:&I,专:&[Q],丏:O)->K<J>{乙::伙(l,&专[C::from(丏.乎)%专.len()],丏.乘,)}pub
fn 久(l:&I,专:&[Q],丅:&[O],rest:O,丟:C,)->I{let mut 丘=l.clone();for turn in 0..丟{if 丘.下().is_some(){break;}let 丏=丅.get(turn).copied().unwrap_or(rest);丘=advance(&丘,&丁(&丘,专,丏));}丘}fn
乆(l:&I,专:&[Q])->K<(O,K<J>)>{let mut 丆:K<(O,K<J>)> =K::new();for 乎 in 0..专.len().min(C::from(u8::MAX)){for&乘 in&仭{let 丏=O{乎:乎 as u8,乘,};let c=丁(l,专,丏);if!丆.iter().any(|(_,丞)|*丞==c){丆.push((丏,c));}}}丆}pub
fn 仼(l:&I,专:&[Q],rest:O,丟:C,丿:C,乑:Option<乀>,乒:&乼,)->K<O>{let 一=l.与();let 亐=||乑.is_some_and(|t|乀::now()>=t);let 仆=|丅:&[O]|value(&久(l,专,丅,rest,丟),一,乒);let
mut 丈:(F,K<O>)=(仆(&[]),K::new());let mut 丶:K<(I,K<O>)> =vec![(l.clone(),K::new())];for 依 in 0..丟{let mut 丵:K<(F,I,K<O>)> =K::new();let mut 丰:HashMap<u64,C>
=HashMap::new();for(亃,丅)in&丶{if 亐(){break;}for(丏,c)in 乆(亃,专){let next=advance(亃,&c);let mut 亙=丅.clone();亙.push(丏);let v=value(&next,一,乒);let key=了(&next,一);match
丰.get(&key){Some(&丩)if 丵[丩].0>=v=>{}Some(&丩)=>丵[丩]=(v,next,亙),None=>{丰.insert(key,丵.len());丵.push((v,next,亙));}}}}if 丵.is_empty(){break;}丵.sort_by(|a,b|b.0
.cmp(&a.0));丵.truncate(丿);let 乳=依+1==丟||亐();for(i,(_,_,丅))in 丵.iter().take(if 乳{丿}else{1}).enumerate(){if i>0&&亐(){break;}let v=仆(丅);if v>丈.0{丈=(v,丅.clone());}}if
乳{break;}丶=丵.into_iter().map(|(_,s,p)|(s,p)).collect();}丈.1}pub fn 丅(l:&I,专:&[Q],rest:O,丟:C,侭:C,乑:Option<乀>,乒:&乼,)->K<O>{let 一=l.与();let 仆=|丅:&[O]|value(&久(l,专,丅,rest,丟),一,乒);let
mut 丈:(F,K<O>)=(仆(&[]),K::new());let mut 丿=2;loop{let 並=仼(l,专,rest,丟,丿,乑,乒);let v=仆(&並);if v>丈.0{丈=(v,並);}let 亐=乑.is_some_and(|t|乀::now()>=t);if 亐||(乑.is_none()&&丿>=侭){break;}丿*=2;}丈.1}}}mod
佄{pub mod io{type C=usize;type 丢=String;use std::io::{BufRead,Write};use std::time::Instant;const 佭:C=4+12;pub fn 丟(仝:impl BufRead,mut 以:impl Write,mut
answer:impl FnMut(&str,&str,u32,Instant)->丢,)->Result<(),丢>{let mut lines=Lines{佐:仝};let Some(first)=lines.line()?else{return Ok(());};let mut 亢=format!("{first}\n");lines.take(count(&first,"mine spot count")?,&mut
亢)?;let mut turn=0;loop{let Some(first)=lines.line()?else{return Ok(());};let 互=Instant::now();turn+=1;let mut 乓=format!("{first}\n");lines.take(佭-1,&mut
乓)?;for what in["building count","unit count"]{let line=lines.佒()?;let n=count(&line,what)?;乓.push_str(&line);乓.push('\n');lines.take(n,&mut 乓)?;}let answer=answer(&亢,&乓,turn,互);writeln!(以,"{answer}").and_then(|()|以.flush()).map_err(|err|format!("turn {turn}: cannot write the answer: {err}"))?;}}struct
Lines<R>{佐:R,}impl<R:BufRead>Lines<R>{fn line(&mut self)->Result<Option<丢>,丢>{let mut line=丢::new();match self.佐.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(err)=>Err(format!("cannot read input: {err}")),}}fn
佒(&mut self)->Result<丢,丢>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:C,乓:&mut String)->Result<(),丢>{for
_ in 0..n{乓.push_str(&self.佒()?);乓.push('\n');}Ok(())}}fn count(line:&str,what:&str)->Result<C,丢>{match line.trim().parse::<C>(){Ok(n)if n<=佶=>Ok(n),_=>Err(format!("bad {what} {line:?}")),}}const
佶:C=144;}mod 丅{type C=usize;type F=i64;type P<T>=Option<T>;use crate::丕::map::义;use crate::丕::l::乤;use crate::丕::{G,J,丠,I,M,T,SIZE};const 侊:F=3;const 佰:F=2;const
佺:C=16;pub fn 丅(l:&I,丆:&mut Vec<J>){let z=l.与();let mut 乵=乖{s:l.clone(),z,丛:1-z,丐:T[1-z],仛:T[z],turn:l.乞(),丆,};乵.run();}struct 乖<'a>{s:I,z:C,丛:C,丐:C,仛:C,turn:u32,丆:&'a
mut Vec<J>,}fn 丹(q:C)->(u8,u8){((q%SIZE)as u8,(q/SIZE)as u8)}fn 伎(id:u32,q:C)->J{let(x,y)=丹(q);J::乻{id,x,y}}fn 佟(level:u8,q:C)->J{let(x,y)=丹(q);J::乨{level,x,y}}impl
乖<'_>{fn run(&mut self){let j=self;if j.仟(){return;}j.习();if j.仟(){return;}j.乐();j.俐();}fn 乌(&self,from:C,乮:C)->F{F::from(self.s.map().乂(from,乮))}fn 两(&mut
self,三:J)->bool{let 个=self.s.两(三);if 个{self.丆.push(三);}个}fn 亴(&self,业:&I)->bool{业.下()==Some(丠::乗(self.z))}fn 仟(&mut self)->bool{let j=self;if j.s.下().is_some(){return
false;}for q in j.s.亂().iter(){if j.乌(q,j.丐)!=1{continue;}let Some((id,_))=j.s.今(q)else{continue;};let 三=伎(id,j.丐);let mut 业=j.s.clone();if 业.两(三)&&j.亴(&业){return
j.两(三);}}let Some(chain)=j.侚()else{return false;};let mut 业=j.s.clone();if!chain.iter().all(|&三|业.两(三))||!j.亴(&业){return false;}for 三 in chain{j.两(三);}true}fn
侚(&self)->P<Vec<J>>{let j=self;const 伞:F=F::MAX;let max=j.s.世().乪();let 东:Vec<G> =(1..=max).map(|level|j.s.为(j.z,level)).collect();let entry=|q:C|{东.iter().position(|bits|bits.test(q)).map(|index|(index
as u8+1,10*(index as F+1)))};let 丂=j.s.丂(j.z);let mut 万=[伞;M];let mut 严=[C::MAX;M];let mut 个=[false;M];for q in 丂.iter(){万[q]=0;}loop{let mut next:P<C>
=None;for q in 0..M{if!个[q]&&万[q]<伞&&next.is_none_or(|n|万[q]<万[n]){next=Some(q);}}let Some(q)=next else{break;};个[q]=true;if q==j.丐{break;}for&丽 in&j.s.map().且[0][q]{if
丽==义{break;}let 丽=C::from(丽);if 个[丽]||丂.test(丽){continue;}let Some((_,亄))=entry(丽)else{continue;};if 万[q]+亄<万[丽]{万[丽]=万[q]+亄;严[丽]=q;}}}if 万[j.丐]>j.s.丄(j.z){return
None;}let mut chain=Vec::new();let mut q=j.丐;while!丂.test(q){let(level,_)=entry(q)?;chain.push(佟(level,q));q=严[q];}chain.reverse();Some(chain)}fn 亀(&self,临:&I)->F{let(一,亇)=(self.z,self.丛);let
严=&self.s;let count=|bits:G|F::from(bits.count());let 丑=count(临.丙(一))-count(严.丙(一));let 仪=count(严.丙(亇))-count(临.丙(亇));let 仡=count(严.丂(亇))-count(临.丂(亇));let
仞=严.亦(亇)-临.亦(亇);let 丝=count(严.丝()&严.丙(亇))-count(临.丝()&临.丙(亇));12*丑+20*仪+8*(仡-仪).max(0)+5*仞+25*丝}fn 佘(&self,q:C)->F{let j=self;let 丷=j.s.map().串.test(q);if
丷&&j.s.世().仔()&&!j.s.丙(j.z).test(q){20}else{0}}fn 伺(&self,limit:F)->bool{let j=self;j.s.七(j.丛).iter().any(|q|j.乌(q,j.仛)<=limit)}fn 习(&mut self){let j=self;let
mut 七:Vec<(F,C)> =j.s.亂().iter().map(|q|(j.乌(q,j.丐),q)).collect();七.sort_unstable();for(侨,q)in 七{if j.s.下().is_some(){return;}let Some((id,level))=j.s.今(q)else{continue;};let
东=j.s.为(j.z,level);let 丽=j.s.map().且[j.z][q];let mut 丈:P<(F,C)> =None;for&next in 丽.iter().take_while(|&&next|next!=义){let next=C::from(next);if!东.test(next){continue;}let
mut 业=j.s.clone();if!业.两(伎(id,next)){continue;}let value=if j.亴(&业){100000}else{j.亀(&业)+10*(侨-j.乌(next,j.丐))+j.佘(next)};if 丈.is_none_or(|(乔,_)|value>乔){丈=Some((value,next));}}if
let Some((value,next))=丈{if value>0{j.两(伎(id,next));}}}}fn 乐(&mut self){let j=self;if j.s.下().is_some(){return;}if j.伺(侊){j.俎();}else{j.丝();}}fn 俎(&mut
self){let j=self;if!j.s.世().仺()||j.s.丄(j.z)<乤{return;}let 丽=j.s.map().且[j.z][j.仛];let 丌:Vec<C> =丽.iter().take_while(|&&q|q!=义).map(|&q|C::from(q)).collect();let
乃=j.s.丙(j.z);if 丌.iter().any(|&q|乃.test(q)&&j.s.乣().test(q)){return;}for q in 丌{let(x,y)=丹(q);if j.两(J::事{x,y}){return;}}}fn 丝(&mut self){let j=self;if!j.s.世().仔(){return;}while
j.s.丄(j.z)>=乤{let free=j.s.map().串&j.s.丙(j.z)&j.s.丂(j.z)&j.s.什();let Some(q)=free.iter().min_by_key(|&q|(j.乌(q,j.仛),q))else{return;};let(x,y)=丹(q);if!j.两(J::乹{x,y}){return;}}}fn
俐(&mut self){let j=self;let 体=if j.turn<=3{0}else{5};for _ in 0..佺{if j.s.下().is_some(){return;}let 丄=j.s.丄(j.z);if 丄<10{return;}let 侪=j.伺(佰);let 代=j.s.亏(j.z);let
mut 丈:P<(F,J)> =None;let mut 伖:P<(F,J)> =None;let mut 伭=G::Z;for level in 1..=j.s.世().乪(){let 东=j.s.为(j.z,level);let 乚=(代&东).丣(伭);伭=东;let 亄=10*F::from(level);if
亄>丄{continue;}for q in 乚.iter(){let 三=佟(level,q);let mut 业=j.s.clone();if!业.两(三){continue;}let 伊=业.丄(j.z);let 上=业.上(j.z);let 仞=j.s.亦(j.丛)-业.亦(j.丛);let
侒=if 仞>0&&侪{伊+上>=0}else{上>=0&&(伊>=体||仞>0)};if!侒{continue;}let 亀=j.亀(&业);let value=亀+j.佘(q)-6*j.乌(q,j.丐)-10*(F::from(level)-1);if 亀>0{if 丈.is_none_or(|(乔,_)|value>乔){丈=Some((value,三));}}else
if 上>=3&&伊>=体+10&&level==1{if 伖.is_none_or(|(乔,_)|value>乔){伖=Some((value,三));}}}}let Some((_,三))=丈.or(伖)else{return;};if!j.两(三){return;}}}}}use crate::丕::{J,I};pub
fn 伿(l:&I,丆:&mut Vec<J>){丆.clear();丅::丅(l,丆);}}mod 也{pub mod chain{type C=usize;type F=i64;type U=bool;use crate::丕::map::义;use crate::丕::{G,J,丠,I,M,SIZE};pub
const 买:F=F::MAX;const 佹:C=8;pub struct 乯{pub 万:[F;M],pub 严:[u8;M],}pub fn 亚(l:&I,z:C,习:U,丐:C)->F{let mut 万=买;乿(l,z,习,|丏,new|{if new.test(丐){万=10*丏 as
F;}万!=买});万}pub fn 仦(l:&I,z:C,习:U,両:F)->G{let mut 云=G::Z;乿(l,z,习,|丏,new|{if 10*丏 as F>両{return true;}云|=new;false});云.丣(l.丂(z))}fn 乿(l:&I,z:C,习:U,mut 五:impl
FnMut(C,G)->U,){let max=l.世().乪();let mut entry=[G::Z;3];let mut 丰=G::Z;for level in 1..=max{let bits=l.为(z,level);entry[C::from(level)-1]=bits.丣(丰);丰|=bits;}let
mut open=[l.丂(z),G::Z,G::Z,G::Z];if 习&&z==l.与(){for q in l.亂().iter(){if let Some(level)=l.亓(q).filter(|&level|level<=max){open[0]|=G::q(q).且()&l.为(z,level);}}}let
mut 个=G::Z;for 丏 in 0..{let new=open[丏%4].丣(个);open[丏%4]=G::Z;if 五(丏,new){return;}个|=new;let next=new.且().丣(个);for(侓,丌)in entry.iter().enumerate(){open[(丏+侓+1)%4]|=next&*丌;}if
open.iter().all(|bits|bits.丣(个).is_empty()){break;}}}impl 乯{pub fn new(l:&I,z:C,习:U)->乯{let max=C::from(l.世().乪());let mut 东=[G::Z;3];for(index,bits)in
东.iter_mut().enumerate().take(max){*bits=l.为(z,index as u8+1);}let 丂=l.丂(z);let 且=&l.map().且[0];let mut 万=[买;M];let mut 严=[义;M];let mut open=[0u8;M];let
mut count=0;let mut 佖=G::Z;for q in 丂.iter(){万[q]=0;open[count]=q as u8;count+=1;}if 习&&z==l.与(){for from in l.亂().iter(){let Some(level)=l.亓(from)else{continue;};let
乆=G::q(from).且()&东[C::from(level)-1];for q in 乆.iter(){if 万[q]!=0{万[q]=0;open[count]=q as u8;count+=1;}}}}while count>0{let mut 丈=0;for index in 1..count{let(a,b)=(C::from(open[index]),C::from(open[丈]));if(万[a],a)<(万[b],b){丈=index;}}let
q=C::from(open[丈]);count-=1;open[丈]=open[count];佖.set(q);for&next in&且[q]{if next==义{break;}let next=C::from(next);if 佖.test(next)||丂.test(next){continue;}let
Some(level)=东[..max].iter().position(|bits|bits.test(next))else{continue;};let 佞=万[q]+10*(level as F+1);if 佞<万[next]{if 万[next]==买{open[count]=next as
u8;count+=1;}万[next]=佞;严[next]=q as u8;}}}乯{万,严}}pub fn path(&self,丐:C)->Vec<C>{let mut path=Vec::new();if 丐>=M||self.万[丐]==买{return path;}let mut q=丐;while
self.严[q]!=义{path.push(q);q=C::from(self.严[q]);}path.reverse();path}}#[derive(Clone,Debug,PartialEq,Eq)]pub struct 从{pub 丁:Vec<J>,pub 万:F,}fn 侬(l:&I,z:C,q:C)->Option<u8>{(1..=l.世().乪()).find(|&level|l.为(z,level).test(q))}fn
仧(丫:&mut I,一:C,亟:C,両:F,丁:&mut Vec<J>,乢:&mut i64,)->U{loop{if 丫.下()==Some(丠::乗(一)){return true;}if 丫.丙(一).test(亟)&&丫.丂(一).test(亟){return true;}let 丼=乯::new(丫,一,false);let
万=丼.万[亟];if 万==买||*乢+万>両{return false;}let Some(&q)=丼.path(亟).first()else{return false;};let Some(level)=侬(丫,一,q)else{return false;};let 三=J::乨{level,x:(q%SIZE)as
u8,y:(q/SIZE)as u8,};if!丫.两(三){return false;}*乢+=10*F::from(level);丁.push(三);}}fn 伬(l:&I,一:C,侦:&[C],両:F)->Option<从>{let mut 丫=l.clone();let mut 丁=Vec::new();let
mut 乢=0;for&亟 in 侦{if!仧(&mut 丫,一,亟,両,&mut 丁,&mut 乢){return None;}}(乢<=両).then_some(从{丁,万:乢,})}pub fn 亼(l:&I,丐:C,両:F)->Option<从>{if 両<0||l.下().is_some()||丐>=M||l.map().丸.test(丐){return
None;}let 一=l.与();if l.丙(一).test(丐)&&l.丂(一).test(丐){return Some(从{丁:Vec::new(),万:0,});}let mut 丈=伬(l,一,&[丐],両);let first=乯::new(l,一,false);let mut 乣:Vec<C>
=(l.乣()&l.丙(1-一)).iter().filter(|&乇|乇!=丐&&first.万[乇]!=买).collect();乣.sort_by_key(|&乇|(l.map().乂(乇,丐),乇));乣.truncate(佹);for 乇 in 乣{if let Some(chain)=伬(l,一,&[乇,丐],両){if
丈.as_ref().is_none_or(|乔|chain.万<乔.万){丈=Some(chain);}}}丈}}pub mod 丒{type C=usize;type F=i64;use std::collections::hash_map::DefaultHasher;use std::collections::HashMap;use
std::hash::BuildHasherDefault;use crate::丕::{G,J,丠,I,SIZE};pub const 佸:C=20000;#[derive(Clone,Debug,PartialEq,Eq)]pub struct 仏{pub 丁:Vec<J>,pub 万:F,pub
丌:F,pub 七:F,pub 乐:F,pub 上:F,pub 不:F,}#[derive(Clone,Copy)]struct 仲{丌:F,七:F,乐:F,}fn 佁(l:&I,z:C)->仲{let 丂=l.丂(z);let 七=l.七(z)&丂;let value=七.iter().filter_map(|q|l.亓(q)).map(|level|10*F::from(level)).sum();仲{丌:F::from(丂.count()),七:value,乐:F::from(((l.丝()|l.乣())&丂).count()),}}struct
亖{一:C,们:G,严:仲,上:F,交:C,両:F,亩:C,path:Vec<J>,並:Vec<仏>,index:HashMap<Key,C,BuildHasherDefault<DefaultHasher>>,}type Key=[u64;3];fn 俗(mut key:Key,q:C)->Key{key[q/64]|=1<<(q%64);key}impl
亖{fn 伸(&mut self,丫:&I,乳:Option<C>,仪:Key,万:F,)->bool{let j=self;let 一=j.一;let 乚=match 乳{None=>丫.亏(一),Some(q)=>G::q(q).且(),}&j.们;let 乚=乚.丣(丫.丙(一));let mut
东=[G::Z;3];for level in 1..=丫.世().乪(){东[C::from(level)-1]=丫.为(一,level);}for q in 乚.iter(){let max=C::from(丫.世().乪());let Some(index)=东[..max].iter().position(|bits|bits.test(q))else{continue;};let
level=index as u8+1;let 亄=10*F::from(level);if 万+亄>j.両{continue;}if j.亩>=佸{return false;}j.亩+=1;let mut next=丫.clone();let 三=J::乨{level,x:(q%SIZE)as u8,y:(q/SIZE)as
u8,};if!next.两(三){continue;}j.path.push(三);let set=俗(仪,q);j.侺(&next,set,万+亄);let 侜=j.path.len()<j.交&&next.下().is_none();let 侥=!侜||j.伸(&next,Some(q),set,万+亄);j.path.pop();if!侥{return
false;}}true}fn 侺(&mut self,临:&I,set:Key,万:F){let j=self;let 仄=1-j.一;let now=佁(临,仄);let 丌=j.严.丌-now.丌;let 七=j.严.七-now.七;if 丌<=0&&七<=0{return;}let 乐=j.严.乐-now.乐;let
上=临.上(j.一)-j.上;let 俚=临.下()==Some(丠::乗(j.一));let 不=if 俚{F::MAX/2}else{3*丌+七+10*乐+2*上-万};if let Some(&丩)=j.index.get(&set){if 万>=j.並[丩].万{return;}}let 丒=仏{丁:j.path.clone(),万,丌,七,乐,上,不,};match
j.index.get(&set){Some(&丩)=>j.並[丩]=丒,None=>{j.index.insert(set,j.並.len());j.並.push(丒);}}}}pub fn 亜(l:&I,交:C,両:F,limit:C)->Vec<仏>{乍(l,交,両,limit).0}fn 乍(l:&I,交:C,両:F,limit:C)->(Vec<仏>,C){if
l.下().is_some()||交==0||limit==0{return(Vec::new(),0);}let 一=l.与();let 仄=1-一;let mut run=亖{一,们:l.丙(仄)|l.丙(仄).且(),严:佁(l,仄),上:l.上(一),交,両,亩:0,path:Vec::new(),並:Vec::new(),index:HashMap::default(),};run.伸(l,None,[0;3],0);let
亩=run.亩;let mut 並=run.並;並.sort_by(|a,b|b.不.cmp(&a.不).then(a.万.cmp(&b.万)));並.truncate(limit);(並,亩)}}pub mod 乩{type F=i64;use crate::丕::{丠,I,T};use crate::也::chain::{亚,买};pub
const 亻:F=1000000;const 伟:F=200;fn 乶(l:&I,z:usize)->F{let 丂=l.丂(z);let 七:F=(l.七(z)&丂).iter().filter_map(|q|l.亓(q)).map(|level|10*F::from(level)).sum();l.丄(z)+3*l.上(z)+4*F::from(丂.count())+七+10*F::from((l.乣()&丂).count())}fn
住(l:&I,z:usize)->F{let 万=亚(l,z,false,T[1-z]);if 万==买{return-伟;}(l.丄(z)+l.上(z)-万).clamp(-伟,0)}pub fn 伻(l:&I,z:usize)->F{let 丞=1-z;match l.下(){Some(丠::乗(伜))=>return
if 伜==z{亻}else{-亻},Some(丠::伝)=>return 0,None=>{}}let mut 不=乶(l,z)-乶(l,丞);if z==l.与(){let mut next=l.clone();next.亝();if next.下().is_none()&&亚(&next,丞,true,T[z])<=next.丄(丞){不-=亻/2;}}不+住(l,z)-住(l,丞)}}}
