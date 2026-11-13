// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 串=u32;type U=u64;type 仙=i32;type C=i64;type 亝=f64;type Q=bool;type 丱=String;type 乓=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侎:Q=true;fn 俓(l:&J,q:F,仔:C,丯:G,丧:C)->Q{let
丁=l.东();let 乵=1-丁;let 佟=侎&&丧<=l.丈(乵)+2*l.专(乵).max(0);let 任=G::q(q)|G::q(q).且();if!佟&&(任&丯).is_empty(){return false;}let(x,y)=互(q);let mut 为=l.clone();if!为.个(K::亃{x,y}){return
false;}if 倂(&为,丁)<仔{return true;}佟&&伒(&为,乵,false,Z[丁])>丧}const 侔:F=4;const 伱:F=5;#[derive(Clone)]enum 丳{亟(K),企(L<K>),}impl 丳{fn 七(&self)->&[K]{match self{丳::亟(三)=>std::slice::from_ref(三),丳::企(七)=>七,}}}fn
互(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 余(l:&J,丆:&mut Vec<丳>){丆.clear();if l.丗().is_some(){return;}let 丁=l.东();let 乘=l.两().也();let mut 丣=[G::T;4];for 一
in 1..=乘{丣[F::from(一)]=l.丽(丁,一);}for 不 in l.二().iter(){let Some((丐,一))=l.他(不)else{continue;};let 乥=G::q(不).且()&丣[F::from(一)];for 乮 in 乥.iter(){let(x,y)=互(乮);丆.push(丳::亟(K::乱{丐,x,y}));}}let
丈=l.丈(丁);let 仰=l.亗(丁);for 一 in 1..=乘{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=仰&丣[F::from(一)];for 俘 in 1..一{丌=丌.丢(丣[F::from(俘)]);}for q in 丌.iter(){let(x,y)=互(q);丆.push(丳::亟(K::乖{一,x,y}));}}if
丈>=亁{let 乫=l.丟(丁)&仰&l.亥();if l.两().仠(){for q in(乫&l.map().丰).iter(){let(x,y)=互(q);丆.push(丳::亟(K::亂{x,y}));}}if l.两().仡(){let 俀=l.map().valid.丢(l.丟(丁)).且();let
丌=(乫&俀).丢(l.map().丰);let 丯=丯(l,1-丁);let 仔=C::from((丯&l.丄(丁)).count());let 丧=伒(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俓(l,q,仔,丯,丧)){let(x,y)=互(q);丆.push(丳::亟(K::亃{x,y}));}}}for
丝 in 佑(l,伱,丈,侔){if 丝.七.len()>1{丆.push(丳::企(丝.七));}}if let Some(丧)=伍(l,Z[1-丁],丈){if 丧.七.len()>1{丆.push(丳::企(丧.七));}}}const 価:I=2;const 侮:C=5;fn 乙(l:&J,z:F)->G{伡(l,z,価)}fn
伡(l:&J,z:F,不:I)->G{let mut 乙=G::T;if 不>3{return 乙;}for q in(l.丅(z)&l.丄(z)).iter(){if l.亚(q).is_some_and(|一|一>=不){乙|=G::q(q);}}乙}fn 丯(l:&J,乎:F)->G{let 丫=l.丈(乎)+l.专(乎);俨(l,乎,乙(l,乎),丫)}fn
仇(l:&J,z:F)->G{丯(l,1-z)&l.丄(z)}fn 倂(l:&J,z:F)->C{C::from(仇(l,z).count())}const 侞:C=4;fn 俛(丌:C)->C{丌/侞}fn 丞(l:&J,z:F,仇:G)->C{let 丄=l.丄(z);let 丞=(l.丞()&丄).丢(仇);俛(C::from(丄.count()))*C::from(丞.count())}const
侪:C=2;fn 侄(l:&J,z:F)->C{let 丄=l.丄(z);let[a,b,c]=丄.仸();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(伾-1);侶.with(|伋|{let mut 伋=伋.borrow_mut();let 件=&mut 伋[与];if 件.0==丄&&件.1==z as I+1{return 件.2;}let 丛=倎(丄,Z[z]);*件=(丄,z as I+1,丛);丛})}const
伾:F=1<<12;thread_local!{static 侶:std::cell::RefCell<L<(G,I,C)>> =std::cell::RefCell::new(vec![(G::T,0,0);伾]);}fn 倎(丄:G,乭:F)->C{struct 亾{丄:G,久:[I;M],丵:[I;M],size:[I;M],亐:I,乸:C,仯:C,}fn
些(d:&mut 亾,v:F)->I{d.亐+=1;let 久=d.亐;d.久[v]=久;let mut 丵=久;let mut 伪=1u8;let mut 丝=0u8;for w in(G::q(v).且()&d.丄).iter(){if d.久[w]==0{let 佼=些(d,w);伪+=佼;丵=丵.min(d.丵[w]);if
d.丵[w]>=久{丝+=佼;}}else{丵=丵.min(d.久[w]);}}d.丵[v]=丵;d.size[v]=伪;if 丝>0&&d.久[v]>1{d.乸+=1;d.仯=d.仯.max(C::from(丝));}伪}if!丄.test(乭){return 0;}let mut d=亾{丄,久:[0;M],丵:[0;M],size:[0;M],亐:0,乸:0,仯:0,};些(&mut
d,乭);d.乸+d.仯}fn 俯(l:&J,z:F)->C{侪*(侄(l,1-z)-侄(l,z))}fn 佒(l:&J,z:F)->C{if l.丈(z)+l.专(z)>=0{return 0;}(l.丅(z)&l.丄(z)).iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum()}const
侓:C=15;const 侴:[C;2]=[3,1];const 侲:[C;2]=[4,20];fn 俾(l:&J,z:F)->[C;2]{let mut 丑=[0;2];for q in(l.丅(z)&l.丄(z)).iter(){match l.亚(q){Some(2)=>丑[0]+=1,Some(3)=>丑[1]+=1,_=>{}}}丑}fn
佻(l:&J,z:F)->C{let[侁,倃]=俾(l,z);let 佅=|一:F|(3-侴[一])*侲[一];-侓*侁+佅(0)*侁+佅(1)*倃}const 侍:C=0;const 侐:C=2;const 侏:C=100;fn 佹(仮:C)->C{if 仮<0{return 0;}侍+侐*仮.min(侏)/2}const
伶:I=2;fn 伕(l:&J,z:F)->C{let 乙=[伡(l,z,伶),伡(l,1-z,伶),];let(丘,亭)=俌(l,z,乙);let 丘=丘+佹(亭[0])-佹(亭[1]);if l.丗().is_some(){return 丘;}let 仍=仇(l,z);let 乿=仇(l,1-z);let
仔=C::from(乿.count())-C::from(仍.count());丘-2*佒(l,z)+2*佒(l,1-z)+俯(l,z)+佻(l,z)-佻(l,1-z)+侮*仔/4+丞(l,z,仍)-丞(l,1-z,乿)}const 依:F=8;const 侥:F=3;const 侕:F=32;const
侦:U=300;const 侉:亝=0.75;struct 亅{乾:P<乓>,井:P<乓>,今:U,}#[derive(Default)]struct 似{七:L<K>,休:F,乶:U,世:F,丮:F,佊:F,乷:F,}const 侵:Q=true;#[derive(Clone)]struct 乕{七:L<K>,伔:C,}struct
亠{七:L<K>,end:J,first:P<U>,丒:P<L<乕>>,}type 亿=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct 伃{丁:F,limit:亅,乶:U,仉:亿,亘:P<L<丳>>,}const 估:Q=true;const
來:Q=false;const 伯:P<I> =None;fn 侾(l:&J,世:&[乕],丆:&mut Vec<丳>){if!估&&伯.is_none(){return;}let 仒=世.iter().map(|乚|乚.七.as_slice());便(l,仒,(估,來,伯),丆,);}fn 便<'a>(l:&J,仒:impl
Iterator<Item=&'a[K]>,(乒,且,一):(Q,Q,P<I>),丆:&mut Vec<丳>,){let 丁=l.东();let 丷=l.丄(丁);let mut 仅=G::T;for 乚 in 仒{for 三 in 乚{if let K::乖{x,y,..}|K::乱{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丷.test(q){仅|=G::q(q);}}}}if 仅.is_empty(){return;}let 乫=l.亥()&丷;let 丈=l.丈(丁);let 佰=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.七()==[三]){丆.push(丳::亟(三));}};if
乒&&丈>=亁&&l.两().仡(){let mut 丌=仅;if 且{丌|=仅.且();}for q in(丌&乫).丢(l.map().丰).iter(){let(x,y)=互(q);佰(K::亃{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().也()){if
丈>=10*C::from(一){for q in(仅&乫).iter(){let(x,y)=互(q);佰(K::乖{一,x,y},丆);}}}}fn 伉(l:&mut J,七:&[K])->Q{七.iter().all(|&三|l.个(三))}fn 乌(七:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 七{let(仪,a,b,c)=match*三{K::乱{丐,x,y}=>(1,U::from(丐),x,y),K::乖{一,x,y}=>(2,U::from(一),x,y),K::亂{x,y}=>(3,0,x,y),K::亃{x,y}=>(4,0,x,y),};let 亽=仪|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亽).wrapping_mul(0x0100000001b3);}hash}fn
佂(乗:&J,乚:&[K])->J{let mut 丂=乗.clone();丂.仄();for&三 in 乚{丂.个(三);}丂.仄();丂}fn 们(l:&J)->P<L<K>>{let 丁=l.东();let 丏=Z[1-丁];let 亼=|丠:&J|丠.丗()==Some(么::乩(丁));for
q in l.二().iter(){if l.map().乆(q,丏)!=1{continue;}let Some((丐,_))=l.他(q)else{continue;};let(x,y)=互(丏);let 三=K::乱{丐,x,y};let mut 丠=l.clone();if 丠.个(三)&&亼(&丠){return
Some(vec![三]);}}let 丧=伍(l,丏,l.丈(丁))?;let mut 丠=l.clone();(伉(&mut 丠,&丧.七)&&亼(&丠)).then_some(丧.七)}impl 伃{fn 乤(&self)->Q{self.乶>=self.limit.今||self.limit.乾.is_some_and(|t|乓::now()>=t)}fn
丘(&mut self,乗:&J,世:&[乕],伊:C)->C{self.乶+=1;self.伥(乗,世,伊)}fn 伥(&self,乗:&J,世:&[乕],伊:C)->C{if 乗.丗()==Some(么::乩(self.丁)){return 仟;}if 世.is_empty(){return 伕(乗,self.丁);}let
mut 仹=C::MAX;for 乚 in 世{仹=仹.min(伕(&佂(乗,&乚.七),self.丁));if 仹<=伊{break;}}仹}#[allow(clippy::too_many_arguments)]fn 俢(&mut self,乪:&J,交:C,乏:&[丳],亴:&[(C,F)],丼:&mut[Q],世:&[乕],亦:&mut
亿,伛:Q,)->(P<(F,J,C)>,Q){let mut 上=None;let mut 仢=交;for&(仱,与)in 亴{if 丼[与]{continue;}if 伛&&仱!=C::MAX&&仱<=仢-交{return(上,true);}if self.乤(){return(上,false);}丼[与]=true;let
七=乏[与].七();let mut 丂=乪.clone();if!伉(&mut 丂,七){亦.insert(乌(七),C::MIN/4);continue;}let 丘=self.丘(&丂,世,仢);亦.insert(乌(七),丘-交);if 丘>仢{仢=丘;上=Some((与,丂,丘));}}(上,false)}fn
伐(&mut self,乭:&J,世:&[乕],仁:&[U],係:Q)->亠{let j=self;let mut 乊=None;let mut 乪=乭.clone();let mut 七=L::new();let mut 交=j.丘(&乪,世,C::MIN);let mut 亦=亿::default();let
mut 乏=L::new();let mut 亴=L::new();while 乪.丗().is_none()&&!j.乤(){if 七.is_empty()&&係{match&j.亘{Some(亘)=>乏.clone_from(亘),None=>{余(&乪,&mut 乏);j.亘=Some(乏.clone());}}}else{余(&乪,&mut
乏);}侾(&乪,世,&mut 乏);亴.clear();for(与,乲)in 乏.iter().enumerate(){let k=乌(乲.七());let 仱=亦.get(&k).or(j.仉.get(&k)).copied();亴.push((仱.unwrap_or(C::MAX),与));}亴.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丼=vec![false;乏.len()];if 七.is_empty(){for(与,乲)in 乏.iter().enumerate(){丼[与]=仁.contains(&乌(乲.七()));}}let mut 上=None;for
伛 in[true,false]{let(丨,俷)=j.俢(&乪,交,&乏,&亴,&mut 丼,世,&mut 亦,伛,);上=丨;if 上.is_some()||!俷{break;}}let Some((与,丂,丘))=上 else{break;};if 七.is_empty(){乊=Some(乌(乏[与].七()));}七.extend_from_slice(乏[与].七());乪=丂;交=丘;}for(k,乬)in
亦{let 佛=j.仉.entry(k).or_insert(乬);*佛=(*佛).max(乬);}亠{七,end:乪,first:乊,丒:None,}}fn 仒(&self,乗:&J)->L<L<K>>{let mut 丂=乗.clone();丂.仄();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 习=丂.东();let 丈=丂.丈(习);if let Some(丧)=伍(&丂,Z[self.丁],丈){丆.push(丧.七);}for 丝 in 佑(&丂,伱,丈,侥){丆.push(丝.七);}let mut 佚=L::new();俒(&丂,&mut
佚);丆.push(佚);let mut 乿=伃{丁:习,limit:亅{今:侦,..self.limit},乶:0,仉:亿::default(),亘:None,};丆.push(乿.伐(&丂,&[],&[],true).七);丆.retain(|七|!七.is_empty());丆}fn 佃(&self,乗:&J)->L<乕>{self.仒(乗).into_iter().map(|七|{let
伔=伕(&佂(乗,&七),self.丁);乕{七,伔}}).collect()}fn 乷(&mut self,乭:&J,乲:&丳,世:&[乕])->P<亠>{let mut 为=乭.clone();if!伉(&mut 为,乲.七()){return None;}self.乶+=1;let 乛=self.伐(&为,世,&[],false);let
mut 七=乲.七().to_vec();七.extend_from_slice(&乛.七);Some(亠{七,end:乛.end,first:Some(乌(乲.七())),丒:None,})}fn 佖(&self,丕:&亠,世:&[乕])->C{let 丘=self.伥(&丕.end,世,C::MIN);match&丕.丒{Some(丒)if!丒.is_empty()=>丘.min(self.伥(&丕.end,丒,C::MIN)),_=>丘,}}}fn
俪(丨:&[乕],世:&mut Vec<乕>)->Q{let 並:L<_> =世.iter().map(|r|r.七.clone()).collect();for 乚 in 丨{if 世.iter().any(|r|r.七==乚.七){continue;}世.push(乚.clone());}世.sort_by_key(|乚|乚.伔);世.truncate(依);世.iter().map(|r|&r.七).ne(&並)}fn
倁(l:&J,乁:亅,亳:&mut 了)->似{let 丁=l.东();let mut 中=似::default();if l.丗().is_some(){return 中;}if let Some(七)=们(l){中.七=七;return 中;}let 主=|俱:亝|match(乁.乾,乁.井){(Some(乾),Some(井))=>Some(乾+(井-乾).mul_f64(俱)),_=>None,};if
亳.俥(l){中.七=亳.下(l,主(0.4));return 中;}let mut 乍=伃{丁,limit:亅{乾:主(侉),..乁},乶:0,仉:亿::default(),亘:None,};let mut 世=L::new();let mut 丮:L<亠> =L::new();let mut 仁=L::new();let
mut 以=false;let mut 乷:L<丳> =L::new();while 中.休<侕&&(丮.is_empty()||!乍.乤()){let 丕=if 以{let Some(乲)=乷.pop()else{break;};match 乍.乷(l,&乲,&世){Some(丕)=>丕,None=>continue,}}else{乍.伐(l,&世,&仁,true)};中.休+=1;let
併=丮.iter().any(|p|p.七==丕.七);let 乊=丕.first;let 佺=乍.乤();let mut 丕=丕;let mut 佦=false;if!佺&&!併{let 丨=乍.佃(&丕.end);佦=俪(&丨,&mut 世);丕.丒=Some(丨);}if!併{丮.push(丕);}if
佺{break;}if!佦&&!以{match 乊{Some(乊)=>仁.push(乊),None=>{以=true;乷=乍.亘.clone().unwrap_or_default();乷.retain(|a|!仁.contains(&乌(a.七())));let 乬=|a:&丳|{乍.仉.get(&乌(a.七())).copied().unwrap_or(C::MIN)};乷.sort_by_key(乬);}}}}中.乷=F::from(以);中.乶=乍.乶;中.世=世.len();中.丮=丮.len();let
佡=||乁.井.is_some_and(|t|乓::now()>=t);let mut 亷:L<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佡(){break;}亷[与]=Some(乍.佖(&丮[与],&世));}let
亵=|亷:&[P<C>]|{let mut 上:P<(C,F)> =None;for(与,丘)in 亷.iter().enumerate().rev(){if let Some(丘)=*丘{if 上.is_none_or(|(亀,_)|丘>亀){上=Some((丘,与));}}}上};let mut
上=亵(&亷);乍.limit.乾=乁.井;while let Some((_,与))=上{if!侵||丮[与].丒.is_some()||佡(){break;}let 丨=乍.佃(&丮[与].end);丮[与].丒=Some(丨);中.佊+=1;亷[与]=Some(乍.佖(&丮[与],&世));上=亵(&亷);}if
let Some((_,与))=上{let mut 丬=l.clone();for&三 in&丮[与].七{if 丬.个(三){中.七.push(三);}}}中}fn 俤(乁:&dyn Fn(乓,串)->亅,mut 亳:了,会:impl BufRead,仲:impl Write,)->Result<(),丱>{丿(会,仲,|亪,乜,下,亙|match
J::read(亪,乜,乧::份,下){Ok(l)=>{let 中=倁(&l,乁(亙,下),&mut 亳);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",下,中.休,中.乶,中.世,中.佊,中.乷,亙.elapsed());侻(&中.七)}Err(五)=>{eprintln!("plan: cannot read the input: {}",五);"WAIT".to_string()}},)}const
侇:亝=40.0;const 侊:亝=800.0;const 侫:亝=0.6;fn 乁(亙:乓,下:串)->亅{if let Some(今)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亅{乾:None,井:None,今,};}let 仵=倄().min(100.0);let 保=if 下==1{侊}else{侇};let 丫=Duration::from_secs_f64(保/1000.0*仵);亅{乾:Some(亙+丫.mul_f64(侫)),井:Some(亙+丫),今:U::MAX,}}const
侤:F=7;const 侢:亞=亞{丌:3,专:1,丈:1,丝:2,乸:2,仕:2,丯:6,仗:1,};const 侣:I=I::MAX;fn 亳()->了{俠(64)}fn 俠(书:F)->了{let 乽=亡::仞{丿:侤,书,乑:侢,};let 业=侩.iter().map(|&乢|Y{伨:侣,..乢}).collect();了::new(乽,业,Y::伅)}fn
main(){let 俻=io::stdin();let 俼=io::stdout();if let Err(五)=俤(&乁,亳(),俻.lock(),俼.lock()){eprintln!("plan: {}",五);std::process::exit(1);}}const 侯:&str="CG_TIME_SCALE";fn
倄()->亝{信(std::env::var(侯).ok().as_deref())}fn 信(丛:P<&str>)->亝{丛.and_then(|乜|乜.trim().parse::<亝>().ok()).filter(|仵|仵.is_finite()&&*仵>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乔:F=4*O;const 乨:U=(1<<乔)-1;const 仚:U=1|1<<12|1<<24|1<<36;const 仾:U=仚<<11;const 伺:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侅:G=G([乨;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乔]>>(q%乔))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乔]|=1<<(q%乔);}#[inline]fn clear(&mut self,q:F){self.0[q/乔]&=!(1<<(q%乔));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仸(self)->[U;3]{self.0}#[inline]fn count(self)->串{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丢(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仼{仼{伞:self.0,主:0,}}#[inline]fn 俶(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伺)<<(乔-O)),(b>>O)|((c&伺)<<(乔-O)),c>>O,])}#[inline]fn 俳(self)->G{let[a,b,c]=self.0;G([(a<<O)&乨,((b<<O)&乨)|(a>>(乔-O)),((c<<O)&乨)|(b>>(乔-O)),])}#[inline]fn
俴(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仾&乨,(b>>1)&!仾&乨,(c>>1)&!仾&乨,])}#[inline]fn 俵(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仚&乨,(b<<1)&!仚&乨,(c<<1)&!仚&乨,])}#[inline]fn
且(self)->G{let j=self;j.俶()|j.俳()|j.俴()|j.俵()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侅.丢(self)}}struct 仼{伞:[U;3],主:F,}impl Iterator for 仼{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.主<3{let 仭=j.伞[j.主];if 仭!=0{j.伞[j.主]=仭&(仭-1);return Some(j.主*乔+仭.trailing_zeros()as F);}j.主+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乱{丐:串,x:I,y:I},乖{一:I,x:I,y:I},亂{x:I,y:I},亃{x:I,y:I},}impl K{fn 倆(&self)->丱{match*self{K::乱{丐,x,y}=>format!("MOVE {} {x} {y}",丐),K::乖{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::亂{x,y}=>format!("BUILD MINE {x} {y}"),K::亃{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
侻(七:&[K])->丱{if 七.is_empty(){return "WAIT".to_string();}let mut line=丱::with_capacity(16*七.len());for(与,三)in 七.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&三.倆());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乧{伿,倔,侷,#[default]份,}impl 乧{fn 也(self)->I{match self{乧::伿=>1,_=>3,}}fn 仠(self)->Q{matches!(self,乧::侷|乧::份)}fn 仡(self)->Q{self==乧::份}}const 乞:I=I::MAX;struct
买{丹:G,valid:G,丰:G,乆:Box<[I]>,且:[[[I;4];M];2],}thread_local!{static 侒:RefCell<P<Arc<买>>> =const{RefCell::new(None)};}impl 买{fn 俲(丹:G,丰:G)->Arc<买>{侒.with(|乹|{let
mut 乹=乹.borrow_mut();if let Some(京)=乹.as_ref(){if 京.丹==丹&&京.丰==丰{return Arc::clone(京);}}let 京=Arc::new(买::build(丹,丰));*乹=Some(Arc::clone(&京));京})}fn build(丹:G,丰:G)->买{let
乯=!丹;const 侬:[[(仙,仙);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 且=[[[乞;4];M];2];for(z,乥)in 侬.iter().enumerate(){for q in
乯.iter(){let(x,y)=((q%O)as 仙,(q/O)as 仙);let mut 丑=0;for&(俉,俊)in 乥{let(佨,佩)=(x+俉,y+俊);if(0..O as 仙).contains(&佨)&&(0..O as 仙).contains(&佩){let 丂=佩 as F*O+佨
as F;if 乯.test(丂){且[z][q][丑]=丂 as I;丑+=1;}}}}}let mut 乆=vec![乞;M*M].into_boxed_slice();let mut 伤=[0u8;M];for 不 in 乯.iter(){let 于=&mut 乆[不*M..(不+1)*M];于[不]=0;伤[0]=不
as I;let(mut 众,mut 伬)=(0,1);while 众<伬{let q=伤[众]as F;众+=1;for&丂 in&且[0][q]{if 丂!=乞&&于[丂 as F]==乞{于[丂 as F]=于[q]+1;伤[伬]=丂;伬+=1;}}}}买{丹,valid:乯,丰,乆,且,}}#[inline]fn
乆(&self,不:F,乮:F)->I{self.乆[不*M+乮]}}const 侜:串=100;const 亁:C=15;const 侳:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乩(F),伳,}enum
九{伂,伷,伸,}#[derive(Clone)]struct J{map:Arc<买>,两:乧,丒:[G;2],丄:[G;2],丷:G,乇:G,丅:[G;2],一:[G;3],事:G,亻:[串;M],丈:[C;2],专:[C;2],东:F,亊:串,亰:串,丗:P<么>,}fn 仈(丒:G,亩:F)->G{if!丒.test(亩){return
G::T;}let mut 仑=G::q(亩);loop{let 丂=(仑|仑.且())&丒;if 丂==仑{return 仑;}仑=丂;}}impl J{fn 东(&self)->F{self.东}fn 亊(&self)->串{self.亊}fn 丗(&self)->P<么>{self.丗}fn 两(&self)->乧{self.两}fn
亯(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
专(&self,z:F)->C{self.专[z]}fn 伭(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&买{&self.map}fn 丟(&self,z:F)->G{self.丒[z]}fn 丄(&self,z:F)->G{self.丄[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 他(&self,q:F)->P<(串,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亻[q],j.伝(q)))}fn 二(&self)->G{self.丅[self.东]&self.事}fn
丞(&self)->G{self.丷}fn 乒(&self)->G{self.乇}fn 亚(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伝(q))}fn 俺(&mut self,z:F){let j=self;j.丄=[仈(j.丒[0],Z[0]),仈(j.丒[1],Z[1])];let
俅=j.丅[z].丢(j.丄[z]);j.伦(俅);j.佋();j.丈[z]+=j.专[z];if j.丈[z]<0{j.丈[z]=0;j.伦(j.丅[z]);}j.事|=j.丅[z];}fn 佋(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
丅=丄&j.丅[z];let mut 专=C::from(丄.count())+4*C::from((丄&j.丷).count());for(一,倌)in 侳.iter().enumerate(){专-=倌*C::from((丅&j.一[一]).count());}j.专[z]=专;}}fn 仄(&mut
self){let j=self;if j.丗.is_some(){return;}if j.东==1{if j.亊>=侜{j.丗=Some(match j.亯(0).cmp(&j.亯(1)){std::cmp::Ordering::Greater=>么::乩(0),std::cmp::Ordering::Less=>么::乩(1),std::cmp::Ordering::Equal=>么::伳,});return;}j.亊+=1;}j.东=1-j.东;j.俺(j.东);}fn
佮(&mut self,七:&[K]){let j=self;if j.丗.is_some(){return;}let 伖=j.亰;for&三 in 七{if j.俈(三,伖){continue;}j.个(三);if j.丗.is_some(){return;}}j.仄();}fn 俈(&self,三:K,伖:串)->Q{let
j=self;match 三{K::乱{丐,x,y}=>{j.亣(x,y).is_none()||丐>=伖||j.佗(j.东,丐).is_none()}K::乖{一,x,y}=>{j.亣(x,y).is_none()||一==0||一>j.两.也()}K::亂{x,y}|K::亃{x,y}=>{j.亣(x,y).is_none()}}}#[inline]fn
亣(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丹.test(y*O+x)).then_some(y*O+x)}fn 佗(&self,z:F,丐:串)->P<F>{self.丅[z].iter().find(|&q|self.亻[q]==丐)}#[inline]fn
伝(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 侽(&self,习:F)->G{let 乒=self.乇&self.丒[习];let 丄=self.丄[习];乒|(丄&(丄&乒).且())}fn
丽(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 习=1-z;let 丅=j.丅[0]|j.丅[1];let 俋=j.亥();let 倊=match
一{1 if j.两==乧::伿=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::T,};let 丽=俋|j.丒[习].丢(倊);if 一<3{丽.丢(j.侽(习))}else{丽}}fn 亗(&self,z:F)->G{self.丄[z]|self.丄[z].且()}fn
亥(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 俕=G::q(Z[0])|G::q(Z[1]);j.map.valid.丢(丅|j.丷|j.乇|俕)}fn 个(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.东;let 亓=match 三{K::乖{一,x,y}=>j.倈(z,一,x,y),K::乱{丐,x,y}=>j.俜(z,丐,x,y),K::亂{x,y}=>j.build(z,true,x,y),K::亃{x,y}=>j.build(z,false,x,y),};let
Some(亓)=亓 else{return false;};match 亓{九::伂=>{}九::伷=>j.丄[z]=仈(j.丒[z],Z[z]),九::伸=>{j.丄=[仈(j.丒[0],Z[0]),仈(j.丒[1],Z[1])];}}j.佋();if!j.丒[0].test(Z[0]){j.丗=Some(么::乩(1));}else
if!j.丒[1].test(Z[1]){j.丗=Some(么::乩(0));}true}fn 伦(&mut self,丌:G){let j=self;j.丅[0]=j.丅[0].丢(丌);j.丅[1]=j.丅[1].丢(丌);for 一 in&mut j.一{*一=一.丢(丌);}j.事=j.事.丢(丌);}fn
佽(&mut self,z:F,q:F)->九{let j=self;j.伦(G::q(q));j.丷.clear(q);j.乇.clear(q);let 习=1-z;if j.丒[习].test(q){j.丒[习].clear(q);j.丒[z].set(q);九::伸}else if j.丒[z].test(q){九::伂}else{j.丒[z].set(q);九::伷}}fn
倈(&mut self,z:F,一:I,x:I,y:I)->P<九>{let j=self;if 一==0||一>j.两.也(){return None;}let q=j.亣(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亗(z).test(q)||!j.丽(z,一).test(q){return
None;}let 亓=j.佽(z,q);j.佬(z,一,q,j.亰);j.亰=j.亰.saturating_add(1);j.丈[z]-=万;Some(亓)}fn 佬(&mut self,z:F,一:I,q:F,丐:串){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亻[q]=丐;}fn
俜(&mut self,z:F,丐:串,x:I,y:I)->P<九>{let j=self;let 丏=j.亣(x,y)?;let 不=j.佗(z,丐)?;if!j.事.test(不){return None;}let 一=j.伝(不);let 丣=j.丽(z,一);let 乀=j.map.且[z][不];if
乀.contains(&(丏 as I))&&!丣.test(丏){return None;}let mut 上=j.map.乆(不,丏);let mut 仂=None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let 丂=F::from(丂);let 乆=j.map.乆(丂,丏);if
乆<上&&丣.test(丂){上=乆;仂=Some(丂);}}let 仂=仂?;let 亓=j.佽(z,仂);j.丅[z].clear(不);j.一[F::from(一)-1].clear(不);j.事.clear(不);j.佬(z,一,仂,丐);Some(亓)}fn build(&mut self,z:F,丷:Q,x:I,y:I)->P<九>{let
j=self;let 侺=if 丷{j.两.仠()}else{j.两.仡()};let q=j.亣(x,y)?;let 俞=j.map.丰.test(q);if!侺||j.丈[z]<亁||俞!=丷||!j.丒[z].test(q)||!j.亗(z).test(q)||!j.亥().test(q){return
None;}if 丷{j.丷.set(q);}else{j.乇.set(q);}j.丈[z]-=亁;Some(九::伂)}}const 侗:C=1000000000;const 仿:C=0;const 侟:C=1;const 侰:C=2;impl J{fn read(亪:&str,下:&str,两:乧,亊:串,)->Result<J,丱>{let
mut 丰=L::new();let mut 严=亪.lines();let 丑=伓(&mut 严,"mine spot count")?;for _ in 0..丑{let[x,y]=乻(亱(&mut 严,"a mine spot")?,"a mine spot")?;丰.push((令(x)?,令(y)?));}佔(严,"the initial input")?;let
mut 严=下.lines();let mut 优=[0i64;4];for(丛,丶)in 优.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伣]=乻(亱(&mut 严,丶)?,丶)?;if 伣.abs()>侗{return
Err(format!("{}: {} is out of range",丶,伣));}*丛=伣;}let mut 丹=G::T;let mut 丌=[G::T;2];let mut 丄=[G::T;2];for y in 0..O{let 于=亱(&mut 严,"a map row")?.trim();if
于.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,于));}for(x,仓)in 于.bytes().enumerate(){let q=y*O+x;match 仓{b'#'=>丹.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乂=F::from(仓==b'X'||仓==b'x');丌[乂].set(q);if 仓.is_ascii_uppercase(){丄[乂].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仓 as char))}}}}let
俁=伓(&mut 严,"building count")?;let mut 乐=L::new();for _ in 0..俁{let 乻=乻(亱(&mut 严,"a building")?,"a building")?;乐.push(乻);}let 伭=伓(&mut 严,"unit count")?;let
mut 丅=L::new();for _ in 0..伭{let 乻=乻(亱(&mut 严,"a unit")?,"a unit")?;丅.push(乻);}佔(严,"the turn's input")?;let 佫=乐.iter().find(|佇|佇[0]==0&&佇[1]==仿).ok_or("the bot's own HQ is not among the buildings")?;let
佞=伌(佫[2],佫[3],丹)?;let z=Z.iter().position(|&亩|亩==佞).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佞))?;let mut 佷=G::T;for(x,y)in 丰{if
丹.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佷.set(y*O+x);}let 伆=|乂:F|if 乂==0{z}else{1-z};let mut l=J{map:买::俲(丹,佷),两,丒:[G::T;2],丄:[G::T;2],丷:G::T,乇:G::T,丅:[G::T;2],一:[G::T;3],事:G::T,亻:[0;M],丈:[0;2],专:[0;2],东:z,亊,亰:1,丗:None,};for
乂 in 0..2{let 乡=伆(乂);l.丒[乡]=丌[乂];l.丄[乡]=丄[乂];l.丈[乡]=优[2*乂];l.专[乡]=优[2*乂+1];}let mut 佪=G::T;for[乡,仪,x,y]in 乐{let 乂=佲(乡)?;let q=伌(x,y,丹)?;if!丌[乂].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佪.test(q){return Err(format!("two buildings at ({x}, {y})"));}佪.set(q);match 仪{仿
if q==Z[伆(乂)]=>{}仿=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侟=>l.丷.set(q),侰=>l.乇.set(q),_=>return Err(format!("unknown building type {}",仪)),}}let
mut 乃=G::T;let mut 仫=0u32;for[乡,丐,一,x,y]in 丅{let 乂=佲(乡)?;let q=伌(x,y,丹)?;if!丌[乂].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乃.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丐=串::try_from(丐).map_err(|_|format!("unit id {} is out of range",丐))?;if
乃.iter().any(|丩|l.亻[丩]==丐){return Err(format!("two units have id {}",丐));}乃.set(q);l.丅[伆(乂)].set(q);l.一[一 as F-1].set(q);l.亻[q]=丐;仫=仫.max(丐);}if!乃.is_empty(){l.亰=仫.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仫))?;}l.事=l.丅[z];Ok(l)}}fn
亱<'a>(严:&mut Lines<'a>,丶:&str)->Result<&'a str,丱>{严.next().ok_or_else(||format!("the text ends before {}",丶))}fn 佔(mut 严:Lines<'_>,丶:&str)->Result<(),丱>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丶)),None=>Ok(()),}}fn 乻<const N:F>(line:&str,丶:&str)->Result<[C;N],丱>{let
mut 仸=line.split_whitespace();let mut 侃=[0i64;N];for 丛 in&mut 侃{let 亽=仸.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丶))?;*丛=亽.parse().map_err(|_|format!("{}: {:?} is not an integer",丶,亽))?;}if
仸.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丶));}Ok(侃)}fn 伓(严:&mut Lines<'_>,丶:&str)->Result<F,丱>{let[丑]=乻(亱(严,丶)?,丶)?;F::try_from(丑).ok().filter(|&丑|丑<=M).ok_or_else(||format!("{}: {} is out of range",丶,丑))}fn
令(丛:C)->Result<F,丱>{F::try_from(丛).ok().filter(|&丛|丛<O).ok_or_else(||format!("coordinate {} is off the map",丛))}fn 伌(x:C,y:C,丹:G)->Result<F,丱>{let q=令(y)?*O+令(x)?;if
丹.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佲(乡:C)->Result<F,丱>{match 乡{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乡)),}}const
侌:C=15;const 侠:C=6;const 伽:C=10;const 侨:I=3;#[derive(Clone,Copy)]struct Y{open:C,代:C,低:I,伙:C,佝:C,佣:C,伨:I,位:C,佌:C,佁:C,亨:C,亹:C,丸:C,享:C,云:C,}impl Y{const
伅:Y=Y{open:3,代:2,低:12,伙:4,佝:侌,佣:侠,伨:侨,位:2,佌:2,佁:0,亨:0,亹:0,丸:0,享:0,云:0,};const fn 倒(self,亨:C,亹:C,丸:C)->Y{Y{亨,亹,丸,..self}}const fn 們(self,享:C,云:C)->Y{Y{享,云,..self}}}fn
付(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仃(l:&J,不:G)->[I;M]{let mut 义=[I::MAX;M];let 乯=l.map().valid;let mut 之=不&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.且()&乯).丢(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn 俏(l:&J)->I{let 义=仃(l,l.丟(0));l.丟(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亇<'a>{乢:&'a Y,丁:F,theirs:[I;M],人:[I;M],free:G,乎:G,ours:G,丰:G,}impl 亇<'_>{fn new<'a>(l:&J,乢:&'a Y)->亇<'a>{let 丁=l.东();let 丷=仃(l,l.丟(丁));let 乿=仃(l,l.丟(1-丁));let
人=仃(l,G::q(Z[1-丁]));let 乫=l.map().valid.丢(l.丟(0)|l.丟(1));let mut 乎=G::T;for q in 乫.iter(){if 丷[q]<=乿[q]{乎.set(q);}}亇{乢,丁,theirs:乿,人,free:乫,乎,ours:l.丟(丁),丰:l.map().丰,}}fn
value(&self,q:F)->C{let j=self;let p=j.乢;let 举=C::from((G::q(q).且()&j.free).count());let 佉=p.低;let 代=C::from(佉)-C::from(j.theirs[q].min(佉));let 亩=if p.伙>0{C::from(j.人[q].min(30))/p.伙}else{0};let
丒=C::from((G::q(q).且()&j.ours).count());let line=if 丒<=1{p.亹}else{0};let 丸=if j.丰.test(q){p.丸}else{0};let 俿=if 举==0{p.享}else{0};let 云=p.云*(22-C::from(j.人[q].min(22)));p.open*举+p.代*代-亩+p.亨*丒-line+丸-俿+云}fn
上(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伏(&mut self,q:F){self.free.clear(q);self.乎.clear(q);self.ours.set(q);}}fn
仏(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 佀=l.个(三);if 佀{丆.push(三);}佀}fn 丙(l:&mut J,不:F,乮:F,丆:&mut Vec<K>)->Q{let Some((丐,_))=l.他(不)else{return false;};let(x,y)=付(乮);仏(l,K::乱{丐,x,y},丆)}fn
個(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伽}#[derive(Clone,Copy)]enum 乄{仝,亄{丞:I,丅:I},}fn 侀(l:&J,乢:&Y,争:乄)->L<K>{let mut 丬=l.clone();let
mut 下=亇::new(l,乢);let mut 丆=L::new();乺(&mut 丬,&mut 下,&mut 丆);match 争{乄::仝=>俧(&mut 丬,&mut 下,&mut 丆),乄::亄{丞,丅}=>{俍(&mut 丬,&mut 下,丞,丅,&mut 丆)}}丆}fn 乺(l:&mut
J,下:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=下.丁;let mut 仨:L<F> =L::new();loop{let 丣=l.丽(丁,1);let mut 亵:P<(串,F)> =None;for 不 in l.二().iter(){if 仨.contains(&不){continue;}let
乼=(G::q(不).且()&下.free&丣).count();if 乼==0{仨.push(不);}else if 亵.is_none_or(|(n,_)|乼<n){亵=Some((乼,不));}}let Some((_,不))=亵 else{break;};match 下.上(G::q(不).且()&下.free&丣){Some(乮)if
丙(l,不,乮,丆)=>下.伏(乮),_=>仨.push(不),}}for 不 in 仨{if!l.二().test(不){continue;}let 丏=[下.乎,下.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().乆(不,q),q))});if
let Some(丏)=丏{丙(l,不,丏,丆);}}}fn 俧(l:&mut J,下:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=下.丁;let p=下.乢;let h=(p.佝-C::from(l.亊())).max(p.佣);let 倅=(C::from(俏(l))-p.位).max(0)/p.佌.max(1)+p.佁;loop{let
丈=l.丈(丁);let 丅=l.伭(丁)as C;let 俬=C::from(下.乎.count());let q=下.上(l.亗(丁)&下.free&l.丽(丁,1));let 侂=個(h,(俬/(丅+1)).min(倅));let 佈=q.is_some()&&丈>=伽&&侂>0;let 佤=4*h-亁;let
丸=佥(l,下);let 促=丸.is_some()&&丈>=亁&&佤>0;if 促&&(!佈||佤>=侂){let(x,y)=付(丸.unwrap_or_default());if 仏(l,K::亂{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佈)else{break;};let(x,y)=付(q);if!仏(l,K::乖{一:1,x,y},丆){break;}下.伏(q);}}fn
佥(l:&J,下:&亇<'_>)->P<F>{let 丁=下.丁;(l.map().丰&l.丄(丁)&l.亥()).iter().filter(|&丸|下.theirs[丸]>下.乢.伨).min_by_key(|&丸|(l.map().乆(Z[丁],丸),丸))}fn 俍(l:&mut J,下:&mut
亇<'_>,丞:I,丅:I,丆:&mut Vec<K>,){let 丁=下.丁;for _ in 0..丞{let Some(丸)=佥(l,下)else{break;};let(x,y)=付(丸);if!仏(l,K::亂{x,y},丆){break;}}for _ in 0..丅{let Some(q)=下.上(l.亗(丁)&下.free&l.丽(丁,1))else{break;};let(x,y)=付(q);if!仏(l,K::乖{一:1,x,y},丆){break;}下.伏(q);}}fn
仴(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丯=亢::new(l,丁,true);l.丟(1-丁).iter().any(|q|丯.万[q]<=丈)}#[derive(Clone,Copy)]struct
両{亜:I,争:乄,}const 伮:I=I::MAX;const 侈:[乄;8]=[乄::仝,乄::亄{丞:0,丅:0},乄::亄{丞:0,丅:1},乄::亄{丞:0,丅:2},乄::亄{丞:0,丅:伮,},乄::亄{丞:1,丅:0},乄::亄{丞:1,丅:1},乄::亄{丞:1,丅:伮,},];struct
伲<'a>{业:&'a[Y],介:Y,rest:両,乴:P<乓>,俭:Settings,}impl 伲<'_>{fn 乢(&self,丙:両)->&Y{&self.业[F::from(丙.亜)%self.业.len()]}fn 七(&self,l:&J,丙:両)->L<K>{侀(l,self.乢(丙),丙.争)}}fn
伩(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伟=|亽:U|hash=(hash^亽).wrapping_mul(0x0100000001b3);for(倀,临)in[l.丟(丁),l.丅(丁),l.丞()].into_iter().enumerate(){for
q in 临.iter(){伟(((倀 as U)<<8)|q as U);}伟(0xff);}伟(l.丈(丁)as U);hash}enum 亡{仞{丿:F,书:F,乑:亞,},}struct 了{乽:亡,业:L<Y>,介:Y,丕:L<両>,仦:L<U>,亶:F,丮:串,仳:Q,}impl 了{fn
new(乽:亡,业:L<Y>,介:Y)->了{了{乽,业,介,丕:L::new(),仦:L::new(),亶:0,丮:0,仳:false,}}fn 俥(&mut self,l:&J)->Q{let j=self;let 俸=matches!(j.乽,亡::仞{..});let 俦=!俸&&j.丮>0&&j.亶>=j.仦.len();if!j.仳&&(俦||仴(l)){j.仳=true;}!j.仳}fn
下(&mut self,l:&J,乴:P<乓>)->L<K>{let j=self;let 佐=伲{业:&j.业,介:j.介,rest:両{亜:0,争:乄::仝,},乴,俭:Settings::DEFAULT,};let 丁=l.东();let 俟=j.仦.get(j.亶)==Some(&伩(l,丁));if!俟{let
乊=j.丮==0;j.丕=match j.乽{亡::仞{丿,书,乑,}=>{let 乑=&乑;let 乛=侧;let mut 乥=丕(l,&j.业,乛,丿,书,乴,乑,);乥.resize(丿,乛);乥}};j.仦=match j.乽{亡::仞{..}=>俹(l,&j.业,&j.丕),};j.亶=0;j.丮+=1;}let
丙=j.丕.get(j.亶).copied().unwrap_or(佐.rest);j.亶+=1;佐.七(l,丙)}}fn 俹(l:&J,业:&[Y],丕:&[両])->L<U>{let 丁=l.东();let mut 你=L::with_capacity(丕.len());let mut 乣=l.clone();for&丙
in 丕{if 乣.丗().is_some(){break;}你.push(伩(&乣,丁));乣=伇(&乣,&七(&乣,业,丙));}你}const 侱:C=3;struct Settings{丿:C,}impl Settings{const DEFAULT:Settings=Settings{丿:侱};}fn
乥(l:&J,不:G)->[I;M]{let 乯=l.map().valid;let mut 义=[I::MAX;M];let mut 之=不&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.且()&乯).丢(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn
倍(l:&J,z:F)->C{let 仍=乥(l,l.丄(z));let 乿=乥(l,l.丄(1-z));l.map().valid.iter().map(|q|match 仍[q].cmp(&乿[q]){_ if 仍[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侩:[Y;27]=业();const 侧:両=両{亜:16,争:乄::仝,};const fn 业()->[Y;27]{let 亨=[0,2,4];let 亹=[0,3,6];let 云=[0,1,2];let mut 丆=[Y::伅;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
享=if g>0||t>0{4}else{0};丆[i]=Y::伅.倒(亨[g],亹[t],0).們(享,云[w]);i+=1;}丆}#[derive(Default)]struct 伄{丌:C,专:C,丈:C,乸:C,丝:C,仕:C,丯:C,仗:C,}#[derive(Clone,Copy)]struct
亞{丌:C,专:C,丈:C,丝:C,乸:C,仕:C,丯:C,仗:C,}fn 修(l:&J,z:F)->[C;M]{struct 亾{丄:G,久:[u16;M],丵:[u16;M],size:[C;M],丝:[C;M],亐:u16,}fn 些(d:&mut 亾,v:F){d.亐+=1;d.久[v]=d.亐;d.丵[v]=d.亐;d.size[v]=1;for
w in(G::q(v).且()&d.丄).iter(){if d.久[w]==0{些(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.久[v]{d.丝[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.久[w]);}}}let
mut d=亾{丄:l.丄(z),久:[0;M],丵:[0;M],size:[0;M],丝:[0;M],亐:0,};let 乭=Z[z];if d.丄.test(乭){些(&mut d,乭);}d.丝[乭]=0;d.丝}fn 俰(l:&J,z:F)->伄{let 丄=l.丄(z);let 丝=修(l,z);let
人=仃(l,G::q(Z[1-z]));let mut 丆=伄{丌:C::from(丄.count()),专:l.专(z),丈:l.丈(z),丯:C::MAX,仗:倍(l,z)/2,..伄::default()};for q in 丄.iter(){if 丝[q]>0{丆.乸+=1;丆.丝=丆.丝.max(丝[q]);}if
q!=Z[z]&&(G::q(q).且()&丄).count()<=1{丆.仕+=1;}丆.丯=丆.丯.min(C::from(人[q]));}丆}fn 丛(l:&J,z:F,乑:&亞)->C{let s=俰(l,z);let o=乑;o.丌*s.丌+o.专*s.专+o.丈*s.丈-o.丝*s.丝-o.乸*s.乸-o.仕*s.仕-o.丯*s.丯.min(30)+o.仗*s.仗}fn
伇(l:&J,七:&[K])->J{let mut 丂=l.clone();丂.佮(七);if 丂.丗().is_none(){丂.佮(&[]);}丂}fn 七(l:&J,业:&[Y],丙:両)->L<K>{侀(l,&业[F::from(丙.亜)%业.len()],丙.争,)}fn 佭(l:&J,业:&[Y],丕:&[両],乛:両,丿:F,)->J{let
mut 乣=l.clone();for 下 in 0..丿{if 乣.丗().is_some(){break;}let 丙=丕.get(下).copied().unwrap_or(乛);乣=伇(&乣,&七(&乣,业,丙));}乣}fn 俽(l:&J,业:&[Y])->L<(両,L<K>)>{let mut
丆:L<(両,L<K>)> =L::new();for 亜 in 0..业.len().min(F::from(I::MAX)){for&争 in&侈{let 丙=両{亜:亜 as I,争,};let c=七(l,业,丙);if!丆.iter().any(|(_,丩)|*丩==c){丆.push((丙,c));}}}丆}fn
侼(l:&J,业:&[Y],乛:両,丿:F,书:F,乴:P<乓>,乑:&亞,)->L<両>{let 丁=l.东();let 从=||乴.is_some_and(|t|乓::now()>=t);let 仐=|丕:&[両]|丛(&佭(l,业,丕,乛,丿),丁,乑);let mut 上:(C,L<両>)=(仐(&[]),L::new());let
mut 之:L<(J,L<両>)> =vec![(l.clone(),L::new())];for 俇 in 0..丿{let mut 乳:L<(C,J,L<両>)> =L::new();let mut 乃:HashMap<U,F> =HashMap::new();for(佧,丕)in&之{if 从(){break;}for(丙,c)in
俽(佧,业){let 丂=伇(佧,&c);let mut 伎=丕.clone();伎.push(丙);let v=丛(&丂,丁,乑);let 乌=伩(&丂,丁);match 乃.get(&乌){Some(&主)if 乳[主].0>=v=>{}Some(&主)=>乳[主]=(v,丂,伎),None=>{乃.insert(乌,乳.len());乳.push((v,丂,伎));}}}}if
乳.is_empty(){break;}乳.sort_by(|a,b|b.0 .cmp(&a.0));乳.truncate(书);let 乹=俇+1==丿||从();for(i,(_,_,丕))in 乳.iter().take(if 乹{书}else{1}).enumerate(){if i>0&&从(){break;}let
v=仐(丕);if v>上.0{上=(v,丕.clone());}}if 乹{break;}之=乳.into_iter().map(|(_,s,p)|(s,p)).collect();}上.1}fn 丕(l:&J,业:&[Y],乛:両,丿:F,俚:F,乴:P<乓>,乑:&亞,)->L<両>{let 丁=l.东();let
仐=|丕:&[両]|丛(&佭(l,业,丕,乛,丿),丁,乑);let mut 上:(C,L<両>)=(仐(&[]),L::new());let mut 书=2;loop{let 丨=侼(l,业,乛,丿,书,乴,乑);let v=仐(&丨);if v>上.0{上=(v,丨);}let 从=乴.is_some_and(|t|乓::now()>=t);if
从||(乴.is_none()&&书>=俚){break;}书*=2;}上.1}const 例:F=4+12;fn 丿(会:impl BufRead,mut 仲:impl Write,mut 伈:impl FnMut(&str,&str,串,乓)->丱,)->Result<(),丱>{let mut
严=伵{佱:会};let Some(乊)=严.line()?else{return Ok(());};let mut 亪=format!("{}\n",乊);严.take(丑(&乊,"mine spot count")?,&mut 亪)?;let mut 下=0;loop{let Some(乊)=严.line()?else{return
Ok(());};let 亙=乓::now();下+=1;let mut 乜=format!("{}\n",乊);严.take(例-1,&mut 乜)?;for 丶 in["building count","unit count"]{let line=严.佳()?;let n=丑(&line,丶)?;乜.push_str(&line);乜.push('\n');严.take(n,&mut
乜)?;}let 伈=伈(&亪,&乜,下,亙);writeln!(仲,"{}",伈).and_then(|()|仲.flush()).map_err(|五|format!("turn {}: cannot write the answer: {}",下,五))?;}}struct 伵<R>{佱:R,}impl<R:BufRead>伵<R>{fn
line(&mut self)->Result<P<丱>,丱>{let mut line=丱::new();match self.佱.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(五)=>Err(format!("cannot read input: {}",五)),}}fn
佳(&mut self)->Result<丱,丱>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乜:&mut String)->Result<(),丱>{for
_ in 0..n{乜.push_str(&self.佳()?);乜.push('\n');}Ok(())}}fn 丑(line:&str,丶:&str)->Result<F,丱>{match line.trim().parse::<F>(){Ok(n)if n<=侖=>Ok(n),_=>Err(format!("bad {} {line:?}",丶)),}}const
侖:F=144;const 侭:C=3;const 侑:C=2;const 供:F=16;fn 俣(l:&J,丆:&mut Vec<K>){let z=l.东();let mut 乽=伹{s:l.clone(),z,乵:1-z,丏:Z[1-z],仧:Z[z],下:l.亊(),丆,};乽.run();}struct
伹<'a>{s:J,z:F,乵:F,丏:F,仧:F,下:串,丆:&'a mut Vec<K>,}fn 仺(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 传(丐:串,q:F)->K{let(x,y)=仺(q);K::乱{丐,x,y}}fn 使(一:I,q:F)->K{let(x,y)=仺(q);K::乖{一,x,y}}impl
伹<'_>{fn run(&mut self){let j=self;if j.们(){return;}j.乺();if j.们(){return;}j.乐();j.倉();}fn 义(&self,不:F,乮:F)->C{C::from(self.s.map().乆(不,乮))}fn 个(&mut self,三:K)->Q{let
丼=self.s.个(三);if 丼{self.丆.push(三);}丼}fn 亼(&self,丠:&J)->Q{丠.丗()==Some(么::乩(self.z))}fn 们(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.二().iter(){if j.义(q,j.丏)!=1{continue;}let Some((丐,_))=j.s.他(q)else{continue;};let 三=传(丐,j.丏);let mut 丠=j.s.clone();if 丠.个(三)&&j.亼(&丠){return j.个(三);}}let
Some(丧)=j.俄()else{return false;};let mut 丠=j.s.clone();if!丧.iter().all(|&三|丠.个(三))||!j.亼(&丠){return false;}for 三 in 丧{j.个(三);}true}fn 俄(&self)->P<L<K>>{let
j=self;const 伴:C=C::MAX;let 乘=j.s.两().也();let 丣:L<G> =(1..=乘).map(|一|j.s.丽(j.z,一)).collect();let 仆=|q:F|{丣.iter().position(|临|临.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 丄=j.s.丄(j.z);let mut 万=[伴;M];let mut 並=[F::MAX;M];let mut 丼=[false;M];for q in 丄.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丼[q]&&万[q]<伴&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丼[q]=true;if q==j.丏{break;}for&乀 in&j.s.map().且[0][q]{if 乀==乞{break;}let 乀=F::from(乀);if 丼[乀]||丄.test(乀){continue;}let Some((_,亍))=仆(乀)else{continue;};if
万[q]+亍<万[乀]{万[乀]=万[q]+亍;並[乀]=q;}}}if 万[j.丏]>j.s.丈(j.z){return None;}let mut 丧=L::new();let mut q=j.丏;while!丄.test(q){let(一,_)=仆(q)?;丧.push(使(一,q));q=並[q];}丧.reverse();Some(丧)}fn
乬(&self,为:&J)->C{let(丁,亏)=(self.z,self.乵);let 並=&self.s;let 丑=|临:G|C::from(临.count());let 丒=丑(为.丟(丁))-丑(並.丟(丁));let 价=丑(並.丟(亏))-丑(为.丟(亏));let 俗=丑(並.丄(亏))-丑(为.丄(亏));let
仩=並.亯(亏)-为.亯(亏);let 丞=丑(並.丞()&並.丟(亏))-丑(为.丞()&为.丟(亏));12*丒+20*价+8*(俗-价).max(0)+5*仩+25*丞}fn 佸(&self,q:F)->C{let j=self;let 丸=j.s.map().丰.test(q);if 丸&&j.s.两().仠()&&!j.s.丟(j.z).test(q){20}else{0}}fn
何(&self,乁:C)->Q{let j=self;j.s.丅(j.乵).iter().any(|q|j.义(q,j.仧)<=乁)}fn 乺(&mut self){let j=self;let mut 丅:L<(C,F)> =j.s.二().iter().map(|q|(j.义(q,j.丏),q)).collect();丅.sort_unstable();for(俔,q)in
丅{if j.s.丗().is_some(){return;}let Some((丐,一))=j.s.他(q)else{continue;};let 丣=j.s.丽(j.z,一);let 乀=j.s.map().且[j.z][q];let mut 上:P<(C,F)> =None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 丠=j.s.clone();if!丠.个(传(丐,丂)){continue;}let 丛=if j.亼(&丠){100000}else{j.乬(&丠)+10*(俔-j.义(丂,j.丏))+j.佸(丂)};if 上.is_none_or(|(亀,_)|丛>亀){上=Some((丛,丂));}}if
let Some((丛,丂))=上{if 丛>0{j.个(传(丐,丂));}}}}fn 乐(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.何(侭){j.倇();}else{j.丞();}}fn 倇(&mut self){let j=self;if!j.s.两().仡()||j.s.丈(j.z)<亁{return;}let
乀=j.s.map().且[j.z][j.仧];let 丌:L<F> =乀.iter().take_while(|&&q|q!=乞).map(|&q|F::from(q)).collect();let 丷=j.s.丟(j.z);if 丌.iter().any(|&q|丷.test(q)&&j.s.乒().test(q)){return;}for
q in 丌{let(x,y)=仺(q);if j.个(K::亃{x,y}){return;}}}fn 丞(&mut self){let j=self;if!j.s.两().仠(){return;}while j.s.丈(j.z)>=亁{let 乫=j.s.map().丰&j.s.丟(j.z)&j.s.丄(j.z)&j.s.亥();let
Some(q)=乫.iter().min_by_key(|&q|(j.义(q,j.仧),q))else{return;};let(x,y)=仺(q);if!j.个(K::亂{x,y}){return;}}}fn 倉(&mut self){let j=self;let 佴=if j.下<=3{0}else{5};for
_ in 0..供{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俖=j.何(侑);let 仰=j.s.亗(j.z);let mut 上:P<(C,K)> =None;let mut 伫:P<(C,K)> =None;let
mut 但=G::T;for 一 in 1..=j.s.两().也(){let 丣=j.s.丽(j.z,一);let 乼=(仰&丣).丢(但);但=丣;let 亍=10*C::from(一);if 亍>丈{continue;}for q in 乼.iter(){let 三=使(一,q);let mut
丠=j.s.clone();if!丠.个(三){continue;}let 伜=丠.丈(j.z);let 专=丠.专(j.z);let 仩=j.s.亯(j.乵)-丠.亯(j.乵);let 侸=if 仩>0&&俖{伜+专>=0}else{专>=0&&(伜>=佴||仩>0)};if!侸{continue;}let
乬=j.乬(&丠);let 丛=乬+j.佸(q)-6*j.义(q,j.丏)-10*(C::from(一)-1);if 乬>0{if 上.is_none_or(|(亀,_)|丛>亀){上=Some((丛,三));}}else if 专>=3&&伜>=佴+10&&一==1{if 伫.is_none_or(|(亀,_)|丛>亀){伫=Some((丛,三));}}}}let
Some((_,三))=上.or(伫)else{return;};if!j.个(三){return;}}}}fn 俒(l:&J,丆:&mut Vec<K>){丆.clear();俣(l,丆);}const 予:C=C::MAX;const 侚:F=8;struct 亢{万:[C;M],並:[I;M],}fn
伒(l:&J,z:F,乺:Q,丏:F)->C{住(l,z,亖(l,z,乺),丏)}fn 住(l:&J,z:F,亖:G,丏:F)->C{let mut 万=予;佘(l,z,亖,|丙,new|{if new.test(丏){万=10*丙 as C;}万!=予});万}fn 俨(l:&J,z:F,亖:G,丫:C)->G{let
mut 仴=G::T;佘(l,z,亖,|丙,new|{if 10*丙 as C>丫{return true;}仴|=new;false});仴.丢(l.丄(z))}fn 亖(l:&J,z:F,乺:Q)->G{if 乺&&z==l.东(){l.二()}else{G::T}}fn 佘(l:&J,z:F,亖:G,mut
些:impl FnMut(F,G)->Q,){let 乘=l.两().也();let mut 仆=[G::T;3];let mut 乃=G::T;for 一 in 1..=乘{let 临=l.丽(z,一);仆[F::from(一)-1]=临.丢(乃);乃|=临;}let mut 举=[l.丄(z),G::T,G::T,G::T];for
q in(亖&l.丅(z)).iter(){if let Some(一)=l.亚(q).filter(|&一|一<=乘){举[0]|=G::q(q).且()&l.丽(z,一);}}let mut 丼=G::T;for 丙 in 0..{let new=举[丙%4].丢(丼);举[丙%4]=G::T;if
些(丙,new){return;}丼|=new;let 丂=new.且().丢(丼);for(侹,丌)in 仆.iter().enumerate(){举[(丙+侹+1)%4]|=丂&*丌;}if 举.iter().all(|临|临.丢(丼).is_empty()){break;}}}impl 亢{fn
new(l:&J,z:F,乺:Q)->亢{let 乘=F::from(l.两().也());let mut 丣=[G::T;3];for(与,临)in 丣.iter_mut().enumerate().take(乘){*临=l.丽(z,与 as I+1);}let 丄=l.丄(z);let 且=&l.map().且[0];let
mut 万=[予;M];let mut 並=[乞;M];let mut 举=[0u8;M];let mut 丑=0;let mut 佶=G::T;for q in 丄.iter(){万[q]=0;举[丑]=q as I;丑+=1;}if 乺&&z==l.东(){for 不 in l.二().iter(){let
Some(一)=l.亚(不)else{continue;};let 乥=G::q(不).且()&丣[F::from(一)-1];for q in 乥.iter(){if 万[q]!=0{万[q]=0;举[丑]=q as I;丑+=1;}}}}while 丑>0{let mut 上=0;for 与 in
1..丑{let(a,b)=(F::from(举[与]),F::from(举[上]));if(万[a],a)<(万[b],b){上=与;}}let q=F::from(举[上]);丑-=1;举[上]=举[丑];佶.set(q);for&丂 in&且[q]{if 丂==乞{break;}let 丂=F::from(丂);if
佶.test(丂)||丄.test(丂){continue;}let Some(一)=丣[..乘].iter().position(|临|临.test(丂))else{continue;};let 佾=万[q]+10*(一 as C+1);if 佾<万[丂]{if 万[丂]==予{举[丑]=丂 as
I;丑+=1;}万[丂]=佾;並[丂]=q as I;}}}亢{万,並}}fn path(&self,丏:F)->L<F>{let mut path=L::new();if 丏>=M||self.万[丏]==予{return path;}let mut q=丏;while self.並[q]!=乞{path.push(q);q=F::from(self.並[q]);}path.reverse();path}}struct
仛{七:L<K>,万:C,}fn 俙(l:&J,z:F,q:F)->P<I>{(1..=l.两().也()).find(|&一|l.丽(z,一).test(q))}fn 俫(丬:&mut J,丁:F,产:F,丫:C,七:&mut Vec<K>,乤:&mut i64,)->Q{loop{if 丬.丗()==Some(么::乩(丁)){return
true;}if 丬.丟(丁).test(产)&&丬.丄(丁).test(产){return true;}let 丯=亢::new(丬,丁,false);let 万=丯.万[产];if 万==予||*乤+万>丫{return false;}let Some(&q)=丯.path(产).first()else{return
false;};let Some(一)=俙(丬,丁,q)else{return false;};let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(三){return false;}*乤+=10*C::from(一);七.push(三);}}fn 佄(l:&J,丁:F,俑:&[F],丫:C)->P<仛>{let
mut 丬=l.clone();let mut 七=L::new();let mut 乤=0;for&产 in 俑{if!俫(&mut 丬,丁,产,丫,&mut 七,&mut 乤){return None;}}(乤<=丫).then_some(仛{七,万:乤,})}fn 伍(l:&J,丏:F,丫:C)->P<仛>{if
丫<0||l.丗().is_some()||丏>=M||l.map().丹.test(丏){return None;}let 丁=l.东();if l.丟(丁).test(丏)&&l.丄(丁).test(丏){return Some(仛{七:L::new(),万:0,});}let mut 上=佄(l,丁,&[丏],丫);let
乊=亢::new(l,丁,false);let mut 乒:L<F> =(l.乒()&l.丟(1-丁)).iter().filter(|&乇|乇!=丏&&乊.万[乇]!=予).collect();乒.sort_by_key(|&乇|(l.map().乆(乇,丏),乇));乒.truncate(侚);for
乇 in 乒{if let Some(丧)=佄(l,丁,&[乇,丏],丫){if 上.as_ref().is_none_or(|亀|丧.万<亀.万){上=Some(丧);}}}上}const 侘:F=20000;struct 仜{七:L<K>,万:C,丌:C,丅:C,乐:C,专:C,丘:C,}struct
伀{丌:C,丅:C,乐:C,}fn 作(l:&J,z:F)->伀{let 丄=l.丄(z);let 丅=l.丅(z)&丄;let 丛=丅.iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum();伀{丌:C::from(丄.count()),丅:丛,乐:C::from(((l.丞()|l.乒())&丄).count()),}}struct
伻{丁:F,任:G,並:伀,专:C,亮:F,丫:C,亲:F,path:L<K>,丨:L<仜>,index:HashMap<什,F,BuildHasherDefault<DefaultHasher>>,}type 什=[U;3];fn 倐(mut 乌:什,q:F)->什{乌[q/64]|=1<<(q%64);乌}impl
伻{fn 体(&mut self,丬:&J,乹:P<F>,价:什,万:C,)->Q{let j=self;let 丁=j.丁;let 乼=match 乹{None=>丬.亗(丁),Some(q)=>G::q(q).且(),}&j.任;let 乼=乼.丢(丬.丟(丁));let mut 丣=[G::T;3];for
一 in 1..=丬.两().也(){丣[F::from(一)-1]=丬.丽(丁,一);}for q in 乼.iter(){let 乘=F::from(丬.两().也());let Some(与)=丣[..乘].iter().position(|临|临.test(q))else{continue;};let
一=与 as I+1;let 亍=10*C::from(一);if 万+亍>j.丫{continue;}if j.亲>=侘{return false;}j.亲+=1;let mut 丂=丬.clone();let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(三){continue;}j.path.push(三);let
亸=倐(价,q);j.俩(&丂,亸,万+亍);let 俆=j.path.len()<j.亮&&丂.丗().is_none();let 俐=!俆||j.体(&丂,Some(q),亸,万+亍);j.path.pop();if!俐{return false;}}true}fn 俩(&mut self,为:&J,亸:什,万:C){let
j=self;let 仌=1-j.丁;let 伢=作(为,仌);let 丌=j.並.丌-伢.丌;let 丅=j.並.丅-伢.丅;if 丌<=0&&丅<=0{return;}let 乐=j.並.乐-伢.乐;let 专=为.专(j.丁)-j.专;let 倓=为.丗()==Some(么::乩(j.丁));let
丘=if 倓{C::MAX/2}else{3*丌+丅+10*乐+2*专-万};if let Some(&主)=j.index.get(&亸){if 万>=j.丨[主].万{return;}}let 丝=仜{七:j.path.clone(),万,丌,丅,乐,专,丘,};match j.index.get(&亸){Some(&主)=>j.丨[主]=丝,None=>{j.index.insert(亸,j.丨.len());j.丨.push(丝);}}}}fn
佑(l:&J,亮:F,丫:C,乁:F)->L<仜>{乍(l,亮,丫,乁).0}fn 乍(l:&J,亮:F,丫:C,乁:F)->(L<仜>,F){if l.丗().is_some()||亮==0||乁==0{return(L::new(),0);}let 丁=l.东();let 仌=1-丁;let mut
伧=伻{丁,任:l.丟(仌)|l.丟(仌).且(),並:作(l,仌),专:l.专(丁),亮,丫,亲:0,path:L::new(),丨:L::new(),index:HashMap::default(),};伧.体(l,None,[0;3],0);let 亲=伧.亲;let mut 丨=伧.丨;丨.sort_by(|a,b|b.丘.cmp(&a.丘).then(a.万.cmp(&b.万)));丨.truncate(乁);(丨,亲)}const
仟:C=1000000;const 侙:C=200;fn 乎(l:&J,z:F)->C{let 丄=l.丄(z);let 丅:C=(l.丅(z)&丄).iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.专(z)+4*C::from(丄.count())+丅+10*C::from((l.乒()&丄).count())}fn
佢(l:&J,z:F,乙:G)->C{俎(l,z,住(l,z,乙,Z[1-z]),)}fn 俎(l:&J,z:F,万:C)->C{if 万==予{return C::MIN;}l.丈(z)+l.专(z)-万}fn 佯(仮:C)->C{仮.clamp(-侙,0)}fn 俌(l:&J,z:F,乙:[G;2],)->(C,[C;2]){let
丩=1-z;match l.丗(){Some(么::乩(倏))=>{return(if 倏==z{仟}else{-仟},[0,0])}Some(么::伳)=>return(0,[0,0]),None=>{}}let mut 丘=乎(l,z)-乎(l,丩);if z==l.东(){let mut 丂=l.clone();丂.仄();if
丂.丗().is_none()&&伒(&丂,丩,true,Z[z])<=丂.丈(丩){丘-=仟/2;}}let 亭=[佢(l,z,乙[0]),佢(l,丩,乙[1]),];(丘+佯(亭[0])-佯(亭[1]),亭)}
