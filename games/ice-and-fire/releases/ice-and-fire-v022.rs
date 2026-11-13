// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 串=u32;type U=u64;type 仚=i32;type C=i64;type 亝=f64;type Q=bool;type 丱=String;type 乓=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侏:Q=true;fn 俔(l:&J,q:F,仕:C,丯:G,丧:C)->Q{let
丁=l.东();let 乵=1-丁;let 你=侏&&丧<=l.丈(乵)+2*l.且(乵).max(0);let 任=G::q(q)|G::q(q).丒();if!你&&(任&丯).is_empty(){return false;}let(x,y)=互(q);let mut 为=l.clone();if!为.个(K::亃{x,y}){return
false;}if 倃(&为,丁)<仕{return true;}你&&伓(&为,乵,false,Z[丁])>丧}const 侦:I=2;const 侕:F=4;const 伲:F=5;#[derive(Clone)]enum 丳{亟(K),企(L<K>),}impl 丳{fn 丄(&self)->&[K]{match
self{丳::亟(三)=>std::slice::from_ref(三),丳::企(丄)=>丄,}}}fn 互(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 佚(l:&J,丆:&mut Vec<丳>){丆.clear();if l.丗().is_some(){return;}let
丁=l.东();let 乘=l.两().也();let mut 丣=[G::T;4];for 一 in 1..=乘{丣[F::from(一)]=l.丽(丁,一);}for 不 in l.二().iter(){let Some((丐,一))=l.仗(不)else{continue;};let 乥=G::q(不).丒()&丣[F::from(一)];for
乮 in 乥.iter(){let(x,y)=互(乮);丆.push(丳::亟(K::乱{丐,x,y}));}}let 丈=l.丈(丁);let 七=l.七(丁);let 亰=l.亗(丁);let 倊=match 侦{0=>亰,1=>{let 伊=l.map().valid.丝(七).丒()&七;亰.丝(七)|伊}_=>亰.丝(七),};for
一 in 1..=乘{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=倊&丣[F::from(一)];for 俙 in 1..一{丌=丌.丝(丣[F::from(俙)]);}for q in 丌.iter(){let(x,y)=互(q);丆.push(丳::亟(K::乖{一,x,y}));}}if
丈>=亁{let 乫=l.丠(丁)&亰&l.亥();if l.两().仡(){for q in(乫&l.map().丰).iter(){let(x,y)=互(q);丆.push(丳::亟(K::亂{x,y}));}}if l.两().仢(){let 伊=l.map().valid.丝(l.丠(丁)).丒();let
丌=(乫&伊).丝(l.map().丰);let 丯=丯(l,1-丁);let 仕=C::from((丯&l.七(丁)).count());let 丧=伓(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俔(l,q,仕,丯,丧)){let(x,y)=互(q);丆.push(丳::亟(K::亃{x,y}));}}}for
丞 in 佒(l,伲,丈,侕){if 丞.丄.len()>1{丆.push(丳::企(丞.丄));}}if let Some(丧)=伎(l,Z[1-丁],丈){if 丧.丄.len()>1{丆.push(丳::企(丧.丄));}}}const 侢:I=2;const 侰:C=5;fn 乙(l:&J,z:F)->G{伢(l,z,侢)}fn
伢(l:&J,z:F,不:I)->G{let mut 乙=G::T;if 不>3{return 乙;}for q in(l.丅(z)&l.七(z)).iter(){if l.亚(q).is_some_and(|一|一>=不){乙|=G::q(q);}}乙}fn 丯(l:&J,乎:F)->G{let 丫=l.丈(乎)+l.且(乎);俩(l,乎,乙(l,乎),丫)}fn
仈(l:&J,z:F)->G{丯(l,1-z)&l.七(z)}fn 倃(l:&J,z:F)->C{C::from(仈(l,z).count())}const 侟:C=4;fn 俜(丌:C)->C{丌/侟}fn 丟(l:&J,z:F,仈:G)->C{let 七=l.七(z);let 丟=(l.丟()&七).丝(仈);俜(C::from(七.count()))*C::from(丟.count())}const
侬:C=2;fn 侅(l:&J,z:F)->C{let 七=l.七(z);let[a,b,c]=七.仸();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(伿-1);侸.with(|伌|{let mut 伌=伌.borrow_mut();let 件=&mut 伌[与];if 件.0==七&&件.1==z as I+1{return 件.2;}let 丛=倐(七,Z[z]);*件=(七,z as I+1,丛);丛})}const
伿:F=1<<12;thread_local!{static 侸:std::cell::RefCell<L<(G,I,C)>> =std::cell::RefCell::new(vec![(G::T,0,0);伿]);}fn 倐(七:G,乭:F)->C{struct 亿{七:G,久:[I;M],丵:[I;M],size:[I;M],亐:I,乸:C,仰:C,}fn
些(d:&mut 亿,v:F)->I{d.亐+=1;let 久=d.亐;d.久[v]=久;let mut 丵=久;let mut 伫=1u8;let mut 丞=0u8;for w in(G::q(v).丒()&d.七).iter(){if d.久[w]==0{let 佽=些(d,w);伫+=佽;丵=丵.min(d.丵[w]);if
d.丵[w]>=久{丞+=佽;}}else{丵=丵.min(d.久[w]);}}d.丵[v]=丵;d.size[v]=伫;if 丞>0&&d.久[v]>1{d.乸+=1;d.仰=d.仰.max(C::from(丞));}伫}if!七.test(乭){return 0;}let mut d=亿{七,久:[0;M],丵:[0;M],size:[0;M],亐:0,乸:0,仰:0,};些(&mut
d,乭);d.乸+d.仰}fn 俰(l:&J,z:F)->C{侬*(侅(l,1-z)-侅(l,z))}fn 体(l:&J,z:F)->C{if l.丈(z)+l.且(z)>=0{return 0;}(l.丅(z)&l.七(z)).iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum()}const
侔:C=15;const 侶:[C;2]=[3,1];const 侴:[C;2]=[4,20];fn 俿(l:&J,z:F)->[C;2]{let mut 丑=[0;2];for q in(l.丅(z)&l.七(z)).iter(){match l.亚(q){Some(2)=>丑[0]+=1,Some(3)=>丑[1]+=1,_=>{}}}丑}fn
佼(l:&J,z:F)->C{let[侂,倄]=俿(l,z);let 但=|一:F|(3-侶[一])*侴[一];-侔*侂+但(0)*侂+但(1)*倄}const 侎:C=0;const 侑:C=2;const 侐:C=100;fn 佺(仯:C)->C{if 仯<0{return 0;}侎+侑*仯.min(侐)/2}const
伷:I=2;fn 伖(l:&J,z:F)->C{let 乙=[伢(l,z,伷),伢(l,1-z,伷),];let(丘,亭)=俍(l,z,乙);let 丘=丘+佺(亭[0])-佺(亭[1]);if l.丗().is_some(){return 丘;}let 从=仈(l,z);let 乿=仈(l,1-z);let
仕=C::from(乿.count())-C::from(从.count());丘-2*体(l,z)+2*体(l,1-z)+俰(l,z)+佼(l,z)-佼(l,1-z)+侰*仕/4+丟(l,z,从)-丟(l,1-z,乿)}const 侞:F=8;const 侧:F=3;const 侖:F=32;const
侨:U=300;const 侊:亝=0.75;struct 亅{乾:P<乓>,井:P<乓>,介:U,}#[derive(Default)]struct 伽{丄:L<K>,伒:F,乶:U,世:F,丮:F,佋:F,乷:F,}const 侷:Q=true;#[derive(Clone)]struct 乕{丄:L<K>,伕:C,}struct
亠{丄:L<K>,end:J,first:P<U>,专:P<L<乕>>,}type 什=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct 伃{丁:F,limit:亅,乶:U,今:什,亘:P<L<丳>>,}const 伱:Q=true;const
侇:Q=false;const 估:P<I> =None;fn 俀(l:&J,世:&[乕],丆:&mut Vec<丳>){if!伱&&估.is_none(){return;}let 仓=世.iter().map(|乚|乚.丄.as_slice());俁(l,仓,(伱,侇,估),丆,);}fn 俁<'a>(l:&J,仓:impl
Iterator<Item=&'a[K]>,(乒,丒,一):(Q,Q,P<I>),丆:&mut Vec<丳>,){let 丁=l.东();let 丷=l.七(丁);let mut 仆=G::T;for 乚 in 仓{for 三 in 乚{if let K::乖{x,y,..}|K::乱{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丷.test(q){仆|=G::q(q);}}}}if 仆.is_empty(){return;}let 乫=l.亥()&丷;let 丈=l.丈(丁);let 佱=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丄()==[三]){丆.push(丳::亟(三));}};if
乒&&丈>=亁&&l.两().仢(){let mut 丌=仆;if 丒{丌|=仆.丒();}for q in(丌&乫).丝(l.map().丰).iter(){let(x,y)=互(q);佱(K::亃{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().也()){if
丈>=10*C::from(一){for q in(仆&乫).iter(){let(x,y)=互(q);佱(K::乖{一,x,y},丆);}}}}fn 伉(l:&mut J,丄:&[K])->Q{丄.iter().all(|&三|l.个(三))}fn 乌(丄:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丄{let(仫,a,b,c)=match*三{K::乱{丐,x,y}=>(1,U::from(丐),x,y),K::乖{一,x,y}=>(2,U::from(一),x,y),K::亂{x,y}=>(3,0,x,y),K::亃{x,y}=>(4,0,x,y),};let 亾=仫|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亾).wrapping_mul(0x0100000001b3);}hash}fn
佃(乗:&J,乚:&[K])->J{let mut 丂=乗.clone();丂.仅();for&三 in 乚{丂.个(三);}丂.仅();丂}fn 仭(l:&J)->P<L<K>>{let 丁=l.东();let 丏=Z[1-丁];let 亽=|両:&J|両.丗()==Some(么::乩(丁));for
q in l.二().iter(){if l.map().乆(q,丏)!=1{continue;}let Some((丐,_))=l.仗(q)else{continue;};let(x,y)=互(丏);let 三=K::乱{丐,x,y};let mut 両=l.clone();if 両.个(三)&&亽(&両){return
Some(vec![三]);}}let 丧=伎(l,丏,l.丈(丁))?;let mut 両=l.clone();(伉(&mut 両,&丧.丄)&&亽(&両)).then_some(丧.丄)}impl 伃{fn 乤(&self)->Q{self.乶>=self.limit.介||self.limit.乾.is_some_and(|t|乓::now()>=t)}fn
丘(&mut self,乗:&J,世:&[乕],伋:C)->C{self.乶+=1;self.伦(乗,世,伋)}fn 伦(&self,乗:&J,世:&[乕],伋:C)->C{if 乗.丗()==Some(么::乩(self.丁)){return 仠;}if 世.is_empty(){return 伖(乗,self.丁);}let
mut 仹=C::MAX;for 乚 in 世{仹=仹.min(伖(&佃(乗,&乚.丄),self.丁));if 仹<=伋{break;}}仹}#[allow(clippy::too_many_arguments)]fn 俣(&mut self,乪:&J,交:C,乏:&[丳],亵:&[(C,F)],丼:&mut[Q],世:&[乕],亦:&mut
什,伜:Q,)->(P<(F,J,C)>,Q){let mut 上=None;let mut 代=交;for&(仱,与)in 亵{if 丼[与]{continue;}if 伜&&仱!=C::MAX&&仱<=代-交{return(上,true);}if self.乤(){return(上,false);}丼[与]=true;let
丄=乏[与].丄();let mut 丂=乪.clone();if!伉(&mut 丂,丄){亦.insert(乌(丄),C::MIN/4);continue;}let 丘=self.丘(&丂,世,代);亦.insert(乌(丄),丘-交);if 丘>代{代=丘;上=Some((与,丂,丘));}}(上,false)}fn
休(&mut self,乭:&J,世:&[乕],仂:&[U],促:Q)->亠{let j=self;let mut 乊=None;let mut 乪=乭.clone();let mut 丄=L::new();let mut 交=j.丘(&乪,世,C::MIN);let mut 亦=什::default();let
mut 乏=L::new();let mut 亵=L::new();while 乪.丗().is_none()&&!j.乤(){if 丄.is_empty()&&促{match&j.亘{Some(亘)=>乏.clone_from(亘),None=>{佚(&乪,&mut 乏);j.亘=Some(乏.clone());}}}else{佚(&乪,&mut
乏);}俀(&乪,世,&mut 乏);亵.clear();for(与,乲)in 乏.iter().enumerate(){let k=乌(乲.丄());let 仱=亦.get(&k).or(j.今.get(&k)).copied();亵.push((仱.unwrap_or(C::MAX),与));}亵.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丼=vec![false;乏.len()];if 丄.is_empty(){for(与,乲)in 乏.iter().enumerate(){丼[与]=仂.contains(&乌(乲.丄()));}}let mut 上=None;for
伜 in[true,false]{let(丨,俸)=j.俣(&乪,交,&乏,&亵,&mut 丼,世,&mut 亦,伜,);上=丨;if 上.is_some()||!俸{break;}}let Some((与,丂,丘))=上 else{break;};if 丄.is_empty(){乊=Some(乌(乏[与].丄()));}丄.extend_from_slice(乏[与].丄());乪=丂;交=丘;}for(k,乬)in
亦{let 作=j.今.entry(k).or_insert(乬);*作=(*作).max(乬);}亠{丄,end:乪,first:乊,专:None,}}fn 仓(&self,乗:&J)->L<L<K>>{let mut 丂=乗.clone();丂.仅();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 习=丂.东();let 丈=丂.丈(习);if let Some(丧)=伎(&丂,Z[self.丁],丈){丆.push(丧.丄);}for 丞 in 佒(&丂,伲,丈,侧){丆.push(丞.丄);}let mut 佛=L::new();俓(&丂,&mut
佛);丆.push(佛);let mut 乿=伃{丁:习,limit:亅{介:侨,..self.limit},乶:0,今:什::default(),亘:None,};丆.push(乿.休(&丂,&[],&[],true).丄);丆.retain(|丄|!丄.is_empty());丆}fn 佄(&self,乗:&J)->L<乕>{self.仓(乗).into_iter().map(|丄|{let
伕=伖(&佃(乗,&丄),self.丁);乕{丄,伕}}).collect()}fn 乷(&mut self,乭:&J,乲:&丳,世:&[乕])->P<亠>{let mut 为=乭.clone();if!伉(&mut 为,乲.丄()){return None;}self.乶+=1;let 乛=self.休(&为,世,&[],false);let
mut 丄=乲.丄().to_vec();丄.extend_from_slice(&乛.丄);Some(亠{丄,end:乛.end,first:Some(乌(乲.丄())),专:None,})}fn 佗(&self,丕:&亠,世:&[乕])->C{let 丘=self.伦(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>丘.min(self.伦(&丕.end,专,C::MIN)),_=>丘,}}}fn
俫(丨:&[乕],世:&mut Vec<乕>)->Q{let 並:L<_> =世.iter().map(|r|r.丄.clone()).collect();for 乚 in 丨{if 世.iter().any(|r|r.丄==乚.丄){continue;}世.push(乚.clone());}世.sort_by_key(|乚|乚.伕);世.truncate(侞);世.iter().map(|r|&r.丄).ne(&並)}fn
倂(l:&J,乁:亅,亴:&mut 了)->伽{let 丁=l.东();let mut 中=伽::default();if l.丗().is_some(){return 中;}if let Some(丄)=仭(l){中.丄=丄;return 中;}let 主=|俲:亝|match(乁.乾,乁.井){(Some(乾),Some(井))=>Some(乾+(井-乾).mul_f64(俲)),_=>None,};if
亴.俦(l){中.丄=亴.下(l,主(0.4));return 中;}let mut 乍=伃{丁,limit:亅{乾:主(侊),..乁},乶:0,今:什::default(),亘:None,};let mut 世=L::new();let mut 丮:L<亠> =L::new();let mut 仂=L::new();let
mut 仦=false;let mut 乷:L<丳> =L::new();while 中.伒<侖&&(丮.is_empty()||!乍.乤()){let 丕=if 仦{let Some(乲)=乷.pop()else{break;};match 乍.乷(l,&乲,&世){Some(丕)=>丕,None=>continue,}}else{乍.休(l,&世,&仂,true)};中.伒+=1;let
佶=丮.iter().any(|p|p.丄==丕.丄);let 乊=丕.first;let 佻=乍.乤();let mut 丕=丕;let mut 佧=false;if!佻&&!佶{let 丨=乍.佄(&丕.end);佧=俫(&丨,&mut 世);丕.专=Some(丨);}if!佶{丮.push(丕);}if
佻{break;}if!佧&&!仦{match 乊{Some(乊)=>仂.push(乊),None=>{仦=true;乷=乍.亘.clone().unwrap_or_default();乷.retain(|a|!仂.contains(&乌(a.丄())));let 乬=|a:&丳|{乍.今.get(&乌(a.丄())).copied().unwrap_or(C::MIN)};乷.sort_by_key(乬);}}}}中.乷=F::from(仦);中.乶=乍.乶;中.世=世.len();中.丮=丮.len();let
佢=||乁.井.is_some_and(|t|乓::now()>=t);let mut 亸:L<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佢(){break;}亸[与]=Some(乍.佗(&丮[与],&世));}let
亶=|亸:&[P<C>]|{let mut 上:P<(C,F)> =None;for(与,丘)in 亸.iter().enumerate().rev(){if let Some(丘)=*丘{if 上.is_none_or(|(亀,_)|丘>亀){上=Some((丘,与));}}}上};let mut
上=亶(&亸);乍.limit.乾=乁.井;while let Some((_,与))=上{if!侷||丮[与].专.is_some()||佢(){break;}let 丨=乍.佄(&丮[与].end);丮[与].专=Some(丨);中.佋+=1;亸[与]=Some(乍.佗(&丮[与],&世));上=亶(&亸);}if
let Some((_,与))=上{let mut 丬=l.clone();for&三 in&丮[与].丄{if 丬.个(三){中.丄.push(三);}}}中}fn 俥(乁:&dyn Fn(乓,串)->亅,mut 亴:了,伛:impl BufRead,仲:impl Write,)->Result<(),丱>{丿(伛,仲,|亪,乜,下,亙|match
J::read(亪,乜,乧::份,下){Ok(l)=>{let 中=倂(&l,乁(亙,下),&mut 亴);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",下,中.伒,中.乶,中.世,中.佋,中.乷,亙.elapsed());侽(&中.丄)}Err(五)=>{eprintln!("plan: cannot read the input: {}",五);"WAIT".to_string()}},)}const
侈:亝=40.0;const 例:亝=800.0;const 侭:亝=0.6;fn 乁(亙:乓,下:串)->亅{if let Some(介)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亅{乾:None,井:None,介,};}let 仵=倅().min(100.0);let 俞=if 下==1{例}else{侈};let 丫=Duration::from_secs_f64(俞/1000.0*仵);亅{乾:Some(亙+丫.mul_f64(侭)),井:Some(亙+丫),介:U::MAX,}}const
侥:F=7;const 侣:亞=亞{丌:3,且:1,丈:1,丞:2,乸:2,他:2,丯:6,付:1,};const 侤:I=I::MAX;fn 亴()->了{信(64)}fn 信(书:F)->了{let 乽=亡::仟{丿:侥,书,乑:侣,};let 业=侫.iter().map(|&乢|Y{伩:侤,..乢}).collect();了::new(乽,业,Y::伅)}fn
main(){let 俼=io::stdin();let 俽=io::stdout();if let Err(五)=俥(&乁,亴(),俼.lock(),俽.lock()){eprintln!("plan: {}",五);std::process::exit(1);}}const 侱:&str="CG_TIME_SCALE";fn
倅()->亝{俢(std::env::var(侱).ok().as_deref())}fn 俢(丛:P<&str>)->亝{丛.and_then(|乜|乜.trim().parse::<亝>().ok()).filter(|仵|仵.is_finite()&&*仵>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乔:F=4*O;const 乨:U=(1<<乔)-1;const 仛:U=1|1<<12|1<<24|1<<36;const 仾:U=仛<<11;const 伻:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 來:G=G([乨;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乔]>>(q%乔))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乔]|=1<<(q%乔);}#[inline]fn clear(&mut self,q:F){self.0[q/乔]&=!(1<<(q%乔));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仸(self)->[U;3]{self.0}#[inline]fn count(self)->串{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丝(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仼{仼{伟:self.0,主:0,}}#[inline]fn 俷(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伻)<<(乔-O)),(b>>O)|((c&伻)<<(乔-O)),c>>O,])}#[inline]fn 俴(self)->G{let[a,b,c]=self.0;G([(a<<O)&乨,((b<<O)&乨)|(a>>(乔-O)),((c<<O)&乨)|(b>>(乔-O)),])}#[inline]fn
俵(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仾&乨,(b>>1)&!仾&乨,(c>>1)&!仾&乨,])}#[inline]fn 俶(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仛&乨,(b<<1)&!仛&乨,(c<<1)&!仛&乨,])}#[inline]fn
丒(self)->G{let j=self;j.俷()|j.俴()|j.俵()|j.俶()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::來.丝(self)}}struct 仼{伟:[U;3],主:F,}impl Iterator for 仼{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.主<3{let 仮=j.伟[j.主];if 仮!=0{j.伟[j.主]=仮&(仮-1);return Some(j.主*乔+仮.trailing_zeros()as F);}j.主+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乱{丐:串,x:I,y:I},乖{一:I,x:I,y:I},亂{x:I,y:I},亃{x:I,y:I},}impl K{fn 倇(&self)->丱{match*self{K::乱{丐,x,y}=>format!("MOVE {} {x} {y}",丐),K::乖{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::亂{x,y}=>format!("BUILD MINE {x} {y}"),K::亃{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
侽(丄:&[K])->丱{if 丄.is_empty(){return "WAIT".to_string();}let mut line=丱::with_capacity(16*丄.len());for(与,三)in 丄.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&三.倇());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乧{佀,倖,侹,#[default]份,}impl 乧{fn 也(self)->I{match self{乧::佀=>1,_=>3,}}fn 仡(self)->Q{matches!(self,乧::侹|乧::份)}fn 仢(self)->Q{self==乧::份}}const 乞:I=I::MAX;struct
买{丹:G,valid:G,丰:G,乆:Box<[I]>,丒:[[[I;4];M];2],}thread_local!{static 侓:RefCell<P<Arc<买>>> =const{RefCell::new(None)};}impl 买{fn 俳(丹:G,丰:G)->Arc<买>{侓.with(|乹|{let
mut 乹=乹.borrow_mut();if let Some(京)=乹.as_ref(){if 京.丹==丹&&京.丰==丰{return Arc::clone(京);}}let 京=Arc::new(买::build(丹,丰));*乹=Some(Arc::clone(&京));京})}fn build(丹:G,丰:G)->买{let
乯=!丹;const 侮:[[(仚,仚);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丒=[[[乞;4];M];2];for(z,乥)in 侮.iter().enumerate(){for q in
乯.iter(){let(x,y)=((q%O)as 仚,(q/O)as 仚);let mut 丑=0;for&(俊,俋)in 乥{let(佩,佪)=(x+俊,y+俋);if(0..O as 仚).contains(&佩)&&(0..O as 仚).contains(&佪){let 丂=佪 as F*O+佩
as F;if 乯.test(丂){丒[z][q][丑]=丂 as I;丑+=1;}}}}}let mut 乆=vec![乞;M*M].into_boxed_slice();let mut 伥=[0u8;M];for 不 in 乯.iter(){let 于=&mut 乆[不*M..(不+1)*M];于[不]=0;伥[0]=不
as I;let(mut 优,mut 伭)=(0,1);while 优<伭{let q=伥[优]as F;优+=1;for&丂 in&丒[0][q]{if 丂!=乞&&于[丂 as F]==乞{于[丂 as F]=于[q]+1;伥[伭]=丂;伭+=1;}}}}买{丹,valid:乯,丰,乆,丒,}}#[inline]fn
乆(&self,不:F,乮:F)->I{self.乆[不*M+乮]}}const 依:串=100;const 亁:C=15;const 侵:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乩(F),伴,}enum
九{伂,伸,伹,}#[derive(Clone)]struct J{map:Arc<买>,两:乧,专:[G;2],七:[G;2],丷:G,乇:G,丅:[G;2],一:[G;3],事:G,亼:[串;M],丈:[C;2],且:[C;2],东:F,亊:串,亱:串,丗:P<么>,}fn 仉(专:G,亩:F)->G{if!专.test(亩){return
G::T;}let mut 仒=G::q(亩);loop{let 丂=(仒|仒.丒())&专;if 丂==仒{return 仒;}仒=丂;}}impl J{fn 东(&self)->F{self.东}fn 亊(&self)->串{self.亊}fn 丗(&self)->P<么>{self.丗}fn 两(&self)->乧{self.两}fn
亯(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伮(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&买{&self.map}fn 丠(&self,z:F)->G{self.专[z]}fn 七(&self,z:F)->G{self.七[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 仗(&self,q:F)->P<(串,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亼[q],j.伞(q)))}fn 二(&self)->G{self.丅[self.东]&self.事}fn
丟(&self)->G{self.丷}fn 乒(&self)->G{self.乇}fn 亚(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伞(q))}fn 俻(&mut self,z:F){let j=self;j.七=[仉(j.专[0],Z[0]),仉(j.专[1],Z[1])];let
俆=j.丅[z].丝(j.七[z]);j.伧(俆);j.佌();j.丈[z]+=j.且[z];if j.丈[z]<0{j.丈[z]=0;j.伧(j.丅[z]);}j.事|=j.丅[z];}fn 佌(&mut self){let j=self;for z in 0..2{let 七=j.七[z];let
丅=七&j.丅[z];let mut 且=C::from(七.count())+4*C::from((七&j.丷).count());for(一,倎)in 侵.iter().enumerate(){且-=倎*C::from((丅&j.一[一]).count());}j.且[z]=且;}}fn 仅(&mut
self){let j=self;if j.丗.is_some(){return;}if j.东==1{if j.亊>=依{j.丗=Some(match j.亯(0).cmp(&j.亯(1)){std::cmp::Ordering::Greater=>么::乩(0),std::cmp::Ordering::Less=>么::乩(1),std::cmp::Ordering::Equal=>么::伴,});return;}j.亊+=1;}j.东=1-j.东;j.俻(j.东);}fn
佯(&mut self,丄:&[K]){let j=self;if j.丗.is_some(){return;}let 众=j.亱;for&三 in 丄{if j.俉(三,众){continue;}j.个(三);if j.丗.is_some(){return;}}j.仅();}fn 俉(&self,三:K,众:串)->Q{let
j=self;match 三{K::乱{丐,x,y}=>{j.亣(x,y).is_none()||丐>=众||j.佘(j.东,丐).is_none()}K::乖{一,x,y}=>{j.亣(x,y).is_none()||一==0||一>j.两.也()}K::亂{x,y}|K::亃{x,y}=>{j.亣(x,y).is_none()}}}#[inline]fn
亣(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丹.test(y*O+x)).then_some(y*O+x)}fn 佘(&self,z:F,丐:串)->P<F>{self.丅[z].iter().find(|&q|self.亼[q]==丐)}#[inline]fn
伞(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 便(&self,习:F)->G{let 乒=self.乇&self.专[习];let 七=self.七[习];乒|(七&(七&乒).丒())}fn
丽(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 习=1-z;let 丅=j.丅[0]|j.丅[1];let 俌=j.亥();let 倌=match
一{1 if j.两==乧::佀=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::T,};let 丽=俌|j.专[习].丝(倌);if 一<3{丽.丝(j.便(习))}else{丽}}fn 亗(&self,z:F)->G{self.七[z]|self.七[z].丒()}fn
亥(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 俖=G::q(Z[0])|G::q(Z[1]);j.map.valid.丝(丅|j.丷|j.乇|俖)}fn 个(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.东;let 亓=match 三{K::乖{一,x,y}=>j.倉(z,一,x,y),K::乱{丐,x,y}=>j.保(z,丐,x,y),K::亂{x,y}=>j.build(z,true,x,y),K::亃{x,y}=>j.build(z,false,x,y),};let
Some(亓)=亓 else{return false;};match 亓{九::伂=>{}九::伸=>j.七[z]=仉(j.专[z],Z[z]),九::伹=>{j.七=[仉(j.专[0],Z[0]),仉(j.专[1],Z[1])];}}j.佌();if!j.专[0].test(Z[0]){j.丗=Some(么::乩(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(么::乩(0));}true}fn 伧(&mut self,丌:G){let j=self;j.丅[0]=j.丅[0].丝(丌);j.丅[1]=j.丅[1].丝(丌);for 一 in&mut j.一{*一=一.丝(丌);}j.事=j.事.丝(丌);}fn
佾(&mut self,z:F,q:F)->九{let j=self;j.伧(G::q(q));j.丷.clear(q);j.乇.clear(q);let 习=1-z;if j.专[习].test(q){j.专[习].clear(q);j.专[z].set(q);九::伹}else if j.专[z].test(q){九::伂}else{j.专[z].set(q);九::伸}}fn
倉(&mut self,z:F,一:I,x:I,y:I)->P<九>{let j=self;if 一==0||一>j.两.也(){return None;}let q=j.亣(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亗(z).test(q)||!j.丽(z,一).test(q){return
None;}let 亓=j.佾(z,q);j.佭(z,一,q,j.亱);j.亱=j.亱.saturating_add(1);j.丈[z]-=万;Some(亓)}fn 佭(&mut self,z:F,一:I,q:F,丐:串){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亼[q]=丐;}fn
保(&mut self,z:F,丐:串,x:I,y:I)->P<九>{let j=self;let 丏=j.亣(x,y)?;let 不=j.佘(z,丐)?;if!j.事.test(不){return None;}let 一=j.伞(不);let 丣=j.丽(z,一);let 乀=j.map.丒[z][不];if
乀.contains(&(丏 as I))&&!丣.test(丏){return None;}let mut 上=j.map.乆(不,丏);let mut 仃=None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let 丂=F::from(丂);let 乆=j.map.乆(丂,丏);if
乆<上&&丣.test(丂){上=乆;仃=Some(丂);}}let 仃=仃?;let 亓=j.佾(z,仃);j.丅[z].clear(不);j.一[F::from(一)-1].clear(不);j.事.clear(不);j.佭(z,一,仃,丐);Some(亓)}fn build(&mut self,z:F,丷:Q,x:I,y:I)->P<九>{let
j=self;let 侼=if 丷{j.两.仡()}else{j.两.仢()};let q=j.亣(x,y)?;let 俟=j.map.丰.test(q);if!侼||j.丈[z]<亁||俟!=丷||!j.专[z].test(q)||!j.亗(z).test(q)||!j.亥().test(q){return
None;}if 丷{j.丷.set(q);}else{j.乇.set(q);}j.丈[z]-=亁;Some(九::伂)}}const 侘:C=1000000000;const 仿:C=0;const 侠:C=1;const 侲:C=2;impl J{fn read(亪:&str,下:&str,两:乧,亊:串,)->Result<J,丱>{let
mut 丰=L::new();let mut 严=亪.lines();let 丑=伔(&mut 严,"mine spot count")?;for _ in 0..丑{let[x,y]=乻(亲(&mut 严,"a mine spot")?,"a mine spot")?;丰.push((以(x)?,以(y)?));}何(严,"the initial input")?;let
mut 严=下.lines();let mut 伙=[0i64;4];for(丛,丶)in 伙.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伤]=乻(亲(&mut 严,丶)?,丶)?;if 伤.abs()>侘{return
Err(format!("{}: {} is out of range",丶,伤));}*丛=伤;}let mut 丹=G::T;let mut 丌=[G::T;2];let mut 七=[G::T;2];for y in 0..O{let 于=亲(&mut 严,"a map row")?.trim();if
于.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,于));}for(x,仔)in 于.bytes().enumerate(){let q=y*O+x;match 仔{b'#'=>丹.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乂=F::from(仔==b'X'||仔==b'x');丌[乂].set(q);if 仔.is_ascii_uppercase(){七[乂].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仔 as char))}}}}let
係=伔(&mut 严,"building count")?;let mut 乐=L::new();for _ in 0..係{let 乻=乻(亲(&mut 严,"a building")?,"a building")?;乐.push(乻);}let 伮=伔(&mut 严,"unit count")?;let
mut 丅=L::new();for _ in 0..伮{let 乻=乻(亲(&mut 严,"a unit")?,"a unit")?;丅.push(乻);}何(严,"the turn's input")?;let 佬=乐.iter().find(|佈|佈[0]==0&&佈[1]==仿).ok_or("the bot's own HQ is not among the buildings")?;let
佟=伍(佬[2],佬[3],丹)?;let z=Z.iter().position(|&亩|亩==佟).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佟))?;let mut 佸=G::T;for(x,y)in 丰{if
丹.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佸.set(y*O+x);}let 伆=|乂:F|if 乂==0{z}else{1-z};let mut l=J{map:买::俳(丹,佸),两,专:[G::T;2],七:[G::T;2],丷:G::T,乇:G::T,丅:[G::T;2],一:[G::T;3],事:G::T,亼:[0;M],丈:[0;2],且:[0;2],东:z,亊,亱:1,丗:None,};for
乂 in 0..2{let 乡=伆(乂);l.专[乡]=丌[乂];l.七[乡]=七[乂];l.丈[乡]=伙[2*乂];l.且[乡]=伙[2*乂+1];}let mut 佫=G::T;for[乡,仫,x,y]in 乐{let 乂=佳(乡)?;let q=伍(x,y,丹)?;if!丌[乂].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佫.test(q){return Err(format!("two buildings at ({x}, {y})"));}佫.set(q);match 仫{仿
if q==Z[伆(乂)]=>{}仿=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侠=>l.丷.set(q),侲=>l.乇.set(q),_=>return Err(format!("unknown building type {}",仫)),}}let
mut 乃=G::T;let mut 们=0u32;for[乡,丐,一,x,y]in 丅{let 乂=佳(乡)?;let q=伍(x,y,丹)?;if!丌[乂].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乃.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丐=串::try_from(丐).map_err(|_|format!("unit id {} is out of range",丐))?;if
乃.iter().any(|丩|l.亼[丩]==丐){return Err(format!("two units have id {}",丐));}乃.set(q);l.丅[伆(乂)].set(q);l.一[一 as F-1].set(q);l.亼[q]=丐;们=们.max(丐);}if!乃.is_empty(){l.亱=们.checked_add(1).ok_or_else(||format!("unit id {} is out of range",们))?;}l.事=l.丅[z];Ok(l)}}fn
亲<'a>(严:&mut Lines<'a>,丶:&str)->Result<&'a str,丱>{严.next().ok_or_else(||format!("the text ends before {}",丶))}fn 何(mut 严:Lines<'_>,丶:&str)->Result<(),丱>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丶)),None=>Ok(()),}}fn 乻<const N:F>(line:&str,丶:&str)->Result<[C;N],丱>{let
mut 仸=line.split_whitespace();let mut 侄=[0i64;N];for 丛 in&mut 侄{let 亾=仸.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丶))?;*丛=亾.parse().map_err(|_|format!("{}: {:?} is not an integer",丶,亾))?;}if
仸.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丶));}Ok(侄)}fn 伔(严:&mut Lines<'_>,丶:&str)->Result<F,丱>{let[丑]=乻(亲(严,丶)?,丶)?;F::try_from(丑).ok().filter(|&丑|丑<=M).ok_or_else(||format!("{}: {} is out of range",丶,丑))}fn
以(丛:C)->Result<F,丱>{F::try_from(丛).ok().filter(|&丛|丛<O).ok_or_else(||format!("coordinate {} is off the map",丛))}fn 伍(x:C,y:C,丹:G)->Result<F,丱>{let q=以(y)?*O+以(x)?;if
丹.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佳(乡:C)->Result<F,丱>{match 乡{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乡)),}}const
侍:C=15;const 価:C=6;const 伾:C=10;const 侪:I=3;#[derive(Clone,Copy)]struct Y{open:C,令:C,住:I,会:C,佞:C,佤:C,伩:I,低:C,位:C,佂:C,亨:C,人:C,丸:C,享:C,云:C,}impl Y{const
伅:Y=Y{open:3,令:2,住:12,会:4,佞:侍,佤:価,伩:侪,低:2,位:2,佂:0,亨:0,人:0,丸:0,享:0,云:0,};const fn 倔(self,亨:C,人:C,丸:C)->Y{Y{亨,人,丸,..self}}const fn 倓(self,享:C,云:C)->Y{Y{享,云,..self}}}fn
仙(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仄(l:&J,不:G)->[I;M]{let mut 义=[I::MAX;M];let 乯=l.map().valid;let mut 之=不&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.丒()&乯).丝(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn 俐(l:&J)->I{let 义=仄(l,l.丠(0));l.丠(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亇<'a>{乢:&'a Y,丁:F,theirs:[I;M],亻:[I;M],free:G,乎:G,ours:G,丰:G,}impl 亇<'_>{fn new<'a>(l:&J,乢:&'a Y)->亇<'a>{let 丁=l.东();let 丷=仄(l,l.丠(丁));let 乿=仄(l,l.丠(1-丁));let
亻=仄(l,G::q(Z[1-丁]));let 乫=l.map().valid.丝(l.丠(0)|l.丠(1));let mut 乎=G::T;for q in 乫.iter(){if 丷[q]<=乿[q]{乎.set(q);}}亇{乢,丁,theirs:乿,亻,free:乫,乎,ours:l.丠(丁),丰:l.map().丰,}}fn
value(&self,q:F)->C{let j=self;let p=j.乢;let 举=C::from((G::q(q).丒()&j.free).count());let 佊=p.住;let 令=C::from(佊)-C::from(j.theirs[q].min(佊));let 亩=if p.会>0{C::from(j.亻[q].min(30))/p.会}else{0};let
专=C::from((G::q(q).丒()&j.ours).count());let line=if 专<=1{p.人}else{0};let 丸=if j.丰.test(q){p.丸}else{0};let 倀=if 举==0{p.享}else{0};let 云=p.云*(22-C::from(j.亻[q].min(22)));p.open*举+p.令*令-亩+p.亨*专-line+丸-倀+云}fn
上(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伐(&mut self,q:F){self.free.clear(q);self.乎.clear(q);self.ours.set(q);}}fn
仐(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 佁=l.个(三);if 佁{丆.push(三);}佁}fn 丙(l:&mut J,不:F,乮:F,丆:&mut Vec<K>)->Q{let Some((丐,_))=l.仗(不)else{return false;};let(x,y)=仙(乮);仐(l,K::乱{丐,x,y},丆)}fn
倍(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伾}#[derive(Clone,Copy)]enum 乄{仞,亄{丟:I,丅:I},}fn 侁(l:&J,乢:&Y,争:乄)->L<K>{let mut 丬=l.clone();let
mut 下=亇::new(l,乢);let mut 丆=L::new();乺(&mut 丬,&mut 下,&mut 丆);match 争{乄::仞=>俨(&mut 丬,&mut 下,&mut 丆),乄::亄{丟,丅}=>{俎(&mut 丬,&mut 下,丟,丅,&mut 丆)}}丆}fn 乺(l:&mut
J,下:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=下.丁;let mut 仩:L<F> =L::new();loop{let 丣=l.丽(丁,1);let mut 亶:P<(串,F)> =None;for 不 in l.二().iter(){if 仩.contains(&不){continue;}let
乼=(G::q(不).丒()&下.free&丣).count();if 乼==0{仩.push(不);}else if 亶.is_none_or(|(n,_)|乼<n){亶=Some((乼,不));}}let Some((_,不))=亶 else{break;};match 下.上(G::q(不).丒()&下.free&丣){Some(乮)if
丙(l,不,乮,丆)=>下.伐(乮),_=>仩.push(不),}}for 不 in 仩{if!l.二().test(不){continue;}let 丏=[下.乎,下.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().乆(不,q),q))});if
let Some(丏)=丏{丙(l,不,丏,丆);}}}fn 俨(l:&mut J,下:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=下.丁;let p=下.乢;let h=(p.佞-C::from(l.亊())).max(p.佤);let 倆=(C::from(俐(l))-p.低).max(0)/p.位.max(1)+p.佂;loop{let
丈=l.丈(丁);let 丅=l.伮(丁)as C;let 俭=C::from(下.乎.count());let q=下.上(l.亗(丁)&下.free&l.丽(丁,1));let 侃=倍(h,(俭/(丅+1)).min(倆));let 佉=q.is_some()&&丈>=伾&&侃>0;let 佥=4*h-亁;let
丸=佦(l,下);let 俄=丸.is_some()&&丈>=亁&&佥>0;if 俄&&(!佉||佥>=侃){let(x,y)=仙(丸.unwrap_or_default());if 仐(l,K::亂{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佉)else{break;};let(x,y)=仙(q);if!仐(l,K::乖{一:1,x,y},丆){break;}下.伐(q);}}fn
佦(l:&J,下:&亇<'_>)->P<F>{let 丁=下.丁;(l.map().丰&l.七(丁)&l.亥()).iter().filter(|&丸|下.theirs[丸]>下.乢.伩).min_by_key(|&丸|(l.map().乆(Z[丁],丸),丸))}fn 俎(l:&mut J,下:&mut
亇<'_>,丟:I,丅:I,丆:&mut Vec<K>,){let 丁=下.丁;for _ in 0..丟{let Some(丸)=佦(l,下)else{break;};let(x,y)=仙(丸);if!仐(l,K::亂{x,y},丆){break;}}for _ in 0..丅{let Some(q)=下.上(l.亗(丁)&下.free&l.丽(丁,1))else{break;};let(x,y)=仙(q);if!仐(l,K::乖{一:1,x,y},丆){break;}下.伐(q);}}fn
仴(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丯=亢::new(l,丁,true);l.丠(1-丁).iter().any(|q|丯.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亜:I,争:乄,}const 伯:I=I::MAX;const 侉:[乄;8]=[乄::仞,乄::亄{丟:0,丅:0},乄::亄{丟:0,丅:1},乄::亄{丟:0,丅:2},乄::亄{丟:0,丅:伯,},乄::亄{丟:1,丅:0},乄::亄{丟:1,丅:1},乄::亄{丟:1,丅:伯,},];struct
伳<'a>{业:&'a[Y],仌:Y,rest:丢,乴:P<乓>,修:Settings,}impl 伳<'_>{fn 乢(&self,丙:丢)->&Y{&self.业[F::from(丙.亜)%self.业.len()]}fn 丄(&self,l:&J,丙:丢)->L<K>{侁(l,self.乢(丙),丙.争)}}fn
伪(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 传=|亾:U|hash=(hash^亾).wrapping_mul(0x0100000001b3);for(倁,临)in[l.丠(丁),l.丅(丁),l.丟()].into_iter().enumerate(){for
q in 临.iter(){传(((倁 as U)<<8)|q as U);}传(0xff);}传(l.丈(丁)as U);hash}enum 亡{仟{丿:F,书:F,乑:亞,},}struct 了{乽:亡,业:L<Y>,仌:Y,丕:L<丢>,仧:L<U>,亷:F,丮:串,仳:Q,}impl 了{fn
new(乽:亡,业:L<Y>,仌:Y)->了{了{乽,业,仌,丕:L::new(),仧:L::new(),亷:0,丮:0,仳:false,}}fn 俦(&mut self,l:&J)->Q{let j=self;let 俹=matches!(j.乽,亡::仟{..});let 俧=!俹&&j.丮>0&&j.亷>=j.仧.len();if!j.仳&&(俧||仴(l)){j.仳=true;}!j.仳}fn
下(&mut self,l:&J,乴:P<乓>)->L<K>{let j=self;let 佑=伳{业:&j.业,仌:j.仌,rest:丢{亜:0,争:乄::仞,},乴,修:Settings::DEFAULT,};let 丁=l.东();let 俠=j.仧.get(j.亷)==Some(&伪(l,丁));if!俠{let
乊=j.丮==0;j.丕=match j.乽{亡::仟{丿,书,乑,}=>{let 乑=&乑;let 乛=侩;let mut 乥=丕(l,&j.业,乛,丿,书,乴,乑,);乥.resize(丿,乛);乥}};j.仧=match j.乽{亡::仟{..}=>俺(l,&j.业,&j.丕),};j.亷=0;j.丮+=1;}let
丙=j.丕.get(j.亷).copied().unwrap_or(佑.rest);j.亷+=1;佑.丄(l,丙)}}fn 俺(l:&J,业:&[Y],丕:&[丢])->L<U>{let 丁=l.东();let mut 佡=L::with_capacity(丕.len());let mut 乣=l.clone();for&丙
in 丕{if 乣.丗().is_some(){break;}佡.push(伪(&乣,丁));乣=伇(&乣,&丄(&乣,业,丙));}佡}const 侳:C=3;struct Settings{丿:C,}impl Settings{const DEFAULT:Settings=Settings{丿:侳};}fn
乥(l:&J,不:G)->[I;M]{let 乯=l.map().valid;let mut 义=[I::MAX;M];let mut 之=不&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.丒()&乯).丝(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn
倏(l:&J,z:F)->C{let 从=乥(l,l.七(z));let 乿=乥(l,l.七(1-z));l.map().valid.iter().map(|q|match 从[q].cmp(&乿[q]){_ if 从[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侫:[Y;27]=业();const 侩:丢=丢{亜:16,争:乄::仞,};const fn 业()->[Y;27]{let 亨=[0,2,4];let 人=[0,3,6];let 云=[0,1,2];let mut 丆=[Y::伅;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
享=if g>0||t>0{4}else{0};丆[i]=Y::伅.倔(亨[g],人[t],0).倓(享,云[w]);i+=1;}丆}#[derive(Default)]struct 伄{丌:C,且:C,丈:C,乸:C,丞:C,他:C,丯:C,付:C,}#[derive(Clone,Copy)]struct
亞{丌:C,且:C,丈:C,丞:C,乸:C,他:C,丯:C,付:C,}fn 俯(l:&J,z:F)->[C;M]{struct 亿{七:G,久:[u16;M],丵:[u16;M],size:[C;M],丞:[C;M],亐:u16,}fn 些(d:&mut 亿,v:F){d.亐+=1;d.久[v]=d.亐;d.丵[v]=d.亐;d.size[v]=1;for
w in(G::q(v).丒()&d.七).iter(){if d.久[w]==0{些(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.久[v]{d.丞[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.久[w]);}}}let
mut d=亿{七:l.七(z),久:[0;M],丵:[0;M],size:[0;M],丞:[0;M],亐:0,};let 乭=Z[z];if d.七.test(乭){些(&mut d,乭);}d.丞[乭]=0;d.丞}fn 俱(l:&J,z:F)->伄{let 七=l.七(z);let 丞=俯(l,z);let
亻=仄(l,G::q(Z[1-z]));let mut 丆=伄{丌:C::from(七.count()),且:l.且(z),丈:l.丈(z),丯:C::MAX,付:倏(l,z)/2,..伄::default()};for q in 七.iter(){if 丞[q]>0{丆.乸+=1;丆.丞=丆.丞.max(丞[q]);}if
q!=Z[z]&&(G::q(q).丒()&七).count()<=1{丆.他+=1;}丆.丯=丆.丯.min(C::from(亻[q]));}丆}fn 丛(l:&J,z:F,乑:&亞)->C{let s=俱(l,z);let o=乑;o.丌*s.丌+o.且*s.且+o.丈*s.丈-o.丞*s.丞-o.乸*s.乸-o.他*s.他-o.丯*s.丯.min(30)+o.付*s.付}fn
伇(l:&J,丄:&[K])->J{let mut 丂=l.clone();丂.佯(丄);if 丂.丗().is_none(){丂.佯(&[]);}丂}fn 丄(l:&J,业:&[Y],丙:丢)->L<K>{侁(l,&业[F::from(丙.亜)%业.len()],丙.争,)}fn 佮(l:&J,业:&[Y],丕:&[丢],乛:丢,丿:F,)->J{let
mut 乣=l.clone();for 下 in 0..丿{if 乣.丗().is_some(){break;}let 丙=丕.get(下).copied().unwrap_or(乛);乣=伇(&乣,&丄(&乣,业,丙));}乣}fn 俾(l:&J,业:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亜 in 0..业.len().min(F::from(I::MAX)){for&争 in&侉{let 丙=丢{亜:亜 as I,争,};let c=丄(l,业,丙);if!丆.iter().any(|(_,丩)|*丩==c){丆.push((丙,c));}}}丆}fn
侾(l:&J,业:&[Y],乛:丢,丿:F,书:F,乴:P<乓>,乑:&亞,)->L<丢>{let 丁=l.东();let 仏=||乴.is_some_and(|t|乓::now()>=t);let 仑=|丕:&[丢]|丛(&佮(l,业,丕,乛,丿),丁,乑);let mut 上:(C,L<丢>)=(仑(&[]),L::new());let
mut 之:L<(J,L<丢>)> =vec![(l.clone(),L::new())];for 俈 in 0..丿{let mut 乳:L<(C,J,L<丢>)> =L::new();let mut 乃:HashMap<U,F> =HashMap::new();for(佨,丕)in&之{if 仏(){break;}for(丙,c)in
俾(佨,业){let 丂=伇(佨,&c);let mut 伏=丕.clone();伏.push(丙);let v=丛(&丂,丁,乑);let 乌=伪(&丂,丁);match 乃.get(&乌){Some(&主)if 乳[主].0>=v=>{}Some(&主)=>乳[主]=(v,丂,伏),None=>{乃.insert(乌,乳.len());乳.push((v,丂,伏));}}}}if
乳.is_empty(){break;}乳.sort_by(|a,b|b.0 .cmp(&a.0));乳.truncate(书);let 乹=俈+1==丿||仏();for(i,(_,_,丕))in 乳.iter().take(if 乹{书}else{1}).enumerate(){if i>0&&仏(){break;}let
v=仑(丕);if v>上.0{上=(v,丕.clone());}}if 乹{break;}之=乳.into_iter().map(|(_,s,p)|(s,p)).collect();}上.1}fn 丕(l:&J,业:&[Y],乛:丢,丿:F,俛:F,乴:P<乓>,乑:&亞,)->L<丢>{let 丁=l.东();let
仑=|丕:&[丢]|丛(&佮(l,业,丕,乛,丿),丁,乑);let mut 上:(C,L<丢>)=(仑(&[]),L::new());let mut 书=2;loop{let 丨=侾(l,业,乛,丿,书,乴,乑);let v=仑(&丨);if v>上.0{上=(v,丨);}let 仏=乴.is_some_and(|t|乓::now()>=t);if
仏||(乴.is_none()&&书>=俛){break;}书*=2;}上.1}const 侌:F=4+12;fn 丿(伛:impl BufRead,mut 仲:impl Write,mut 伈:impl FnMut(&str,&str,串,乓)->丱,)->Result<(),丱>{let mut
严=伶{佲:伛};let Some(乊)=严.line()?else{return Ok(());};let mut 亪=format!("{}\n",乊);严.take(丑(&乊,"mine spot count")?,&mut 亪)?;let mut 下=0;loop{let Some(乊)=严.line()?else{return
Ok(());};let 亙=乓::now();下+=1;let mut 乜=format!("{}\n",乊);严.take(侌-1,&mut 乜)?;for 丶 in["building count","unit count"]{let line=严.佴()?;let n=丑(&line,丶)?;乜.push_str(&line);乜.push('\n');严.take(n,&mut
乜)?;}let 伈=伈(&亪,&乜,下,亙);writeln!(仲,"{}",伈).and_then(|()|仲.flush()).map_err(|五|format!("turn {}: cannot write the answer: {}",下,五))?;}}struct 伶<R>{佲:R,}impl<R:BufRead>伶<R>{fn
line(&mut self)->Result<P<丱>,丱>{let mut line=丱::new();match self.佲.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(五)=>Err(format!("cannot read input: {}",五)),}}fn
佴(&mut self)->Result<丱,丱>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乜:&mut String)->Result<(),丱>{for
_ in 0..n{乜.push_str(&self.佴()?);乜.push('\n');}Ok(())}}fn 丑(line:&str,丶:&str)->Result<F,丱>{match line.trim().parse::<F>(){Ok(n)if n<=侗=>Ok(n),_=>Err(format!("bad {} {line:?}",丶)),}}const
侗:F=144;const 侯:C=3;const 侒:C=2;const 侜:F=16;fn 俤(l:&J,丆:&mut Vec<K>){let z=l.东();let mut 乽=伺{s:l.clone(),z,乵:1-z,丏:Z[1-z],仨:Z[z],下:l.亊(),丆,};乽.run();}struct
伺<'a>{s:J,z:F,乵:F,丏:F,仨:F,下:串,丆:&'a mut Vec<K>,}fn 仺(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伡(丐:串,q:F)->K{let(x,y)=仺(q);K::乱{丐,x,y}}fn 侀(一:I,q:F)->K{let(x,y)=仺(q);K::乖{一,x,y}}impl
伺<'_>{fn run(&mut self){let j=self;if j.仭(){return;}j.乺();if j.仭(){return;}j.乐();j.個();}fn 义(&self,不:F,乮:F)->C{C::from(self.s.map().乆(不,乮))}fn 个(&mut self,三:K)->Q{let
丼=self.s.个(三);if 丼{self.丆.push(三);}丼}fn 亽(&self,両:&J)->Q{両.丗()==Some(么::乩(self.z))}fn 仭(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.二().iter(){if j.义(q,j.丏)!=1{continue;}let Some((丐,_))=j.s.仗(q)else{continue;};let 三=伡(丐,j.丏);let mut 両=j.s.clone();if 両.个(三)&&j.亽(&両){return j.个(三);}}let
Some(丧)=j.俅()else{return false;};let mut 両=j.s.clone();if!丧.iter().all(|&三|両.个(三))||!j.亽(&両){return false;}for 三 in 丧{j.个(三);}true}fn 俅(&self)->P<L<K>>{let
j=self;const 伵:C=C::MAX;let 乘=j.s.两().也();let 丣:L<G> =(1..=乘).map(|一|j.s.丽(j.z,一)).collect();let 仇=|q:F|{丣.iter().position(|临|临.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 七=j.s.七(j.z);let mut 万=[伵;M];let mut 並=[F::MAX;M];let mut 丼=[false;M];for q in 七.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丼[q]&&万[q]<伵&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丼[q]=true;if q==j.丏{break;}for&乀 in&j.s.map().丒[0][q]{if 乀==乞{break;}let 乀=F::from(乀);if 丼[乀]||七.test(乀){continue;}let Some((_,亍))=仇(乀)else{continue;};if
万[q]+亍<万[乀]{万[乀]=万[q]+亍;並[乀]=q;}}}if 万[j.丏]>j.s.丈(j.z){return None;}let mut 丧=L::new();let mut q=j.丏;while!七.test(q){let(一,_)=仇(q)?;丧.push(侀(一,q));q=並[q];}丧.reverse();Some(丧)}fn
乬(&self,为:&J)->C{let(丁,亏)=(self.z,self.乵);let 並=&self.s;let 丑=|临:G|C::from(临.count());let 专=丑(为.丠(丁))-丑(並.丠(丁));let 价=丑(並.丠(亏))-丑(为.丠(亏));let 俘=丑(並.七(亏))-丑(为.七(亏));let
仪=並.亯(亏)-为.亯(亏);let 丟=丑(並.丟()&並.丠(亏))-丑(为.丟()&为.丠(亏));12*专+20*价+8*(俘-价).max(0)+5*仪+25*丟}fn 佹(&self,q:F)->C{let j=self;let 丸=j.s.map().丰.test(q);if 丸&&j.s.两().仡()&&!j.s.丠(j.z).test(q){20}else{0}}fn
佖(&self,乁:C)->Q{let j=self;j.s.丅(j.乵).iter().any(|q|j.义(q,j.仨)<=乁)}fn 乺(&mut self){let j=self;let mut 丅:L<(C,F)> =j.s.二().iter().map(|q|(j.义(q,j.丏),q)).collect();丅.sort_unstable();for(俕,q)in
丅{if j.s.丗().is_some(){return;}let Some((丐,一))=j.s.仗(q)else{continue;};let 丣=j.s.丽(j.z,一);let 乀=j.s.map().丒[j.z][q];let mut 上:P<(C,F)> =None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.个(伡(丐,丂)){continue;}let 丛=if j.亽(&両){100000}else{j.乬(&両)+10*(俕-j.义(丂,j.丏))+j.佹(丂)};if 上.is_none_or(|(亀,_)|丛>亀){上=Some((丛,丂));}}if
let Some((丛,丂))=上{if 丛>0{j.个(伡(丐,丂));}}}}fn 乐(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.佖(侯){j.倈();}else{j.丟();}}fn 倈(&mut self){let j=self;if!j.s.两().仢()||j.s.丈(j.z)<亁{return;}let
乀=j.s.map().丒[j.z][j.仨];let 丌:L<F> =乀.iter().take_while(|&&q|q!=乞).map(|&q|F::from(q)).collect();let 丷=j.s.丠(j.z);if 丌.iter().any(|&q|丷.test(q)&&j.s.乒().test(q)){return;}for
q in 丌{let(x,y)=仺(q);if j.个(K::亃{x,y}){return;}}}fn 丟(&mut self){let j=self;if!j.s.两().仡(){return;}while j.s.丈(j.z)>=亁{let 乫=j.s.map().丰&j.s.丠(j.z)&j.s.七(j.z)&j.s.亥();let
Some(q)=乫.iter().min_by_key(|&q|(j.义(q,j.仨),q))else{return;};let(x,y)=仺(q);if!j.个(K::亂{x,y}){return;}}}fn 個(&mut self){let j=self;let 併=if j.下<=3{0}else{5};for
_ in 0..侜{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俗=j.佖(侒);let 亰=j.s.亗(j.z);let mut 上:P<(C,K)> =None;let mut 伬:P<(C,K)> =None;let
mut 佇=G::T;for 一 in 1..=j.s.两().也(){let 丣=j.s.丽(j.z,一);let 乼=(亰&丣).丝(佇);佇=丣;let 亍=10*C::from(一);if 亍>丈{continue;}for q in 乼.iter(){let 三=侀(一,q);let mut
両=j.s.clone();if!両.个(三){continue;}let 伝=両.丈(j.z);let 且=両.且(j.z);let 仪=j.s.亯(j.乵)-両.亯(j.乵);let 侺=if 仪>0&&俗{伝+且>=0}else{且>=0&&(伝>=併||仪>0)};if!侺{continue;}let
乬=j.乬(&両);let 丛=乬+j.佹(q)-6*j.义(q,j.丏)-10*(C::from(一)-1);if 乬>0{if 上.is_none_or(|(亀,_)|丛>亀){上=Some((丛,三));}}else if 且>=3&&伝>=併+10&&一==1{if 伬.is_none_or(|(亀,_)|丛>亀){伬=Some((丛,三));}}}}let
Some((_,三))=上.or(伬)else{return;};if!j.个(三){return;}}}}fn 俓(l:&J,丆:&mut Vec<K>){丆.clear();俤(l,丆);}const 予:C=C::MAX;const 供:F=8;struct 亢{万:[C;M],並:[I;M],}fn
伓(l:&J,z:F,乺:Q,丏:F)->C{佐(l,z,亖(l,z,乺),丏)}fn 佐(l:&J,z:F,亖:G,丏:F)->C{let mut 万=予;余(l,z,亖,|丙,new|{if new.test(丏){万=10*丙 as C;}万!=予});万}fn 俩(l:&J,z:F,亖:G,丫:C)->G{let
mut 仴=G::T;余(l,z,亖,|丙,new|{if 10*丙 as C>丫{return true;}仴|=new;false});仴.丝(l.七(z))}fn 亖(l:&J,z:F,乺:Q)->G{if 乺&&z==l.东(){l.二()}else{G::T}}fn 余(l:&J,z:F,亖:G,mut
些:impl FnMut(F,G)->Q,){let 乘=l.两().也();let mut 仇=[G::T;3];let mut 乃=G::T;for 一 in 1..=乘{let 临=l.丽(z,一);仇[F::from(一)-1]=临.丝(乃);乃|=临;}let mut 举=[l.七(z),G::T,G::T,G::T];for
q in(亖&l.丅(z)).iter(){if let Some(一)=l.亚(q).filter(|&一|一<=乘){举[0]|=G::q(q).丒()&l.丽(z,一);}}let mut 丼=G::T;for 丙 in 0..{let new=举[丙%4].丝(丼);举[丙%4]=G::T;if
些(丙,new){return;}丼|=new;let 丂=new.丒().丝(丼);for(侻,丌)in 仇.iter().enumerate(){举[(丙+侻+1)%4]|=丂&*丌;}if 举.iter().all(|临|临.丝(丼).is_empty()){break;}}}impl 亢{fn
new(l:&J,z:F,乺:Q)->亢{let 乘=F::from(l.两().也());let mut 丣=[G::T;3];for(与,临)in 丣.iter_mut().enumerate().take(乘){*临=l.丽(z,与 as I+1);}let 七=l.七(z);let 丒=&l.map().丒[0];let
mut 万=[予;M];let mut 並=[乞;M];let mut 举=[0u8;M];let mut 丑=0;let mut 佷=G::T;for q in 七.iter(){万[q]=0;举[丑]=q as I;丑+=1;}if 乺&&z==l.东(){for 不 in l.二().iter(){let
Some(一)=l.亚(不)else{continue;};let 乥=G::q(不).丒()&丣[F::from(一)-1];for q in 乥.iter(){if 万[q]!=0{万[q]=0;举[丑]=q as I;丑+=1;}}}}while 丑>0{let mut 上=0;for 与 in
1..丑{let(a,b)=(F::from(举[与]),F::from(举[上]));if(万[a],a)<(万[b],b){上=与;}}let q=F::from(举[上]);丑-=1;举[上]=举[丑];佷.set(q);for&丂 in&丒[q]{if 丂==乞{break;}let 丂=F::from(丂);if
佷.test(丂)||七.test(丂){continue;}let Some(一)=丣[..乘].iter().position(|临|临.test(丂))else{continue;};let 使=万[q]+10*(一 as C+1);if 使<万[丂]{if 万[丂]==予{举[丑]=丂 as
I;丑+=1;}万[丂]=使;並[丂]=q as I;}}}亢{万,並}}fn path(&self,丏:F)->L<F>{let mut path=L::new();if 丏>=M||self.万[丏]==予{return path;}let mut q=丏;while self.並[q]!=乞{path.push(q);q=F::from(self.並[q]);}path.reverse();path}}struct
仜{丄:L<K>,万:C,}fn 俚(l:&J,z:F,q:F)->P<I>{(1..=l.两().也()).find(|&一|l.丽(z,一).test(q))}fn 俬(丬:&mut J,丁:F,产:F,丫:C,丄:&mut Vec<K>,乤:&mut i64,)->Q{loop{if 丬.丗()==Some(么::乩(丁)){return
true;}if 丬.丠(丁).test(产)&&丬.七(丁).test(产){return true;}let 丯=亢::new(丬,丁,false);let 万=丯.万[产];if 万==予||*乤+万>丫{return false;}let Some(&q)=丯.path(产).first()else{return
false;};let Some(一)=俚(丬,丁,q)else{return false;};let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(三){return false;}*乤+=10*C::from(一);丄.push(三);}}fn 佅(l:&J,丁:F,俒:&[F],丫:C)->P<仜>{let
mut 丬=l.clone();let mut 丄=L::new();let mut 乤=0;for&产 in 俒{if!俬(&mut 丬,丁,产,丫,&mut 丄,&mut 乤){return None;}}(乤<=丫).then_some(仜{丄,万:乤,})}fn 伎(l:&J,丏:F,丫:C)->P<仜>{if
丫<0||l.丗().is_some()||丏>=M||l.map().丹.test(丏){return None;}let 丁=l.东();if l.丠(丁).test(丏)&&l.七(丁).test(丏){return Some(仜{丄:L::new(),万:0,});}let mut 上=佅(l,丁,&[丏],丫);let
乊=亢::new(l,丁,false);let mut 乒:L<F> =(l.乒()&l.丠(1-丁)).iter().filter(|&乇|乇!=丏&&乊.万[乇]!=予).collect();乒.sort_by_key(|&乇|(l.map().乆(乇,丏),乇));乒.truncate(供);for
乇 in 乒{if let Some(丧)=佅(l,丁,&[乇,丏],丫){if 上.as_ref().is_none_or(|亀|丧.万<亀.万){上=Some(丧);}}}上}const 侙:F=20000;struct 仝{丄:L<K>,万:C,丌:C,丅:C,乐:C,且:C,丘:C,}struct
伀{丌:C,丅:C,乐:C,}fn 佝(l:&J,z:F)->伀{let 七=l.七(z);let 丅=l.丅(z)&七;let 丛=丅.iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum();伀{丌:C::from(七.count()),丅:丛,乐:C::from(((l.丟()|l.乒())&七).count()),}}struct
似{丁:F,任:G,並:伀,且:C,亮:F,丫:C,亳:F,path:L<K>,丨:L<仝>,index:HashMap<仁,F,BuildHasherDefault<DefaultHasher>>,}type 仁=[U;3];fn 倒(mut 乌:仁,q:F)->仁{乌[q/64]|=1<<(q%64);乌}impl
似{fn 佔(&mut self,丬:&J,乹:P<F>,价:仁,万:C,)->Q{let j=self;let 丁=j.丁;let 乼=match 乹{None=>丬.亗(丁),Some(q)=>G::q(q).丒(),}&j.任;let 乼=乼.丝(丬.丠(丁));let mut 丣=[G::T;3];for
一 in 1..=丬.两().也(){丣[F::from(一)-1]=丬.丽(丁,一);}for q in 乼.iter(){let 乘=F::from(丬.两().也());let Some(与)=丣[..乘].iter().position(|临|临.test(q))else{continue;};let
一=与 as I+1;let 亍=10*C::from(一);if 万+亍>j.丫{continue;}if j.亳>=侙{return false;}j.亳+=1;let mut 丂=丬.clone();let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(三){continue;}j.path.push(三);let
亹=倒(价,q);j.俪(&丂,亹,万+亍);let 俇=j.path.len()<j.亮&&丂.丗().is_none();let 俑=!俇||j.佔(&丂,Some(q),亹,万+亍);j.path.pop();if!俑{return false;}}true}fn 俪(&mut self,为:&J,亹:仁,万:C){let
j=self;let 仍=1-j.丁;let 伣=佝(为,仍);let 丌=j.並.丌-伣.丌;let 丅=j.並.丅-伣.丅;if 丌<=0&&丅<=0{return;}let 乐=j.並.乐-伣.乐;let 且=为.且(j.丁)-j.且;let 倕=为.丗()==Some(么::乩(j.丁));let
丘=if 倕{C::MAX/2}else{3*丌+丅+10*乐+2*且-万};if let Some(&主)=j.index.get(&亹){if 万>=j.丨[主].万{return;}}let 丞=仝{丄:j.path.clone(),万,丌,丅,乐,且,丘,};match j.index.get(&亹){Some(&主)=>j.丨[主]=丞,None=>{j.index.insert(亹,j.丨.len());j.丨.push(丞);}}}}fn
佒(l:&J,亮:F,丫:C,乁:F)->L<仝>{乍(l,亮,丫,乁).0}fn 乍(l:&J,亮:F,丫:C,乁:F)->(L<仝>,F){if l.丗().is_some()||亮==0||乁==0{return(L::new(),0);}let 丁=l.东();let 仍=1-丁;let mut
伨=似{丁,任:l.丠(仍)|l.丠(仍).丒(),並:佝(l,仍),且:l.且(丁),亮,丫,亳:0,path:L::new(),丨:L::new(),index:HashMap::default(),};伨.佔(l,None,[0;3],0);let 亳=伨.亳;let mut 丨=伨.丨;丨.sort_by(|a,b|b.丘.cmp(&a.丘).then(a.万.cmp(&b.万)));丨.truncate(乁);(丨,亳)}const
仠:C=1000000;const 侚:C=200;fn 乎(l:&J,z:F)->C{let 七=l.七(z);let 丅:C=(l.丅(z)&七).iter().filter_map(|q|l.亚(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.且(z)+4*C::from(七.count())+丅+10*C::from((l.乒()&七).count())}fn
佣(l:&J,z:F,乙:G)->C{俏(l,z,佐(l,z,乙,Z[1-z]),)}fn 俏(l:&J,z:F,万:C)->C{if 万==予{return C::MIN;}l.丈(z)+l.且(z)-万}fn 佰(仯:C)->C{仯.clamp(-侚,0)}fn 俍(l:&J,z:F,乙:[G;2],)->(C,[C;2]){let
丩=1-z;match l.丗(){Some(么::乩(們))=>{return(if 們==z{仠}else{-仠},[0,0])}Some(么::伴)=>return(0,[0,0]),None=>{}}let mut 丘=乎(l,z)-乎(l,丩);if z==l.东(){let mut 丂=l.clone();丂.仅();if
丂.丗().is_none()&&伓(&丂,丩,true,Z[z])<=丂.丈(丩){丘-=仠/2;}}let 亭=[佣(l,z,乙[0]),佣(l,丩,乙[1]),];(丘+佰(亭[0])-佰(亭[1]),亭)}
