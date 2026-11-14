// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 串=u32;type U=u64;type 仛=i32;type C=i64;type 亟=f64;type Q=bool;type 丱=String;type 乓=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侓:Q=true;fn 俘(l:&J,q:F,他:C,丯:G,丧:C)->Q{let
丁=l.丝();let 乵=1-丁;let 佢=侓&&丧<=l.丈(乵)+2*l.且(乵).max(0);let 任=G::q(q)|G::q(q).丒();if!佢&&(任&丯).is_empty(){return false;}let(x,y)=五(q);let mut 为=l.clone();if!为.个(K::亃{x,y}){return
false;}if 倈(&为,丁)<他{return true;}佢&&伓(&为,乵,false,Z[丁])>丧}const 侪:I=2;const 侙:F=4;const 伲:F=5;#[derive(Clone)]enum 丳{亡(K),企(L<K>),}impl 丳{fn 丄(&self)->&[K]{match
self{丳::亡(三)=>std::slice::from_ref(三),丳::企(丄)=>丄,}}}fn 五(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 作(l:&J,丆:&mut Vec<丳>){丆.clear();if l.丗().is_some(){return;}let
丁=l.丝();let 乘=l.两().也();let mut 丣=[G::T;4];for 一 in 1..=乘{丣[F::from(一)]=l.丽(丁,一);}for 不 in l.于().iter(){let Some((丐,一))=l.付(不)else{continue;};let 乥=G::q(不).丒()&丣[F::from(一)];for
乮 in 乥.iter(){let(x,y)=五(乮);丆.push(丳::亡(K::乱{丐,x,y}));}}let 丈=l.丈(丁);let 七=l.七(丁);let 亱=l.亙(丁);let 倏=match 侪{0=>亱,1=>{let 伊=l.map().valid.丞(七).丒()&七;亱.丞(七)|伊}_=>亱.丞(七),};for
一 in 1..=乘{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=倏&丣[F::from(一)];for 俞 in 1..一{丌=丌.丞(丣[F::from(俞)]);}for q in 丌.iter(){let(x,y)=五(q);丆.push(丳::亡(K::乖{一,x,y}));}}if
丈>=亁{let 乫=l.丠(丁)&亱&l.产();if l.两().仢(){for q in(乫&l.map().丰).iter(){let(x,y)=五(q);丆.push(丳::亡(K::亂{x,y}));}}if l.两().代(){let 伊=l.map().valid.丞(l.丠(丁)).丒();let
丌=(乫&伊).丞(l.map().丰);let 丯=丯(l,1-丁);let 他=C::from((丯&l.七(丁)).count());let 丧=伓(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俘(l,q,他,丯,丧)){let(x,y)=五(q);丆.push(丳::亡(K::亃{x,y}));}}}for
东 in 佔(l,伲,丈,侙){if 东.丄.len()>1{丆.push(丳::企(东.丄));}}if let Some(丧)=伎(l,Z[1-丁],丈){if 丧.丄.len()>1{丆.push(丳::企(丧.丄));}}}const 侦:I=2;const 侴:C=5;fn 乙(l:&J,z:F)->G{伢(l,z,侦)}fn
伢(l:&J,z:F,不:I)->G{let mut 乙=G::T;if 不>3{return 乙;}for q in(l.丅(z)&l.七(z)).iter(){if l.亜(q).is_some_and(|一|一>=不){乙|=G::q(q);}}乙}fn 丯(l:&J,乎:F)->G{let 丫=l.丈(乎)+l.且(乎);修(l,乎,乙(l,乎),丫)}fn
仉(l:&J,z:F)->G{丯(l,1-z)&l.七(z)}fn 倈(l:&J,z:F)->C{C::from(仉(l,z).count())}const 侣:C=4;fn 信(丌:C)->C{丌/侣}fn 丟(l:&J,z:F,仉:G)->C{let 七=l.七(z);let 丟=(l.丟()&七).丞(仉);信(C::from(七.count()))*C::from(丟.count())}const
侰:C=2;fn 侇(l:&J,z:F)->C{let 七=l.七(z);let[a,b,c]=七.仸();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(佀-1);侼.with(|伌|{let mut 伌=伌.borrow_mut();let 件=&mut 伌[与];if 件.0==七&&件.1==z as I+1{return 件.2;}let 丛=倕(七,Z[z]);*件=(七,z as I+1,丛);丛})}const
佀:F=1<<12;thread_local!{static 侼:std::cell::RefCell<L<(G,I,C)>> =std::cell::RefCell::new(vec![(G::T,0,0);佀]);}fn 倕(七:G,乭:F)->C{struct 什{七:G,久:[I;M],丵:[I;M],size:[I;M],互:I,乸:C,仰:C,}fn
亝(d:&mut 什,v:F)->I{d.互+=1;let 久=d.互;d.久[v]=久;let mut 丵=久;let mut 伫=1u8;let mut 东=0u8;for w in(G::q(v).丒()&d.七).iter(){if d.久[w]==0{let 使=亝(d,w);伫+=使;丵=丵.min(d.丵[w]);if
d.丵[w]>=久{东+=使;}}else{丵=丵.min(d.久[w]);}}d.丵[v]=丵;d.size[v]=伫;if 东>0&&d.久[v]>1{d.乸+=1;d.仰=d.仰.max(C::from(东));}伫}if!七.test(乭){return 0;}let mut d=什{七,久:[0;M],丵:[0;M],size:[0;M],互:0,乸:0,仰:0,};亝(&mut
d,乭);d.乸+d.仰}fn 俵(l:&J,z:F)->C{侰*(侇(l,1-z)-侇(l,z))}fn 何(l:&J,z:F)->C{if l.丈(z)+l.且(z)>=0{return 0;}(l.丅(z)&l.七(z)).iter().filter_map(|q|l.亜(q)).map(|一|10*C::from(一)).sum()}const
侘:C=15;const 侺:[C;2]=[3,1];const 侸:[C;2]=[4,20];fn 倄(l:&J,z:F)->[C;2]{let mut 丑=[0;2];for q in(l.丅(z)&l.七(z)).iter(){match l.亜(q){Some(2)=>丑[0]+=1,Some(3)=>丑[1]+=1,_=>{}}}丑}fn
佾(l:&J,z:F)->C{let[侄,倉]=倄(l,z);let 佇=|一:F|(3-侺[一])*侸[一];-侘*侄+佇(0)*侄+佇(1)*倉}const 侒:C=0;const 侕:C=2;const 侔:C=100;fn 佼(事:C)->C{if 事<0{return 0;}侒+侕*事.min(侔)/2}const
伷:I=2;const 侉:P<C> =Some(-5);const 侊:C=3;const 伸:C=200;fn 俜(l:&J,z:F)->C{C::from(l.七(z).count())-C::from(l.七(1-z).count())}fn 佈(l:&J,z:F,事:C)->C{match
侉{Some(丌)if 事!=C::MIN&&俜(l,z)<=丌=>{let 东=(4-侊)*l.丈(z)/4;(事-东).clamp(-伸,0)-事.clamp(-伸,0)}_=>0,}}fn 伖(l:&J,z:F)->C{let 乙=[伢(l,z,伷),伢(l,1-z,伷),];let(丘,二)=俑(l,z,乙);let
丘=丘+佼(二[0])-佼(二[1]);if l.丗().is_some(){return 丘;}let 仏=仉(l,z);let 乿=仉(l,1-z);let 他=C::from(乿.count())-C::from(仏.count());丘+佈(l,z,二[0])-佈(l,1-z,二[1])-2*何(l,z)+2*何(l,1-z)+俵(l,z)+佾(l,z)-佾(l,1-z)+侴*他/4+丟(l,z,仏)-丟(l,1-z,乿)}const
侢:F=8;const 侫:F=3;const 侚:F=32;const 侬:U=300;const 侎:亟=0.75;struct 亅{乾:P<乓>,亗:P<乓>,仌:U,}#[derive(Default)]struct 伾{丄:L<K>,伒:F,乶:U,世:F,丮:F,位:F,乷:F,}const
侻:Q=true;#[derive(Clone)]struct 乕{丄:L<K>,伕:C,}struct 亢{丄:L<K>,end:J,first:P<U>,专:P<L<乕>>,}type 仁=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
伃{丁:F,limit:亅,乶:U,介:仁,亚:P<L<丳>>,}const 伱:Q=true;const 例:Q=false;const 估:P<I> =None;fn 俄(l:&J,世:&[乕],丆:&mut Vec<丳>){if!伱&&估.is_none(){return;}let 仔=世.iter().map(|乚|乚.丄.as_slice());俅(l,仔,(伱,例,估),丆,);}fn
俅<'a>(l:&J,仔:impl Iterator<Item=&'a[K]>,(乒,丒,一):(Q,Q,P<I>),丆:&mut Vec<丳>,){let 丁=l.丝();let 丷=l.七(丁);let mut 仇=G::T;for 乚 in 仔{for 三 in 乚{if let K::乖{x,y,..}|K::乱{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丷.test(q){仇|=G::q(q);}}}}if 仇.is_empty(){return;}let 乫=l.产()&丷;let 丈=l.丈(丁);let 佳=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丄()==[三]){丆.push(丳::亡(三));}};if
乒&&丈>=亁&&l.两().代(){let mut 丌=仇;if 丒{丌|=仇.丒();}for q in(丌&乫).丞(l.map().丰).iter(){let(x,y)=五(q);佳(K::亃{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().也()){if
丈>=10*C::from(一){for q in(仇&乫).iter(){let(x,y)=五(q);佳(K::乖{一,x,y},丆);}}}}fn 伉(l:&mut J,丄:&[K])->Q{丄.iter().all(|&三|l.个(三))}fn 乌(丄:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丄{let(们,a,b,c)=match*三{K::乱{丐,x,y}=>(1,U::from(丐),x,y),K::乖{一,x,y}=>(2,U::from(一),x,y),K::亂{x,y}=>(3,0,x,y),K::亃{x,y}=>(4,0,x,y),};let 亿=们|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亿).wrapping_mul(0x0100000001b3);}hash}fn
佄(乗:&J,乚:&[K])->J{let mut 丂=乗.clone();丂.仆();for&三 in 乚{丂.个(三);}丂.仆();丂}fn 仮(l:&J)->P<L<K>>{let 丁=l.丝();let 丏=Z[1-丁];let 亾=|両:&J|両.丗()==Some(么::乩(丁));for
q in l.于().iter(){if l.map().乆(q,丏)!=1{continue;}let Some((丐,_))=l.付(q)else{continue;};let(x,y)=五(丏);let 三=K::乱{丐,x,y};let mut 両=l.clone();if 両.个(三)&&亾(&両){return
Some(vec![三]);}}let 丧=伎(l,丏,l.丈(丁))?;let mut 両=l.clone();(伉(&mut 両,&丧.丄)&&亾(&両)).then_some(丧.丄)}impl 伃{fn 乤(&self)->Q{self.乶>=self.limit.仌||self.limit.乾.is_some_and(|t|乓::now()>=t)}fn
丘(&mut self,乗:&J,世:&[乕],伋:C)->C{self.乶+=1;self.伦(乗,世,伋)}fn 伦(&self,乗:&J,世:&[乕],伋:C)->C{if 乗.丗()==Some(么::乩(self.丁)){return 仡;}if 世.is_empty(){return 伖(乗,self.丁);}let
mut 仹=C::MAX;for 乚 in 世{仹=仹.min(伖(&佄(乗,&乚.丄),self.丁));if 仹<=伋{break;}}仹}#[allow(clippy::too_many_arguments)]fn 俨(&mut self,乪:&J,亦:C,乏:&[丳],亶:&[(C,F)],丼:&mut[Q],世:&[乕],亨:&mut
仁,伜:Q,)->(P<(F,J,C)>,Q){let mut 上=None;let mut 令=亦;for&(仱,与)in 亶{if 丼[与]{continue;}if 伜&&仱!=C::MAX&&仱<=令-亦{return(上,true);}if self.乤(){return(上,false);}丼[与]=true;let
丄=乏[与].丄();let mut 丂=乪.clone();if!伉(&mut 丂,丄){亨.insert(乌(丄),C::MIN/4);continue;}let 丘=self.丘(&丂,世,令);亨.insert(乌(丄),丘-亦);if 丘>令{令=丘;上=Some((与,丂,丘));}}(上,false)}fn
休(&mut self,乭:&J,世:&[乕],仃:&[U],俇:Q)->亢{let j=self;let mut 乊=None;let mut 乪=乭.clone();let mut 丄=L::new();let mut 亦=j.丘(&乪,世,C::MIN);let mut 亨=仁::default();let
mut 乏=L::new();let mut 亶=L::new();while 乪.丗().is_none()&&!j.乤(){if 丄.is_empty()&&俇{match&j.亚{Some(亚)=>乏.clone_from(亚),None=>{作(&乪,&mut 乏);j.亚=Some(乏.clone());}}}else{作(&乪,&mut
乏);}俄(&乪,世,&mut 乏);亶.clear();for(与,乲)in 乏.iter().enumerate(){let k=乌(乲.丄());let 仱=亨.get(&k).or(j.介.get(&k)).copied();亶.push((仱.unwrap_or(C::MAX),与));}亶.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丼=vec![false;乏.len()];if 丄.is_empty(){for(与,乲)in 乏.iter().enumerate(){丼[与]=仃.contains(&乌(乲.丄()));}}let mut 上=None;for
伜 in[true,false]{let(丨,俽)=j.俨(&乪,亦,&乏,&亶,&mut 丼,世,&mut 亨,伜,);上=丨;if 上.is_some()||!俽{break;}}let Some((与,丂,丘))=上 else{break;};if 丄.is_empty(){乊=Some(乌(乏[与].丄()));}丄.extend_from_slice(乏[与].丄());乪=丂;亦=丘;}for(k,乬)in
亨{let 佞=j.介.entry(k).or_insert(乬);*佞=(*佞).max(乬);}亢{丄,end:乪,first:乊,专:None,}}fn 仔(&self,乗:&J)->L<L<K>>{let mut 丂=乗.clone();丂.仆();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 习=丂.丝();let 丈=丂.丈(习);if let Some(丧)=伎(&丂,Z[self.丁],丈){丆.push(丧.丄);}for 东 in 佔(&丂,伲,丈,侫){丆.push(东.丄);}let mut 佝=L::new();俗(&丂,&mut
佝);丆.push(佝);let mut 乿=伃{丁:习,limit:亅{仌:侬,..self.limit},乶:0,介:仁::default(),亚:None,};丆.push(乿.休(&丂,&[],&[],true).丄);丆.retain(|丄|!丄.is_empty());丆}fn 佅(&self,乗:&J)->L<乕>{self.仔(乗).into_iter().map(|丄|{let
伕=伖(&佄(乗,&丄),self.丁);乕{丄,伕}}).collect()}fn 乷(&mut self,乭:&J,乲:&丳,世:&[乕])->P<亢>{let mut 为=乭.clone();if!伉(&mut 为,乲.丄()){return None;}self.乶+=1;let 乛=self.休(&为,世,&[],false);let
mut 丄=乲.丄().to_vec();丄.extend_from_slice(&乛.丄);Some(亢{丄,end:乛.end,first:Some(乌(乲.丄())),专:None,})}fn 余(&self,丕:&亢,世:&[乕])->C{let 丘=self.伦(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>丘.min(self.伦(&丕.end,专,C::MIN)),_=>丘,}}}fn
俰(丨:&[乕],世:&mut Vec<乕>)->Q{let 並:L<_> =世.iter().map(|r|r.丄.clone()).collect();for 乚 in 丨{if 世.iter().any(|r|r.丄==乚.丄){continue;}世.push(乚.clone());}世.sort_by_key(|乚|乚.伕);世.truncate(侢);世.iter().map(|r|&r.丄).ne(&並)}fn
倇(l:&J,乁:亅,亵:&mut 了)->伾{let 丁=l.丝();let mut 中=伾::default();if l.丗().is_some(){return 中;}if let Some(丄)=仮(l){中.丄=丄;return 中;}let 主=|俷:亟|match(乁.乾,乁.亗){(Some(乾),Some(亗))=>Some(乾+(亗-乾).mul_f64(俷)),_=>None,};if
亵.俫(l){中.丄=亵.下(l,主(0.4));return 中;}let mut 乍=伃{丁,limit:亅{乾:主(侎),..乁},乶:0,介:仁::default(),亚:None,};let mut 世=L::new();let mut 丮:L<亢> =L::new();let mut 仃=L::new();let
mut 仧=false;let mut 乷:L<丳> =L::new();while 中.伒<侚&&(丮.is_empty()||!乍.乤()){let 丕=if 仧{let Some(乲)=乷.pop()else{break;};match 乍.乷(l,&乲,&世){Some(丕)=>丕,None=>continue,}}else{乍.休(l,&世,&仃,true)};中.伒+=1;let
佸=丮.iter().any(|p|p.丄==丕.丄);let 乊=丕.first;let 佽=乍.乤();let mut 丕=丕;let mut 佩=false;if!佽&&!佸{let 丨=乍.佅(&丕.end);佩=俰(&丨,&mut 世);丕.专=Some(丨);}if!佸{丮.push(丕);}if
佽{break;}if!佩&&!仧{match 乊{Some(乊)=>仃.push(乊),None=>{仧=true;乷=乍.亚.clone().unwrap_or_default();乷.retain(|a|!仃.contains(&乌(a.丄())));let 乬=|a:&丳|{乍.介.get(&乌(a.丄())).copied().unwrap_or(C::MIN)};乷.sort_by_key(乬);}}}}中.乷=F::from(仧);中.乶=乍.乶;中.世=世.len();中.丮=丮.len();let
佤=||乁.亗.is_some_and(|t|乓::now()>=t);let mut 亹:L<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佤(){break;}亹[与]=Some(乍.余(&丮[与],&世));}let
亷=|亹:&[P<C>]|{let mut 上:P<(C,F)> =None;for(与,丘)in 亹.iter().enumerate().rev(){if let Some(丘)=*丘{if 上.is_none_or(|(亀,_)|丘>亀){上=Some((丘,与));}}}上};let mut
上=亷(&亹);乍.limit.乾=乁.亗;while let Some((_,与))=上{if!侻||丮[与].专.is_some()||佤(){break;}let 丨=乍.佅(&丮[与].end);丮[与].专=Some(丨);中.位+=1;亹[与]=Some(乍.余(&丮[与],&世));上=亷(&亹);}if
let Some((_,与))=上{let mut 丬=l.clone();for&三 in&丮[与].丄{if 丬.个(三){中.丄.push(三);}}}中}fn 俪(乁:&dyn Fn(乓,串)->亅,mut 亵:了,伛:impl BufRead,仲:impl Write,)->Result<(),丱>{丿(伛,仲,|京,乜,下,些|match
J::read(京,乜,乧::份,下){Ok(l)=>{let 中=倇(&l,乁(些,下),&mut 亵);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",下,中.伒,中.乶,中.世,中.位,中.乷,些.elapsed());俁(&中.丄)}Err(亖)=>{eprintln!("plan: cannot read the input: {}",亖);"WAIT".to_string()}},)}const
侌:亟=40.0;const 侏:亟=800.0;const 侱:亟=0.6;fn 乁(些:乓,下:串)->亅{if let Some(仌)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亅{乾:None,亗:None,仌,};}let 仵=倊().min(100.0);let 俣=if 下==1{侏}else{侌};let 丫=Duration::from_secs_f64(俣/1000.0*仵);亅{乾:Some(些+丫.mul_f64(侱)),亗:Some(些+丫),仌:U::MAX,}}const
侩:F=7;const 侧:亠=亠{丌:3,且:1,丈:1,东:2,乸:2,仗:2,丯:6,仙:1,};const 侨:I=I::MAX;fn 亵()->了{俦(64)}fn 俦(书:F)->了{let 乽=亣::仠{丿:侩,书,乑:侧,};let 业=侯.iter().map(|&乢|Y{伩:侨,..乢}).collect();了::new(乽,业,Y::伅)}fn
main(){let 倁=io::stdin();let 倂=io::stdout();if let Err(亖)=俪(&乁,亵(),倁.lock(),倂.lock()){eprintln!("plan: {}",亖);std::process::exit(1);}}const 侵:&str="CG_TIME_SCALE";fn
倊()->亟{俧(std::env::var(侵).ok().as_deref())}fn 俧(丛:P<&str>)->亟{丛.and_then(|乜|乜.trim().parse::<亟>().ok()).filter(|仵|仵.is_finite()&&*仵>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乔:F=4*O;const 乨:U=(1<<乔)-1;const 仜:U=1|1<<12|1<<24|1<<36;const 仾:U=仜<<11;const 似:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侈:G=G([乨;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乔]>>(q%乔))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乔]|=1<<(q%乔);}#[inline]fn clear(&mut self,q:F){self.0[q/乔]&=!(1<<(q%乔));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仸(self)->[U;3]{self.0}#[inline]fn count(self)->串{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丞(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仼{仼{伟:self.0,主:0,}}#[inline]fn 俼(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&似)<<(乔-O)),(b>>O)|((c&似)<<(乔-O)),c>>O,])}#[inline]fn 俹(self)->G{let[a,b,c]=self.0;G([(a<<O)&乨,((b<<O)&乨)|(a>>(乔-O)),((c<<O)&乨)|(b>>(乔-O)),])}#[inline]fn
俺(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仾&乨,(b>>1)&!仾&乨,(c>>1)&!仾&乨,])}#[inline]fn 俻(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仜&乨,(b<<1)&!仜&乨,(c<<1)&!仜&乨,])}#[inline]fn
丒(self)->G{let j=self;j.俼()|j.俹()|j.俺()|j.俻()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侈.丞(self)}}struct 仼{伟:[U;3],主:F,}impl Iterator for 仼{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.主<3{let 仯=j.伟[j.主];if 仯!=0{j.伟[j.主]=仯&(仯-1);return Some(j.主*乔+仯.trailing_zeros()as F);}j.主+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乱{丐:串,x:I,y:I},乖{一:I,x:I,y:I},亂{x:I,y:I},亃{x:I,y:I},}impl K{fn 倌(&self)->丱{match*self{K::乱{丐,x,y}=>format!("MOVE {} {x} {y}",丐),K::乖{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::亂{x,y}=>format!("BUILD MINE {x} {y}"),K::亃{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
俁(丄:&[K])->丱{if 丄.is_empty(){return "WAIT".to_string();}let mut line=丱::with_capacity(16*丄.len());for(与,三)in 丄.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&三.倌());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乧{佁,倛,侽,#[default]份,}impl 乧{fn 也(self)->I{match self{乧::佁=>1,_=>3,}}fn 仢(self)->Q{matches!(self,乧::侽|乧::份)}fn 代(self)->Q{self==乧::份}}const 乞:I=I::MAX;struct
买{丹:G,valid:G,丰:G,乆:Box<[I]>,丒:[[[I;4];M];2],}thread_local!{static 侗:RefCell<P<Arc<买>>> =const{RefCell::new(None)};}impl 买{fn 俸(丹:G,丰:G)->Arc<买>{侗.with(|乹|{let
mut 乹=乹.borrow_mut();if let Some(亮)=乹.as_ref(){if 亮.丹==丹&&亮.丰==丰{return Arc::clone(亮);}}let 亮=Arc::new(买::build(丹,丰));*乹=Some(Arc::clone(&亮));亮})}fn build(丹:G,丰:G)->买{let
乯=!丹;const 侲:[[(仛,仛);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丒=[[[乞;4];M];2];for(z,乥)in 侲.iter().enumerate(){for q in
乯.iter(){let(x,y)=((q%O)as 仛,(q/O)as 仛);let mut 丑=0;for&(俎,俏)in 乥{let(佫,佬)=(x+俎,y+俏);if(0..O as 仛).contains(&佫)&&(0..O as 仛).contains(&佬){let 丂=佬 as F*O+佫
as F;if 乯.test(丂){丒[z][q][丑]=丂 as I;丑+=1;}}}}}let mut 乆=vec![乞;M*M].into_boxed_slice();let mut 伥=[0u8;M];for 不 in 乯.iter(){let 亐=&mut 乆[不*M..(不+1)*M];亐[不]=0;伥[0]=不
as I;let(mut 优,mut 伭)=(0,1);while 优<伭{let q=伥[优]as F;优+=1;for&丂 in&丒[0][q]{if 丂!=乞&&亐[丂 as F]==乞{亐[丂 as F]=亐[q]+1;伥[伭]=丂;伭+=1;}}}}买{丹,valid:乯,丰,乆,丒,}}#[inline]fn
乆(&self,不:F,乮:F)->I{self.乆[不*M+乮]}}const 価:串=100;const 亁:C=15;const 侹:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乩(F),伴,}enum
九{伂,伹,伺,}#[derive(Clone)]struct J{map:Arc<买>,两:乧,专:[G;2],七:[G;2],丷:G,乇:G,丅:[G;2],一:[G;3],亍:G,亽:[串;M],丈:[C;2],且:[C;2],丝:F,亊:串,亲:串,丗:P<么>,}fn 今(专:G,享:F)->G{if!专.test(享){return
G::T;}let mut 仓=G::q(享);loop{let 丂=(仓|仓.丒())&专;if 丂==仓{return 仓;}仓=丂;}}impl J{fn 丝(&self)->F{self.丝}fn 亊(&self)->串{self.亊}fn 丗(&self)->P<么>{self.丗}fn 两(&self)->乧{self.两}fn
亰(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伮(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&买{&self.map}fn 丠(&self,z:F)->G{self.专[z]}fn 七(&self,z:F)->G{self.七[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 付(&self,q:F)->P<(串,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亽[q],j.伞(q)))}fn 于(&self)->G{self.丅[self.丝]&self.亍}fn
丟(&self)->G{self.丷}fn 乒(&self)->G{self.乇}fn 亜(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伞(q))}fn 倀(&mut self,z:F){let j=self;j.七=[今(j.专[0],Z[0]),今(j.专[1],Z[1])];let
俊=j.丅[z].丞(j.七[z]);j.伧(俊);j.低();j.丈[z]+=j.且[z];if j.丈[z]<0{j.丈[z]=0;j.伧(j.丅[z]);}j.亍|=j.丅[z];}fn 低(&mut self){let j=self;for z in 0..2{let 七=j.七[z];let
丅=七&j.丅[z];let mut 且=C::from(七.count())+4*C::from((七&j.丷).count());for(一,倓)in 侹.iter().enumerate(){且-=倓*C::from((丅&j.一[一]).count());}j.且[z]=且;}}fn 仆(&mut
self){let j=self;if j.丗.is_some(){return;}if j.丝==1{if j.亊>=価{j.丗=Some(match j.亰(0).cmp(&j.亰(1)){std::cmp::Ordering::Greater=>么::乩(0),std::cmp::Ordering::Less=>么::乩(1),std::cmp::Ordering::Equal=>么::伴,});return;}j.亊+=1;}j.丝=1-j.丝;j.倀(j.丝);}fn
佱(&mut self,丄:&[K]){let j=self;if j.丗.is_some(){return;}let 众=j.亲;for&三 in 丄{if j.俍(三,众){continue;}j.个(三);if j.丗.is_some(){return;}}j.仆();}fn 俍(&self,三:K,众:串)->Q{let
j=self;match 三{K::乱{丐,x,y}=>{j.亥(x,y).is_none()||丐>=众||j.佚(j.丝,丐).is_none()}K::乖{一,x,y}=>{j.亥(x,y).is_none()||一==0||一>j.两.也()}K::亂{x,y}|K::亃{x,y}=>{j.亥(x,y).is_none()}}}#[inline]fn
亥(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丹.test(y*O+x)).then_some(y*O+x)}fn 佚(&self,z:F,丐:串)->P<F>{self.丅[z].iter().find(|&q|self.亽[q]==丐)}#[inline]fn
伞(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 促(&self,习:F)->G{let 乒=self.乇&self.专[习];let 七=self.七[习];乒|(七&(七&乒).丒())}fn
丽(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 习=1-z;let 丅=j.丅[0]|j.丅[1];let 俐=j.产();let 們=match
一{1 if j.两==乧::佁=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::T,};let 丽=俐|j.专[习].丞(們);if 一<3{丽.丞(j.促(习))}else{丽}}fn 亙(&self,z:F)->G{self.七[z]|self.七[z].丒()}fn
产(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 俚=G::q(Z[0])|G::q(Z[1]);j.map.valid.丞(丅|j.丷|j.乇|俚)}fn 个(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.丝;let 井=match 三{K::乖{一,x,y}=>j.倎(z,一,x,y),K::乱{丐,x,y}=>j.俢(z,丐,x,y),K::亂{x,y}=>j.build(z,true,x,y),K::亃{x,y}=>j.build(z,false,x,y),};let
Some(井)=井 else{return false;};match 井{九::伂=>{}九::伹=>j.七[z]=今(j.专[z],Z[z]),九::伺=>{j.七=[今(j.专[0],Z[0]),今(j.专[1],Z[1])];}}j.低();if!j.专[0].test(Z[0]){j.丗=Some(么::乩(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(么::乩(0));}true}fn 伧(&mut self,丌:G){let j=self;j.丅[0]=j.丅[0].丞(丌);j.丅[1]=j.丅[1].丞(丌);for 一 in&mut j.一{*一=一.丞(丌);}j.亍=j.亍.丞(丌);}fn
侀(&mut self,z:F,q:F)->九{let j=self;j.伧(G::q(q));j.丷.clear(q);j.乇.clear(q);let 习=1-z;if j.专[习].test(q){j.专[习].clear(q);j.专[z].set(q);九::伺}else if j.专[z].test(q){九::伂}else{j.专[z].set(q);九::伹}}fn
倎(&mut self,z:F,一:I,x:I,y:I)->P<九>{let j=self;if 一==0||一>j.两.也(){return None;}let q=j.亥(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亙(z).test(q)||!j.丽(z,一).test(q){return
None;}let 井=j.侀(z,q);j.佯(z,一,q,j.亲);j.亲=j.亲.saturating_add(1);j.丈[z]-=万;Some(井)}fn 佯(&mut self,z:F,一:I,q:F,丐:串){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亽[q]=丐;}fn
俢(&mut self,z:F,丐:串,x:I,y:I)->P<九>{let j=self;let 丏=j.亥(x,y)?;let 不=j.佚(z,丐)?;if!j.亍.test(不){return None;}let 一=j.伞(不);let 丣=j.丽(z,一);let 乀=j.map.丒[z][不];if
乀.contains(&(丏 as I))&&!丣.test(丏){return None;}let mut 上=j.map.乆(不,丏);let mut 仄=None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let 丂=F::from(丂);let 乆=j.map.乆(丂,丏);if
乆<上&&丣.test(丂){上=乆;仄=Some(丂);}}let 仄=仄?;let 井=j.侀(z,仄);j.丅[z].clear(不);j.一[F::from(一)-1].clear(不);j.亍.clear(不);j.佯(z,一,仄,丐);Some(井)}fn build(&mut self,z:F,丷:Q,x:I,y:I)->P<九>{let
j=self;let 俀=if 丷{j.两.仢()}else{j.两.代()};let q=j.亥(x,y)?;let 俤=j.map.丰.test(q);if!俀||j.丈[z]<亁||俤!=丷||!j.专[z].test(q)||!j.亙(z).test(q)||!j.产().test(q){return
None;}if 丷{j.丷.set(q);}else{j.乇.set(q);}j.丈[z]-=亁;Some(九::伂)}}const 侜:C=1000000000;const 仿:C=0;const 侤:C=1;const 侶:C=2;impl J{fn read(京:&str,下:&str,两:乧,亊:串,)->Result<J,丱>{let
mut 丰=L::new();let mut 严=京.lines();let 丑=伔(&mut 严,"mine spot count")?;for _ in 0..丑{let[x,y]=乻(亳(&mut 严,"a mine spot")?,"a mine spot")?;丰.push((仦(x)?,仦(y)?));}佗(严,"the initial input")?;let
mut 严=下.lines();let mut 伙=[0i64;4];for(丛,丶)in 伙.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伤]=乻(亳(&mut 严,丶)?,丶)?;if 伤.abs()>侜{return
Err(format!("{}: {} is out of range",丶,伤));}*丛=伤;}let mut 丹=G::T;let mut 丌=[G::T;2];let mut 七=[G::T;2];for y in 0..O{let 亐=亳(&mut 严,"a map row")?.trim();if
亐.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,亐));}for(x,仕)in 亐.bytes().enumerate(){let q=y*O+x;match 仕{b'#'=>丹.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乂=F::from(仕==b'X'||仕==b'x');丌[乂].set(q);if 仕.is_ascii_uppercase(){七[乂].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仕 as char))}}}}let
俆=伔(&mut 严,"building count")?;let mut 乐=L::new();for _ in 0..俆{let 乻=乻(亳(&mut 严,"a building")?,"a building")?;乐.push(乻);}let 伮=伔(&mut 严,"unit count")?;let
mut 丅=L::new();for _ in 0..伮{let 乻=乻(亳(&mut 严,"a unit")?,"a unit")?;丅.push(乻);}佗(严,"the turn's input")?;let 佮=乐.iter().find(|佊|佊[0]==0&&佊[1]==仿).ok_or("the bot's own HQ is not among the buildings")?;let
佡=伍(佮[2],佮[3],丹)?;let z=Z.iter().position(|&享|享==佡).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佡))?;let mut 佺=G::T;for(x,y)in 丰{if
丹.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佺.set(y*O+x);}let 伆=|乂:F|if 乂==0{z}else{1-z};let mut l=J{map:买::俸(丹,佺),两,专:[G::T;2],七:[G::T;2],丷:G::T,乇:G::T,丅:[G::T;2],一:[G::T;3],亍:G::T,亽:[0;M],丈:[0;2],且:[0;2],丝:z,亊,亲:1,丗:None,};for
乂 in 0..2{let 乡=伆(乂);l.专[乡]=丌[乂];l.七[乡]=七[乂];l.丈[乡]=伙[2*乂];l.且[乡]=伙[2*乂+1];}let mut 佭=G::T;for[乡,们,x,y]in 乐{let 乂=併(乡)?;let q=伍(x,y,丹)?;if!丌[乂].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佭.test(q){return Err(format!("two buildings at ({x}, {y})"));}佭.set(q);match 们{仿
if q==Z[伆(乂)]=>{}仿=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侤=>l.丷.set(q),侶=>l.乇.set(q),_=>return Err(format!("unknown building type {}",们)),}}let
mut 乃=G::T;let mut 仭=0u32;for[乡,丐,一,x,y]in 丅{let 乂=併(乡)?;let q=伍(x,y,丹)?;if!丌[乂].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乃.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丐=串::try_from(丐).map_err(|_|format!("unit id {} is out of range",丐))?;if
乃.iter().any(|丩|l.亽[丩]==丐){return Err(format!("two units have id {}",丐));}乃.set(q);l.丅[伆(乂)].set(q);l.一[一 as F-1].set(q);l.亽[q]=丐;仭=仭.max(丐);}if!乃.is_empty(){l.亲=仭.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仭))?;}l.亍=l.丅[z];Ok(l)}}fn
亳<'a>(严:&mut Lines<'a>,丶:&str)->Result<&'a str,丱>{严.next().ok_or_else(||format!("the text ends before {}",丶))}fn 佗(mut 严:Lines<'_>,丶:&str)->Result<(),丱>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丶)),None=>Ok(()),}}fn 乻<const N:F>(line:&str,丶:&str)->Result<[C;N],丱>{let
mut 仸=line.split_whitespace();let mut 來=[0i64;N];for 丛 in&mut 來{let 亿=仸.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丶))?;*丛=亿.parse().map_err(|_|format!("{}: {:?} is not an integer",丶,亿))?;}if
仸.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丶));}Ok(來)}fn 伔(严:&mut Lines<'_>,丶:&str)->Result<F,丱>{let[丑]=乻(亳(严,丶)?,丶)?;F::try_from(丑).ok().filter(|&丑|丑<=M).ok_or_else(||format!("{}: {} is out of range",丶,丑))}fn
仦(丛:C)->Result<F,丱>{F::try_from(丛).ok().filter(|&丛|丛<O).ok_or_else(||format!("coordinate {} is off the map",丛))}fn 伍(x:C,y:C,丹:G)->Result<F,丱>{let q=仦(y)?*O+仦(x)?;if
丹.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 併(乡:C)->Result<F,丱>{match 乡{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乡)),}}const
侑:C=15;const 侥:C=6;const 伿:C=10;const 侮:I=3;#[derive(Clone,Copy)]struct Y{open:C,以:C,佑:I,会:C,你:C,佦:C,伩:I,佐:C,住:C,佃:C,亪:C,亻:C,丸:C,亭:C,亓:C,}impl Y{const
伅:Y=Y{open:3,以:2,佑:12,会:4,你:侑,佦:侥,伩:侮,佐:2,住:2,佃:0,亪:0,亻:0,丸:0,亭:0,亓:0,};const fn 候(self,亪:C,亻:C,丸:C)->Y{Y{亪,亻,丸,..self}}const fn 倘(self,亭:C,亓:C)->Y{Y{亭,亓,..self}}}fn
仚(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仅(l:&J,不:G)->[I;M]{let mut 义=[I::MAX;M];let 乯=l.map().valid;let mut 之=不&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.丒()&乯).丞(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn 俔(l:&J)->I{let 义=仅(l,l.丠(0));l.丠(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亇<'a>{乢:&'a Y,丁:F,theirs:[I;M],亼:[I;M],free:G,乎:G,ours:G,丰:G,}impl 亇<'_>{fn new<'a>(l:&J,乢:&'a Y)->亇<'a>{let 丁=l.丝();let 丷=仅(l,l.丠(丁));let 乿=仅(l,l.丠(1-丁));let
亼=仅(l,G::q(Z[1-丁]));let 乫=l.map().valid.丞(l.丠(0)|l.丠(1));let mut 乎=G::T;for q in 乫.iter(){if 丷[q]<=乿[q]{乎.set(q);}}亇{乢,丁,theirs:乿,亼,free:乫,乎,ours:l.丠(丁),丰:l.map().丰,}}fn
value(&self,q:F)->C{let j=self;let p=j.乢;let 举=C::from((G::q(q).丒()&j.free).count());let 佌=p.佑;let 以=C::from(佌)-C::from(j.theirs[q].min(佌));let 享=if p.会>0{C::from(j.亼[q].min(30))/p.会}else{0};let
专=C::from((G::q(q).丒()&j.ours).count());let line=if 专<=1{p.亻}else{0};let 丸=if j.丰.test(q){p.丸}else{0};let 倅=if 举==0{p.亭}else{0};let 亓=p.亓*(22-C::from(j.亼[q].min(22)));p.open*举+p.以*以-享+p.亪*专-line+丸-倅+亓}fn
上(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伐(&mut self,q:F){self.free.clear(q);self.乎.clear(q);self.ours.set(q);}}fn
仑(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 佂=l.个(三);if 佂{丆.push(三);}佂}fn 丙(l:&mut J,不:F,乮:F,丆:&mut Vec<K>)->Q{let Some((丐,_))=l.付(不)else{return false;};let(x,y)=仚(乮);仑(l,K::乱{丐,x,y},丆)}fn
倒(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伿}#[derive(Clone,Copy)]enum 乄{仟,亄{丟:I,丅:I},}fn 侃(l:&J,乢:&Y,争:乄)->L<K>{let mut 丬=l.clone();let
mut 下=亇::new(l,乢);let mut 丆=L::new();乺(&mut 丬,&mut 下,&mut 丆);match 争{乄::仟=>俭(&mut 丬,&mut 下,&mut 丆),乄::亄{丟,丅}=>{俒(&mut 丬,&mut 下,丟,丅,&mut 丆)}}丆}fn 乺(l:&mut
J,下:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=下.丁;let mut 仪:L<F> =L::new();loop{let 丣=l.丽(丁,1);let mut 亷:P<(串,F)> =None;for 不 in l.于().iter(){if 仪.contains(&不){continue;}let
乼=(G::q(不).丒()&下.free&丣).count();if 乼==0{仪.push(不);}else if 亷.is_none_or(|(n,_)|乼<n){亷=Some((乼,不));}}let Some((_,不))=亷 else{break;};match 下.上(G::q(不).丒()&下.free&丣){Some(乮)if
丙(l,不,乮,丆)=>下.伐(乮),_=>仪.push(不),}}for 不 in 仪{if!l.于().test(不){continue;}let 丏=[下.乎,下.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().乆(不,q),q))});if
let Some(丏)=丏{丙(l,不,丏,丆);}}}fn 俭(l:&mut J,下:&mut 亇<'_>,丆:&mut Vec<K>){let 丁=下.丁;let p=下.乢;let h=(p.你-C::from(l.亊())).max(p.佦);let 個=(C::from(俔(l))-p.佐).max(0)/p.住.max(1)+p.佃;loop{let
丈=l.丈(丁);let 丅=l.伮(丁)as C;let 俲=C::from(下.乎.count());let q=下.上(l.亙(丁)&下.free&l.丽(丁,1));let 侅=倒(h,(俲/(丅+1)).min(個));let 佋=q.is_some()&&丈>=伿&&侅>0;let 佧=4*h-亁;let
丸=佨(l,下);let 俈=丸.is_some()&&丈>=亁&&佧>0;if 俈&&(!佋||佧>=侅){let(x,y)=仚(丸.unwrap_or_default());if 仑(l,K::亂{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佋)else{break;};let(x,y)=仚(q);if!仑(l,K::乖{一:1,x,y},丆){break;}下.伐(q);}}fn
佨(l:&J,下:&亇<'_>)->P<F>{let 丁=下.丁;(l.map().丰&l.七(丁)&l.产()).iter().filter(|&丸|下.theirs[丸]>下.乢.伩).min_by_key(|&丸|(l.map().乆(Z[丁],丸),丸))}fn 俒(l:&mut J,下:&mut
亇<'_>,丟:I,丅:I,丆:&mut Vec<K>,){let 丁=下.丁;for _ in 0..丟{let Some(丸)=佨(l,下)else{break;};let(x,y)=仚(丸);if!仑(l,K::亂{x,y},丆){break;}}for _ in 0..丅{let Some(q)=下.上(l.亙(丁)&下.free&l.丽(丁,1))else{break;};let(x,y)=仚(q);if!仑(l,K::乖{一:1,x,y},丆){break;}下.伐(q);}}fn
仴(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.丝();let 丈=l.丈(丁);let 丯=交::new(l,丁,true);l.丠(1-丁).iter().any(|q|丯.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亞:I,争:乄,}const 伯:I=I::MAX;const 侍:[乄;8]=[乄::仟,乄::亄{丟:0,丅:0},乄::亄{丟:0,丅:1},乄::亄{丟:0,丅:2},乄::亄{丟:0,丅:伯,},乄::亄{丟:1,丅:0},乄::亄{丟:1,丅:1},乄::亄{丟:1,丅:伯,},];struct
伳<'a>{业:&'a[Y],仍:Y,rest:丢,乴:P<乓>,俳:Settings,}impl 伳<'_>{fn 乢(&self,丙:丢)->&Y{&self.业[F::from(丙.亞)%self.业.len()]}fn 丄(&self,l:&J,丙:丢)->L<K>{侃(l,self.乢(丙),丙.争)}}fn
伪(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 传=|亿:U|hash=(hash^亿).wrapping_mul(0x0100000001b3);for(倆,临)in[l.丠(丁),l.丅(丁),l.丟()].into_iter().enumerate(){for
q in 临.iter(){传(((倆 as U)<<8)|q as U);}传(0xff);}传(l.丈(丁)as U);hash}enum 亣{仠{丿:F,书:F,乑:亠,},}struct 了{乽:亣,业:L<Y>,仍:Y,丕:L<丢>,仨:L<U>,亸:F,丮:串,仳:Q,}impl 了{fn
new(乽:亣,业:L<Y>,仍:Y)->了{了{乽,业,仍,丕:L::new(),仨:L::new(),亸:0,丮:0,仳:false,}}fn 俫(&mut self,l:&J)->Q{let j=self;let 俾=matches!(j.乽,亣::仠{..});let 俬=!俾&&j.丮>0&&j.亸>=j.仨.len();if!j.仳&&(俬||仴(l)){j.仳=true;}!j.仳}fn
下(&mut self,l:&J,乴:P<乓>)->L<K>{let j=self;let 体=伳{业:&j.业,仍:j.仍,rest:丢{亞:0,争:乄::仟,},乴,俳:Settings::DEFAULT,};let 丁=l.丝();let 俥=j.仨.get(j.亸)==Some(&伪(l,丁));if!俥{let
乊=j.丮==0;j.丕=match j.乽{亣::仠{丿,书,乑,}=>{let 乑=&乑;let 乛=侭;let mut 乥=丕(l,&j.业,乛,丿,书,乴,乑,);乥.resize(丿,乛);乥}};j.仨=match j.乽{亣::仠{..}=>俿(l,&j.业,&j.丕),};j.亸=0;j.丮+=1;}let
丙=j.丕.get(j.亸).copied().unwrap_or(体.rest);j.亸+=1;体.丄(l,丙)}}fn 俿(l:&J,业:&[Y],丕:&[丢])->L<U>{let 丁=l.丝();let mut 佣=L::with_capacity(丕.len());let mut 乣=l.clone();for&丙
in 丕{if 乣.丗().is_some(){break;}佣.push(伪(&乣,丁));乣=伇(&乣,&丄(&乣,业,丙));}佣}const 侷:C=3;struct Settings{丿:C,}impl Settings{const DEFAULT:Settings=Settings{丿:侷};}fn
乥(l:&J,不:G)->[I;M]{let 乯=l.map().valid;let mut 义=[I::MAX;M];let mut 之=不&乯;let mut 乃=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.丒()&乯).丞(乃);乃|=丂;之=丂;d=d.saturating_add(1);}义}fn
倔(l:&J,z:F)->C{let 仏=乥(l,l.七(z));let 乿=乥(l,l.七(1-z));l.map().valid.iter().map(|q|match 仏[q].cmp(&乿[q]){_ if 仏[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侯:[Y;27]=业();const 侭:丢=丢{亞:16,争:乄::仟,};const fn 业()->[Y;27]{let 亪=[0,2,4];let 亻=[0,3,6];let 亓=[0,1,2];let mut 丆=[Y::伅;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亭=if g>0||t>0{4}else{0};丆[i]=Y::伅.候(亪[g],亻[t],0).倘(亭,亓[w]);i+=1;}丆}#[derive(Default)]struct 伄{丌:C,且:C,丈:C,乸:C,东:C,仗:C,丯:C,仙:C,}#[derive(Clone,Copy)]struct
亠{丌:C,且:C,丈:C,东:C,乸:C,仗:C,丯:C,仙:C,}fn 俴(l:&J,z:F)->[C;M]{struct 什{七:G,久:[u16;M],丵:[u16;M],size:[C;M],东:[C;M],互:u16,}fn 亝(d:&mut 什,v:F){d.互+=1;d.久[v]=d.互;d.丵[v]=d.互;d.size[v]=1;for
w in(G::q(v).丒()&d.七).iter(){if d.久[w]==0{亝(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.久[v]{d.东[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.久[w]);}}}let
mut d=什{七:l.七(z),久:[0;M],丵:[0;M],size:[0;M],东:[0;M],互:0,};let 乭=Z[z];if d.七.test(乭){亝(&mut d,乭);}d.东[乭]=0;d.东}fn 俶(l:&J,z:F)->伄{let 七=l.七(z);let 东=俴(l,z);let
亼=仅(l,G::q(Z[1-z]));let mut 丆=伄{丌:C::from(七.count()),且:l.且(z),丈:l.丈(z),丯:C::MAX,仙:倔(l,z)/2,..伄::default()};for q in 七.iter(){if 东[q]>0{丆.乸+=1;丆.东=丆.东.max(东[q]);}if
q!=Z[z]&&(G::q(q).丒()&七).count()<=1{丆.仗+=1;}丆.丯=丆.丯.min(C::from(亼[q]));}丆}fn 丛(l:&J,z:F,乑:&亠)->C{let s=俶(l,z);let o=乑;o.丌*s.丌+o.且*s.且+o.丈*s.丈-o.东*s.东-o.乸*s.乸-o.仗*s.仗-o.丯*s.丯.min(30)+o.仙*s.仙}fn
伇(l:&J,丄:&[K])->J{let mut 丂=l.clone();丂.佱(丄);if 丂.丗().is_none(){丂.佱(&[]);}丂}fn 丄(l:&J,业:&[Y],丙:丢)->L<K>{侃(l,&业[F::from(丙.亞)%业.len()],丙.争,)}fn 佰(l:&J,业:&[Y],丕:&[丢],乛:丢,丿:F,)->J{let
mut 乣=l.clone();for 下 in 0..丿{if 乣.丗().is_some(){break;}let 丙=丕.get(下).copied().unwrap_or(乛);乣=伇(&乣,&丄(&乣,业,丙));}乣}fn 倃(l:&J,业:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亞 in 0..业.len().min(F::from(I::MAX)){for&争 in&侍{let 丙=丢{亞:亞 as I,争,};let c=丄(l,业,丙);if!丆.iter().any(|(_,丩)|*丩==c){丆.push((丙,c));}}}丆}fn
係(l:&J,业:&[Y],乛:丢,丿:F,书:F,乴:P<乓>,乑:&亠,)->L<丢>{let 丁=l.丝();let 仐=||乴.is_some_and(|t|乓::now()>=t);let 仒=|丕:&[丢]|丛(&佰(l,业,丕,乛,丿),丁,乑);let mut 上:(C,L<丢>)=(仒(&[]),L::new());let
mut 之:L<(J,L<丢>)> =vec![(l.clone(),L::new())];for 俌 in 0..丿{let mut 乳:L<(C,J,L<丢>)> =L::new();let mut 乃:HashMap<U,F> =HashMap::new();for(佪,丕)in&之{if 仐(){break;}for(丙,c)in
倃(佪,业){let 丂=伇(佪,&c);let mut 伏=丕.clone();伏.push(丙);let v=丛(&丂,丁,乑);let 乌=伪(&丂,丁);match 乃.get(&乌){Some(&主)if 乳[主].0>=v=>{}Some(&主)=>乳[主]=(v,丂,伏),None=>{乃.insert(乌,乳.len());乳.push((v,丂,伏));}}}}if
乳.is_empty(){break;}乳.sort_by(|a,b|b.0 .cmp(&a.0));乳.truncate(书);let 乹=俌+1==丿||仐();for(i,(_,_,丕))in 乳.iter().take(if 乹{书}else{1}).enumerate(){if i>0&&仐(){break;}let
v=仒(丕);if v>上.0{上=(v,丕.clone());}}if 乹{break;}之=乳.into_iter().map(|(_,s,p)|(s,p)).collect();}上.1}fn 丕(l:&J,业:&[Y],乛:丢,丿:F,俠:F,乴:P<乓>,乑:&亠,)->L<丢>{let 丁=l.丝();let
仒=|丕:&[丢]|丛(&佰(l,业,丕,乛,丿),丁,乑);let mut 上:(C,L<丢>)=(仒(&[]),L::new());let mut 书=2;loop{let 丨=係(l,业,乛,丿,书,乴,乑);let v=仒(&丨);if v>上.0{上=(v,丨);}let 仐=乴.is_some_and(|t|乓::now()>=t);if
仐||(乴.is_none()&&书>=俠){break;}书*=2;}上.1}const 侐:F=4+12;fn 丿(伛:impl BufRead,mut 仲:impl Write,mut 伈:impl FnMut(&str,&str,串,乓)->丱,)->Result<(),丱>{let mut
严=伶{佴:伛};let Some(乊)=严.line()?else{return Ok(());};let mut 京=format!("{}\n",乊);严.take(丑(&乊,"mine spot count")?,&mut 京)?;let mut 下=0;loop{let Some(乊)=严.line()?else{return
Ok(());};let 些=乓::now();下+=1;let mut 乜=format!("{}\n",乊);严.take(侐-1,&mut 乜)?;for 丶 in["building count","unit count"]{let line=严.佶()?;let n=丑(&line,丶)?;乜.push_str(&line);乜.push('\n');严.take(n,&mut
乜)?;}let 伈=伈(&京,&乜,下,些);writeln!(仲,"{}",伈).and_then(|()|仲.flush()).map_err(|亖|format!("turn {}: cannot write the answer: {}",下,亖))?;}}struct 伶<R>{佴:R,}impl<R:BufRead>伶<R>{fn
line(&mut self)->Result<P<丱>,丱>{let mut line=丱::new();match self.佴.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(亖)=>Err(format!("cannot read input: {}",亖)),}}fn
佶(&mut self)->Result<丱,丱>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乜:&mut String)->Result<(),丱>{for
_ in 0..n{乜.push_str(&self.佶()?);乜.push('\n');}Ok(())}}fn 丑(line:&str,丶:&str)->Result<F,丱>{match line.trim().parse::<F>(){Ok(n)if n<=供=>Ok(n),_=>Err(format!("bad {} {line:?}",丶)),}}const
供:F=144;const 侳:C=3;const 侖:C=2;const 侠:F=16;fn 俩(l:&J,丆:&mut Vec<K>){let z=l.丝();let mut 乽=伻{s:l.clone(),z,乵:1-z,丏:Z[1-z],仩:Z[z],下:l.亊(),丆,};乽.run();}struct
伻<'a>{s:J,z:F,乵:F,丏:F,仩:F,下:串,丆:&'a mut Vec<K>,}fn 仺(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伡(丐:串,q:F)->K{let(x,y)=仺(q);K::乱{丐,x,y}}fn 侂(一:I,q:F)->K{let(x,y)=仺(q);K::乖{一,x,y}}impl
伻<'_>{fn run(&mut self){let j=self;if j.仮(){return;}j.乺();if j.仮(){return;}j.乐();j.倐();}fn 义(&self,不:F,乮:F)->C{C::from(self.s.map().乆(不,乮))}fn 个(&mut self,三:K)->Q{let
丼=self.s.个(三);if 丼{self.丆.push(三);}丼}fn 亾(&self,両:&J)->Q{両.丗()==Some(么::乩(self.z))}fn 仮(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.于().iter(){if j.义(q,j.丏)!=1{continue;}let Some((丐,_))=j.s.付(q)else{continue;};let 三=伡(丐,j.丏);let mut 両=j.s.clone();if 両.个(三)&&j.亾(&両){return j.个(三);}}let
Some(丧)=j.俉()else{return false;};let mut 両=j.s.clone();if!丧.iter().all(|&三|両.个(三))||!j.亾(&両){return false;}for 三 in 丧{j.个(三);}true}fn 俉(&self)->P<L<K>>{let
j=self;const 伵:C=C::MAX;let 乘=j.s.两().也();let 丣:L<G> =(1..=乘).map(|一|j.s.丽(j.z,一)).collect();let 仈=|q:F|{丣.iter().position(|临|临.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 七=j.s.七(j.z);let mut 万=[伵;M];let mut 並=[F::MAX;M];let mut 丼=[false;M];for q in 七.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丼[q]&&万[q]<伵&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丼[q]=true;if q==j.丏{break;}for&乀 in&j.s.map().丒[0][q]{if 乀==乞{break;}let 乀=F::from(乀);if 丼[乀]||七.test(乀){continue;}let Some((_,亏))=仈(乀)else{continue;};if
万[q]+亏<万[乀]{万[乀]=万[q]+亏;並[乀]=q;}}}if 万[j.丏]>j.s.丈(j.z){return None;}let mut 丧=L::new();let mut q=j.丏;while!七.test(q){let(一,_)=仈(q)?;丧.push(侂(一,q));q=並[q];}丧.reverse();Some(丧)}fn
乬(&self,为:&J)->C{let(丁,云)=(self.z,self.乵);let 並=&self.s;let 丑=|临:G|C::from(临.count());let 专=丑(为.丠(丁))-丑(並.丠(丁));let 价=丑(並.丠(云))-丑(为.丠(云));let 保=丑(並.七(云))-丑(为.七(云));let
仫=並.亰(云)-为.亰(云);let 丟=丑(並.丟()&並.丠(云))-丑(为.丟()&为.丠(云));12*专+20*价+8*(保-价).max(0)+5*仫+25*丟}fn 佻(&self,q:F)->C{let j=self;let 丸=j.s.map().丰.test(q);if 丸&&j.s.两().仢()&&!j.s.丠(j.z).test(q){20}else{0}}fn
佘(&self,乁:C)->Q{let j=self;j.s.丅(j.乵).iter().any(|q|j.义(q,j.仩)<=乁)}fn 乺(&mut self){let j=self;let mut 丅:L<(C,F)> =j.s.于().iter().map(|q|(j.义(q,j.丏),q)).collect();丅.sort_unstable();for(俙,q)in
丅{if j.s.丗().is_some(){return;}let Some((丐,一))=j.s.付(q)else{continue;};let 丣=j.s.丽(j.z,一);let 乀=j.s.map().丒[j.z][q];let mut 上:P<(C,F)> =None;for&丂 in 乀.iter().take_while(|&&丂|丂!=乞){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.个(伡(丐,丂)){continue;}let 丛=if j.亾(&両){100000}else{j.乬(&両)+10*(俙-j.义(丂,j.丏))+j.佻(丂)};if 上.is_none_or(|(亀,_)|丛>亀){上=Some((丛,丂));}}if
let Some((丛,丂))=上{if 丛>0{j.个(伡(丐,丂));}}}}fn 乐(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.佘(侳){j.倍();}else{j.丟();}}fn 倍(&mut self){let j=self;if!j.s.两().代()||j.s.丈(j.z)<亁{return;}let
乀=j.s.map().丒[j.z][j.仩];let 丌:L<F> =乀.iter().take_while(|&&q|q!=乞).map(|&q|F::from(q)).collect();let 丷=j.s.丠(j.z);if 丌.iter().any(|&q|丷.test(q)&&j.s.乒().test(q)){return;}for
q in 丌{let(x,y)=仺(q);if j.个(K::亃{x,y}){return;}}}fn 丟(&mut self){let j=self;if!j.s.两().仢(){return;}while j.s.丈(j.z)>=亁{let 乫=j.s.map().丰&j.s.丠(j.z)&j.s.七(j.z)&j.s.产();let
Some(q)=乫.iter().min_by_key(|&q|(j.义(q,j.仩),q))else{return;};let(x,y)=仺(q);if!j.个(K::亂{x,y}){return;}}}fn 倐(&mut self){let j=self;let 佷=if j.下<=3{0}else{5};for
_ in 0..侠{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俛=j.佘(侖);let 亱=j.s.亙(j.z);let mut 上:P<(C,K)> =None;let mut 伬:P<(C,K)> =None;let
mut 佉=G::T;for 一 in 1..=j.s.两().也(){let 丣=j.s.丽(j.z,一);let 乼=(亱&丣).丞(佉);佉=丣;let 亏=10*C::from(一);if 亏>丈{continue;}for q in 乼.iter(){let 三=侂(一,q);let mut
両=j.s.clone();if!両.个(三){continue;}let 伝=両.丈(j.z);let 且=両.且(j.z);let 仫=j.s.亰(j.乵)-両.亰(j.乵);let 侾=if 仫>0&&俛{伝+且>=0}else{且>=0&&(伝>=佷||仫>0)};if!侾{continue;}let
乬=j.乬(&両);let 丛=乬+j.佻(q)-6*j.义(q,j.丏)-10*(C::from(一)-1);if 乬>0{if 上.is_none_or(|(亀,_)|丛>亀){上=Some((丛,三));}}else if 且>=3&&伝>=佷+10&&一==1{if 伬.is_none_or(|(亀,_)|丛>亀){伬=Some((丛,三));}}}}let
Some((_,三))=上.or(伬)else{return;};if!j.个(三){return;}}}}fn 俗(l:&J,丆:&mut Vec<K>){丆.clear();俩(l,丆);}const 予:C=C::MAX;const 侟:F=8;struct 交{万:[C;M],並:[I;M],}fn
伓(l:&J,z:F,乺:Q,丏:F)->C{佒(l,z,亘(l,z,乺),丏)}fn 佒(l:&J,z:F,亘:G,丏:F)->C{let mut 万=予;佛(l,z,亘,|丙,new|{if new.test(丏){万=10*丙 as C;}万!=予});万}fn 修(l:&J,z:F,亘:G,丫:C)->G{let
mut 仴=G::T;佛(l,z,亘,|丙,new|{if 10*丙 as C>丫{return true;}仴|=new;false});仴.丞(l.七(z))}fn 亘(l:&J,z:F,乺:Q)->G{if 乺&&z==l.丝(){l.于()}else{G::T}}fn 佛(l:&J,z:F,亘:G,mut
亝:impl FnMut(F,G)->Q,){let 乘=l.两().也();let mut 仈=[G::T;3];let mut 乃=G::T;for 一 in 1..=乘{let 临=l.丽(z,一);仈[F::from(一)-1]=临.丞(乃);乃|=临;}let mut 举=[l.七(z),G::T,G::T,G::T];for
q in(亘&l.丅(z)).iter(){if let Some(一)=l.亜(q).filter(|&一|一<=乘){举[0]|=G::q(q).丒()&l.丽(z,一);}}let mut 丼=G::T;for 丙 in 0..{let new=举[丙%4].丞(丼);举[丙%4]=G::T;if
亝(丙,new){return;}丼|=new;let 丂=new.丒().丞(丼);for(便,丌)in 仈.iter().enumerate(){举[(丙+便+1)%4]|=丂&*丌;}if 举.iter().all(|临|临.丞(丼).is_empty()){break;}}}impl 交{fn
new(l:&J,z:F,乺:Q)->交{let 乘=F::from(l.两().也());let mut 丣=[G::T;3];for(与,临)in 丣.iter_mut().enumerate().take(乘){*临=l.丽(z,与 as I+1);}let 七=l.七(z);let 丒=&l.map().丒[0];let
mut 万=[予;M];let mut 並=[乞;M];let mut 举=[0u8;M];let mut 丑=0;let mut 佹=G::T;for q in 七.iter(){万[q]=0;举[丑]=q as I;丑+=1;}if 乺&&z==l.丝(){for 不 in l.于().iter(){let
Some(一)=l.亜(不)else{continue;};let 乥=G::q(不).丒()&丣[F::from(一)-1];for q in 乥.iter(){if 万[q]!=0{万[q]=0;举[丑]=q as I;丑+=1;}}}}while 丑>0{let mut 上=0;for 与 in
1..丑{let(a,b)=(F::from(举[与]),F::from(举[上]));if(万[a],a)<(万[b],b){上=与;}}let q=F::from(举[上]);丑-=1;举[上]=举[丑];佹.set(q);for&丂 in&丒[q]{if 丂==乞{break;}let 丂=F::from(丂);if
佹.test(丂)||七.test(丂){continue;}let Some(一)=丣[..乘].iter().position(|临|临.test(丂))else{continue;};let 侁=万[q]+10*(一 as C+1);if 侁<万[丂]{if 万[丂]==予{举[丑]=丂 as
I;丑+=1;}万[丂]=侁;並[丂]=q as I;}}}交{万,並}}fn path(&self,丏:F)->L<F>{let mut path=L::new();if 丏>=M||self.万[丏]==予{return path;}let mut q=丏;while self.並[q]!=乞{path.push(q);q=F::from(self.並[q]);}path.reverse();path}}struct
仝{丄:L<K>,万:C,}fn 俟(l:&J,z:F,q:F)->P<I>{(1..=l.两().也()).find(|&一|l.丽(z,一).test(q))}fn 俱(丬:&mut J,丁:F,亩:F,丫:C,丄:&mut Vec<K>,乤:&mut i64,)->Q{loop{if 丬.丗()==Some(么::乩(丁)){return
true;}if 丬.丠(丁).test(亩)&&丬.七(丁).test(亩){return true;}let 丯=交::new(丬,丁,false);let 万=丯.万[亩];if 万==予||*乤+万>丫{return false;}let Some(&q)=丯.path(亩).first()else{return
false;};let Some(一)=俟(丬,丁,q)else{return false;};let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(三){return false;}*乤+=10*C::from(一);丄.push(三);}}fn 但(l:&J,丁:F,俖:&[F],丫:C)->P<仝>{let
mut 丬=l.clone();let mut 丄=L::new();let mut 乤=0;for&亩 in 俖{if!俱(&mut 丬,丁,亩,丫,&mut 丄,&mut 乤){return None;}}(乤<=丫).then_some(仝{丄,万:乤,})}fn 伎(l:&J,丏:F,丫:C)->P<仝>{if
丫<0||l.丗().is_some()||丏>=M||l.map().丹.test(丏){return None;}let 丁=l.丝();if l.丠(丁).test(丏)&&l.七(丁).test(丏){return Some(仝{丄:L::new(),万:0,});}let mut 上=但(l,丁,&[丏],丫);let
乊=交::new(l,丁,false);let mut 乒:L<F> =(l.乒()&l.丠(1-丁)).iter().filter(|&乇|乇!=丏&&乊.万[乇]!=予).collect();乒.sort_by_key(|&乇|(l.map().乆(乇,丏),乇));乒.truncate(侟);for
乇 in 乒{if let Some(丧)=但(l,丁,&[乇,丏],丫){if 上.as_ref().is_none_or(|亀|丧.万<亀.万){上=Some(丧);}}}上}const 依:F=20000;struct 仞{丄:L<K>,万:C,丌:C,丅:C,乐:C,且:C,丘:C,}struct
伀{丌:C,丅:C,乐:C,}fn 佟(l:&J,z:F)->伀{let 七=l.七(z);let 丅=l.丅(z)&七;let 丛=丅.iter().filter_map(|q|l.亜(q)).map(|一|10*C::from(一)).sum();伀{丌:C::from(七.count()),丅:丛,乐:C::from(((l.丟()|l.乒())&七).count()),}}struct
伽{丁:F,任:G,並:伀,且:C,亯:F,丫:C,亴:F,path:L<K>,丨:L<仞>,index:HashMap<仂,F,BuildHasherDefault<DefaultHasher>>,}type 仂=[U;3];fn 倗(mut 乌:仂,q:F)->仂{乌[q/64]|=1<<(q%64);乌}impl
伽{fn 佖(&mut self,丬:&J,乹:P<F>,价:仂,万:C,)->Q{let j=self;let 丁=j.丁;let 乼=match 乹{None=>丬.亙(丁),Some(q)=>G::q(q).丒(),}&j.任;let 乼=乼.丞(丬.丠(丁));let mut 丣=[G::T;3];for
一 in 1..=丬.两().也(){丣[F::from(一)-1]=丬.丽(丁,一);}for q in 乼.iter(){let 乘=F::from(丬.两().也());let Some(与)=丣[..乘].iter().position(|临|临.test(q))else{continue;};let
一=与 as I+1;let 亏=10*C::from(一);if 万+亏>j.丫{continue;}if j.亴>=依{return false;}j.亴+=1;let mut 丂=丬.clone();let 三=K::乖{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(三){continue;}j.path.push(三);let
人=倗(价,q);j.俯(&丂,人,万+亏);let 俋=j.path.len()<j.亯&&丂.丗().is_none();let 俕=!俋||j.佖(&丂,Some(q),人,万+亏);j.path.pop();if!俕{return false;}}true}fn 俯(&mut self,为:&J,人:仂,万:C){let
j=self;let 从=1-j.丁;let 伣=佟(为,从);let 丌=j.並.丌-伣.丌;let 丅=j.並.丅-伣.丅;if 丌<=0&&丅<=0{return;}let 乐=j.並.乐-伣.乐;let 且=为.且(j.丁)-j.且;let 倚=为.丗()==Some(么::乩(j.丁));let
丘=if 倚{C::MAX/2}else{3*丌+丅+10*乐+2*且-万};if let Some(&主)=j.index.get(&人){if 万>=j.丨[主].万{return;}}let 东=仞{丄:j.path.clone(),万,丌,丅,乐,且,丘,};match j.index.get(&人){Some(&主)=>j.丨[主]=东,None=>{j.index.insert(人,j.丨.len());j.丨.push(东);}}}}fn
佔(l:&J,亯:F,丫:C,乁:F)->L<仞>{乍(l,亯,丫,乁).0}fn 乍(l:&J,亯:F,丫:C,乁:F)->(L<仞>,F){if l.丗().is_some()||亯==0||乁==0{return(L::new(),0);}let 丁=l.丝();let 从=1-丁;let mut
伨=伽{丁,任:l.丠(从)|l.丠(从).丒(),並:佟(l,从),且:l.且(丁),亯,丫,亴:0,path:L::new(),丨:L::new(),index:HashMap::default(),};伨.佖(l,None,[0;3],0);let 亴=伨.亴;let mut 丨=伨.丨;丨.sort_by(|a,b|b.丘.cmp(&a.丘).then(a.万.cmp(&b.万)));丨.truncate(乁);(丨,亴)}const
仡:C=1000000;const 侞:C=200;fn 乎(l:&J,z:F)->C{let 七=l.七(z);let 丅:C=(l.丅(z)&七).iter().filter_map(|q|l.亜(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.且(z)+4*C::from(七.count())+丅+10*C::from((l.乒()&七).count())}fn
佥(l:&J,z:F,乙:G)->C{俓(l,z,佒(l,z,乙,Z[1-z]),)}fn 俓(l:&J,z:F,万:C)->C{if 万==予{return C::MIN;}l.丈(z)+l.且(z)-万}fn 佲(事:C)->C{事.clamp(-侞,0)}fn 俑(l:&J,z:F,乙:[G;2],)->(C,[C;2]){let
丩=1-z;match l.丗(){Some(么::乩(倖))=>{return(if 倖==z{仡}else{-仡},[0,0])}Some(么::伴)=>return(0,[0,0]),None=>{}}let mut 丘=乎(l,z)-乎(l,丩);if z==l.丝(){let mut 丂=l.clone();丂.仆();if
丂.丗().is_none()&&伓(&丂,丩,true,Z[z])<=丂.丈(丩){丘-=仡/2;}}let 二=[佥(l,z,乙[0]),佥(l,丩,乙[1]),];(丘+佲(二[0])-佲(二[1]),二)}
