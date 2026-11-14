// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 丮=u32;type U=u64;type 仜=i32;type C=i64;type 亞=f64;type T=bool;type 串=String;type 乕=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侙:T=true;fn 俣(l:&J,q:F,仗:C,丰:G,丧:C)->T{let
丁=l.丝();let 乵=1-丁;let 佧=侙&&丧<=l.丈(乵)+2*l.且(乵).max(0);let 仾=G::q(q)|G::q(q).丐();if!佧&&(仾&丰).is_empty(){return false;}let(x,y)=亓(q);let mut 为=l.clone();if!为.个(K::亄{x,y}){return
false;}if 們(&为,丁)<仗{return true;}佧&&众(&为,乵,false,Z[丁])>丧}const 侴:I=2;const 侠:F=4;const 伸:F=5;#[derive(Clone)]enum 丳{亠(K),伄(L<K>),}impl 丳{fn 丅(&self)->&[K]{match
self{丳::亠(三)=>std::slice::from_ref(三),丳::伄(丅)=>丅,}}}fn 亓(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 佢(l:&J,丆:&mut Vec<丳>){丆.clear();if l.丗().is_some(){return;}let
丁=l.丝();let 乚=l.两().乡();let mut 丣=[G::Q;4];for 一 in 1..=乚{丣[F::from(一)]=l.举(丁,一);}for 不 in l.二().iter(){let Some((丑,一))=l.仙(不)else{continue;};let 乇=G::q(不).丐()&丣[F::from(一)];for
乯 in 乇.iter(){let(x,y)=亓(乯);丆.push(丳::亠(K::乱{丑,x,y}));}}let 丈=l.丈(丁);let 丄=l.丄(丁);let 亱=l.亙(丁);let 倘=match 侴{0=>亱,1=>{let 伎=l.map().valid.丘(丄).丐()&丄;亱.丘(丄)|伎}_=>亱.丘(丄),};for
一 in 1..=乚{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=倘&丣[F::from(一)];for 俧 in 1..一{丌=丌.丘(丣[F::from(俧)]);}for q in 丌.iter(){let(x,y)=亓(q);丆.push(丳::亠(K::乘{一,x,y}));}}if
丈>=亂{let 乬=l.丞(丁)&亱&l.亦();if l.两().代(){for q in(乬&l.map().丱).iter(){let(x,y)=亓(q);丆.push(丳::亠(K::亃{x,y}));}}if l.两().令(){let 伎=l.map().valid.丘(l.丞(丁)).丐();let
丌=(乬&伎).丘(l.map().丱);let 丰=丰(l,1-丁);let 仗=C::from((丰&l.丄(丁)).count());let 丧=众(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俣(l,q,仗,丰,丧)){let(x,y)=亓(q);丆.push(丳::亠(K::亄{x,y}));}}}for
丟 in 佚(l,伸,丈,侠){if 丟.丅.len()>1{丆.push(丳::伄(丟.丅));}}if let Some(丧)=伒(l,Z[1-丁],丈){if 丧.丅.len()>1{丆.push(丳::伄(丧.丅));}}}const 侬:I=2;const 侾:C=5;fn 乛(l:&J,z:F)->G{伨(l,z,侬)}fn
伨(l:&J,z:F,不:I)->G{let mut 乛=G::Q;if 不>3{return 乛;}for q in(l.七(z)&l.丄(z)).iter(){if l.互(q).is_some_and(|一|一>=不){乛|=G::q(q);}}乛}fn 丰(l:&J,乏:F)->G{let 丫=l.丈(乏)+l.且(乏);俷(l,乏,乛(l,乏),丫)}fn
今(l:&J,z:F)->G{丰(l,1-z)&l.丄(z)}fn 們(l:&J,z:F)->C{C::from(今(l,z).count())}const 佀:C=4;fn 俪(丌:C)->C{丌/佀}fn 丠(l:&J,z:F,今:G)->C{let 丄=l.丄(z);let 丠=(l.丠()&丄).丘(今);let
丌=C::from(丄.count());let 俐=if l.乸()<=侔{丌/侓+侒}else{俪(丌)};俐*C::from(丠.count())}const 侔:丮=15;const 侒:C=6;const 侓:C=佀;const 侺:C=2;fn 侌(l:&J,z:F)->C{let 丄=l.丄(z);let[a,b,c]=丄.任();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 与=(hash as F)&(佈-1);俆.with(|伐|{let mut 伐=伐.borrow_mut();let
仸=&mut 伐[与];if 仸.0==丄&&仸.1==z as I+1{return 仸.2;}let 东=倞(丄,Z[z]);*仸=(丄,z as I+1,东);东})}const 佈:F=1<<12;thread_local!{static 俆:std::cell::RefCell<L<(G,I,C)>>
=std::cell::RefCell::new(vec![(G::Q,0,0);佈]);}fn 倞(丄:G,乮:F)->C{struct 什{丄:G,乆:[I;M],丵:[I;M],size:[I;M],亐:I,乹:C,仲:C,}fn 亜(d:&mut 什,v:F)->I{d.亐+=1;let 乆=d.亐;d.乆[v]=乆;let
mut 丵=乆;let mut 伱=1u8;let mut 丟=0u8;for w in(G::q(v).丐()&d.丄).iter(){if d.乆[w]==0{let 侄=亜(d,w);伱+=侄;丵=丵.min(d.丵[w]);if d.丵[w]>=乆{丟+=侄;}}else{丵=丵.min(d.乆[w]);}}d.丵[v]=丵;d.size[v]=伱;if
丟>0&&d.乆[v]>1{d.乹+=1;d.仲=d.仲.max(C::from(丟));}伱}if!丄.test(乮){return 0;}let mut d=什{丄,乆:[0;M],丵:[0;M],size:[0;M],亐:0,乹:0,仲:0,};亜(&mut d,乮);d.乹+d.仲}fn 俾(l:&J,z:F)->C{侺*(侌(l,1-z)-侌(l,z))}fn
佛(l:&J,z:F)->C{if l.丈(z)+l.且(z)>=0{return 0;}(l.七(z)&l.丄(z)).iter().filter_map(|q|l.互(q)).map(|一|10*C::from(一)).sum()}const 侟:C=15;const 俄:[C;2]=[3,1];const
係:[C;2]=[4,20];fn 倍(l:&J,z:F)->[C;2]{let mut 丒=[0;2];for q in(l.七(z)&l.丄(z)).iter(){match l.互(q){Some(2)=>丒[0]+=1,Some(3)=>丒[1]+=1,_=>{}}}丒}fn 侃(l:&J,z:F)->C{let[侉,倒]=倍(l,z);let
低=|一:F|(3-俄[一])*係[一];-侟*侉+低(0)*侉+低(1)*倒}const 侘:C=0;const 供:C=2;const 侚:C=100;fn 侁(仱:C)->C{if 仱<0{return 0;}侘+供*仱.min(侚)/2}const 伿:I=2;const 伽:C=2;const
似:C=4;const 侜:I=3;fn 亗(l:&J,z:F)->C{if 伽==0{return 0;}let 乔=l.map().valid;let mut 七=G::Q;for q in(l.七(z)&l.丄(z)).iter(){if l.互(q).is_some_and(|一|一<=侜){七|=G::q(q);}}let
mut 仺=乔.丘(l.丞(z));let mut 伙=G::Q;let mut 乇=0;for 丼 in 1..=似{仺=(仺|仺.丐())&乔;let 仪=(七&仺).丘(伙);乇+=(丼-1)*C::from(仪.count());伙|=仪;}乇+=似*C::from(七.丘(伙).count());伽*乇}fn
伛(l:&J,z:F)->C{let 乛=[伨(l,z,伿),伨(l,1-z,伿),];let(丙,亮)=俜(l,z,乛);let 丙=丙+侁(亮[0])-侁(亮[1]);if l.丗().is_some(){return 丙;}let 仐=今(l,z);let 亀=今(l,1-z);let 仗=C::from(亀.count())-C::from(仐.count());丙-2*佛(l,z)+2*佛(l,1-z)+俾(l,z)+侃(l,z)-侃(l,1-z)-亗(l,z)+亗(l,1-z)+侾*仗/4+丠(l,z,仐)-丠(l,1-z,亀)}const
侩:F=8;const 侵:F=3;const 価:F=32;const 侶:U=300;const 侑:亞=0.75;struct 了{乿:P<乕>,亖:P<乕>,仍:U,}#[derive(Default)]struct 但{丅:L<K>,伖:F,乶:U,世:F,丯:F,体:F,乷:F,}const
俅:T=true;#[derive(Clone)]struct 乗{丅:L<K>,会:C,}struct 亡{丅:L<K>,end:J,first:P<U>,专:P<L<乗>>,}type 仁=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
伆{丁:F,limit:了,乶:U,仌:仁,亚:P<L<丳>>,}const 伷:T=true;const 侎:T=false;const 伶:P<I> =None;fn 俎(l:&J,世:&[乗],丆:&mut Vec<丳>){if!伷&&伶.is_none(){return;}let 仕=世.iter().map(|乜|乜.丅.as_slice());俏(l,仕,(伷,侎,伶),丆,);}fn
俏<'a>(l:&J,仕:impl Iterator<Item=&'a[K]>,(乓,丐,一):(T,T,P<I>),丆:&mut Vec<丳>,){let 丁=l.丝();let 丷=l.丄(丁);let mut 仈=G::Q;for 乜 in 仕{for 三 in 乜{if let K::乘{x,y,..}|K::乱{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丷.test(q){仈|=G::q(q);}}}}if 仈.is_empty(){return;}let 乬=l.亦()&丷;let 丈=l.丈(丁);let 佸=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丅()==[三]){丆.push(丳::亠(三));}};if
乓&&丈>=亂&&l.两().令(){let mut 丌=仈;if 丐{丌|=仈.丐();}for q in(丌&乬).丘(l.map().丱).iter(){let(x,y)=亓(q);佸(K::亄{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().乡()){if
丈>=10*C::from(一){for q in(仈&乬).iter(){let(x,y)=亓(q);佸(K::乘{一,x,y},丆);}}}}fn 伍(l:&mut J,丅:&[K])->T{丅.iter().all(|&三|l.个(三))}fn 乍(丅:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丅{let(仭,a,b,c)=match*三{K::乱{丑,x,y}=>(1,U::from(丑),x,y),K::乘{一,x,y}=>(2,U::from(一),x,y),K::亃{x,y}=>(3,0,x,y),K::亄{x,y}=>(4,0,x,y),};let 亿=仭|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亿).wrapping_mul(0x0100000001b3);}hash}fn
佋(乙:&J,乜:&[K])->J{let mut 丂=乙.clone();丂.仇();for&三 in 乜{丂.个(三);}丂.仇();丂}fn 仯(l:&J)->P<L<K>>{let 丁=l.丝();let 丏=Z[1-丁];let 亾=|両:&J|両.丗()==Some(义::乪(丁));for
q in l.二().iter(){if l.map().丼(q,丏)!=1{continue;}let Some((丑,_))=l.仙(q)else{continue;};let(x,y)=亓(丏);let 三=K::乱{丑,x,y};let mut 両=l.clone();if 両.个(三)&&亾(&両){return
Some(vec![三]);}}let 丧=伒(l,丏,l.丈(丁))?;let mut 両=l.clone();(伍(&mut 両,&丧.丅)&&亾(&両)).then_some(丧.丅)}impl 伆{fn 书(&self)->T{self.乶>=self.limit.仍||self.limit.乿.is_some_and(|t|乕::now()>=t)}fn
丙(&mut self,乙:&J,世:&[乗],伏:C)->C{self.乶+=1;self.伬(乙,世,伏)}fn 伬(&self,乙:&J,世:&[乗],伏:C)->C{if 乙.丗()==Some(义::乪(self.丁)){return 仢;}if 世.is_empty(){return 伛(乙,self.丁);}let
mut 仼=C::MAX;for 乜 in 世{仼=仼.min(伛(&佋(乙,&乜.丅),self.丁));if 仼<=伏{break;}}仼}#[allow(clippy::too_many_arguments)]fn 俱(&mut self,乫:&J,亥:C,乐:&[丳],亶:&[(C,F)],丽:&mut[T],世:&[乗],产:&mut
仁,伢:T,)->(P<(F,J,C)>,T){let mut 下=None;let mut 以=亥;for&(仳,与)in 亶{if 丽[与]{continue;}if 伢&&仳!=C::MAX&&仳<=以-亥{return(下,true);}if self.书(){return(下,false);}丽[与]=true;let
丅=乐[与].丅();let mut 丂=乫.clone();if!伍(&mut 丂,丅){产.insert(乍(丅),C::MIN/4);continue;}let 丙=self.丙(&丂,世,以);产.insert(乍(丅),丙-亥);if 丙>以{以=丙;下=Some((与,丂,丙));}}(下,false)}fn
伕(&mut self,乮:&J,世:&[乗],仃:&[U],俒:T)->亡{let j=self;let mut 之=None;let mut 乫=乮.clone();let mut 丅=L::new();let mut 亥=j.丙(&乫,世,C::MIN);let mut 产=仁::default();let
mut 乐=L::new();let mut 亶=L::new();while 乫.丗().is_none()&&!j.书(){if 丅.is_empty()&&俒{match&j.亚{Some(亚)=>乐.clone_from(亚),None=>{佢(&乫,&mut 乐);j.亚=Some(乐.clone());}}}else{佢(&乫,&mut
乐);}俎(&乫,世,&mut 乐);亶.clear();for(与,乲)in 乐.iter().enumerate(){let k=乍(乲.丅());let 仳=产.get(&k).or(j.仌.get(&k)).copied();亶.push((仳.unwrap_or(C::MAX),与));}亶.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丽=vec![false;乐.len()];if 丅.is_empty(){for(与,乲)in 乐.iter().enumerate(){丽[与]=仃.contains(&乍(乲.丅()));}}let mut 下=None;for
伢 in[true,false]{let(丨,倆)=j.俱(&乫,亥,&乐,&亶,&mut 丽,世,&mut 产,伢,);下=丨;if 下.is_some()||!倆{break;}}let Some((与,丂,丙))=下 else{break;};if 丅.is_empty(){之=Some(乍(乐[与].丅()));}丅.extend_from_slice(乐[与].丅());乫=丂;亥=丙;}for(k,乭)in
产{let 佤=j.仌.entry(k).or_insert(乭);*佤=(*佤).max(乭);}亡{丅,end:乫,first:之,专:None,}}fn 仕(&self,乙:&J)->L<L<K>>{let mut 丂=乙.clone();丂.仇();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 乢=丂.丝();let 丈=丂.丈(乢);if let Some(丧)=伒(&丂,Z[self.丁],丈){丆.push(丧.丅);}for 丟 in 佚(&丂,伸,丈,侵){丆.push(丟.丅);}let mut 佣=L::new();俢(&丂,&mut
佣);丆.push(佣);let mut 亀=伆{丁:乢,limit:了{仍:侶,..self.limit},乶:0,仌:仁::default(),亚:None,};丆.push(亀.伕(&丂,&[],&[],true).丅);丆.retain(|丅|!丅.is_empty());丆}fn 佌(&self,乙:&J)->L<乗>{self.仕(乙).into_iter().map(|丅|{let
会=伛(&佋(乙,&丅),self.丁);乗{丅,会}}).collect()}fn 乷(&mut self,乮:&J,乲:&丳,世:&[乗])->P<亡>{let mut 为=乮.clone();if!伍(&mut 为,乲.丅()){return None;}self.乶+=1;let 九=self.伕(&为,世,&[],false);let
mut 丅=乲.丅().to_vec();丅.extend_from_slice(&九.丅);Some(亡{丅,end:九.end,first:Some(乍(乲.丅())),专:None,})}fn 佟(&self,丕:&亡,世:&[乗])->C{let 丙=self.伬(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>丙.min(self.伬(&丕.end,专,C::MIN)),_=>丙,}}}fn
俹(丨:&[乗],世:&mut Vec<乗>)->T{let 並:L<_> =世.iter().map(|r|r.丅.clone()).collect();for 乜 in 丨{if 世.iter().any(|r|r.丅==乜.丅){continue;}世.push(乜.clone());}世.sort_by_key(|乜|乜.会);世.truncate(侩);世.iter().map(|r|&r.丅).ne(&並)}fn
倐(l:&J,乂:了,亵:&mut 亇)->但{let 丁=l.丝();let mut 中=但::default();if l.丗().is_some(){return 中;}if let Some(丅)=仯(l){中.丅=丅;return 中;}let 主=|倀:亞|match(乂.乿,乂.亖){(Some(乿),Some(亖))=>Some(乿+(亖-乿).mul_f64(倀)),_=>None,};if
亵.俴(l){中.丅=亵.上(l,主(0.4));return 中;}let mut 乎=伆{丁,limit:了{乿:主(侑),..乂},乶:0,仌:仁::default(),亚:None,};let mut 世=L::new();let mut 丯:L<亡> =L::new();let mut 仃=L::new();let
mut 仨=false;let mut 乷:L<丳> =L::new();while 中.伖<価&&(丯.is_empty()||!乎.书()){let 丕=if 仨{let Some(乲)=乷.pop()else{break;};match 乎.乷(l,&乲,&世){Some(丕)=>丕,None=>continue,}}else{乎.伕(l,&世,&仃,true)};中.伖+=1;let
佽=丯.iter().any(|p|p.丅==丕.丅);let 之=丕.first;let 侂=乎.书();let mut 丕=丕;let mut 佮=false;if!侂&&!佽{let 丨=乎.佌(&丕.end);佮=俹(&丨,&mut 世);丕.专=Some(丨);}if!佽{丯.push(丕);}if
侂{break;}if!佮&&!仨{match 之{Some(之)=>仃.push(之),None=>{仨=true;乷=乎.亚.clone().unwrap_or_default();乷.retain(|a|!仃.contains(&乍(a.丅())));let 乭=|a:&丳|{乎.仌.get(&乍(a.丅())).copied().unwrap_or(C::MIN)};乷.sort_by_key(乭);}}}}中.乷=F::from(仨);中.乶=乎.乶;中.世=世.len();中.丯=丯.len();let
佩=||乂.亖.is_some_and(|t|乕::now()>=t);let mut 亹:L<P<C>> =vec![None;丯.len()];for 与 in(0..丯.len()).rev(){if 与+1<丯.len()&&佩(){break;}亹[与]=Some(乎.佟(&丯[与],&世));}let
亷=|亹:&[P<C>]|{let mut 下:P<(C,F)> =None;for(与,丙)in 亹.iter().enumerate().rev(){if let Some(丙)=*丙{if 下.is_none_or(|(亁,_)|丙>亁){下=Some((丙,与));}}}下};let mut
下=亷(&亹);乎.limit.乿=乂.亖;while let Some((_,与))=下{if!俅||丯[与].专.is_some()||佩(){break;}let 丨=乎.佌(&丯[与].end);丯[与].专=Some(丨);中.体+=1;亹[与]=Some(乎.佟(&丯[与],&世));下=亷(&亹);}if
let Some((_,与))=下{let mut 丬=l.clone();for&三 in&丯[与].丅{if 丬.个(三){中.丅.push(三);}}}中}fn 俳(乂:&dyn Fn(乕,丮)->了,mut 亵:亇,伡:impl BufRead,仴:impl Write,)->Result<(),串>{乀(伡,仴,|享,乞,上,些|match
J::read(享,乞,乨::伀,上){Ok(l)=>{let 中=倐(&l,乂(些,上),&mut 亵);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,中.伖,中.乶,中.世,中.体,中.乷,些.elapsed());俋(&中.丅)}Err(井)=>{eprintln!("plan: cannot read the input: {}",井);"WAIT".to_string()}},)}const
侏:亞=45.0;const 侕:亞=800.0;const 侻:亞=0.6;fn 乂(些:乕,上:丮)->了{if let Some(仍)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
了{乿:None,亖:None,仍,};}let 价=倓().min(100.0);let 俬=if 上==1{侕}else{侏};let 丫=Duration::from_secs_f64(俬/1000.0*价);了{乿:Some(些+丫.mul_f64(侻)),亖:Some(些+丫),仍:U::MAX,}}const
侳:F=9;const 侭:亟=亟{丌:3,且:1,丈:1,丟:2,乹:2,付:2,丰:6,仚:1,};const 侲:I=I::MAX;const 侯:C=2;const 侰:C=18;const 侮:C=0;const 侱:F=32;fn 亵()->亇{俯(侱)}fn 俯(乧:F)->亇{let
乾=亢::仡{乀:侳,乧,乒:侭,};let 丛=侹.iter().map(|&乤|Y{伯:侲,仄:侯,伟:侰,伋:侮,..乤}).collect();亇::new(乾,丛,Y::伈)}fn main(){let 倊=io::stdin();let 個=io::stdout();if let Err(井)=俳(&乂,亵(),倊.lock(),個.lock()){eprintln!("plan: {}",井);std::process::exit(1);}}const
便:&str="CG_TIME_SCALE";fn 倓()->亞{俰(std::env::var(便).ok().as_deref())}fn 俰(东:P<&str>)->亞{东.and_then(|乞|乞.trim().parse::<亞>().ok()).filter(|价|价.is_finite()&&*价>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乖:F=4*O;const 乩:U=(1<<乖)-1;const 仝:U=1|1<<12|1<<24|1<<36;const 企:U=仝<<11;const 佄:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const Q:G=G([0;3]);const 侍:G=G([乩;3]);#[inline]fn q(q:F)->G{let mut 临=G::Q;临.set(q);临}#[inline]fn test(self,q:F)->T{(self.0[q/乖]>>(q%乖))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乖]|=1<<(q%乖);}#[inline]fn clear(&mut self,q:F){self.0[q/乖]&=!(1<<(q%乖));}#[inline]fn is_empty(self)->T{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
任(self)->[U;3]{self.0}#[inline]fn count(self)->丮{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丘(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仿{仿{伥:self.0,主:0,}}#[inline]fn 倅(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&佄)<<(乖-O)),(b>>O)|((c&佄)<<(乖-O)),c>>O,])}#[inline]fn 倂(self)->G{let[a,b,c]=self.0;G([(a<<O)&乩,((b<<O)&乩)|(a>>(乖-O)),((c<<O)&乩)|(b>>(乖-O)),])}#[inline]fn
倃(self)->G{let[a,b,c]=self.0;G([(a>>1)&!企&乩,(b>>1)&!企&乩,(c>>1)&!企&乩,])}#[inline]fn 倄(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仝&乩,(b<<1)&!仝&乩,(c<<1)&!仝&乩,])}#[inline]fn
丐(self)->G{let j=self;j.倅()|j.倂()|j.倃()|j.倄()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侍.丘(self)}}struct 仿{伥:[U;3],主:F,}impl Iterator for 仿{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.主<3{let 仰=j.伥[j.主];if 仰!=0{j.伥[j.主]=仰&(仰-1);return Some(j.主*乖+仰.trailing_zeros()as F);}j.主+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乱{丑:丮,x:I,y:I},乘{一:I,x:I,y:I},亃{x:I,y:I},亄{x:I,y:I},}impl K{fn 倕(&self)->串{match*self{K::乱{丑,x,y}=>format!("MOVE {} {x} {y}",丑),K::乘{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::亃{x,y}=>format!("BUILD MINE {x} {y}"),K::亄{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
俋(丅:&[K])->串{if 丅.is_empty(){return "WAIT".to_string();}let mut line=串::with_capacity(16*丅.len());for(与,三)in 丅.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&三.倕());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乨{佉,値,俇,#[default]伀,}impl 乨{fn 乡(self)->I{match self{乨::佉=>1,_=>3,}}fn 代(self)->T{matches!(self,乨::俇|乨::伀)}fn 令(self)->T{self==乨::伀}}const 习:I=I::MAX;struct
买{丹:G,valid:G,丱:G,丼:Box<[I]>,丐:[[[I;4];M];2],}thread_local!{static 侞:RefCell<P<Arc<买>>> =const{RefCell::new(None)};}impl 买{fn 倁(丹:G,丱:G)->Arc<买>{侞.with(|乺|{let
mut 乺=乺.borrow_mut();if let Some(亭)=乺.as_ref(){if 亭.丹==丹&&亭.丱==丱{return Arc::clone(亭);}}let 亭=Arc::new(买::build(丹,丱));*乺=Some(Arc::clone(&亭));亭})}fn build(丹:G,丱:G)->买{let
乔=!丹;const 侼:[[(仜,仜);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丐=[[[习;4];M];2];for(z,乇)in 侼.iter().enumerate(){for q in
乔.iter(){let(x,y)=((q%O)as 仜,(q/O)as 仜);let mut 丒=0;for&(俙,俚)in 乇{let(佰,佱)=(x+俙,y+俚);if(0..O as 仜).contains(&佰)&&(0..O as 仜).contains(&佱){let 丂=佱 as F*O+佰
as F;if 乔.test(丂){丐[z][q][丒]=丂 as I;丒+=1;}}}}}let mut 丼=vec![习;M*M].into_boxed_slice();let mut 伫=[0u8;M];for 不 in 乔.iter(){let 于=&mut 丼[不*M..(不+1)*M];于[不]=0;伫[0]=不
as I;let(mut 伝,mut 伳)=(0,1);while 伝<伳{let q=伫[伝]as F;伝+=1;for&丂 in&丐[0][q]{if 丂!=习&&于[丂 as F]==习{于[丂 as F]=于[q]+1;伫[伳]=丂;伳+=1;}}}}买{丹,valid:乔,丱,丼,丐,}}#[inline]fn
丼(&self,不:F,乯:F)->I{self.丼[不*M+乯]}}const 侨:丮=100;const 亂:C=15;const 促:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 义{乪(F),伺,}enum
也{伅,佁,佂,}#[derive(Clone)]struct J{map:Arc<买>,两:乨,专:[G;2],丄:[G;2],丷:G,么:G,七:[G;2],一:[G;3],事:G,亽:[丮;M],丈:[C;2],且:[C;2],丝:F,乸:丮,亲:丮,丗:P<义>,}fn 介(专:G,亪:F)->G{if!专.test(亪){return
G::Q;}let mut 仔=G::q(亪);loop{let 丂=(仔|仔.丐())&专;if 丂==仔{return 仔;}仔=丂;}}impl J{fn 丝(&self)->F{self.丝}fn 乸(&self)->丮{self.乸}fn 丗(&self)->P<义>{self.丗}fn 两(&self)->乨{self.两}fn
亰(&self,z:F)->C{let j=self;let 七=j.七[z];j.丈[z]+10*C::from((七&j.一[0]).count())+20*C::from((七&j.一[1]).count())+30*C::from((七&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伴(&self,z:F)->F{self.七[z].count()as F}fn map(&self)->&买{&self.map}fn 丞(&self,z:F)->G{self.专[z]}fn 丄(&self,z:F)->G{self.丄[z]}fn
七(&self,z:F)->G{self.七[z]}fn 仙(&self,q:F)->P<(丮,I)>{let j=self;(j.七[0]|j.七[1]).test(q).then(||(j.亽[q],j.伤(q)))}fn 二(&self)->G{self.七[self.丝]&self.事}fn
丠(&self)->G{self.丷}fn 乓(&self)->G{self.么}fn 互(&self,q:F)->P<I>{(q<M&&(self.七[0]|self.七[1]).test(q)).then(||self.伤(q))}fn 倉(&mut self,z:F){let j=self;j.丄=[介(j.专[0],Z[0]),介(j.专[1],Z[1])];let
俕=j.七[z].丘(j.丄[z]);j.伭(俕);j.佔();j.丈[z]+=j.且[z];if j.丈[z]<0{j.丈[z]=0;j.伭(j.七[z]);}j.事|=j.七[z];}fn 佔(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
七=丄&j.七[z];let mut 且=C::from(丄.count())+4*C::from((丄&j.丷).count());for(一,倜)in 促.iter().enumerate(){且-=倜*C::from((七&j.一[一]).count());}j.且[z]=且;}}fn 仇(&mut
self){let j=self;if j.丗.is_some(){return;}if j.丝==1{if j.乸>=侨{j.丗=Some(match j.亰(0).cmp(&j.亰(1)){std::cmp::Ordering::Greater=>义::乪(0),std::cmp::Ordering::Less=>义::乪(1),std::cmp::Ordering::Equal=>义::伺,});return;}j.乸+=1;}j.丝=1-j.丝;j.倉(j.丝);}fn
佶(&mut self,丅:&[K]){let j=self;if j.丗.is_some(){return;}let 伜=j.亲;for&三 in 丅{if j.俘(三,伜){continue;}j.个(三);if j.丗.is_some(){return;}}j.仇();}fn 俘(&self,三:K,伜:丮)->T{let
j=self;match 三{K::乱{丑,x,y}=>{j.交(x,y).is_none()||丑>=伜||j.你(j.丝,丑).is_none()}K::乘{一,x,y}=>{j.交(x,y).is_none()||一==0||一>j.两.乡()}K::亃{x,y}|K::亄{x,y}=>{j.交(x,y).is_none()}}}#[inline]fn
交(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丹.test(y*O+x)).then_some(y*O+x)}fn 你(&self,z:F,丑:丮)->P<F>{self.七[z].iter().find(|&q|self.亽[q]==丑)}#[inline]fn
伤(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 俍(&self,乢:F)->G{let 乓=self.么&self.专[乢];let 丄=self.丄[乢];乓|(丄&(丄&乓).丐())}fn
举(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乢=1-z;let 七=j.七[0]|j.七[1];let 俛=j.亦();let 倚=match
一{1 if j.两==乨::佉=>j.一[1]|j.一[2],1=>七,2=>j.一[1]|j.一[2],_=>G::Q,};let 举=俛|j.专[乢].丘(倚);if 一<3{举.丘(j.俍(乢))}else{举}}fn 亙(&self,z:F)->G{self.丄[z]|self.丄[z].丐()}fn
亦(&self)->G{let j=self;let 七=j.七[0]|j.七[1];let 俤=G::q(Z[0])|G::q(Z[1]);j.map.valid.丘(七|j.丷|j.么|俤)}fn 个(&mut self,三:K)->T{let j=self;if j.丗.is_some(){return
false;}let z=j.丝;let 五=match 三{K::乘{一,x,y}=>j.倗(z,一,x,y),K::乱{丑,x,y}=>j.俫(z,丑,x,y),K::亃{x,y}=>j.build(z,true,x,y),K::亄{x,y}=>j.build(z,false,x,y),};let
Some(五)=五 else{return false;};match 五{也::伅=>{}也::佁=>j.丄[z]=介(j.专[z],Z[z]),也::佂=>{j.丄=[介(j.专[0],Z[0]),介(j.专[1],Z[1])];}}j.佔();if!j.专[0].test(Z[0]){j.丗=Some(义::乪(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(义::乪(0));}true}fn 伭(&mut self,丌:G){let j=self;j.七[0]=j.七[0].丘(丌);j.七[1]=j.七[1].丘(丌);for 一 in&mut j.一{*一=一.丘(丌);}j.事=j.事.丘(丌);}fn
侅(&mut self,z:F,q:F)->也{let j=self;j.伭(G::q(q));j.丷.clear(q);j.么.clear(q);let 乢=1-z;if j.专[乢].test(q){j.专[乢].clear(q);j.专[z].set(q);也::佂}else if j.专[z].test(q){也::伅}else{j.专[z].set(q);也::佁}}fn
倗(&mut self,z:F,一:I,x:I,y:I)->P<也>{let j=self;if 一==0||一>j.两.乡(){return None;}let q=j.交(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亙(z).test(q)||!j.举(z,一).test(q){return
None;}let 五=j.侅(z,q);j.佴(z,一,q,j.亲);j.亲=j.亲.saturating_add(1);j.丈[z]-=万;Some(五)}fn 佴(&mut self,z:F,一:I,q:F,丑:丮){self.七[z].set(q);self.一[F::from(一)-1].set(q);self.亽[q]=丑;}fn
俫(&mut self,z:F,丑:丮,x:I,y:I)->P<也>{let j=self;let 丏=j.交(x,y)?;let 不=j.你(z,丑)?;if!j.事.test(不){return None;}let 一=j.伤(不);let 丣=j.举(z,一);let 乁=j.map.丐[z][不];if
乁.contains(&(丏 as I))&&!丣.test(丏){return None;}let mut 下=j.map.丼(不,丏);let mut 仅=None;for&丂 in 乁.iter().take_while(|&&丂|丂!=习){let 丂=F::from(丂);let 丼=j.map.丼(丂,丏);if
丼<下&&丣.test(丂){下=丼;仅=Some(丂);}}let 仅=仅?;let 五=j.侅(z,仅);j.七[z].clear(不);j.一[F::from(一)-1].clear(不);j.事.clear(不);j.佴(z,一,仅,丑);Some(五)}fn build(&mut self,z:F,丷:T,x:I,y:I)->P<也>{let
j=self;let 俊=if 丷{j.两.代()}else{j.两.令()};let q=j.交(x,y)?;let 俭=j.map.丱.test(q);if!俊||j.丈[z]<亂||俭!=丷||!j.专[z].test(q)||!j.亙(z).test(q)||!j.亦().test(q){return
None;}if 丷{j.丷.set(q);}else{j.么.set(q);}j.丈[z]-=亂;Some(也::伅)}}const 侣:C=1000000000;const 伂:C=0;const 侪:C=1;const 俀:C=2;impl J{fn read(享:&str,上:&str,两:乨,乸:丮,)->Result<J,串>{let
mut 丱=L::new();let mut 严=享.lines();let 丒=优(&mut 严,"mine spot count")?;for _ in 0..丒{let[x,y]=乼(亳(&mut 严,"a mine spot")?,"a mine spot")?;丱.push((仧(x)?,仧(y)?));}佝(严,"the initial input")?;let
mut 严=上.lines();let mut 伞=[0i64;4];for(东,丶)in 伞.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伪]=乼(亳(&mut 严,丶)?,丶)?;if 伪.abs()>侣{return
Err(format!("{}: {} is out of range",丶,伪));}*东=伪;}let mut 丹=G::Q;let mut 丌=[G::Q;2];let mut 丄=[G::Q;2];for y in 0..O{let 于=亳(&mut 严,"a map row")?.trim();if
于.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,于));}for(x,他)in 于.bytes().enumerate(){let q=y*O+x;match 他{b'#'=>丹.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乃=F::from(他==b'X'||他==b'x');丌[乃].set(q);if 他.is_ascii_uppercase(){丄[乃].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",他 as char))}}}}let
俑=优(&mut 严,"building count")?;let mut 乑=L::new();for _ in 0..俑{let 乼=乼(亳(&mut 严,"a building")?,"a building")?;乑.push(乼);}let 伴=优(&mut 严,"unit count")?;let
mut 七=L::new();for _ in 0..伴{let 乼=乼(亳(&mut 严,"a unit")?,"a unit")?;七.push(乼);}佝(严,"the turn's input")?;let 佳=乑.iter().find(|佐|佐[0]==0&&佐[1]==伂).ok_or("the bot's own HQ is not among the buildings")?;let
佦=休(佳[2],佳[3],丹)?;let z=Z.iter().position(|&亪|亪==佦).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佦))?;let mut 使=G::Q;for(x,y)in 丱{if
丹.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}使.set(y*O+x);}let 伉=|乃:F|if 乃==0{z}else{1-z};let mut l=J{map:买::倁(丹,使),两,专:[G::Q;2],丄:[G::Q;2],丷:G::Q,么:G::Q,七:[G::Q;2],一:[G::Q;3],事:G::Q,亽:[0;M],丈:[0;2],且:[0;2],丝:z,乸,亲:1,丗:None,};for
乃 in 0..2{let 乣=伉(乃);l.专[乣]=丌[乃];l.丄[乣]=丄[乃];l.丈[乣]=伞[2*乃];l.且[乣]=伞[2*乃+1];}let mut 佲=G::Q;for[乣,仭,x,y]in 乑{let 乃=佺(乣)?;let q=休(x,y,丹)?;if!丌[乃].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佲.test(q){return Err(format!("two buildings at ({x}, {y})"));}佲.set(q);match 仭{伂
if q==Z[伉(乃)]=>{}伂=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侪=>l.丷.set(q),俀=>l.么.set(q),_=>return Err(format!("unknown building type {}",仭)),}}let
mut 乄=G::Q;let mut 仮=0u32;for[乣,丑,一,x,y]in 七{let 乃=佺(乣)?;let q=休(x,y,丹)?;if!丌[乃].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乄.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丑=丮::try_from(丑).map_err(|_|format!("unit id {} is out of range",丑))?;if
乄.iter().any(|丩|l.亽[丩]==丑){return Err(format!("two units have id {}",丑));}乄.set(q);l.七[伉(乃)].set(q);l.一[一 as F-1].set(q);l.亽[q]=丑;仮=仮.max(丑);}if!乄.is_empty(){l.亲=仮.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仮))?;}l.事=l.七[z];Ok(l)}}fn
亳<'a>(严:&mut Lines<'a>,丶:&str)->Result<&'a str,串>{严.next().ok_or_else(||format!("the text ends before {}",丶))}fn 佝(mut 严:Lines<'_>,丶:&str)->Result<(),串>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丶)),None=>Ok(()),}}fn 乼<const N:F>(line:&str,丶:&str)->Result<[C;N],串>{let
mut 任=line.split_whitespace();let mut 例=[0i64;N];for 东 in&mut 例{let 亿=任.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丶))?;*东=亿.parse().map_err(|_|format!("{}: {:?} is not an integer",丶,亿))?;}if
任.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丶));}Ok(例)}fn 优(严:&mut Lines<'_>,丶:&str)->Result<F,串>{let[丒]=乼(亳(严,丶)?,丶)?;F::try_from(丒).ok().filter(|&丒|丒<=M).ok_or_else(||format!("{}: {} is out of range",丶,丒))}fn
仧(东:C)->Result<F,串>{F::try_from(东).ok().filter(|&东|东<O).ok_or_else(||format!("coordinate {} is off the map",东))}fn 休(x:C,y:C,丹:G)->Result<F,串>{let q=仧(y)?*O+仧(x)?;if
丹.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佺(乣:C)->Result<F,串>{match 乣{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乣)),}}const
侗:C=15;const 侫:C=6;const 佇:C=10;const 侸:I=3;#[derive(Clone,Copy)]struct Y{open:C,仦:C,佗:I,传:C,伟:C,佫:C,伯:I,佖:C,何:C,伋:C,亩:C,亻:C,丸:C,京:C,云:C,仄:C,}impl Y{const
伈:Y=Y{open:3,仦:2,佗:12,传:4,伟:侗,佫:侫,伯:侸,佖:2,何:2,伋:0,亩:0,亻:0,丸:0,京:0,云:0,仄:0,};const fn 倢(self,亩:C,亻:C,丸:C)->Y{Y{亩,亻,丸,..self}}const fn 倡(self,京:C,云:C)->Y{Y{京,云,..self}}}fn
仛(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仆(l:&J,不:G)->[I;M]{let mut 乊=[I::MAX;M];let 乔=l.map().valid;let mut 乌=不&乔;let mut 乄=乌;let mut d=0u8;while!乌.is_empty(){for
q in 乌.iter(){乊[q]=d;}let 丂=(乌.丐()&乔).丘(乄);乄|=丂;乌=丂;d=d.saturating_add(1);}乊}fn 俟(l:&J)->I{let 乊=仆(l,l.丞(0));l.丞(1).iter().map(|q|乊[q]).min().unwrap_or(I::MAX)}struct
予<'a>{乤:&'a Y,丁:F,theirs:[I;M],亼:[I;M],free:G,乏:G,ours:G,丱:G,七:G,}impl 予<'_>{fn new<'a>(l:&J,乤:&'a Y)->予<'a>{let 丁=l.丝();let 丷=仆(l,l.丞(丁));let 亀=仆(l,l.丞(1-丁));let
亼=仆(l,G::q(Z[1-丁]));let 乬=l.map().valid.丘(l.丞(0)|l.丞(1));let mut 乏=G::Q;for q in 乬.iter(){if 丷[q]<=亀[q]{乏.set(q);}}予{乤,丁,theirs:亀,亼,free:乬,乏,ours:l.丞(丁),丱:l.map().丱,七:l.七(丁),}}fn
value(&self,q:F)->C{let j=self;let p=j.乤;let 丿=C::from((G::q(q).丐()&j.free).count());let 佒=p.佗;let 仦=C::from(佒)-C::from(j.theirs[q].min(佒));let 亪=if p.传>0{C::from(j.亼[q].min(30))/p.传}else{0};let
专=C::from((G::q(q).丐()&j.ours).count());let line=if 专<=1{p.亻}else{0};let 丸=if j.丱.test(q){p.丸}else{0};let 倎=if 丿==0{p.京}else{0};let 云=p.云*(22-C::from(j.亼[q].min(22)));let
仄=p.仄*C::from((G::q(q).丐()&j.七).count());p.open*丿+p.仦*仦-亪+p.亩*专-line+丸-倎+云-仄}fn 下(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
伔(&mut self,q:F){let j=self;j.free.clear(q);j.乏.clear(q);j.ours.set(q);j.七.set(q);}}fn 仒(l:&mut J,三:K,丆:&mut Vec<K>)->T{let 佊=l.个(三);if 佊{丆.push(三);}佊}fn
业(l:&mut J,不:F,乯:F,丆:&mut Vec<K>)->T{let Some((丑,_))=l.仙(不)else{return false;};let(x,y)=仛(乯);仒(l,K::乱{丑,x,y},丆)}fn 倛(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-佇}#[derive(Clone,Copy)]enum
久{仠,亅{丠:I,七:I},}fn 侈(l:&J,乤:&Y,亊:久)->L<K>{let mut 丬=l.clone();let mut 上=予::new(l,乤);let mut 丆=L::new();乻(&mut 丬,&mut 上,&mut 丆);match 亊{久::仠=>俶(&mut 丬,&mut
上,&mut 丆),久::亅{丠,七}=>{保(&mut 丬,&mut 上,丠,七,&mut 丆)}}丆}fn 乻(l:&mut J,上:&mut 予<'_>,丆:&mut Vec<K>){let 丁=上.丁;let mut 亗:L<F> =L::new();loop{let 丣=l.举(丁,1);let
mut 亷:P<(丮,F)> =None;for 不 in l.二().iter(){if 亗.contains(&不){continue;}let 乽=(G::q(不).丐()&上.free&丣).count();if 乽==0{亗.push(不);}else if 亷.is_none_or(|(n,_)|乽<n){亷=Some((乽,不));}}let
Some((_,不))=亷 else{break;};上.七.clear(不);match 上.下(G::q(不).丐()&上.free&丣){Some(乯)if 业(l,不,乯,丆)=>上.伔(乯),_=>{上.七.set(不);亗.push(不);}}}for 不 in 亗{if!l.二().test(不){continue;}let
丏=[上.乏,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().丼(不,q),q))});if let Some(丏)=丏{业(l,不,丏,丆);}}}fn 俶(l:&mut J,上:&mut 予<'_>,丆:&mut
Vec<K>){let 丁=上.丁;let p=上.乤;let h=(p.伟-C::from(l.乸())).max(p.佫);let 倔=(C::from(俟(l))-p.佖).max(0)/p.何.max(1)+p.伋;loop{let 丈=l.丈(丁);let 七=l.伴(丁)as C;let
俻=C::from(上.乏.count());let q=上.下(l.亙(丁)&上.free&l.举(丁,1));let 侊=倛(h,(俻/(七+1)).min(倔));let 佑=q.is_some()&&丈>=佇&&侊>0;let 佬=4*h-亂;let 丸=佭(l,上);let 俓=丸.is_some()&&丈>=亂&&佬>0;if
俓&&(!佑||佬>=侊){let(x,y)=仛(丸.unwrap_or_default());if 仒(l,K::亃{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佑)else{break;};let(x,y)=仛(q);if!仒(l,K::乘{一:1,x,y},丆){break;}上.伔(q);}}fn
佭(l:&J,上:&予<'_>)->P<F>{let 丁=上.丁;(l.map().丱&l.丄(丁)&l.亦()).iter().filter(|&丸|上.theirs[丸]>上.乤.伯).min_by_key(|&丸|(l.map().丼(Z[丁],丸),丸))}fn 保(l:&mut J,上:&mut
予<'_>,丠:I,七:I,丆:&mut Vec<K>,){let 丁=上.丁;for _ in 0..丠{let Some(丸)=佭(l,上)else{break;};let(x,y)=仛(丸);if!仒(l,K::亃{x,y},丆){break;}}for _ in 0..七{let Some(q)=上.下(l.亙(丁)&上.free&l.举(丁,1))else{break;};let(x,y)=仛(q);if!仒(l,K::乘{一:1,x,y},丆){break;}上.伔(q);}}fn
件(l:&J)->T{if l.丗().is_some(){return false;}let 丁=l.丝();let 丈=l.丈(丁);let 丰=亣::new(l,丁,true);l.丞(1-丁).iter().any(|q|丰.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亝:I,亊:久,}const 伵:I=I::MAX;const 侐:[久;8]=[久::仠,久::亅{丠:0,七:0},久::亅{丠:0,七:1},久::亅{丠:0,七:2},久::亅{丠:0,七:伵,},久::亅{丠:1,七:0},久::亅{丠:1,七:1},久::亅{丠:1,七:伵,},];struct
伹<'a>{丛:&'a[Y],从:Y,rest:丢,乴:P<乕>,俼:Settings,}impl 伹<'_>{fn 乤(&self,业:丢)->&Y{&self.丛[F::from(业.亝)%self.丛.len()]}fn 丅(&self,l:&J,业:丢)->L<K>{侈(l,self.乤(业),业.亊)}}fn
估(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伦=|亿:U|hash=(hash^亿).wrapping_mul(0x0100000001b3);for(倏,临)in[l.丞(丁),l.七(丁),l.丠()].into_iter().enumerate(){for
q in 临.iter(){伦(((倏 as U)<<8)|q as U);}伦(0xff);}伦(l.丈(丁)as U);hash}enum 亢{仡{乀:F,乧:F,乒:亟,},}struct 亇{乾:亢,丛:L<Y>,从:Y,丕:L<丢>,仩:L<U>,亸:F,丯:丮,仵:T,}impl 亇{fn
new(乾:亢,丛:L<Y>,从:Y)->亇{亇{乾,丛,从,丕:L::new(),仩:L::new(),亸:0,丯:0,仵:false,}}fn 俴(&mut self,l:&J)->T{let j=self;let 倇=matches!(j.乾,亢::仡{..});let 俵=!倇&&j.丯>0&&j.亸>=j.仩.len();if!j.仵&&(俵||件(l)){j.仵=true;}!j.仵}fn
上(&mut self,l:&J,乴:P<乕>)->L<K>{let j=self;let 余=伹{丛:&j.丛,从:j.从,rest:丢{亝:0,亊:久::仠,},乴,俼:Settings::DEFAULT,};let 丁=l.丝();let 修=j.仩.get(j.亸)==Some(&估(l,丁));if!修{let
之=j.丯==0;j.丕=match j.乾{亢::仡{乀,乧,乒,}=>{let 乒=&乒;let 九=侷;let mut 乇=丕(l,&j.丛,九,乀,乧,乴,乒,);乇.resize(乀,九);乇}};j.仩=match j.乾{亢::仡{..}=>倈(l,&j.丛,&j.丕),};j.亸=0;j.丯+=1;}let
业=j.丕.get(j.亸).copied().unwrap_or(余.rest);j.亸+=1;余.丅(l,业)}}fn 倈(l:&J,丛:&[Y],丕:&[丢])->L<U>{let 丁=l.丝();let mut 佨=L::with_capacity(丕.len());let mut 乥=l.clone();for&业
in 丕{if 乥.丗().is_some(){break;}佨.push(估(&乥,丁));乥=伊(&乥,&丅(&乥,丛,业));}佨}const 俁:C=3;struct Settings{乀:C,}impl Settings{const DEFAULT:Settings=Settings{乀:俁};}fn
乇(l:&J,不:G)->[I;M]{let 乔=l.map().valid;let mut 乊=[I::MAX;M];let mut 乌=不&乔;let mut 乄=乌;let mut d=0u8;while!乌.is_empty(){for q in 乌.iter(){乊[q]=d;}let 丂=(乌.丐()&乔).丘(乄);乄|=丂;乌=丂;d=d.saturating_add(1);}乊}fn
倝(l:&J,z:F)->C{let 仐=乇(l,l.丄(z));let 亀=乇(l,l.丄(1-z));l.map().valid.iter().map(|q|match 仐[q].cmp(&亀[q]){_ if 仐[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侹:[Y;27]=丛();const 侷:丢=丢{亝:16,亊:久::仠,};const fn 丛()->[Y;27]{let 亩=[0,2,4];let 亻=[0,3,6];let 云=[0,1,2];let mut 丆=[Y::伈;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
京=if g>0||t>0{4}else{0};丆[i]=Y::伈.倢(亩[g],亻[t],0).倡(京,云[w]);i+=1;}丆}#[derive(Default)]struct 伇{丌:C,且:C,丈:C,乹:C,丟:C,付:C,丰:C,仚:C,}#[derive(Clone,Copy)]struct
亟{丌:C,且:C,丈:C,丟:C,乹:C,付:C,丰:C,仚:C,}fn 俽(l:&J,z:F)->[C;M]{struct 什{丄:G,乆:[u16;M],丵:[u16;M],size:[C;M],丟:[C;M],亐:u16,}fn 亜(d:&mut 什,v:F){d.亐+=1;d.乆[v]=d.亐;d.丵[v]=d.亐;d.size[v]=1;for
w in(G::q(v).丐()&d.丄).iter(){if d.乆[w]==0{亜(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.乆[v]{d.丟[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.乆[w]);}}}let
mut d=什{丄:l.丄(z),乆:[0;M],丵:[0;M],size:[0;M],丟:[0;M],亐:0,};let 乮=Z[z];if d.丄.test(乮){亜(&mut d,乮);}d.丟[乮]=0;d.丟}fn 俿(l:&J,z:F)->伇{let 丄=l.丄(z);let 丟=俽(l,z);let
亼=仆(l,G::q(Z[1-z]));let mut 丆=伇{丌:C::from(丄.count()),且:l.且(z),丈:l.丈(z),丰:C::MAX,仚:倝(l,z)/2,..伇::default()};for q in 丄.iter(){if 丟[q]>0{丆.乹+=1;丆.丟=丆.丟.max(丟[q]);}if
q!=Z[z]&&(G::q(q).丐()&丄).count()<=1{丆.付+=1;}丆.丰=丆.丰.min(C::from(亼[q]));}丆}fn 东(l:&J,z:F,乒:&亟)->C{let s=俿(l,z);let o=乒;o.丌*s.丌+o.且*s.且+o.丈*s.丈-o.丟*s.丟-o.乹*s.乹-o.付*s.付-o.丰*s.丰.min(30)+o.仚*s.仚}fn
伊(l:&J,丅:&[K])->J{let mut 丂=l.clone();丂.佶(丅);if 丂.丗().is_none(){丂.佶(&[]);}丂}fn 丅(l:&J,丛:&[Y],业:丢)->L<K>{侈(l,&丛[F::from(业.亝)%丛.len()],业.亊,)}fn 併(l:&J,丛:&[Y],丕:&[丢],九:丢,乀:F,)->J{let
mut 乥=l.clone();for 上 in 0..乀{if 乥.丗().is_some(){break;}let 业=丕.get(上).copied().unwrap_or(九);乥=伊(&乥,&丅(&乥,丛,业));}乥}fn 倌(l:&J,丛:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亝 in 0..丛.len().min(F::from(I::MAX)){for&亊 in&侐{let 业=丢{亝:亝 as I,亊,};let c=丅(l,丛,业);if!丆.iter().any(|(_,丩)|*丩==c){丆.push((业,c));}}}丆}fn
俌(l:&J,丛:&[Y],九:丢,乀:F,乧:F,乴:P<乕>,乒:&亟,)->L<丢>{let 丁=l.丝();let 仑=||乴.is_some_and(|t|乕::now()>=t);let 仓=|丕:&[丢]|东(&併(l,丛,丕,九,乀),丁,乒);let mut 下:(C,L<丢>)=(仓(&[]),L::new());let
mut 乌:L<(J,L<丢>)> =vec![(l.clone(),L::new())];for 俗 in 0..乀{let mut 乳:L<(C,J,L<丢>)> =L::new();let mut 乄:HashMap<U,F> =HashMap::new();for(佯,丕)in&乌{if 仑(){break;}for(业,c)in
倌(佯,丛){let 丂=伊(佯,&c);let mut 伓=丕.clone();伓.push(业);let v=东(&丂,丁,乒);let 乍=估(&丂,丁);match 乄.get(&乍){Some(&主)if 乳[主].0>=v=>{}Some(&主)=>乳[主]=(v,丂,伓),None=>{乄.insert(乍,乳.len());乳.push((v,丂,伓));}}}}if
乳.is_empty(){break;}乳.sort_by(|a,b|b.0 .cmp(&a.0));乳.truncate(乧);let 乺=俗+1==乀||仑();for(i,(_,_,丕))in 乳.iter().take(if 乺{乧}else{1}).enumerate(){if i>0&&仑(){break;}let
v=仓(丕);if v>下.0{下=(v,丕.clone());}}if 乺{break;}乌=乳.into_iter().map(|(_,s,p)|(s,p)).collect();}下.1}fn 丕(l:&J,丛:&[Y],九:丢,乀:F,俩:F,乴:P<乕>,乒:&亟,)->L<丢>{let 丁=l.丝();let
仓=|丕:&[丢]|东(&併(l,丛,丕,九,乀),丁,乒);let mut 下:(C,L<丢>)=(仓(&[]),L::new());let mut 乧=2;loop{let 丨=俌(l,丛,九,乀,乧,乴,乒);let v=仓(&丨);if v>下.0{下=(v,丨);}let 仑=乴.is_some_and(|t|乕::now()>=t);if
仑||(乴.is_none()&&乧>=俩){break;}乧*=2;}下.1}const 侖:F=4+12;fn 乀(伡:impl BufRead,mut 仴:impl Write,mut 伌:impl FnMut(&str,&str,丮,乕)->串,)->Result<(),串>{let mut
严=伾{佹:伡};let Some(之)=严.line()?else{return Ok(());};let mut 享=format!("{}\n",之);严.take(丒(&之,"mine spot count")?,&mut 享)?;let mut 上=0;loop{let Some(之)=严.line()?else{return
Ok(());};let 些=乕::now();上+=1;let mut 乞=format!("{}\n",之);严.take(侖-1,&mut 乞)?;for 丶 in["building count","unit count"]{let line=严.佻()?;let n=丒(&line,丶)?;乞.push_str(&line);乞.push('\n');严.take(n,&mut
乞)?;}let 伌=伌(&享,&乞,上,些);writeln!(仴,"{}",伌).and_then(|()|仴.flush()).map_err(|井|format!("turn {}: cannot write the answer: {}",上,井))?;}}struct 伾<R>{佹:R,}impl<R:BufRead>伾<R>{fn
line(&mut self)->Result<P<串>,串>{let mut line=串::new();match self.佹.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(井)=>Err(format!("cannot read input: {}",井)),}}fn
佻(&mut self)->Result<串,串>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乞:&mut String)->Result<(),串>{for
_ in 0..n{乞.push_str(&self.佻()?);乞.push('\n');}Ok(())}}fn 丒(line:&str,丶:&str)->Result<F,串>{match line.trim().parse::<F>(){Ok(n)if n<=侢=>Ok(n),_=>Err(format!("bad {} {line:?}",丶)),}}const
侢:F=144;const 侽:C=3;const 依:C=2;const 侧:F=16;fn 俲(l:&J,丆:&mut Vec<K>){let z=l.丝();let mut 乾=佃{s:l.clone(),z,乵:1-z,丏:Z[1-z],仫:Z[z],上:l.乸(),丆,};乾.run();}struct
佃<'a>{s:J,z:F,乵:F,丏:F,仫:F,上:丮,丆:&'a mut Vec<K>,}fn 份(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伧(丑:丮,q:F)->K{let(x,y)=份(q);K::乱{丑,x,y}}fn 侇(一:I,q:F)->K{let(x,y)=份(q);K::乘{一,x,y}}impl
佃<'_>{fn run(&mut self){let j=self;if j.仯(){return;}j.乻();if j.仯(){return;}j.乑();j.候();}fn 乊(&self,不:F,乯:F)->C{C::from(self.s.map().丼(不,乯))}fn 个(&mut self,三:K)->T{let
丽=self.s.个(三);if 丽{self.丆.push(三);}丽}fn 亾(&self,両:&J)->T{両.丗()==Some(义::乪(self.z))}fn 仯(&mut self)->T{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.二().iter(){if j.乊(q,j.丏)!=1{continue;}let Some((丑,_))=j.s.仙(q)else{continue;};let 三=伧(丑,j.丏);let mut 両=j.s.clone();if 両.个(三)&&j.亾(&両){return j.个(三);}}let
Some(丧)=j.俔()else{return false;};let mut 両=j.s.clone();if!丧.iter().all(|&三|両.个(三))||!j.亾(&両){return false;}for 三 in 丧{j.个(三);}true}fn 俔(&self)->P<L<K>>{let
j=self;const 伻:C=C::MAX;let 乚=j.s.两().乡();let 丣:L<G> =(1..=乚).map(|一|j.s.举(j.z,一)).collect();let 仉=|q:F|{丣.iter().position(|临|临.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 丄=j.s.丄(j.z);let mut 万=[伻;M];let mut 並=[F::MAX;M];let mut 丽=[false;M];for q in 丄.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丽[q]&&万[q]<伻&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丽[q]=true;if q==j.丏{break;}for&乁 in&j.s.map().丐[0][q]{if 乁==习{break;}let 乁=F::from(乁);if 丽[乁]||丄.test(乁){continue;}let Some((_,亍))=仉(乁)else{continue;};if
万[q]+亍<万[乁]{万[乁]=万[q]+亍;並[乁]=q;}}}if 万[j.丏]>j.s.丈(j.z){return None;}let mut 丧=L::new();let mut q=j.丏;while!丄.test(q){let(一,_)=仉(q)?;丧.push(侇(一,q));q=並[q];}丧.reverse();Some(丧)}fn
乭(&self,为:&J)->C{let(丁,亏)=(self.z,self.乵);let 並=&self.s;let 丒=|临:G|C::from(临.count());let 专=丒(为.丞(丁))-丒(並.丞(丁));let 仹=丒(並.丞(亏))-丒(为.丞(亏));let 俦=丒(並.丄(亏))-丒(为.丄(亏));let
们=並.亰(亏)-为.亰(亏);let 丠=丒(並.丠()&並.丞(亏))-丒(为.丠()&为.丞(亏));12*专+20*仹+8*(俦-仹).max(0)+5*们+25*丠}fn 侀(&self,q:F)->C{let j=self;let 丸=j.s.map().丱.test(q);if 丸&&j.s.两().代()&&!j.s.丞(j.z).test(q){20}else{0}}fn
佞(&self,乂:C)->T{let j=self;j.s.七(j.乵).iter().any(|q|j.乊(q,j.仫)<=乂)}fn 乻(&mut self){let j=self;let mut 七:L<(C,F)> =j.s.二().iter().map(|q|(j.乊(q,j.丏),q)).collect();七.sort_unstable();for(仪,q)in
七{if j.s.丗().is_some(){return;}let Some((丑,一))=j.s.仙(q)else{continue;};let 丣=j.s.举(j.z,一);let 乁=j.s.map().丐[j.z][q];let mut 下:P<(C,F)> =None;for&丂 in 乁.iter().take_while(|&&丂|丂!=习){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.个(伧(丑,丂)){continue;}let 东=if j.亾(&両){100000}else{j.乭(&両)+10*(仪-j.乊(丂,j.丏))+j.侀(丂)};if 下.is_none_or(|(亁,_)|东>亁){下=Some((东,丂));}}if
let Some((东,丂))=下{if 东>0{j.个(伧(丑,丂));}}}}fn 乑(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.佞(侽){j.倖();}else{j.丠();}}fn 倖(&mut self){let j=self;if!j.s.两().令()||j.s.丈(j.z)<亂{return;}let
乁=j.s.map().丐[j.z][j.仫];let 丌:L<F> =乁.iter().take_while(|&&q|q!=习).map(|&q|F::from(q)).collect();let 丷=j.s.丞(j.z);if 丌.iter().any(|&q|丷.test(q)&&j.s.乓().test(q)){return;}for
q in 丌{let(x,y)=份(q);if j.个(K::亄{x,y}){return;}}}fn 丠(&mut self){let j=self;if!j.s.两().代(){return;}while j.s.丈(j.z)>=亂{let 乬=j.s.map().丱&j.s.丞(j.z)&j.s.丄(j.z)&j.s.亦();let
Some(q)=乬.iter().min_by_key(|&q|(j.乊(q,j.仫),q))else{return;};let(x,y)=份(q);if!j.个(K::亃{x,y}){return;}}}fn 候(&mut self){let j=self;let 佼=if j.上<=3{0}else{5};for
_ in 0..侧{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俥=j.佞(依);let 亱=j.s.亙(j.z);let mut 下:P<(C,K)> =None;let mut 伲:P<(C,K)> =None;let
mut 住=G::Q;for 一 in 1..=j.s.两().乡(){let 丣=j.s.举(j.z,一);let 乽=(亱&丣).丘(住);住=丣;let 亍=10*C::from(一);if 亍>丈{continue;}for q in 乽.iter(){let 三=侇(一,q);let mut
両=j.s.clone();if!両.个(三){continue;}let 伣=両.丈(j.z);let 且=両.且(j.z);let 们=j.s.亰(j.乵)-両.亰(j.乵);let 俈=if 们>0&&俥{伣+且>=0}else{且>=0&&(伣>=佼||们>0)};if!俈{continue;}let
乭=j.乭(&両);let 东=乭+j.侀(q)-6*j.乊(q,j.丏)-10*(C::from(一)-1);if 乭>0{if 下.is_none_or(|(亁,_)|东>亁){下=Some((东,三));}}else if 且>=3&&伣>=佼+10&&一==1{if 伲.is_none_or(|(亁,_)|东>亁){伲=Some((东,三));}}}}let
Some((_,三))=下.or(伲)else{return;};if!j.个(三){return;}}}}fn 俢(l:&J,丆:&mut Vec<K>){丆.clear();俲(l,丆);}const 争:C=C::MAX;const 侦:F=8;struct 亣{万:[C;M],並:[I;M],}fn
众(l:&J,z:F,乻:T,丏:F)->C{佘(l,z,亘(l,z,乻),丏)}fn 佘(l:&J,z:F,亘:G,丏:F)->C{let mut 万=争;佡(l,z,亘,|业,new|{if new.test(丏){万=10*业 as C;}万!=争});万}fn 俷(l:&J,z:F,亘:G,丫:C)->G{let
mut 件=G::Q;佡(l,z,亘,|业,new|{if 10*业 as C>丫{return true;}件|=new;false});件.丘(l.丄(z))}fn 亘(l:&J,z:F,乻:T)->G{if 乻&&z==l.丝(){l.二()}else{G::Q}}fn 佡(l:&J,z:F,亘:G,mut
亜:impl FnMut(F,G)->T,){let 乚=l.两().乡();let mut 仉=[G::Q;3];let mut 乄=G::Q;for 一 in 1..=乚{let 临=l.举(z,一);仉[F::from(一)-1]=临.丘(乄);乄|=临;}let mut 丿=[l.丄(z),G::Q,G::Q,G::Q];for
q in(亘&l.七(z)).iter(){if let Some(一)=l.互(q).filter(|&一|一<=乚){丿[0]|=G::q(q).丐()&l.举(z,一);}}let mut 丽=G::Q;for 业 in 0..{let new=丿[业%4].丘(丽);丿[业%4]=G::Q;if
亜(业,new){return;}丽|=new;let 丂=new.丐().丘(丽);for(俉,丌)in 仉.iter().enumerate(){丿[(业+俉+1)%4]|=丂&*丌;}if 丿.iter().all(|临|临.丘(丽).is_empty()){break;}}}impl 亣{fn
new(l:&J,z:F,乻:T)->亣{let 乚=F::from(l.两().乡());let mut 丣=[G::Q;3];for(与,临)in 丣.iter_mut().enumerate().take(乚){*临=l.举(z,与 as I+1);}let 丄=l.丄(z);let 丐=&l.map().丐[0];let
mut 万=[争;M];let mut 並=[习;M];let mut 丿=[0u8;M];let mut 丒=0;let mut 佾=G::Q;for q in 丄.iter(){万[q]=0;丿[丒]=q as I;丒+=1;}if 乻&&z==l.丝(){for 不 in l.二().iter(){let
Some(一)=l.互(不)else{continue;};let 乇=G::q(不).丐()&丣[F::from(一)-1];for q in 乇.iter(){if 万[q]!=0{万[q]=0;丿[丒]=q as I;丒+=1;}}}}while 丒>0{let mut 下=0;for 与 in
1..丒{let(a,b)=(F::from(丿[与]),F::from(丿[下]));if(万[a],a)<(万[b],b){下=与;}}let q=F::from(丿[下]);丒-=1;丿[下]=丿[丒];佾.set(q);for&丂 in&丐[q]{if 丂==习{break;}let 丂=F::from(丂);if
佾.test(丂)||丄.test(丂){continue;}let Some(一)=丣[..乚].iter().position(|临|临.test(丂))else{continue;};let 來=万[q]+10*(一 as C+1);if 來<万[丂]{if 万[丂]==争{丿[丒]=丂 as
I;丒+=1;}万[丂]=來;並[丂]=q as I;}}}亣{万,並}}fn path(&self,丏:F)->L<F>{let mut path=L::new();if 丏>=M||self.万[丏]==争{return path;}let mut q=丏;while self.並[q]!=习{path.push(q);q=F::from(self.並[q]);}path.reverse();path}}struct
仞{丅:L<K>,万:C,}fn 俨(l:&J,z:F,q:F)->P<I>{(1..=l.两().乡()).find(|&一|l.举(z,一).test(q))}fn 俺(丬:&mut J,丁:F,亨:F,丫:C,丅:&mut Vec<K>,书:&mut i64,)->T{loop{if 丬.丗()==Some(义::乪(丁)){return
true;}if 丬.丞(丁).test(亨)&&丬.丄(丁).test(亨){return true;}let 丰=亣::new(丬,丁,false);let 万=丰.万[亨];if 万==争||*书+万>丫{return false;}let Some(&q)=丰.path(亨).first()else{return
false;};let Some(一)=俨(丬,丁,q)else{return false;};let 三=K::乘{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(三){return false;}*书+=10*C::from(一);丅.push(三);}}fn 位(l:&J,丁:F,信:&[F],丫:C)->P<仞>{let
mut 丬=l.clone();let mut 丅=L::new();let mut 书=0;for&亨 in 信{if!俺(&mut 丬,丁,亨,丫,&mut 丅,&mut 书){return None;}}(书<=丫).then_some(仞{丅,万:书,})}fn 伒(l:&J,丏:F,丫:C)->P<仞>{if
丫<0||l.丗().is_some()||丏>=M||l.map().丹.test(丏){return None;}let 丁=l.丝();if l.丞(丁).test(丏)&&l.丄(丁).test(丏){return Some(仞{丅:L::new(),万:0,});}let mut 下=位(l,丁,&[丏],丫);let
之=亣::new(l,丁,false);let mut 乓:L<F> =(l.乓()&l.丞(1-丁)).iter().filter(|&么|么!=丏&&之.万[么]!=争).collect();乓.sort_by_key(|&么|(l.map().丼(么,丏),么));乓.truncate(侦);for
么 in 乓{if let Some(丧)=位(l,丁,&[么,丏],丫){if 下.as_ref().is_none_or(|亁|丧.万<亁.万){下=Some(丧);}}}下}const 侤:F=20000;struct 仟{丅:L<K>,万:C,丌:C,七:C,乑:C,且:C,丙:C,}struct
伃{丌:C,七:C,乑:C,}fn 佥(l:&J,z:F)->伃{let 丄=l.丄(z);let 七=l.七(z)&丄;let 东=七.iter().filter_map(|q|l.互(q)).map(|一|10*C::from(一)).sum();伃{丌:C::from(丄.count()),七:东,乑:C::from(((l.丠()|l.乓())&丄).count()),}}struct
佅{丁:F,仾:G,並:伃,且:C,亯:F,丫:C,亴:F,path:L<K>,丨:L<仟>,index:HashMap<仂,F,BuildHasherDefault<DefaultHasher>>,}type 仂=[U;3];fn 倠(mut 乍:仂,q:F)->仂{乍[q/64]|=1<<(q%64);乍}impl
佅{fn 作(&mut self,丬:&J,乺:P<F>,仹:仂,万:C,)->T{let j=self;let 丁=j.丁;let 乽=match 乺{None=>丬.亙(丁),Some(q)=>G::q(q).丐(),}&j.仾;let 乽=乽.丘(丬.丞(丁));let mut 丣=[G::Q;3];for
一 in 1..=丬.两().乡(){丣[F::from(一)-1]=丬.举(丁,一);}for q in 乽.iter(){let 乚=F::from(丬.两().乡());let Some(与)=丣[..乚].iter().position(|临|临.test(q))else{continue;};let
一=与 as I+1;let 亍=10*C::from(一);if 万+亍>j.丫{continue;}if j.亴>=侤{return false;}j.亴+=1;let mut 丂=丬.clone();let 三=K::乘{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(三){continue;}j.path.push(三);let
人=倠(仹,q);j.俸(&丂,人,万+亍);let 俖=j.path.len()<j.亯&&丂.丗().is_none();let 俠=!俖||j.作(&丂,Some(q),人,万+亍);j.path.pop();if!俠{return false;}}true}fn 俸(&mut self,为:&J,人:仂,万:C){let
j=self;let 仏=1-j.丁;let 伩=佥(为,仏);let 丌=j.並.丌-伩.丌;let 七=j.並.七-伩.七;if 丌<=0&&七<=0{return;}let 乑=j.並.乑-伩.乑;let 且=为.且(j.丁)-j.且;let 倣=为.丗()==Some(义::乪(j.丁));let
丙=if 倣{C::MAX/2}else{3*丌+七+10*乑+2*且-万};if let Some(&主)=j.index.get(&人){if 万>=j.丨[主].万{return;}}let 丟=仟{丅:j.path.clone(),万,丌,七,乑,且,丙,};match j.index.get(&人){Some(&主)=>j.丨[主]=丟,None=>{j.index.insert(人,j.丨.len());j.丨.push(丟);}}}}fn
佚(l:&J,亯:F,丫:C,乂:F)->L<仟>{乎(l,亯,丫,乂).0}fn 乎(l:&J,亯:F,丫:C,乂:F)->(L<仟>,F){if l.丗().is_some()||亯==0||乂==0{return(L::new(),0);}let 丁=l.丝();let 仏=1-丁;let mut
伮=佅{丁,仾:l.丞(仏)|l.丞(仏).丐(),並:佥(l,仏),且:l.且(丁),亯,丫,亴:0,path:L::new(),丨:L::new(),index:HashMap::default(),};伮.作(l,None,[0;3],0);let 亴=伮.亴;let mut 丨=伮.丨;丨.sort_by(|a,b|b.丙.cmp(&a.丙).then(a.万.cmp(&b.万)));丨.truncate(乂);(丨,亴)}const
仢:C=1000000;const 侥:C=200;fn 乏(l:&J,z:F)->C{let 丄=l.丄(z);let 七:C=(l.七(z)&丄).iter().filter_map(|q|l.互(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.且(z)+4*C::from(丄.count())+七+10*C::from((l.乓()&丄).count())}fn
佪(l:&J,z:F,乛:G)->C{俞(l,z,佘(l,z,乛,Z[1-z]),)}fn 俞(l:&J,z:F,万:C)->C{if 万==争{return C::MIN;}l.丈(z)+l.且(z)-万}fn 佷(仱:C)->C{仱.clamp(-侥,0)}fn 俜(l:&J,z:F,乛:[G;2],)->(C,[C;2]){let
丩=1-z;match l.丗(){Some(义::乪(借))=>{return(if 借==z{仢}else{-仢},[0,0])}Some(义::伺)=>return(0,[0,0]),None=>{}}let mut 丙=乏(l,z)-乏(l,丩);if z==l.丝(){let mut 丂=l.clone();丂.仇();if
丂.丗().is_none()&&众(&丂,丩,true,Z[z])<=丂.丈(丩){丙-=仢/2;}}let 亮=[佪(l,z,乛[0]),佪(l,丩,乛[1]),];(丙+佷(亮[0])-佷(亮[1]),亮)}
