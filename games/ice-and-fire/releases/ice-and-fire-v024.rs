// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 串=u32;type U=u64;type 仛=i32;type C=i64;type 亝=f64;type Q=bool;type 丱=String;type 乓=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侐:Q=true;fn 俙(l:&J,q:F,他:C,丯:G,丧:C)->Q{let
丁=l.东();let 乵=1-丁;let 佡=侐&&丧<=l.丈(乵)+2*l.且(乵).max(0);let 仼=G::q(q)|G::q(q).丑();if!佡&&(仼&丯).is_empty(){return false;}let(x,y)=互(q);let mut 为=l.clone();if!为.个(K::亃{x,y}){return
false;}if 倈(&为,丁)<他{return true;}佡&&伕(&为,乵,false,Z[丁])>丧}const 侫:I=2;const 侖:F=4;const 伵:F=5;#[derive(Clone)]enum 丳{亟(K),伂(L<K>),}impl 丳{fn 丄(&self)->&[K]{match
self{丳::亟(三)=>std::slice::from_ref(三),丳::伂(丄)=>丄,}}}fn 互(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 作(l:&J,丆:&mut Vec<丳>){丆.clear();if l.丗().is_some(){return;}let
丁=l.东();let 乘=l.两().也();let mut 丣=[G::T;4];for 一 in 1..=乘{丣[F::from(一)]=l.丽(丁,一);}for 丌 in l.二().iter(){let Some((丐,一))=l.付(丌)else{continue;};let 乥=G::q(丌).丑()&丣[F::from(一)];for
乮 in 乥.iter(){let(x,y)=互(乮);丆.push(丳::亟(K::乱{丐,x,y}));}}let 丈=l.丈(丁);let 七=l.七(丁);let 亰=l.亗(丁);let 倏=match 侫{0=>亰,1=>{let 伌=l.map().valid.丝(七).丑()&七;亰.丝(七)|伌}_=>亰.丝(七),};for
一 in 1..=乘{let 万=10*C::from(一);if 丈<万{break;}let mut 不=倏&丣[F::from(一)];for 俞 in 1..一{不=不.丝(丣[F::from(俞)]);}for q in 不.iter(){let(x,y)=互(q);丆.push(丳::亟(K::乖{一,x,y}));}}if
丈>=亁{let 乫=l.丠(丁)&亰&l.亥();if l.两().仢(){for q in(乫&l.map().丰).iter(){let(x,y)=互(q);丆.push(丳::亟(K::亂{x,y}));}}if l.两().代(){let 伌=l.map().valid.丝(l.丠(丁)).丑();let
不=(乫&伌).丝(l.map().丰);let 丯=丯(l,1-丁);let 他=C::from((丯&l.七(丁)).count());let 丧=伕(l,1-丁,false,Z[丁]);for q in 不.iter().filter(|&q|俙(l,q,他,丯,丧)){let(x,y)=互(q);丆.push(丳::亟(K::亃{x,y}));}}}for
丞 in 佔(l,伵,丈,侖){if 丞.丄.len()>1{丆.push(丳::伂(丞.丄));}}if let Some(丧)=伐(l,Z[1-丁],丈){if 丧.丄.len()>1{丆.push(丳::伂(丧.丄));}}}const 侣:I=2;const 侵:C=5;fn 乙(l:&J,z:F)->G{伥(l,z,侣)}fn
伥(l:&J,z:F,丌:I)->G{let mut 乙=G::T;if 丌>3{return 乙;}for q in(l.丅(z)&l.七(z)).iter(){if l.亚(q).is_some_and(|一|一>=丌){乙|=G::q(q);}}乙}fn 丯(l:&J,乎:F)->G{let 丫=l.丈(乎)+l.且(乎);修(l,乎,乙(l,乎),丫)}fn
仉(l:&J,z:F)->G{丯(l,1-z)&l.七(z)}fn 倈(l:&J,z:F)->C{C::from(仉(l,z).count())}const 侠:C=4;fn 信(不:C)->C{不/侠}fn 丟(l:&J,z:F,仉:G)->C{let 七=l.七(z);let 丟=(l.丟()&七).丝(仉);信(C::from(七.count()))*C::from(丟.count())}const
侱:C=2;fn 來(l:&J,z:F)->C{let 七=l.七(z);let[a,b,c]=七.仹();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(佂-1);侽.with(|伎|{let mut 伎=伎.borrow_mut();let 价=&mut 伎[与];if 价.0==七&&价.1==z as I+1{return 价.2;}let 丛=倕(七,Z[z]);*价=(七,z as I+1,丛);丛})}const
佂:F=1<<12;thread_local!{static 侽:std::cell::RefCell<L<(G,I,C)>> =std::cell::RefCell::new(vec![(G::T,0,0);佂]);}fn 倕(七:G,乭:F)->C{struct 亿{七:G,久:[I;M],丵:[I;M],size:[I;M],亐:I,乸:C,仱:C,}fn
些(d:&mut 亿,v:F)->I{d.亐+=1;let 久=d.亐;d.久[v]=久;let mut 丵=久;let mut 伮=1u8;let mut 丞=0u8;for w in(G::q(v).丑()&d.七).iter(){if d.久[w]==0{let 佾=些(d,w);伮+=佾;丵=丵.min(d.丵[w]);if
d.丵[w]>=久{丞+=佾;}}else{丵=丵.min(d.久[w]);}}d.丵[v]=丵;d.size[v]=伮;if 丞>0&&d.久[v]>1{d.乸+=1;d.仱=d.仱.max(C::from(丞));}伮}if!七.test(乭){return 0;}let mut d=亿{七,久:[0;M],丵:[0;M],size:[0;M],亐:0,乸:0,仱:0,};些(&mut
d,乭);d.乸+d.仱}fn 俵(l:&J,z:F)->C{侱*(來(l,1-z)-來(l,z))}fn 何(l:&J,z:F)->C{if l.丈(z)+l.且(z)>=0{return 0;}(l.丅(z)&l.七(z)).iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum()}const
侕:C=15;const 侻:[C;2]=[3,1];const 侹:[C;2]=[4,20];fn 倄(l:&J,z:F)->[C;2]{let mut 丒=[0;2];for q in(l.丅(z)&l.七(z)).iter(){match l.亚(q){Some(2)=>丒[0]+=1,Some(3)=>丒[1]+=1,_=>{}}}丒}fn
佽(l:&J,z:F)->C{let[侃,倉]=倄(l,z);let 佈=|一:F|(3-侻[一])*侹[一];-侕*侃+佈(0)*侃+佈(1)*倉}const 侏:C=0;const 侒:C=2;const 侑:C=100;fn 佻(仰:C)->C{if 仰<0{return 0;}侏+侒*仰.min(侑)/2}const
伺:I=2;fn 优(l:&J,z:F)->C{let 乙=[伥(l,z,伺),伥(l,1-z,伺),];let(丘,亭)=俒(l,z,乙);let 丘=丘+佻(亭[0])-佻(亭[1]);if l.丗().is_some(){return 丘;}let 仏=仉(l,z);let 乿=仉(l,1-z);let
他=C::from(乿.count())-C::from(仏.count());丘-2*何(l,z)+2*何(l,1-z)+俵(l,z)+佽(l,z)-佽(l,1-z)+侵*他/4+丟(l,z,仏)-丟(l,1-z,乿)}const 侟:F=8;const 侬:F=3;const 侗:F=32;const
侭:U=300;const 例:亝=0.75;struct 亅{乾:P<乓>,井:P<乓>,仌:U,}#[derive(Default)]struct 佀{丄:L<K>,伔:F,乶:U,世:F,丮:F,位:F,乷:F,}const 侼:Q=true;#[derive(Clone)]struct 乕{丄:L<K>,众:C,}struct
亠{丄:L<K>,end:J,first:P<U>,专:P<L<乕>>,}type 什=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct 伄{丁:F,limit:亅,乶:U,介:什,亘:P<L<丳>>,}const 伴:Q=true;const
侈:Q=false;const 伳:P<I> =None;fn 俅(l:&J,世:&[乕],丆:&mut Vec<丳>){if!伴&&伳.is_none(){return;}let 仔=世.iter().map(|乚|乚.丄.as_slice());俆(l,仔,(伴,侈,伳),丆,);}fn 俆<'a>(l:&J,仔:impl
Iterator<Item=&'a[K]>,(乒,丑,一):(Q,Q,P<I>),丆:&mut Vec<丳>,){let 丁=l.东();let 丷=l.七(丁);let mut 仇=G::T;for 乚 in 仔{for 三 in 乚{if let K::乖{x,y,..}|K::乱{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丷.test(q){仇|=G::q(q);}}}}if 仇.is_empty(){return;}let 乫=l.亥()&丷;let 丈=l.丈(丁);let 佲=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丄()==[三]){丆.push(丳::亟(三));}};if
乒&&丈>=亁&&l.两().代(){let mut 不=仇;if 丑{不|=仇.丑();}for q in(不&乫).丝(l.map().丰).iter(){let(x,y)=互(q);佲(K::亃{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().也()){if
丈>=10*C::from(一){for q in(仇&乫).iter(){let(x,y)=互(q);佲(K::乖{一,x,y},丆);}}}}fn 伋(l:&mut J,丄:&[K])->Q{丄.iter().all(|&三|l.个(三))}fn 乌(丄:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丄{let(们,a,b,c)=match*三{K::乱{丐,x,y}=>(1,U::from(丐),x,y),K::乖{一,x,y}=>(2,U::from(一),x,y),K::亂{x,y}=>(3,0,x,y),K::亃{x,y}=>(4,0,x,y),};let 亾=们|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亾).wrapping_mul(0x0100000001b3);}hash}fn
佅(乗:&J,乚:&[K])->J{let mut 丂=乗.clone();丂.仆();for&三 in 乚{丂.个(三);}丂.仆();丂}fn 仮(l:&J)->P<L<K>>{let 丁=l.东();let 丏=Z[1-丁];let 亽=|両:&J|両.丗()==Some(么::乩(丁));for
q in l.二().iter(){if l.map().乆(q,丏)!=1{continue;}let Some((丐,_))=l.付(q)else{continue;};let(x,y)=互(丏);let 三=K::乱{丐,x,y};let mut 両=l.clone();if 両.个(三)&&亽(&両){return
Some(vec![三]);}}let 丧=伐(l,丏,l.丈(丁))?;let mut 両=l.clone();(伋(&mut 両,&丧.丄)&&亽(&両)).then_some(丧.丄)}impl 伄{fn 乤(&self)->Q{self.乶>=self.limit.仌||self.limit.乾.is_some_and(|t|乓::now()>=t)}fn
丘(&mut self,乗:&J,世:&[乕],伍:C)->C{self.乶+=1;self.伩(乗,世,伍)}fn 伩(&self,乗:&J,世:&[乕],伍:C)->C{if 乗.丗()==Some(么::乩(self.丁)){return 仡;}if 世.is_empty(){return 优(乗,self.丁);}let
mut 仺=C::MAX;for 乚 in 世{仺=仺.min(优(&佅(乗,&乚.丄),self.丁));if 仺<=伍{break;}}仺}#[allow(clippy::too_many_arguments)]fn 俨(&mut self,乪:&J,交:C,乏:&[丳],亵:&[(C,F)],丼:&mut[Q],世:&[乕],亦:&mut
什,伟:Q,)->(P<(F,J,C)>,Q){let mut 下=None;let mut 令=交;for&(仲,与)in 亵{if 丼[与]{continue;}if 伟&&仲!=C::MAX&&仲<=令-交{return(下,true);}if self.乤(){return(下,false);}丼[与]=true;let
丄=乏[与].丄();let mut 丂=乪.clone();if!伋(&mut 丂,丄){亦.insert(乌(丄),C::MIN/4);continue;}let 丘=self.丘(&丂,世,令);亦.insert(乌(丄),丘-交);if 丘>令{令=丘;下=Some((与,丂,丘));}}(下,false)}fn
伓(&mut self,乭:&J,世:&[乕],仂:&[U],俈:Q)->亠{let j=self;let mut 乊=None;let mut 乪=乭.clone();let mut 丄=L::new();let mut 交=j.丘(&乪,世,C::MIN);let mut 亦=什::default();let
mut 乏=L::new();let mut 亵=L::new();while 乪.丗().is_none()&&!j.乤(){if 丄.is_empty()&&俈{match&j.亘{Some(亘)=>乏.clone_from(亘),None=>{作(&乪,&mut 乏);j.亘=Some(乏.clone());}}}else{作(&乪,&mut
乏);}俅(&乪,世,&mut 乏);亵.clear();for(与,乲)in 乏.iter().enumerate(){let k=乌(乲.丄());let 仲=亦.get(&k).or(j.介.get(&k)).copied();亵.push((仲.unwrap_or(C::MAX),与));}亵.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丼=vec![false;乏.len()];if 丄.is_empty(){for(与,乲)in 乏.iter().enumerate(){丼[与]=仂.contains(&乌(乲.丄()));}}let mut 下=None;for
伟 in[true,false]{let(丨,俽)=j.俨(&乪,交,&乏,&亵,&mut 丼,世,&mut 亦,伟,);下=丨;if 下.is_some()||!俽{break;}}let Some((与,丂,丘))=下 else{break;};if 丄.is_empty(){乊=Some(乌(乏[与].丄()));}丄.extend_from_slice(乏[与].丄());乪=丂;交=丘;}for(k,乬)in
亦{let 佞=j.介.entry(k).or_insert(乬);*佞=(*佞).max(乬);}亠{丄,end:乪,first:乊,专:None,}}fn 仔(&self,乗:&J)->L<L<K>>{let mut 丂=乗.clone();丂.仆();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 习=丂.东();let 丈=丂.丈(习);if let Some(丧)=伐(&丂,Z[self.丁],丈){丆.push(丧.丄);}for 丞 in 佔(&丂,伵,丈,侬){丆.push(丞.丄);}let mut 佝=L::new();俘(&丂,&mut
佝);丆.push(佝);let mut 乿=伄{丁:习,limit:亅{仌:侭,..self.limit},乶:0,介:什::default(),亘:None,};丆.push(乿.伓(&丂,&[],&[],true).丄);丆.retain(|丄|!丄.is_empty());丆}fn 但(&self,乗:&J)->L<乕>{self.仔(乗).into_iter().map(|丄|{let
众=优(&佅(乗,&丄),self.丁);乕{丄,众}}).collect()}fn 乷(&mut self,乭:&J,乲:&丳,世:&[乕])->P<亠>{let mut 为=乭.clone();if!伋(&mut 为,乲.丄()){return None;}self.乶+=1;let 乛=self.伓(&为,世,&[],false);let
mut 丄=乲.丄().to_vec();丄.extend_from_slice(&乛.丄);Some(亠{丄,end:乛.end,first:Some(乌(乲.丄())),专:None,})}fn 余(&self,丕:&亠,世:&[乕])->C{let 丘=self.伩(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>丘.min(self.伩(&丕.end,专,C::MIN)),_=>丘,}}}fn
俰(丨:&[乕],世:&mut Vec<乕>)->Q{let 並:L<_> =世.iter().map(|r|r.丄.clone()).collect();for 乚 in 丨{if 世.iter().any(|r|r.丄==乚.丄){continue;}世.push(乚.clone());}世.sort_by_key(|乚|乚.众);世.truncate(侟);世.iter().map(|r|&r.丄).ne(&並)}fn
倇(l:&J,乁:亅,亴:&mut 了)->佀{let 丁=l.东();let mut 中=佀::default();if l.丗().is_some(){return 中;}if let Some(丄)=仮(l){中.丄=丄;return 中;}let 主=|俷:亝|match(乁.乾,乁.井){(Some(乾),Some(井))=>Some(乾+(井-乾).mul_f64(俷)),_=>None,};if
亴.俫(l){中.丄=亴.上(l,主(0.4));return 中;}let mut 乍=伄{丁,limit:亅{乾:主(例),..乁},乶:0,介:什::default(),亘:None,};let mut 世=L::new();let mut 丮:L<亠> =L::new();let mut 仂=L::new();let
mut 仧=false;let mut 乷:L<丳> =L::new();while 中.伔<侗&&(丮.is_empty()||!乍.乤()){let 丕=if 仧{let Some(乲)=乷.pop()else{break;};match 乍.乷(l,&乲,&世){Some(丕)=>丕,None=>continue,}}else{乍.伓(l,&世,&仂,true)};中.伔+=1;let
佷=丮.iter().any(|p|p.丄==丕.丄);let 乊=丕.first;let 佼=乍.乤();let mut 丕=丕;let mut 佨=false;if!佼&&!佷{let 丨=乍.但(&丕.end);佨=俰(&丨,&mut 世);丕.专=Some(丨);}if!佷{丮.push(丕);}if
佼{break;}if!佨&&!仧{match 乊{Some(乊)=>仂.push(乊),None=>{仧=true;乷=乍.亘.clone().unwrap_or_default();乷.retain(|a|!仂.contains(&乌(a.丄())));let 乬=|a:&丳|{乍.介.get(&乌(a.丄())).copied().unwrap_or(C::MIN)};乷.sort_by_key(乬);}}}}中.乷=F::from(仧);中.乶=乍.乶;中.世=世.len();中.丮=丮.len();let
佣=||乁.井.is_some_and(|t|乓::now()>=t);let mut 亸:L<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佣(){break;}亸[与]=Some(乍.余(&丮[与],&世));}let
亶=|亸:&[P<C>]|{let mut 下:P<(C,F)> =None;for(与,丘)in 亸.iter().enumerate().rev(){if let Some(丘)=*丘{if 下.is_none_or(|(亀,_)|丘>亀){下=Some((丘,与));}}}下};let mut
下=亶(&亸);乍.limit.乾=乁.井;while let Some((_,与))=下{if!侼||丮[与].专.is_some()||佣(){break;}let 丨=乍.但(&丮[与].end);丮[与].专=Some(丨);中.位+=1;亸[与]=Some(乍.余(&丮[与],&世));下=亶(&亸);}if
let Some((_,与))=下{let mut 丬=l.clone();for&三 in&丮[与].丄{if 丬.个(三){中.丄.push(三);}}}中}fn 俪(乁:&dyn Fn(乓,串)->亅,mut 亴:了,伞:impl BufRead,仳:impl Write,)->Result<(),丱>{丿(伞,仳,|亪,乜,上,亙|match
J::read(亪,乜,乧::仾,上){Ok(l)=>{let 中=倇(&l,乁(亙,上),&mut 亴);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,中.伔,中.乶,中.世,中.位,中.乷,亙.elapsed());係(&中.丄)}Err(五)=>{eprintln!("plan: cannot read the input: {}",五);"WAIT".to_string()}},)}const
侉:亝=45.0;const 侌:亝=800.0;const 侲:亝=0.6;fn 乁(亙:乓,上:串)->亅{if let Some(仌)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亅{乾:None,井:None,仌,};}let 件=倊().min(100.0);let 俣=if 上==1{侌}else{侉};let 丫=Duration::from_secs_f64(俣/1000.0*件);亅{乾:Some(亙+丫.mul_f64(侲)),井:Some(亙+丫),仌:U::MAX,}}const
侪:F=9;const 侤:亞=亞{不:3,且:1,丈:1,丞:2,乸:2,仗:2,丯:6,仙:1,};const 侩:I=I::MAX;const 侦:C=2;const 侧:C=18;const 侥:C=0;const 侨:F=32;fn 亴()->了{俦(侨)}fn 俦(书:F)->了{let
乽=亡::仠{丿:侪,书,乑:侤,};let 业=侰.iter().map(|&乢|Y{伬:侩,仃:侦,伜:侧,伉:侥,..乢}).collect();了::new(乽,业,Y::伆)}fn main(){let 倁=io::stdin();let 倂=io::stdout();if let Err(五)=俪(&乁,亴(),倁.lock(),倂.lock()){eprintln!("plan: {}",五);std::process::exit(1);}}const
侶:&str="CG_TIME_SCALE";fn 倊()->亝{俧(std::env::var(侶).ok().as_deref())}fn 俧(丛:P<&str>)->亝{丛.and_then(|乜|乜.trim().parse::<亝>().ok()).filter(|件|件.is_finite()&&*件>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乔:F=4*O;const 乨:U=(1<<乔)-1;const 仜:U=1|1<<12|1<<24|1<<36;const 仿:U=仜<<11;const 伾:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侇:G=G([乨;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乔]>>(q%乔))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乔]|=1<<(q%乔);}#[inline]fn clear(&mut self,q:F){self.0[q/乔]&=!(1<<(q%乔));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仹(self)->[U;3]{self.0}#[inline]fn count(self)->串{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丝(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->份{份{伢:self.0,主:0,}}#[inline]fn 俼(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伾)<<(乔-O)),(b>>O)|((c&伾)<<(乔-O)),c>>O,])}#[inline]fn 俹(self)->G{let[a,b,c]=self.0;G([(a<<O)&乨,((b<<O)&乨)|(a>>(乔-O)),((c<<O)&乨)|(b>>(乔-O)),])}#[inline]fn
俺(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仿&乨,(b>>1)&!仿&乨,(c>>1)&!仿&乨,])}#[inline]fn 俻(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仜&乨,(b<<1)&!仜&乨,(c<<1)&!仜&乨,])}#[inline]fn
丑(self)->G{let j=self;j.俼()|j.俹()|j.俺()|j.俻()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侇.丝(self)}}struct 份{伢:[U;3],主:F,}impl Iterator for 份{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.主<3{let 仯=j.伢[j.主];if 仯!=0{j.伢[j.主]=仯&(仯-1);return Some(j.主*乔+仯.trailing_zeros()as F);}j.主+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乱{丐:串,x:I,y:I},乖{一:I,x:I,y:I},亂{x:I,y:I},亃{x:I,y:I},}impl K{fn 倌(&self)->丱{match*self{K::乱{丐,x,y}=>format!("MOVE {} {x} {y}",丐),K::乖{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::亂{x,y}=>format!("BUILD MINE {x} {y}"),K::亃{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
係(丄:&[K])->丱{if 丄.is_empty(){return "WAIT".to_string();}let mut line=丱::with_capacity(16*丄.len());for(与,三)in 丄.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&三.倌());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乧{佃,倛,侾,#[default]仾,}impl 乧{fn 也(self)->I{match self{乧::佃=>1,_=>3,}}fn 仢(self)->Q{matches!(self,乧::侾|乧::仾)}fn 代(self)->Q{self==乧::仾}}const 乞:I=I::MAX;struct
买{丹:G,valid:G,丰:G,乆:Box<[I]>,丑:[[[I;4];M];2],}thread_local!{static 侔:RefCell<P<Arc<买>>> =const{RefCell::new(None)};}impl 买{fn 俸(丹:G,丰:G)->Arc<买>{侔.with(|乹|{let
mut 乹=乹.borrow_mut();if let Some(京)=乹.as_ref(){if 京.丹==丹&&京.丰==丰{return Arc::clone(京);}}let 京=Arc::new(买::build(丹,丰));*乹=Some(Arc::clone(&京));京})}fn build(丹:G,丰:G)->买{let
乯=!丹;const 侳:[[(仛,仛);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丑=[[[乞;4];M];2];for(z,乥)in 侳.iter().enumerate(){for q in
乯.iter(){let(x,y)=((q%O)as 仛,(q/O)as 仛);let mut 丒=0;for&(俏,俐)in 乥{let(佪,佫)=(x+俏,y+俐);if(0..O as 仛).contains(&佪)&&(0..O as 仛).contains(&佫){let 丂=佫 as F*O+佪
as F;if 乯.test(丂){丑[z][q][丒]=丂 as I;丒+=1;}}}}}let mut 乆=vec![乞;M*M].into_boxed_slice();let mut 伨=[0u8;M];for 丌 in 乯.iter(){let 于=&mut 乆[丌*M..(丌+1)*M];于[丌]=0;伨[0]=丌
as I;let(mut 会,mut 估)=(0,1);while 会<估{let q=伨[会]as F;会+=1;for&丂 in&丑[0][q]{if 丂!=乞&&于[丂 as F]==乞{于[丂 as F]=于[q]+1;伨[估]=丂;估+=1;}}}}买{丹,valid:乯,丰,乆,丑,}}#[inline]fn
乆(&self,丌:F,乮:F)->I{self.乆[丌*M+乮]}}const 侞:串=100;const 亁:C=15;const 侺:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乩(F),伷,}enum
九{伃,伻,似,}#[derive(Clone)]struct J{map:Arc<买>,两:乧,专:[G;2],七:[G;2],丷:G,乇:G,丅:[G;2],一:[G;3],事:G,亼:[串;M],丈:[C;2],且:[C;2],东:F,亊:串,亱:串,丗:P<么>,}fn 今(专:G,亩:F)->G{if!专.test(亩){return
G::T;}let mut 仓=G::q(亩);loop{let 丂=(仓|仓.丑())&专;if 丂==仓{return 仓;}仓=丂;}}impl J{fn 东(&self)->F{self.东}fn 亊(&self)->串{self.亊}fn 丗(&self)->P<么>{self.丗}fn 两(&self)->乧{self.两}fn
亯(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伱(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&买{&self.map}fn 丠(&self,z:F)->G{self.专[z]}fn 七(&self,z:F)->G{self.七[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 付(&self,q:F)->P<(串,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亼[q],j.伡(q)))}fn 二(&self)->G{self.丅[self.东]&self.事}fn
丟(&self)->G{self.丷}fn 乒(&self)->G{self.乇}fn 亚(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伡(q))}fn 倀(&mut self,z:F){let j=self;j.七=[今(j.专[0],Z[0]),今(j.专[1],Z[1])];let
俋=j.丅[z].丝(j.七[z]);j.伪(俋);j.低();j.丈[z]+=j.且[z];if j.丈[z]<0{j.丈[z]=0;j.伪(j.丅[z]);}j.事|=j.丅[z];}fn 低(&mut self){let j=self;for z in 0..2{let 七=j.七[z];let
丅=七&j.丅[z];let mut 且=C::from(七.count())+4*C::from((七&j.丷).count());for(一,倓)in 侺.iter().enumerate(){且-=倓*C::from((丅&j.一[一]).count());}j.且[z]=且;}}fn 仆(&mut
self){let j=self;if j.丗.is_some(){return;}if j.东==1{if j.亊>=侞{j.丗=Some(match j.亯(0).cmp(&j.亯(1)){std::cmp::Ordering::Greater=>么::乩(0),std::cmp::Ordering::Less=>么::乩(1),std::cmp::Ordering::Equal=>么::伷,});return;}j.亊+=1;}j.东=1-j.东;j.倀(j.东);}fn
佰(&mut self,丄:&[K]){let j=self;if j.丗.is_some(){return;}let 伙=j.亱;for&三 in 丄{if j.俎(三,伙){continue;}j.个(三);if j.丗.is_some(){return;}}j.仆();}fn 俎(&self,三:K,伙:串)->Q{let
j=self;match 三{K::乱{丐,x,y}=>{j.亣(x,y).is_none()||丐>=伙||j.佚(j.东,丐).is_none()}K::乖{一,x,y}=>{j.亣(x,y).is_none()||一==0||一>j.两.也()}K::亂{x,y}|K::亃{x,y}=>{j.亣(x,y).is_none()}}}#[inline]fn
亣(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丹.test(y*O+x)).then_some(y*O+x)}fn 佚(&self,z:F,丐:串)->P<F>{self.丅[z].iter().find(|&q|self.亼[q]==丐)}#[inline]fn
伡(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 俄(&self,习:F)->G{let 乒=self.乇&self.专[习];let 七=self.七[习];乒|(七&(七&乒).丑())}fn
丽(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 习=1-z;let 丅=j.丅[0]|j.丅[1];let 俑=j.亥();let 們=match
一{1 if j.两==乧::佃=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::T,};let 丽=俑|j.专[习].丝(們);if 一<3{丽.丝(j.俄(习))}else{丽}}fn 亗(&self,z:F)->G{self.七[z]|self.七[z].丑()}fn
亥(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 俛=G::q(Z[0])|G::q(Z[1]);j.map.valid.丝(丅|j.丷|j.乇|俛)}fn 个(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.东;let 亓=match 三{K::乖{一,x,y}=>j.倎(z,一,x,y),K::乱{丐,x,y}=>j.俢(z,丐,x,y),K::亂{x,y}=>j.build(z,true,x,y),K::亃{x,y}=>j.build(z,false,x,y),};let
Some(亓)=亓 else{return false;};match 亓{九::伃=>{}九::伻=>j.七[z]=今(j.专[z],Z[z]),九::似=>{j.七=[今(j.专[0],Z[0]),今(j.专[1],Z[1])];}}j.低();if!j.专[0].test(Z[0]){j.丗=Some(么::乩(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(么::乩(0));}true}fn 伪(&mut self,不:G){let j=self;j.丅[0]=j.丅[0].丝(不);j.丅[1]=j.丅[1].丝(不);for 一 in&mut j.一{*一=一.丝(不);}j.事=j.事.丝(不);}fn
使(&mut self,z:F,q:F)->九{let j=self;j.伪(G::q(q));j.丷.clear(q);j.乇.clear(q);let 习=1-z;if j.专[习].test(q){j.专[习].clear(q);j.专[z].set(q);九::似}else if j.专[z].test(q){九::伃}else{j.专[z].set(q);九::伻}}fn
倎(&mut self,z:F,一:I,x:I,y:I)->P<九>{let j=self;if 一==0||一>j.两.也(){return None;}let q=j.亣(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亗(z).test(q)||!j.丽(z,一).test(q){return
None;}let 亓=j.使(z,q);j.佮(z,一,q,j.亱);j.亱=j.亱.saturating_add(1);j.丈[z]-=万;Some(亓)}fn 佮(&mut self,z:F,一:I,q:F,丐:串){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亼[q]=丐;}fn
俢(&mut self,z:F,丐:串,x:I,y:I)->P<九>{let j=self;let 丏=j.亣(x,y)?;let 丌=j.佚(z,丐)?;if!j.事.test(丌){return None;}let 一=j.伡(丌);let 丣=j.丽(z,一);let 乀=j.map.丑[z][丌];if
乀.contains(&(丏 as I))&&!丣.test(丏){return None;}let mut 下=j.map.乆(丌,丏);let mut 仄=None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let 丂=F::from(丂);let 乆=j.map.乆(丂,丏);if
乆<下&&丣.test(丂){下=乆;仄=Some(丂);}}let 仄=仄?;let 亓=j.使(z,仄);j.丅[z].clear(丌);j.一[F::from(一)-1].clear(丌);j.事.clear(丌);j.佮(z,一,仄,丐);Some(亓)}fn build(&mut self,z:F,丷:Q,x:I,y:I)->P<九>{let
j=self;let 俁=if 丷{j.两.仢()}else{j.两.代()};let q=j.亣(x,y)?;let 俤=j.map.丰.test(q);if!俁||j.丈[z]<亁||俤!=丷||!j.专[z].test(q)||!j.亗(z).test(q)||!j.亥().test(q){return
None;}if 丷{j.丷.set(q);}else{j.乇.set(q);}j.丈[z]-=亁;Some(九::伃)}}const 侙:C=1000000000;const 伀:C=0;const 価:C=1;const 侷:C=2;impl J{fn read(亪:&str,上:&str,两:乧,亊:串,)->Result<J,丱>{let
mut 丰=L::new();let mut 严=亪.lines();let 丒=伖(&mut 严,"mine spot count")?;for _ in 0..丒{let[x,y]=乻(亲(&mut 严,"a mine spot")?,"a mine spot")?;丰.push((仦(x)?,仦(y)?));}佗(严,"the initial input")?;let
mut 严=上.lines();let mut 伛=[0i64;4];for(丛,丶)in 伛.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伧]=乻(亲(&mut 严,丶)?,丶)?;if 伧.abs()>侙{return
Err(format!("{}: {} is out of range",丶,伧));}*丛=伧;}let mut 丹=G::T;let mut 不=[G::T;2];let mut 七=[G::T;2];for y in 0..O{let 于=亲(&mut 严,"a map row")?.trim();if
于.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,于));}for(x,仕)in 于.bytes().enumerate(){let q=y*O+x;match 仕{b'#'=>丹.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乂=F::from(仕==b'X'||仕==b'x');不[乂].set(q);if 仕.is_ascii_uppercase(){七[乂].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仕 as char))}}}}let
俇=伖(&mut 严,"building count")?;let mut 乐=L::new();for _ in 0..俇{let 乻=乻(亲(&mut 严,"a building")?,"a building")?;乐.push(乻);}let 伱=伖(&mut 严,"unit count")?;let
mut 丅=L::new();for _ in 0..伱{let 乻=乻(亲(&mut 严,"a unit")?,"a unit")?;丅.push(乻);}佗(严,"the turn's input")?;let 佭=乐.iter().find(|佊|佊[0]==0&&佊[1]==伀).ok_or("the bot's own HQ is not among the buildings")?;let
你=伏(佭[2],佭[3],丹)?;let z=Z.iter().position(|&亩|亩==你).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",你))?;let mut 佹=G::T;for(x,y)in 丰{if
丹.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佹.set(y*O+x);}let 伇=|乂:F|if 乂==0{z}else{1-z};let mut l=J{map:买::俸(丹,佹),两,专:[G::T;2],七:[G::T;2],丷:G::T,乇:G::T,丅:[G::T;2],一:[G::T;3],事:G::T,亼:[0;M],丈:[0;2],且:[0;2],东:z,亊,亱:1,丗:None,};for
乂 in 0..2{let 乡=伇(乂);l.专[乡]=不[乂];l.七[乡]=七[乂];l.丈[乡]=伛[2*乂];l.且[乡]=伛[2*乂+1];}let mut 佬=G::T;for[乡,们,x,y]in 乐{let 乂=佴(乡)?;let q=伏(x,y,丹)?;if!不[乂].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佬.test(q){return Err(format!("two buildings at ({x}, {y})"));}佬.set(q);match 们{伀
if q==Z[伇(乂)]=>{}伀=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}価=>l.丷.set(q),侷=>l.乇.set(q),_=>return Err(format!("unknown building type {}",们)),}}let
mut 乃=G::T;let mut 仭=0u32;for[乡,丐,一,x,y]in 丅{let 乂=佴(乡)?;let q=伏(x,y,丹)?;if!不[乂].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乃.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丐=串::try_from(丐).map_err(|_|format!("unit id {} is out of range",丐))?;if
乃.iter().any(|丩|l.亼[丩]==丐){return Err(format!("two units have id {}",丐));}乃.set(q);l.丅[伇(乂)].set(q);l.一[一 as F-1].set(q);l.亼[q]=丐;仭=仭.max(丐);}if!乃.is_empty(){l.亱=仭.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仭))?;}l.事=l.丅[z];Ok(l)}}fn
亲<'a>(严:&mut Lines<'a>,丶:&str)->Result<&'a str,丱>{严.next().ok_or_else(||format!("the text ends before {}",丶))}fn 佗(mut 严:Lines<'_>,丶:&str)->Result<(),丱>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丶)),None=>Ok(()),}}fn 乻<const N:F>(line:&str,丶:&str)->Result<[C;N],丱>{let
mut 仹=line.split_whitespace();let mut 侅=[0i64;N];for 丛 in&mut 侅{let 亾=仹.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丶))?;*丛=亾.parse().map_err(|_|format!("{}: {:?} is not an integer",丶,亾))?;}if
仹.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丶));}Ok(侅)}fn 伖(严:&mut Lines<'_>,丶:&str)->Result<F,丱>{let[丒]=乻(亲(严,丶)?,丶)?;F::try_from(丒).ok().filter(|&丒|丒<=M).ok_or_else(||format!("{}: {} is out of range",丶,丒))}fn
仦(丛:C)->Result<F,丱>{F::try_from(丛).ok().filter(|&丛|丛<O).ok_or_else(||format!("coordinate {} is off the map",丛))}fn 伏(x:C,y:C,丹:G)->Result<F,丱>{let q=仦(y)?*O+仦(x)?;if
丹.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佴(乡:C)->Result<F,丱>{match 乡{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乡)),}}const
侎:C=15;const 侢:C=6;const 佁:C=10;const 侯:I=3;#[derive(Clone,Copy)]struct Y{open:C,以:C,佑:I,伝:C,伜:C,佥:C,伬:I,佐:C,住:C,伉:C,亨:C,人:C,丸:C,享:C,云:C,仃:C,}impl Y{const
伆:Y=Y{open:3,以:2,佑:12,伝:4,伜:侎,佥:侢,伬:侯,佐:2,住:2,伉:0,亨:0,人:0,丸:0,享:0,云:0,仃:0,};const fn 候(self,亨:C,人:C,丸:C)->Y{Y{亨,人,丸,..self}}const fn 倘(self,享:C,云:C)->Y{Y{享,云,..self}}}fn
仚(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仅(l:&J,丌:G)->[I;M]{let mut 义=[I::MAX;M];let 乯=l.map().valid;let mut 之=丌&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.丑()&乯).丝(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn 俕(l:&J)->I{let 义=仅(l,l.丠(0));l.丠(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亇<'a>{乢:&'a Y,丁:F,theirs:[I;M],亻:[I;M],free:G,乎:G,ours:G,丰:G,丅:G,}impl 亇<'_>{fn new<'a>(l:&J,乢:&'a Y)->亇<'a>{let 丁=l.东();let 丷=仅(l,l.丠(丁));let 乿=仅(l,l.丠(1-丁));let
亻=仅(l,G::q(Z[1-丁]));let 乫=l.map().valid.丝(l.丠(0)|l.丠(1));let mut 乎=G::T;for q in 乫.iter(){if 丷[q]<=乿[q]{乎.set(q);}}亇{乢,丁,theirs:乿,亻,free:乫,乎,ours:l.丠(丁),丰:l.map().丰,丅:l.丅(丁),}}fn
value(&self,q:F)->C{let j=self;let p=j.乢;let 举=C::from((G::q(q).丑()&j.free).count());let 佌=p.佑;let 以=C::from(佌)-C::from(j.theirs[q].min(佌));let 亩=if p.伝>0{C::from(j.亻[q].min(30))/p.伝}else{0};let
专=C::from((G::q(q).丑()&j.ours).count());let line=if 专<=1{p.人}else{0};let 丸=if j.丰.test(q){p.丸}else{0};let 倅=if 举==0{p.享}else{0};let 云=p.云*(22-C::from(j.亻[q].min(22)));let
仃=p.仃*C::from((G::q(q).丑()&j.丅).count());p.open*举+p.以*以-亩+p.亨*专-line+丸-倅+云-仃}fn 下(&self,不:G)->P<F>{不.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
伒(&mut self,q:F){let j=self;j.free.clear(q);j.乎.clear(q);j.ours.set(q);j.丅.set(q);}}fn 仑(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 佄=l.个(三);if 佄{丆.push(三);}佄}fn
丙(l:&mut J,丌:F,乮:F,丆:&mut Vec<K>)->Q{let Some((丐,_))=l.付(丌)else{return false;};let(x,y)=仚(乮);仑(l,K::乱{丐,x,y},丆)}fn 倒(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-佁}#[derive(Clone,Copy)]enum
乄{仟,亄{丟:I,丅:I},}fn 侂(l:&J,乢:&Y,争:乄)->L<K>{let mut 丬=l.clone();let mut 上=亇::new(l,乢);let mut 丆=L::new();乺(&mut 丬,&mut 上,&mut 丆);match 争{乄::仟=>俭(&mut 丬,&mut
上,&mut 丆),乄::亄{丟,丅}=>{俓(&mut 丬,&mut 上,丟,丅,&mut 丆)}}丆}fn 乺(l:&mut J,上:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=上.丁;let mut 仪:L<F> =L::new();loop{let 丣=l.丽(丁,1);let
mut 亶:P<(串,F)> =None;for 丌 in l.二().iter(){if 仪.contains(&丌){continue;}let 乼=(G::q(丌).丑()&上.free&丣).count();if 乼==0{仪.push(丌);}else if 亶.is_none_or(|(n,_)|乼<n){亶=Some((乼,丌));}}let
Some((_,丌))=亶 else{break;};上.丅.clear(丌);match 上.下(G::q(丌).丑()&上.free&丣){Some(乮)if 丙(l,丌,乮,丆)=>上.伒(乮),_=>{上.丅.set(丌);仪.push(丌);}}}for 丌 in 仪{if!l.二().test(丌){continue;}let
丏=[上.乎,上.free].into_iter().find_map(|不|{不.iter().min_by_key(|&q|(l.map().乆(丌,q),q))});if let Some(丏)=丏{丙(l,丌,丏,丆);}}}fn 俭(l:&mut J,上:&mut 亇<'_>,丆:&mut
Vec<K>){let 丁=上.丁;let p=上.乢;let h=(p.伜-C::from(l.亊())).max(p.佥);let 個=(C::from(俕(l))-p.佐).max(0)/p.住.max(1)+p.伉;loop{let 丈=l.丈(丁);let 丅=l.伱(丁)as C;let
俲=C::from(上.乎.count());let q=上.下(l.亗(丁)&上.free&l.丽(丁,1));let 侄=倒(h,(俲/(丅+1)).min(個));let 佋=q.is_some()&&丈>=佁&&侄>0;let 佦=4*h-亁;let 丸=佧(l,上);let 俉=丸.is_some()&&丈>=亁&&佦>0;if
俉&&(!佋||佦>=侄){let(x,y)=仚(丸.unwrap_or_default());if 仑(l,K::亂{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佋)else{break;};let(x,y)=仚(q);if!仑(l,K::乖{一:1,x,y},丆){break;}上.伒(q);}}fn
佧(l:&J,上:&亇<'_>)->P<F>{let 丁=上.丁;(l.map().丰&l.七(丁)&l.亥()).iter().filter(|&丸|上.theirs[丸]>上.乢.伬).min_by_key(|&丸|(l.map().乆(Z[丁],丸),丸))}fn 俓(l:&mut J,上:&mut
亇<'_>,丟:I,丅:I,丆:&mut Vec<K>,){let 丁=上.丁;for _ in 0..丟{let Some(丸)=佧(l,上)else{break;};let(x,y)=仚(丸);if!仑(l,K::亂{x,y},丆){break;}}for _ in 0..丅{let Some(q)=上.下(l.亗(丁)&上.free&l.丽(丁,1))else{break;};let(x,y)=仚(q);if!仑(l,K::乖{一:1,x,y},丆){break;}上.伒(q);}}fn
仵(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丯=亢::new(l,丁,true);l.丠(1-丁).iter().any(|q|丯.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亜:I,争:乄,}const 伲:I=I::MAX;const 侊:[乄;8]=[乄::仟,乄::亄{丟:0,丅:0},乄::亄{丟:0,丅:1},乄::亄{丟:0,丅:2},乄::亄{丟:0,丅:伲,},乄::亄{丟:1,丅:0},乄::亄{丟:1,丅:1},乄::亄{丟:1,丅:伲,},];struct
伶<'a>{业:&'a[Y],仍:Y,rest:丢,乴:P<乓>,俳:Settings,}impl 伶<'_>{fn 乢(&self,丙:丢)->&Y{&self.业[F::from(丙.亜)%self.业.len()]}fn 丄(&self,l:&J,丙:丢)->L<K>{侂(l,self.乢(丙),丙.争)}}fn
伭(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伣=|亾:U|hash=(hash^亾).wrapping_mul(0x0100000001b3);for(倆,临)in[l.丠(丁),l.丅(丁),l.丟()].into_iter().enumerate(){for
q in 临.iter(){伣(((倆 as U)<<8)|q as U);}伣(0xff);}伣(l.丈(丁)as U);hash}enum 亡{仠{丿:F,书:F,乑:亞,},}struct 了{乽:亡,业:L<Y>,仍:Y,丕:L<丢>,仨:L<U>,亷:F,丮:串,仴:Q,}impl 了{fn
new(乽:亡,业:L<Y>,仍:Y)->了{了{乽,业,仍,丕:L::new(),仨:L::new(),亷:0,丮:0,仴:false,}}fn 俫(&mut self,l:&J)->Q{let j=self;let 俾=matches!(j.乽,亡::仠{..});let 俬=!俾&&j.丮>0&&j.亷>=j.仨.len();if!j.仴&&(俬||仵(l)){j.仴=true;}!j.仴}fn
上(&mut self,l:&J,乴:P<乓>)->L<K>{let j=self;let 体=伶{业:&j.业,仍:j.仍,rest:丢{亜:0,争:乄::仟,},乴,俳:Settings::DEFAULT,};let 丁=l.东();let 俥=j.仨.get(j.亷)==Some(&伭(l,丁));if!俥{let
乊=j.丮==0;j.丕=match j.乽{亡::仠{丿,书,乑,}=>{let 乑=&乑;let 乛=侮;let mut 乥=丕(l,&j.业,乛,丿,书,乴,乑,);乥.resize(丿,乛);乥}};j.仨=match j.乽{亡::仠{..}=>俿(l,&j.业,&j.丕),};j.亷=0;j.丮+=1;}let
丙=j.丕.get(j.亷).copied().unwrap_or(体.rest);j.亷+=1;体.丄(l,丙)}}fn 俿(l:&J,业:&[Y],丕:&[丢])->L<U>{let 丁=l.东();let mut 佢=L::with_capacity(丕.len());let mut 乣=l.clone();for&丙
in 丕{if 乣.丗().is_some(){break;}佢.push(伭(&乣,丁));乣=伈(&乣,&丄(&乣,业,丙));}佢}const 侸:C=3;struct Settings{丿:C,}impl Settings{const DEFAULT:Settings=Settings{丿:侸};}fn
乥(l:&J,丌:G)->[I;M]{let 乯=l.map().valid;let mut 义=[I::MAX;M];let mut 之=丌&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.丑()&乯).丝(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn
倔(l:&J,z:F)->C{let 仏=乥(l,l.七(z));let 乿=乥(l,l.七(1-z));l.map().valid.iter().map(|q|match 仏[q].cmp(&乿[q]){_ if 仏[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侰:[Y;27]=业();const 侮:丢=丢{亜:16,争:乄::仟,};const fn 业()->[Y;27]{let 亨=[0,2,4];let 人=[0,3,6];let 云=[0,1,2];let mut 丆=[Y::伆;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
享=if g>0||t>0{4}else{0};丆[i]=Y::伆.候(亨[g],人[t],0).倘(享,云[w]);i+=1;}丆}#[derive(Default)]struct 伅{不:C,且:C,丈:C,乸:C,丞:C,仗:C,丯:C,仙:C,}#[derive(Clone,Copy)]struct
亞{不:C,且:C,丈:C,丞:C,乸:C,仗:C,丯:C,仙:C,}fn 俴(l:&J,z:F)->[C;M]{struct 亿{七:G,久:[u16;M],丵:[u16;M],size:[C;M],丞:[C;M],亐:u16,}fn 些(d:&mut 亿,v:F){d.亐+=1;d.久[v]=d.亐;d.丵[v]=d.亐;d.size[v]=1;for
w in(G::q(v).丑()&d.七).iter(){if d.久[w]==0{些(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.久[v]{d.丞[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.久[w]);}}}let
mut d=亿{七:l.七(z),久:[0;M],丵:[0;M],size:[0;M],丞:[0;M],亐:0,};let 乭=Z[z];if d.七.test(乭){些(&mut d,乭);}d.丞[乭]=0;d.丞}fn 俶(l:&J,z:F)->伅{let 七=l.七(z);let 丞=俴(l,z);let
亻=仅(l,G::q(Z[1-z]));let mut 丆=伅{不:C::from(七.count()),且:l.且(z),丈:l.丈(z),丯:C::MAX,仙:倔(l,z)/2,..伅::default()};for q in 七.iter(){if 丞[q]>0{丆.乸+=1;丆.丞=丆.丞.max(丞[q]);}if
q!=Z[z]&&(G::q(q).丑()&七).count()<=1{丆.仗+=1;}丆.丯=丆.丯.min(C::from(亻[q]));}丆}fn 丛(l:&J,z:F,乑:&亞)->C{let s=俶(l,z);let o=乑;o.不*s.不+o.且*s.且+o.丈*s.丈-o.丞*s.丞-o.乸*s.乸-o.仗*s.仗-o.丯*s.丯.min(30)+o.仙*s.仙}fn
伈(l:&J,丄:&[K])->J{let mut 丂=l.clone();丂.佰(丄);if 丂.丗().is_none(){丂.佰(&[]);}丂}fn 丄(l:&J,业:&[Y],丙:丢)->L<K>{侂(l,&业[F::from(丙.亜)%业.len()],丙.争,)}fn 佯(l:&J,业:&[Y],丕:&[丢],乛:丢,丿:F,)->J{let
mut 乣=l.clone();for 上 in 0..丿{if 乣.丗().is_some(){break;}let 丙=丕.get(上).copied().unwrap_or(乛);乣=伈(&乣,&丄(&乣,业,丙));}乣}fn 倃(l:&J,业:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亜 in 0..业.len().min(F::from(I::MAX)){for&争 in&侊{let 丙=丢{亜:亜 as I,争,};let c=丄(l,业,丙);if!丆.iter().any(|(_,丩)|*丩==c){丆.push((丙,c));}}}丆}fn
促(l:&J,业:&[Y],乛:丢,丿:F,书:F,乴:P<乓>,乑:&亞,)->L<丢>{let 丁=l.东();let 仐=||乴.is_some_and(|t|乓::now()>=t);let 仒=|丕:&[丢]|丛(&佯(l,业,丕,乛,丿),丁,乑);let mut 下:(C,L<丢>)=(仒(&[]),L::new());let
mut 之:L<(J,L<丢>)> =vec![(l.clone(),L::new())];for 俍 in 0..丿{let mut 乳:L<(C,J,L<丢>)> =L::new();let mut 乃:HashMap<U,F> =HashMap::new();for(佩,丕)in&之{if 仐(){break;}for(丙,c)in
倃(佩,业){let 丂=伈(佩,&c);let mut 休=丕.clone();休.push(丙);let v=丛(&丂,丁,乑);let 乌=伭(&丂,丁);match 乃.get(&乌){Some(&主)if 乳[主].0>=v=>{}Some(&主)=>乳[主]=(v,丂,休),None=>{乃.insert(乌,乳.len());乳.push((v,丂,休));}}}}if
乳.is_empty(){break;}乳.sort_by(|a,b|b.0 .cmp(&a.0));乳.truncate(书);let 乹=俍+1==丿||仐();for(i,(_,_,丕))in 乳.iter().take(if 乹{书}else{1}).enumerate(){if i>0&&仐(){break;}let
v=仒(丕);if v>下.0{下=(v,丕.clone());}}if 乹{break;}之=乳.into_iter().map(|(_,s,p)|(s,p)).collect();}下.1}fn 丕(l:&J,业:&[Y],乛:丢,丿:F,俠:F,乴:P<乓>,乑:&亞,)->L<丢>{let 丁=l.东();let
仒=|丕:&[丢]|丛(&佯(l,业,丕,乛,丿),丁,乑);let mut 下:(C,L<丢>)=(仒(&[]),L::new());let mut 书=2;loop{let 丨=促(l,业,乛,丿,书,乴,乑);let v=仒(&丨);if v>下.0{下=(v,丨);}let 仐=乴.is_some_and(|t|乓::now()>=t);if
仐||(乴.is_none()&&书>=俠){break;}书*=2;}下.1}const 侍:F=4+12;fn 丿(伞:impl BufRead,mut 仳:impl Write,mut 伊:impl FnMut(&str,&str,串,乓)->丱,)->Result<(),丱>{let mut
严=伹{佳:伞};let Some(乊)=严.line()?else{return Ok(());};let mut 亪=format!("{}\n",乊);严.take(丒(&乊,"mine spot count")?,&mut 亪)?;let mut 上=0;loop{let Some(乊)=严.line()?else{return
Ok(());};let 亙=乓::now();上+=1;let mut 乜=format!("{}\n",乊);严.take(侍-1,&mut 乜)?;for 丶 in["building count","unit count"]{let line=严.併()?;let n=丒(&line,丶)?;乜.push_str(&line);乜.push('\n');严.take(n,&mut
乜)?;}let 伊=伊(&亪,&乜,上,亙);writeln!(仳,"{}",伊).and_then(|()|仳.flush()).map_err(|五|format!("turn {}: cannot write the answer: {}",上,五))?;}}struct 伹<R>{佳:R,}impl<R:BufRead>伹<R>{fn
line(&mut self)->Result<P<丱>,丱>{let mut line=丱::new();match self.佳.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(五)=>Err(format!("cannot read input: {}",五)),}}fn
併(&mut self)->Result<丱,丱>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乜:&mut String)->Result<(),丱>{for
_ in 0..n{乜.push_str(&self.併()?);乜.push('\n');}Ok(())}}fn 丒(line:&str,丶:&str)->Result<F,丱>{match line.trim().parse::<F>(){Ok(n)if n<=侘=>Ok(n),_=>Err(format!("bad {} {line:?}",丶)),}}const
侘:F=144;const 侴:C=3;const 侓:C=2;const 依:F=16;fn 俩(l:&J,丆:&mut Vec<K>){let z=l.东();let mut 乽=伽{s:l.clone(),z,乵:1-z,丏:Z[1-z],仩:Z[z],上:l.亊(),丆,};乽.run();}struct
伽<'a>{s:J,z:F,乵:F,丏:F,仩:F,上:串,丆:&'a mut Vec<K>,}fn 任(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伤(丐:串,q:F)->K{let(x,y)=任(q);K::乱{丐,x,y}}fn 侁(一:I,q:F)->K{let(x,y)=任(q);K::乖{一,x,y}}impl
伽<'_>{fn run(&mut self){let j=self;if j.仮(){return;}j.乺();if j.仮(){return;}j.乐();j.倐();}fn 义(&self,丌:F,乮:F)->C{C::from(self.s.map().乆(丌,乮))}fn 个(&mut self,三:K)->Q{let
丼=self.s.个(三);if 丼{self.丆.push(三);}丼}fn 亽(&self,両:&J)->Q{両.丗()==Some(么::乩(self.z))}fn 仮(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.二().iter(){if j.义(q,j.丏)!=1{continue;}let Some((丐,_))=j.s.付(q)else{continue;};let 三=伤(丐,j.丏);let mut 両=j.s.clone();if 両.个(三)&&j.亽(&両){return j.个(三);}}let
Some(丧)=j.俊()else{return false;};let mut 両=j.s.clone();if!丧.iter().all(|&三|両.个(三))||!j.亽(&両){return false;}for 三 in 丧{j.个(三);}true}fn 俊(&self)->P<L<K>>{let
j=self;const 伸:C=C::MAX;let 乘=j.s.两().也();let 丣:L<G> =(1..=乘).map(|一|j.s.丽(j.z,一)).collect();let 仈=|q:F|{丣.iter().position(|临|临.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 七=j.s.七(j.z);let mut 万=[伸;M];let mut 並=[F::MAX;M];let mut 丼=[false;M];for q in 七.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丼[q]&&万[q]<伸&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丼[q]=true;if q==j.丏{break;}for&乀 in&j.s.map().丑[0][q]{if 乀==乞{break;}let 乀=F::from(乀);if 丼[乀]||七.test(乀){continue;}let Some((_,亍))=仈(乀)else{continue;};if
万[q]+亍<万[乀]{万[乀]=万[q]+亍;並[乀]=q;}}}if 万[j.丏]>j.s.丈(j.z){return None;}let mut 丧=L::new();let mut q=j.丏;while!七.test(q){let(一,_)=仈(q)?;丧.push(侁(一,q));q=並[q];}丧.reverse();Some(丧)}fn
乬(&self,为:&J)->C{let(丁,亏)=(self.z,self.乵);let 並=&self.s;let 丒=|临:G|C::from(临.count());let 专=丒(为.丠(丁))-丒(並.丠(丁));let 仸=丒(並.丠(亏))-丒(为.丠(亏));let 保=丒(並.七(亏))-丒(为.七(亏));let
仫=並.亯(亏)-为.亯(亏);let 丟=丒(並.丟()&並.丠(亏))-丒(为.丟()&为.丠(亏));12*专+20*仸+8*(保-仸).max(0)+5*仫+25*丟}fn 佺(&self,q:F)->C{let j=self;let 丸=j.s.map().丰.test(q);if 丸&&j.s.两().仢()&&!j.s.丠(j.z).test(q){20}else{0}}fn
佘(&self,乁:C)->Q{let j=self;j.s.丅(j.乵).iter().any(|q|j.义(q,j.仩)<=乁)}fn 乺(&mut self){let j=self;let mut 丅:L<(C,F)> =j.s.二().iter().map(|q|(j.义(q,j.丏),q)).collect();丅.sort_unstable();for(俚,q)in
丅{if j.s.丗().is_some(){return;}let Some((丐,一))=j.s.付(q)else{continue;};let 丣=j.s.丽(j.z,一);let 乀=j.s.map().丑[j.z][q];let mut 下:P<(C,F)> =None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.个(伤(丐,丂)){continue;}let 丛=if j.亽(&両){100000}else{j.乬(&両)+10*(俚-j.义(丂,j.丏))+j.佺(丂)};if 下.is_none_or(|(亀,_)|丛>亀){下=Some((丛,丂));}}if
let Some((丛,丂))=下{if 丛>0{j.个(伤(丐,丂));}}}}fn 乐(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.佘(侴){j.倍();}else{j.丟();}}fn 倍(&mut self){let j=self;if!j.s.两().代()||j.s.丈(j.z)<亁{return;}let
乀=j.s.map().丑[j.z][j.仩];let 不:L<F> =乀.iter().take_while(|&&q|q!=乞).map(|&q|F::from(q)).collect();let 丷=j.s.丠(j.z);if 不.iter().any(|&q|丷.test(q)&&j.s.乒().test(q)){return;}for
q in 不{let(x,y)=任(q);if j.个(K::亃{x,y}){return;}}}fn 丟(&mut self){let j=self;if!j.s.两().仢(){return;}while j.s.丈(j.z)>=亁{let 乫=j.s.map().丰&j.s.丠(j.z)&j.s.七(j.z)&j.s.亥();let
Some(q)=乫.iter().min_by_key(|&q|(j.义(q,j.仩),q))else{return;};let(x,y)=任(q);if!j.个(K::亂{x,y}){return;}}}fn 倐(&mut self){let j=self;let 佶=if j.上<=3{0}else{5};for
_ in 0..依{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俜=j.佘(侓);let 亰=j.s.亗(j.z);let mut 下:P<(C,K)> =None;let mut 伯:P<(C,K)> =None;let
mut 佉=G::T;for 一 in 1..=j.s.两().也(){let 丣=j.s.丽(j.z,一);let 乼=(亰&丣).丝(佉);佉=丣;let 亍=10*C::from(一);if 亍>丈{continue;}for q in 乼.iter(){let 三=侁(一,q);let mut
両=j.s.clone();if!両.个(三){continue;}let 传=両.丈(j.z);let 且=両.且(j.z);let 仫=j.s.亯(j.乵)-両.亯(j.乵);let 便=if 仫>0&&俜{传+且>=0}else{且>=0&&(传>=佶||仫>0)};if!便{continue;}let
乬=j.乬(&両);let 丛=乬+j.佺(q)-6*j.义(q,j.丏)-10*(C::from(一)-1);if 乬>0{if 下.is_none_or(|(亀,_)|丛>亀){下=Some((丛,三));}}else if 且>=3&&传>=佶+10&&一==1{if 伯.is_none_or(|(亀,_)|丛>亀){伯=Some((丛,三));}}}}let
Some((_,三))=下.or(伯)else{return;};if!j.个(三){return;}}}}fn 俘(l:&J,丆:&mut Vec<K>){丆.clear();俩(l,丆);}const 予:C=C::MAX;const 侜:F=8;struct 亢{万:[C;M],並:[I;M],}fn
伕(l:&J,z:F,乺:Q,丏:F)->C{佒(l,z,亖(l,z,乺),丏)}fn 佒(l:&J,z:F,亖:G,丏:F)->C{let mut 万=予;佛(l,z,亖,|丙,new|{if new.test(丏){万=10*丙 as C;}万!=予});万}fn 修(l:&J,z:F,亖:G,丫:C)->G{let
mut 仵=G::T;佛(l,z,亖,|丙,new|{if 10*丙 as C>丫{return true;}仵|=new;false});仵.丝(l.七(z))}fn 亖(l:&J,z:F,乺:Q)->G{if 乺&&z==l.东(){l.二()}else{G::T}}fn 佛(l:&J,z:F,亖:G,mut
些:impl FnMut(F,G)->Q,){let 乘=l.两().也();let mut 仈=[G::T;3];let mut 乃=G::T;for 一 in 1..=乘{let 临=l.丽(z,一);仈[F::from(一)-1]=临.丝(乃);乃|=临;}let mut 举=[l.七(z),G::T,G::T,G::T];for
q in(亖&l.丅(z)).iter(){if let Some(一)=l.亚(q).filter(|&一|一<=乘){举[0]|=G::q(q).丑()&l.丽(z,一);}}let mut 丼=G::T;for 丙 in 0..{let new=举[丙%4].丝(丼);举[丙%4]=G::T;if
些(丙,new){return;}丼|=new;let 丂=new.丑().丝(丼);for(俀,不)in 仈.iter().enumerate(){举[(丙+俀+1)%4]|=丂&*不;}if 举.iter().all(|临|临.丝(丼).is_empty()){break;}}}impl 亢{fn
new(l:&J,z:F,乺:Q)->亢{let 乘=F::from(l.两().也());let mut 丣=[G::T;3];for(与,临)in 丣.iter_mut().enumerate().take(乘){*临=l.丽(z,与 as I+1);}let 七=l.七(z);let 丑=&l.map().丑[0];let
mut 万=[予;M];let mut 並=[乞;M];let mut 举=[0u8;M];let mut 丒=0;let mut 佸=G::T;for q in 七.iter(){万[q]=0;举[丒]=q as I;丒+=1;}if 乺&&z==l.东(){for 丌 in l.二().iter(){let
Some(一)=l.亚(丌)else{continue;};let 乥=G::q(丌).丑()&丣[F::from(一)-1];for q in 乥.iter(){if 万[q]!=0{万[q]=0;举[丒]=q as I;丒+=1;}}}}while 丒>0{let mut 下=0;for 与 in
1..丒{let(a,b)=(F::from(举[与]),F::from(举[下]));if(万[a],a)<(万[b],b){下=与;}}let q=F::from(举[下]);丒-=1;举[下]=举[丒];佸.set(q);for&丂 in&丑[q]{if 丂==乞{break;}let 丂=F::from(丂);if
佸.test(丂)||七.test(丂){continue;}let Some(一)=丣[..乘].iter().position(|临|临.test(丂))else{continue;};let 侀=万[q]+10*(一 as C+1);if 侀<万[丂]{if 万[丂]==予{举[丒]=丂 as
I;丒+=1;}万[丂]=侀;並[丂]=q as I;}}}亢{万,並}}fn path(&self,丏:F)->L<F>{let mut path=L::new();if 丏>=M||self.万[丏]==予{return path;}let mut q=丏;while self.並[q]!=乞{path.push(q);q=F::from(self.並[q]);}path.reverse();path}}struct
仝{丄:L<K>,万:C,}fn 俟(l:&J,z:F,q:F)->P<I>{(1..=l.两().也()).find(|&一|l.丽(z,一).test(q))}fn 俱(丬:&mut J,丁:F,产:F,丫:C,丄:&mut Vec<K>,乤:&mut i64,)->Q{loop{if 丬.丗()==Some(么::乩(丁)){return
true;}if 丬.丠(丁).test(产)&&丬.七(丁).test(产){return true;}let 丯=亢::new(丬,丁,false);let 万=丯.万[产];if 万==予||*乤+万>丫{return false;}let Some(&q)=丯.path(产).first()else{return
false;};let Some(一)=俟(丬,丁,q)else{return false;};let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(三){return false;}*乤+=10*C::from(一);丄.push(三);}}fn 佇(l:&J,丁:F,俗:&[F],丫:C)->P<仝>{let
mut 丬=l.clone();let mut 丄=L::new();let mut 乤=0;for&产 in 俗{if!俱(&mut 丬,丁,产,丫,&mut 丄,&mut 乤){return None;}}(乤<=丫).then_some(仝{丄,万:乤,})}fn 伐(l:&J,丏:F,丫:C)->P<仝>{if
丫<0||l.丗().is_some()||丏>=M||l.map().丹.test(丏){return None;}let 丁=l.东();if l.丠(丁).test(丏)&&l.七(丁).test(丏){return Some(仝{丄:L::new(),万:0,});}let mut 下=佇(l,丁,&[丏],丫);let
乊=亢::new(l,丁,false);let mut 乒:L<F> =(l.乒()&l.丠(1-丁)).iter().filter(|&乇|乇!=丏&&乊.万[乇]!=予).collect();乒.sort_by_key(|&乇|(l.map().乆(乇,丏),乇));乒.truncate(侜);for
乇 in 乒{if let Some(丧)=佇(l,丁,&[乇,丏],丫){if 下.as_ref().is_none_or(|亀|丧.万<亀.万){下=Some(丧);}}}下}const 侚:F=20000;struct 仞{丄:L<K>,万:C,不:C,丅:C,乐:C,且:C,丘:C,}struct
企{不:C,丅:C,乐:C,}fn 佟(l:&J,z:F)->企{let 七=l.七(z);let 丅=l.丅(z)&七;let 丛=丅.iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum();企{不:C::from(七.count()),丅:丛,乐:C::from(((l.丟()|l.乒())&七).count()),}}struct
伿{丁:F,仼:G,並:企,且:C,亮:F,丫:C,亳:F,path:L<K>,丨:L<仞>,index:HashMap<仁,F,BuildHasherDefault<DefaultHasher>>,}type 仁=[U;3];fn 倗(mut 乌:仁,q:F)->仁{乌[q/64]|=1<<(q%64);乌}impl
伿{fn 佖(&mut self,丬:&J,乹:P<F>,仸:仁,万:C,)->Q{let j=self;let 丁=j.丁;let 乼=match 乹{None=>丬.亗(丁),Some(q)=>G::q(q).丑(),}&j.仼;let 乼=乼.丝(丬.丠(丁));let mut 丣=[G::T;3];for
一 in 1..=丬.两().也(){丣[F::from(一)-1]=丬.丽(丁,一);}for q in 乼.iter(){let 乘=F::from(丬.两().也());let Some(与)=丣[..乘].iter().position(|临|临.test(q))else{continue;};let
一=与 as I+1;let 亍=10*C::from(一);if 万+亍>j.丫{continue;}if j.亳>=侚{return false;}j.亳+=1;let mut 丂=丬.clone();let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(三){continue;}j.path.push(三);let
亹=倗(仸,q);j.俯(&丂,亹,万+亍);let 俌=j.path.len()<j.亮&&丂.丗().is_none();let 俖=!俌||j.佖(&丂,Some(q),亹,万+亍);j.path.pop();if!俖{return false;}}true}fn 俯(&mut self,为:&J,亹:仁,万:C){let
j=self;let 从=1-j.丁;let 伦=佟(为,从);let 不=j.並.不-伦.不;let 丅=j.並.丅-伦.丅;if 不<=0&&丅<=0{return;}let 乐=j.並.乐-伦.乐;let 且=为.且(j.丁)-j.且;let 倚=为.丗()==Some(么::乩(j.丁));let
丘=if 倚{C::MAX/2}else{3*不+丅+10*乐+2*且-万};if let Some(&主)=j.index.get(&亹){if 万>=j.丨[主].万{return;}}let 丞=仞{丄:j.path.clone(),万,不,丅,乐,且,丘,};match j.index.get(&亹){Some(&主)=>j.丨[主]=丞,None=>{j.index.insert(亹,j.丨.len());j.丨.push(丞);}}}}fn
佔(l:&J,亮:F,丫:C,乁:F)->L<仞>{乍(l,亮,丫,乁).0}fn 乍(l:&J,亮:F,丫:C,乁:F)->(L<仞>,F){if l.丗().is_some()||亮==0||乁==0{return(L::new(),0);}let 丁=l.东();let 从=1-丁;let mut
伫=伿{丁,仼:l.丠(从)|l.丠(从).丑(),並:佟(l,从),且:l.且(丁),亮,丫,亳:0,path:L::new(),丨:L::new(),index:HashMap::default(),};伫.佖(l,None,[0;3],0);let 亳=伫.亳;let mut 丨=伫.丨;丨.sort_by(|a,b|b.丘.cmp(&a.丘).then(a.万.cmp(&b.万)));丨.truncate(乁);(丨,亳)}const
仡:C=1000000;const 供:C=200;fn 乎(l:&J,z:F)->C{let 七=l.七(z);let 丅:C=(l.丅(z)&七).iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.且(z)+4*C::from(七.count())+丅+10*C::from((l.乒()&七).count())}fn
佤(l:&J,z:F,乙:G)->C{俔(l,z,佒(l,z,乙,Z[1-z]),)}fn 俔(l:&J,z:F,万:C)->C{if 万==予{return C::MIN;}l.丈(z)+l.且(z)-万}fn 佱(仰:C)->C{仰.clamp(-供,0)}fn 俒(l:&J,z:F,乙:[G;2],)->(C,[C;2]){let
丩=1-z;match l.丗(){Some(么::乩(倖))=>{return(if 倖==z{仡}else{-仡},[0,0])}Some(么::伷)=>return(0,[0,0]),None=>{}}let mut 丘=乎(l,z)-乎(l,丩);if z==l.东(){let mut 丂=l.clone();丂.仆();if
丂.丗().is_none()&&伕(&丂,丩,true,Z[z])<=丂.丈(丩){丘-=仡/2;}}let 亭=[佤(l,z,乙[0]),佤(l,丩,乙[1]),];(丘+佱(亭[0])-佱(亭[1]),亭)}
