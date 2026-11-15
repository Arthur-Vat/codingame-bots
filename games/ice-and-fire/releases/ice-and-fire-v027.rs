// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 並=u32;type U=u64;type 仝=i32;type C=i64;type 亟=f64;type Q=bool;type 串=String;type 乖=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 依:Q=true;fn 俨(l:&J,q:F,付:C,丰:G,丨:C)->Q{let
丁=l.丝();let 乹=1-丁;let 佧=依&&丨<=l.丈(乹)+2*l.且(乹).max(0);let 仾=G::q(q)|G::q(q).丐();if!佧&&(仾&丰).is_empty(){return false;}let(x,y)=井(q);let mut 为=l.clone();if!为.丬(K::乳{x,y}){return
false;}if 倖(&为,丁)<付{return true;}佧&&众(&为,乹,false,Z[丁])>丨}const 侸:I=2;const 侤:F=4;const 伸:F=5;#[derive(Clone)]enum 丳{亡(K),伄(L<K>),}impl 丳{fn 丅(&self)->&[K]{match
self{丳::亡(三)=>std::slice::from_ref(三),丳::伄(丅)=>丅,}}}fn 井(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 佢(l:&J,丆:&mut Vec<丳>){丆.clear();if l.丗().is_some(){return;}let
丁=l.丝();let 乚=l.两().乢();let mut 丣=[G::T;4];for 一 in 1..=乚{丣[F::from(一)]=l.举(丁,一);}for 不 in l.于().iter(){let Some((丑,一))=l.仚(不)else{continue;};let 乇=G::q(不).丐()&丣[F::from(一)];for
乱 in 乇.iter(){let(x,y)=井(乱);丆.push(丳::亡(K::乵{丑,x,y}));}}let 丈=l.丈(丁);let 丄=l.丄(丁);let 亲=l.亚(丁);let 倝=match 侸{0=>亲,1=>{let 伎=l.map().valid.丘(丄).丐()&丄;亲.丘(丄)|伎}_=>亲.丘(丄),};for
一 in 1..=乚{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=倝&丣[F::from(一)];for 俬 in 1..一{丌=丌.丘(丣[F::from(俬)]);}for q in 丌.iter(){let(x,y)=井(q);丆.push(丳::亡(K::乐{一,x,y}));}}if
丈>=了{let 乭=l.丞(丁)&亲&l.产();if l.两().令(){for q in(乭&l.map().丱).iter(){let(x,y)=井(q);丆.push(丳::亡(K::乲{x,y}));}}if l.两().以(){let 伎=l.map().valid.丘(l.丞(丁)).丐();let
丌=(乭&伎).丘(l.map().丱);let 丰=丰(l,1-丁);let 付=C::from((丰&l.丄(丁)).count());let 丨=众(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俨(l,q,付,丰,丨)){let(x,y)=井(q);丆.push(丳::亡(K::乳{x,y}));}}}for
丟 in 佚(l,伸,丈,侤){if 丟.丅.len()>1{丆.push(丳::伄(丟.丅));}}if let Some(丨)=伒(l,Z[1-丁],丈){if 丨.丅.len()>1{丆.push(丳::伄(丨.丅));}}}const 侰:I=2;const 係:C=5;fn 乛(l:&J,z:F)->G{伨(l,z,侰)}fn
伨(l:&J,z:F,不:I)->G{let mut 乛=G::T;if 不>3{return 乛;}for q in(l.七(z)&l.丄(z)).iter(){if l.五(q).is_some_and(|一|一>=不){乛|=G::q(q);}}乛}fn 丰(l:&J,乏:F)->G{let 中=l.丈(乏)+l.且(乏);俼(l,乏,乛(l,乏),中)}fn
今(l:&J,z:F)->G{丰(l,1-z)&l.丄(z)}fn 倖(l:&J,z:F)->C{C::from(今(l,z).count())}const 佀:C=4;fn 俯(丌:C)->C{丌/佀}fn 丠(l:&J,z:F,今:G)->C{let 丄=l.丄(z);let 丠=(l.丠()&丄).丘(今);let
丌=C::from(丄.count());let 俔=if l.乡()<=侔{丌/侓+侒}else{俯(丌)};俔*C::from(丠.count())}const 侔:並=15;const 侒:C=6;const 侓:C=佀;const 侾:C=2;fn 侌(l:&J,z:F)->C{let 丄=l.丄(z);let[a,b,c]=丄.任();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 与=(hash as F)&(佈-1);俊.with(|伐|{let mut 伐=伐.borrow_mut();let
仸=&mut 伐[与];if 仸.0==丄&&仸.1==z as I+1{return 仸.2;}let 东=倣(丄,Z[z]);*仸=(丄,z as I+1,东);东})}const 佈:F=1<<12;thread_local!{static 俊:std::cell::RefCell<L<(G,I,C)>>
=std::cell::RefCell::new(vec![(G::T,0,0);佈]);}fn 倣(丄:G,买:F)->C{struct 什{丄:G,乆:[I;M],丵:[I;M],size:[I;M],互:I,乽:C,仳:C,}fn 亝(d:&mut 什,v:F)->I{d.互+=1;let 乆=d.互;d.乆[v]=乆;let
mut 丵=乆;let mut 伱=1u8;let mut 丟=0u8;for w in(G::q(v).丐()&d.丄).iter(){if d.乆[w]==0{let 侄=亝(d,w);伱+=侄;丵=丵.min(d.丵[w]);if d.丵[w]>=乆{丟+=侄;}}else{丵=丵.min(d.乆[w]);}}d.丵[v]=丵;d.size[v]=伱;if
丟>0&&d.乆[v]>1{d.乽+=1;d.仳=d.仳.max(C::from(丟));}伱}if!丄.test(买){return 0;}let mut d=什{丄,乆:[0;M],丵:[0;M],size:[0;M],互:0,乽:0,仳:0,};亝(&mut d,买);d.乽+d.仳}fn 倃(l:&J,z:F)->C{侾*(侌(l,1-z)-侌(l,z))}fn
佛(l:&J,z:F)->C{if l.丈(z)+l.且(z)>=0{return 0;}(l.七(z)&l.丄(z)).iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum()}const 侣:C=15;const 俈:[C;2]=[3,1];const
俆:[C;2]=[4,20];fn 倒(l:&J,z:F)->[C;2]{let mut 丒=[0;2];for q in(l.七(z)&l.丄(z)).iter(){match l.五(q){Some(2)=>丒[0]+=1,Some(3)=>丒[1]+=1,_=>{}}}丒}fn 侃(l:&J,z:F)->C{let[侉,倗]=倒(l,z);let
低=|一:F|(3-俈[一])*俆[一];-侣*侉+低(0)*侉+低(1)*倗}const 侜:C=0;const 侟:C=2;const 侞:C=100;fn 侁(仲:C)->C{if 仲<0{return 0;}侜+侟*仲.min(侞)/2}const 伿:I=2;const 伽:C=2;const
似:C=4;const 侠:I=3;fn 乼(l:&J,z:F)->C{if 伽==0{return 0;}let 乕=l.map().valid;let mut 七=G::T;for q in(l.七(z)&l.丄(z)).iter(){if l.五(q).is_some_and(|一|一<=侠){七|=G::q(q);}}let
mut 仺=乕.丘(l.丞(z));let mut 伙=G::T;let mut 乇=0;for 丼 in 1..=似{仺=(仺|仺.丐())&乕;let 仫=(七&仺).丘(伙);乇+=(丼-1)*C::from(仫.count());伙|=仫;}乇+=似*C::from(七.丘(伙).count());伽*乇}fn
伛(l:&J,z:F)->C{let 乛=[伨(l,z,伿),伨(l,1-z,伿),];let(丙,亯)=信(l,z,乛);let 丙=丙+侁(亯[0])-侁(亯[1]);if l.丗().is_some(){return 丙;}let 仐=今(l,z);let 亄=今(l,1-z);let 付=C::from(亄.count())-C::from(仐.count());丙-2*佛(l,z)+2*佛(l,1-z)+倃(l,z)+侃(l,z)-侃(l,1-z)-乼(l,z)+乼(l,1-z)+係*付/4+丠(l,z,仐)-丠(l,1-z,亄)}const
侭:F=8;const 侹:F=3;const 侥:F=32;const 侺:U=300;const 侑:亟=0.75;const 侙:P<並> =Some(7);const 侗:Q=false;const 侖:C=10;const 侘:並=6;struct 予{亃:P<乖>,亘:P<乖>,仍:U,}#[derive(Default)]struct
但{丅:L<K>,伖:F,乺:U,世:F,丯:F,体:F,乻:F,乯:Q,}const 俉:Q=true;#[derive(Clone)]struct 乘{丅:L<K>,会:C,}struct 亢{丅:L<K>,end:J,first:P<U>,专:P<L<乘>>,}type 仁=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
伆{丁:F,limit:予,乺:U,仌:仁,些:P<L<丳>>,}const 伷:Q=true;const 侎:Q=false;const 伶:P<I> =None;fn 俒(l:&J,世:&[乘],丆:&mut Vec<丳>){if!伷&&伶.is_none(){return;}let 他=世.iter().map(|乜|乜.丅.as_slice());俓(l,他,(伷,侎,伶),丆,);}fn
俓<'a>(l:&J,他:impl Iterator<Item=&'a[K]>,(乔,丐,一):(Q,Q,P<I>),丆:&mut Vec<丳>,){let 丁=l.丝();let 丷=l.丄(丁);let mut 仈=G::T;for 乜 in 他{for 三 in 乜{if let K::乐{x,y,..}|K::乵{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丷.test(q){仈|=G::q(q);}}}}if 仈.is_empty(){return;}let 乭=l.产()&丷;let 丈=l.丈(丁);let 佸=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丅()==[三]){丆.push(丳::亡(三));}};if
乔&&丈>=了&&l.两().以(){let mut 丌=仈;if 丐{丌|=仈.丐();}for q in(丌&乭).丘(l.map().丱).iter(){let(x,y)=井(q);佸(K::乳{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().乢()){if
丈>=10*C::from(一){for q in(仈&乭).iter(){let(x,y)=井(q);佸(K::乐{一,x,y},丆);}}}}fn 伍(l:&mut J,丅:&[K])->Q{丅.iter().all(|&三|l.丬(三))}fn 乍(丅:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丅{let(仮,a,b,c)=match*三{K::乵{丑,x,y}=>(1,U::from(丑),x,y),K::乐{一,x,y}=>(2,U::from(一),x,y),K::乲{x,y}=>(3,0,x,y),K::乳{x,y}=>(4,0,x,y),};let 亿=仮|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亿).wrapping_mul(0x0100000001b3);}hash}fn
佋(乙:&J,乜:&[K])->J{let mut 丂=乙.clone();丂.仇();for&三 in 乜{丂.丬(三);}丂.仇();丂}fn 仰(l:&J)->P<L<K>>{let 丁=l.丝();let 丏=Z[1-丁];let 亾=|両:&J|両.丗()==Some(义::乫(丁));for
q in l.于().iter(){if l.map().丼(q,丏)!=1{continue;}let Some((丑,_))=l.仚(q)else{continue;};let(x,y)=井(丏);let 三=K::乵{丑,x,y};let mut 両=l.clone();if 両.丬(三)&&亾(&両){return
Some(vec![三]);}}let 丨=伒(l,丏,l.丈(丁))?;let mut 両=l.clone();(伍(&mut 両,&丨.丅)&&亾(&両)).then_some(丨.丅)}impl 伆{fn 乧(&self)->Q{self.乺>=self.limit.仍||self.limit.亃.is_some_and(|t|乖::now()>=t)}fn
丙(&mut self,乙:&J,世:&[乘],伏:C)->C{self.乺+=1;self.伬(乙,世,伏)}fn 伬(&self,乙:&J,世:&[乘],伏:C)->C{if 乙.丗()==Some(义::乫(self.丁)){return 代;}if 世.is_empty(){return 伛(乙,self.丁);}let
mut 仼=C::MAX;for 乜 in 世{仼=仼.min(伛(&佋(乙,&乜.丅),self.丁));if 仼<=伏{break;}}仼}#[allow(clippy::too_many_arguments)]fn 俶(&mut self,乬:&J,亦:C,乑:&[丳],亶:&[(C,F)],丽:&mut[Q],世:&[乘],亨:&mut
仁,伢:Q,)->(P<(F,J,C)>,Q){let mut 下=None;let mut 仦=亦;for&(仴,与)in 亶{if 丽[与]{continue;}if 伢&&仴!=C::MAX&&仴<=仦-亦{return(下,true);}if self.乧(){return(下,false);}丽[与]=true;let
丅=乑[与].丅();let mut 丂=乬.clone();if!伍(&mut 丂,丅){亨.insert(乍(丅),C::MIN/4);continue;}let 丙=self.丙(&丂,世,仦);亨.insert(乍(丅),丙-亦);if 丙>仦{仦=丙;下=Some((与,丂,丙));}}(下,false)}fn
伕(&mut self,买:&J,世:&[乘],仃:&[U],俗:Q)->亢{let j=self;let mut 之=None;let mut 乬=买.clone();let mut 丅=L::new();let mut 亦=j.丙(&乬,世,C::MIN);let mut 亨=仁::default();let
mut 乑=L::new();let mut 亶=L::new();while 乬.丗().is_none()&&!j.乧(){if 丅.is_empty()&&俗{match&j.些{Some(些)=>乑.clone_from(些),None=>{佢(&乬,&mut 乑);j.些=Some(乑.clone());}}}else{佢(&乬,&mut
乑);}俒(&乬,世,&mut 乑);亶.clear();for(与,乶)in 乑.iter().enumerate(){let k=乍(乶.丅());let 仴=亨.get(&k).or(j.仌.get(&k)).copied();亶.push((仴.unwrap_or(C::MAX),与));}亶.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丽=vec![false;乑.len()];if 丅.is_empty(){for(与,乶)in 乑.iter().enumerate(){丽[与]=仃.contains(&乍(乶.丅()));}}let mut 下=None;for
伢 in[true,false]{let(丩,個)=j.俶(&乬,亦,&乑,&亶,&mut 丽,世,&mut 亨,伢,);下=丩;if 下.is_some()||!個{break;}}let Some((与,丂,丙))=下 else{break;};if 丅.is_empty(){之=Some(乍(乑[与].丅()));}丅.extend_from_slice(乑[与].丅());乬=丂;亦=丙;}for(k,乮)in
亨{let 佤=j.仌.entry(k).or_insert(乮);*佤=(*佤).max(乮);}亢{丅,end:乬,first:之,专:None,}}fn 他(&self,乙:&J)->L<L<K>>{let mut 丂=乙.clone();丂.仇();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 乣=丂.丝();let 丈=丂.丈(乣);if let Some(丨)=伒(&丂,Z[self.丁],丈){丆.push(丨.丅);}for 丟 in 佚(&丂,伸,丈,侹){丆.push(丟.丅);}let mut 佣=L::new();俧(&丂,&mut
佣);丆.push(佣);let mut 亄=伆{丁:乣,limit:予{仍:侺,..self.limit},乺:0,仌:仁::default(),些:None,};丆.push(亄.伕(&丂,&[],&[],true).丅);丆.retain(|丅|!丅.is_empty());丆}fn 佌(&self,乙:&J)->L<乘>{self.他(乙).into_iter().map(|丅|{let
会=伛(&佋(乙,&丅),self.丁);乘{丅,会}}).collect()}fn 乻(&mut self,买:&J,乶:&丳,世:&[乘])->P<亢>{let mut 为=买.clone();if!伍(&mut 为,乶.丅()){return None;}self.乺+=1;let 九=self.伕(&为,世,&[],false);let
mut 丅=乶.丅().to_vec();丅.extend_from_slice(&九.丅);Some(亢{丅,end:九.end,first:Some(乍(乶.丅())),专:None,})}fn 佟(&self,丕:&亢,世:&[乘])->C{let 丙=self.伬(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>丙.min(self.伬(&丕.end,专,C::MIN)),_=>丙,}}}fn
俾(丩:&[乘],世:&mut Vec<乘>)->Q{let 丧:L<_> =世.iter().map(|r|r.丅.clone()).collect();for 乜 in 丩{if 世.iter().any(|r|r.丅==乜.丅){continue;}世.push(乜.clone());}世.sort_by_key(|乜|乜.会);世.truncate(侭);世.iter().map(|r|&r.丅).ne(&丧)}fn
倕(l:&J,乂:予,乯:&mut 争)->但{let 丁=l.丝();let mut 丫=但::default();if l.丗().is_some(){return 丫;}if let Some(丅)=仰(l){丫.丅=丅;return 丫;}let 主=|倅:亟|match(乂.亃,乂.亘){(Some(亃),Some(亘))=>Some(亃+(亘-亃).mul_f64(倅)),_=>None,};if
侙.is_some_and(|上|l.乡()>=上){乯.end();}if 乯.俹(l){let 丅=乯.上(l,主(0.4));let 俖=丅.iter().any(|三|{matches!(三,K::乐{..}|K::乲{..}|K::乳{..})});let 乼=侗&&!俖&&l.乡()>=侘&&l.丈(丁)>=侖;if!乼{丫.乯=true;丫.丅=丅;return
丫;}乯.end();}let mut 乎=伆{丁,limit:予{亃:主(侑),..乂},乺:0,仌:仁::default(),些:None,};let mut 世=L::new();let mut 丯:L<亢> =L::new();let mut 仃=L::new();let mut 仩=false;let
mut 乻:L<丳> =L::new();while 丫.伖<侥&&(丯.is_empty()||!乎.乧()){let 丕=if 仩{let Some(乶)=乻.pop()else{break;};match 乎.乻(l,&乶,&世){Some(丕)=>丕,None=>continue,}}else{乎.伕(l,&世,&仃,true)};丫.伖+=1;let
佽=丯.iter().any(|p|p.丅==丕.丅);let 之=丕.first;let 侂=乎.乧();let mut 丕=丕;let mut 佮=false;if!侂&&!佽{let 丩=乎.佌(&丕.end);佮=俾(&丩,&mut 世);丕.专=Some(丩);}if!佽{丯.push(丕);}if
侂{break;}if!佮&&!仩{match 之{Some(之)=>仃.push(之),None=>{仩=true;乻=乎.些.clone().unwrap_or_default();乻.retain(|a|!仃.contains(&乍(a.丅())));let 乮=|a:&丳|{乎.仌.get(&乍(a.丅())).copied().unwrap_or(C::MIN)};乻.sort_by_key(乮);}}}}丫.乻=F::from(仩);丫.乺=乎.乺;丫.世=世.len();丫.丯=丯.len();let
佩=||乂.亘.is_some_and(|t|乖::now()>=t);let mut 亹:L<P<C>> =vec![None;丯.len()];for 与 in(0..丯.len()).rev(){if 与+1<丯.len()&&佩(){break;}亹[与]=Some(乎.佟(&丯[与],&世));}let
亷=|亹:&[P<C>]|{let mut 下:P<(C,F)> =None;for(与,丙)in 亹.iter().enumerate().rev(){if let Some(丙)=*丙{if 下.is_none_or(|(亅,_)|丙>亅){下=Some((丙,与));}}}下};let mut
下=亷(&亹);乎.limit.亃=乂.亘;while let Some((_,与))=下{if!俉||丯[与].专.is_some()||佩(){break;}let 丩=乎.佌(&丯[与].end);丯[与].专=Some(丩);丫.体+=1;亹[与]=Some(乎.佟(&丯[与],&世));下=亷(&亹);}if
let Some((_,与))=下{let mut 丮=l.clone();for&三 in&丯[与].丅{if 丮.丬(三){丫.丅.push(三);}}}丫}fn 俸(乂:&dyn Fn(乖,並)->予,mut 乯:争,伡:impl BufRead,仵:impl Write,)->Result<(),串>{乀(伡,仵,|京,乞,上,亜|match
J::read(京,乞,乩::伀,上){Ok(l)=>{let 丫=倕(&l,乂(亜,上),&mut 乯);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,丫.伖,丫.乺,丫.世,丫.体,丫.乻,亜.elapsed());俏(&丫.丅)}Err(亗)=>{eprintln!("plan: cannot read the input: {}",亗);"WAIT".to_string()}},)}const
侏:亟=45.0;const 侕:亟=800.0;const 便:亟=0.6;fn 乂(亜:乖,上:並)->予{if let Some(仍)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
予{亃:None,亘:None,仍,};}let 价=倘().min(100.0);let 俱=if 上==1{侕}else{侏};let 中=Duration::from_secs_f64(俱/1000.0*价);予{亃:Some(亜+中.mul_f64(便)),亘:Some(亜+中),仍:U::MAX,}}const
侷:F=9;const 侱:亠=亠{丌:3,且:1,丈:1,丟:2,乽:2,仙:2,丰:6,仛:1,};const 侶:I=I::MAX;const 侳:C=2;const 侴:C=18;const 侲:C=0;const 侵:F=32;fn 乯()->争{俴(侵)}fn 俴(乨:F)->争{let
亂=亣::仢{乀:侷,乨,乓:侱,};let 丛=侽.iter().map(|&乥|Y{伯:侶,仄:侳,伟:侴,伋:侲,..乥}).collect();争::new(亂,丛,Y::伈)}fn main(){let 倏=io::stdin();let 倐=io::stdout();if let Err(亗)=俸(&乂,乯(),倏.lock(),倐.lock()){eprintln!("plan: {}",亗);std::process::exit(1);}}const
促:&str="CG_TIME_SCALE";fn 倘()->亟{俵(std::env::var(促).ok().as_deref())}fn 俵(东:P<&str>)->亟{东.and_then(|乞|乞.trim().parse::<亟>().ok()).filter(|价|价.is_finite()&&*价>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乗:F=4*O;const 乪:U=(1<<乗)-1;const 仞:U=1|1<<12|1<<24|1<<36;const 企:U=仞<<11;const 佄:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侍:G=G([乪;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乗]>>(q%乗))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乗]|=1<<(q%乗);}#[inline]fn clear(&mut self,q:F){self.0[q/乗]&=!(1<<(q%乗));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
任(self)->[U;3]{self.0}#[inline]fn count(self)->並{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丘(self,个:G)->G{G([self.0[0]&!个.0[0],self.0[1]&!个.0[1],self.0[2]&!个.0[2],])}#[inline]fn
iter(self)->仿{仿{伥:self.0,主:0,}}#[inline]fn 倊(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&佄)<<(乗-O)),(b>>O)|((c&佄)<<(乗-O)),c>>O,])}#[inline]fn 倇(self)->G{let[a,b,c]=self.0;G([(a<<O)&乪,((b<<O)&乪)|(a>>(乗-O)),((c<<O)&乪)|(b>>(乗-O)),])}#[inline]fn
倈(self)->G{let[a,b,c]=self.0;G([(a>>1)&!企&乪,(b>>1)&!企&乪,(c>>1)&!企&乪,])}#[inline]fn 倉(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仞&乪,(b<<1)&!仞&乪,(c<<1)&!仞&乪,])}#[inline]fn
丐(self)->G{let j=self;j.倊()|j.倇()|j.倈()|j.倉()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,个:G)->G{G([self.0[0]&个.0[0],self.0[1]&个.0[1],self.0[2]&个.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,个:G)->G{G([self.0[0]|个.0[0],self.0[1]|个.0[1],self.0[2]|个.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,个:G){*self=*self|个;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侍.丘(self)}}struct 仿{伥:[U;3],主:F,}impl Iterator for 仿{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.主<3{let 仱=j.伥[j.主];if 仱!=0{j.伥[j.主]=仱&(仱-1);return Some(j.主*乗+仱.trailing_zeros()as F);}j.主+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乵{丑:並,x:I,y:I},乐{一:I,x:I,y:I},乲{x:I,y:I},乳{x:I,y:I},}impl K{fn 倚(&self)->串{match*self{K::乵{丑,x,y}=>format!("MOVE {} {x} {y}",丑),K::乐{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::乲{x,y}=>format!("BUILD MINE {x} {y}"),K::乳{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
俏(丅:&[K])->串{if 丅.is_empty(){return "WAIT".to_string();}let mut line=串::with_capacity(16*丅.len());for(与,三)in 丅.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&三.倚());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乩{佉,倩,俋,#[default]伀,}impl 乩{fn 乢(self)->I{match self{乩::佉=>1,_=>3,}}fn 令(self)->Q{matches!(self,乩::俋|乩::伀)}fn 以(self)->Q{self==乩::伀}}const 习:I=I::MAX;struct
乴{丹:G,valid:G,丱:G,丼:Box<[I]>,丐:[[[I;4];M];2],}thread_local!{static 侢:RefCell<P<Arc<乴>>> =const{RefCell::new(None)};}impl 乴{fn 倆(丹:G,丱:G)->Arc<乴>{侢.with(|乾|{let
mut 乾=乾.borrow_mut();if let Some(亮)=乾.as_ref(){if 亮.丹==丹&&亮.丱==丱{return Arc::clone(亮);}}let 亮=Arc::new(乴::build(丹,丱));*乾=Some(Arc::clone(&亮));亮})}fn build(丹:G,丱:G)->乴{let
乕=!丹;const 俀:[[(仝,仝);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丐=[[[习;4];M];2];for(z,乇)in 俀.iter().enumerate(){for q in
乕.iter(){let(x,y)=((q%O)as 仝,(q/O)as 仝);let mut 丒=0;for&(俞,俟)in 乇{let(佰,佱)=(x+俞,y+俟);if(0..O as 仝).contains(&佰)&&(0..O as 仝).contains(&佱){let 丂=佱 as F*O+佰
as F;if 乕.test(丂){丐[z][q][丒]=丂 as I;丒+=1;}}}}}let mut 丼=vec![习;M*M].into_boxed_slice();let mut 伫=[0u8;M];for 不 in 乕.iter(){let 亐=&mut 丼[不*M..(不+1)*M];亐[不]=0;伫[0]=不
as I;let(mut 伝,mut 伳)=(0,1);while 伝<伳{let q=伫[伝]as F;伝+=1;for&丂 in&丐[0][q]{if 丂!=习&&亐[丂 as F]==习{亐[丂 as F]=亐[q]+1;伫[伳]=丂;伳+=1;}}}}乴{丹,valid:乕,丱,丼,丐,}}#[inline]fn
丼(&self,不:F,乱:F)->I{self.丼[不*M+乱]}}const 侬:並=100;const 了:C=15;const 俇:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 义{乫(F),伺,}enum
也{伅,佁,佂,}#[derive(Clone)]struct J{map:Arc<乴>,两:乩,专:[G;2],丄:[G;2],丷:G,么:G,七:[G;2],一:[G;3],亍:G,亽:[並;M],丈:[C;2],且:[C;2],丝:F,乡:並,亳:並,丗:P<义>,}fn 介(专:G,享:F)->G{if!专.test(享){return
G::T;}let mut 仕=G::q(享);loop{let 丂=(仕|仕.丐())&专;if 丂==仕{return 仕;}仕=丂;}}impl J{fn 丝(&self)->F{self.丝}fn 乡(&self)->並{self.乡}fn 丗(&self)->P<义>{self.丗}fn 两(&self)->乩{self.两}fn
亱(&self,z:F)->C{let j=self;let 七=j.七[z];j.丈[z]+10*C::from((七&j.一[0]).count())+20*C::from((七&j.一[1]).count())+30*C::from((七&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伴(&self,z:F)->F{self.七[z].count()as F}fn map(&self)->&乴{&self.map}fn 丞(&self,z:F)->G{self.专[z]}fn 丄(&self,z:F)->G{self.丄[z]}fn
七(&self,z:F)->G{self.七[z]}fn 仚(&self,q:F)->P<(並,I)>{let j=self;(j.七[0]|j.七[1]).test(q).then(||(j.亽[q],j.伤(q)))}fn 于(&self)->G{self.七[self.丝]&self.亍}fn
丠(&self)->G{self.丷}fn 乔(&self)->G{self.么}fn 五(&self,q:F)->P<I>{(q<M&&(self.七[0]|self.七[1]).test(q)).then(||self.伤(q))}fn 倎(&mut self,z:F){let j=self;j.丄=[介(j.专[0],Z[0]),介(j.专[1],Z[1])];let
俚=j.七[z].丘(j.丄[z]);j.伭(俚);j.佔();j.丈[z]+=j.且[z];if j.丈[z]<0{j.丈[z]=0;j.伭(j.七[z]);}j.亍|=j.七[z];}fn 佔(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
七=丄&j.七[z];let mut 且=C::from(丄.count())+4*C::from((丄&j.丷).count());for(一,倡)in 俇.iter().enumerate(){且-=倡*C::from((七&j.一[一]).count());}j.且[z]=且;}}fn 仇(&mut
self){let j=self;if j.丗.is_some(){return;}if j.丝==1{if j.乡>=侬{j.丗=Some(match j.亱(0).cmp(&j.亱(1)){std::cmp::Ordering::Greater=>义::乫(0),std::cmp::Ordering::Less=>义::乫(1),std::cmp::Ordering::Equal=>义::伺,});return;}j.乡+=1;}j.丝=1-j.丝;j.倎(j.丝);}fn
佶(&mut self,丅:&[K]){let j=self;if j.丗.is_some(){return;}let 伜=j.亳;for&三 in 丅{if j.保(三,伜){continue;}j.丬(三);if j.丗.is_some(){return;}}j.仇();}fn 保(&self,三:K,伜:並)->Q{let
j=self;match 三{K::乵{丑,x,y}=>{j.亥(x,y).is_none()||丑>=伜||j.你(j.丝,丑).is_none()}K::乐{一,x,y}=>{j.亥(x,y).is_none()||一==0||一>j.两.乢()}K::乲{x,y}|K::乳{x,y}=>{j.亥(x,y).is_none()}}}#[inline]fn
亥(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丹.test(y*O+x)).then_some(y*O+x)}fn 你(&self,z:F,丑:並)->P<F>{self.七[z].iter().find(|&q|self.亽[q]==丑)}#[inline]fn
伤(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 俑(&self,乣:F)->G{let 乔=self.么&self.专[乣];let 丄=self.丄[乣];乔|(丄&(丄&乔).丐())}fn
举(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乣=1-z;let 七=j.七[0]|j.七[1];let 俠=j.产();let 借=match
一{1 if j.两==乩::佉=>j.一[1]|j.一[2],1=>七,2=>j.一[1]|j.一[2],_=>G::T,};let 举=俠|j.专[乣].丘(借);if 一<3{举.丘(j.俑(乣))}else{举}}fn 亚(&self,z:F)->G{self.丄[z]|self.丄[z].丐()}fn
产(&self)->G{let j=self;let 七=j.七[0]|j.七[1];let 俩=G::q(Z[0])|G::q(Z[1]);j.map.valid.丘(七|j.丷|j.么|俩)}fn 丬(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.丝;let 亖=match 三{K::乐{一,x,y}=>j.倜(z,一,x,y),K::乵{丑,x,y}=>j.俰(z,丑,x,y),K::乲{x,y}=>j.build(z,true,x,y),K::乳{x,y}=>j.build(z,false,x,y),};let
Some(亖)=亖 else{return false;};match 亖{也::伅=>{}也::佁=>j.丄[z]=介(j.专[z],Z[z]),也::佂=>{j.丄=[介(j.专[0],Z[0]),介(j.专[1],Z[1])];}}j.佔();if!j.专[0].test(Z[0]){j.丗=Some(义::乫(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(义::乫(0));}true}fn 伭(&mut self,丌:G){let j=self;j.七[0]=j.七[0].丘(丌);j.七[1]=j.七[1].丘(丌);for 一 in&mut j.一{*一=一.丘(丌);}j.亍=j.亍.丘(丌);}fn
侅(&mut self,z:F,q:F)->也{let j=self;j.伭(G::q(q));j.丷.clear(q);j.么.clear(q);let 乣=1-z;if j.专[乣].test(q){j.专[乣].clear(q);j.专[z].set(q);也::佂}else if j.专[z].test(q){也::伅}else{j.专[z].set(q);也::佁}}fn
倜(&mut self,z:F,一:I,x:I,y:I)->P<也>{let j=self;if 一==0||一>j.两.乢(){return None;}let q=j.亥(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亚(z).test(q)||!j.举(z,一).test(q){return
None;}let 亖=j.侅(z,q);j.佴(z,一,q,j.亳);j.亳=j.亳.saturating_add(1);j.丈[z]-=万;Some(亖)}fn 佴(&mut self,z:F,一:I,q:F,丑:並){self.七[z].set(q);self.一[F::from(一)-1].set(q);self.亽[q]=丑;}fn
俰(&mut self,z:F,丑:並,x:I,y:I)->P<也>{let j=self;let 丏=j.亥(x,y)?;let 不=j.你(z,丑)?;if!j.亍.test(不){return None;}let 一=j.伤(不);let 丣=j.举(z,一);let 乁=j.map.丐[z][不];if
乁.contains(&(丏 as I))&&!丣.test(丏){return None;}let mut 下=j.map.丼(不,丏);let mut 仅=None;for&丂 in 乁.iter().take_while(|&&丂|丂!=习){let 丂=F::from(丂);let 丼=j.map.丼(丂,丏);if
丼<下&&丣.test(丂){下=丼;仅=Some(丂);}}let 仅=仅?;let 亖=j.侅(z,仅);j.七[z].clear(不);j.一[F::from(一)-1].clear(不);j.亍.clear(不);j.佴(z,一,仅,丑);Some(亖)}fn build(&mut self,z:F,丷:Q,x:I,y:I)->P<也>{let
j=self;let 俎=if 丷{j.两.令()}else{j.两.以()};let q=j.亥(x,y)?;let 俲=j.map.丱.test(q);if!俎||j.丈[z]<了||俲!=丷||!j.专[z].test(q)||!j.亚(z).test(q)||!j.产().test(q){return
None;}if 丷{j.丷.set(q);}else{j.么.set(q);}j.丈[z]-=了;Some(也::伅)}}const 侧:C=1000000000;const 伂:C=0;const 侮:C=1;const 俄:C=2;impl J{fn read(京:&str,上:&str,两:乩,乡:並,)->Result<J,串>{let
mut 丱=L::new();let mut 严=京.lines();let 丒=优(&mut 严,"mine spot count")?;for _ in 0..丒{let[x,y]=亀(亴(&mut 严,"a mine spot")?,"a mine spot")?;丱.push((仨(x)?,仨(y)?));}佝(严,"the initial input")?;let
mut 严=上.lines();let mut 伞=[0i64;4];for(东,丶)in 伞.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伪]=亀(亴(&mut 严,丶)?,丶)?;if 伪.abs()>侧{return
Err(format!("{}: {} is out of range",丶,伪));}*东=伪;}let mut 丹=G::T;let mut 丌=[G::T;2];let mut 丄=[G::T;2];for y in 0..O{let 亐=亴(&mut 严,"a map row")?.trim();if
亐.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,亐));}for(x,仗)in 亐.bytes().enumerate(){let q=y*O+x;match 仗{b'#'=>丹.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乃=F::from(仗==b'X'||仗==b'x');丌[乃].set(q);if 仗.is_ascii_uppercase(){丄[乃].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仗 as char))}}}}let
俕=优(&mut 严,"building count")?;let mut 乒=L::new();for _ in 0..俕{let 亀=亀(亴(&mut 严,"a building")?,"a building")?;乒.push(亀);}let 伴=优(&mut 严,"unit count")?;let
mut 七=L::new();for _ in 0..伴{let 亀=亀(亴(&mut 严,"a unit")?,"a unit")?;七.push(亀);}佝(严,"the turn's input")?;let 佳=乒.iter().find(|佐|佐[0]==0&&佐[1]==伂).ok_or("the bot's own HQ is not among the buildings")?;let
佦=休(佳[2],佳[3],丹)?;let z=Z.iter().position(|&享|享==佦).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佦))?;let mut 使=G::T;for(x,y)in 丱{if
丹.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}使.set(y*O+x);}let 伉=|乃:F|if 乃==0{z}else{1-z};let mut l=J{map:乴::倆(丹,使),两,专:[G::T;2],丄:[G::T;2],丷:G::T,么:G::T,七:[G::T;2],一:[G::T;3],亍:G::T,亽:[0;M],丈:[0;2],且:[0;2],丝:z,乡,亳:1,丗:None,};for
乃 in 0..2{let 乤=伉(乃);l.专[乤]=丌[乃];l.丄[乤]=丄[乃];l.丈[乤]=伞[2*乃];l.且[乤]=伞[2*乃+1];}let mut 佲=G::T;for[乤,仮,x,y]in 乒{let 乃=佺(乤)?;let q=休(x,y,丹)?;if!丌[乃].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佲.test(q){return Err(format!("two buildings at ({x}, {y})"));}佲.set(q);match 仮{伂
if q==Z[伉(乃)]=>{}伂=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侮=>l.丷.set(q),俄=>l.么.set(q),_=>return Err(format!("unknown building type {}",仮)),}}let
mut 乄=G::T;let mut 仯=0u32;for[乤,丑,一,x,y]in 七{let 乃=佺(乤)?;let q=休(x,y,丹)?;if!丌[乃].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乄.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丑=並::try_from(丑).map_err(|_|format!("unit id {} is out of range",丑))?;if
乄.iter().any(|个|l.亽[个]==丑){return Err(format!("two units have id {}",丑));}乄.set(q);l.七[伉(乃)].set(q);l.一[一 as F-1].set(q);l.亽[q]=丑;仯=仯.max(丑);}if!乄.is_empty(){l.亳=仯.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仯))?;}l.亍=l.七[z];Ok(l)}}fn
亴<'a>(严:&mut Lines<'a>,丶:&str)->Result<&'a str,串>{严.next().ok_or_else(||format!("the text ends before {}",丶))}fn 佝(mut 严:Lines<'_>,丶:&str)->Result<(),串>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丶)),None=>Ok(()),}}fn 亀<const N:F>(line:&str,丶:&str)->Result<[C;N],串>{let
mut 任=line.split_whitespace();let mut 例=[0i64;N];for 东 in&mut 例{let 亿=任.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丶))?;*东=亿.parse().map_err(|_|format!("{}: {:?} is not an integer",丶,亿))?;}if
任.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丶));}Ok(例)}fn 优(严:&mut Lines<'_>,丶:&str)->Result<F,串>{let[丒]=亀(亴(严,丶)?,丶)?;F::try_from(丒).ok().filter(|&丒|丒<=M).ok_or_else(||format!("{}: {} is out of range",丶,丒))}fn
仨(东:C)->Result<F,串>{F::try_from(东).ok().filter(|&东|东<O).ok_or_else(||format!("coordinate {} is off the map",东))}fn 休(x:C,y:C,丹:G)->Result<F,串>{let q=仨(y)?*O+仨(x)?;if
丹.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佺(乤:C)->Result<F,串>{match 乤{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乤)),}}const
供:C=15;const 侯:C=6;const 佇:C=10;const 侼:I=3;#[derive(Clone,Copy)]struct Y{open:C,仧:C,佗:I,传:C,伟:C,佫:C,伯:I,佖:C,何:C,伋:C,亪:C,亻:C,丸:C,亭:C,亓:C,仄:C,}impl Y{const
伈:Y=Y{open:3,仧:2,佗:12,传:4,伟:供,佫:侯,伯:侼,佖:2,何:2,伋:0,亪:0,亻:0,丸:0,亭:0,亓:0,仄:0,};const fn 倧(self,亪:C,亻:C,丸:C)->Y{Y{亪,亻,丸,..self}}const fn 倦(self,亭:C,亓:C)->Y{Y{亭,亓,..self}}}fn
仜(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仆(l:&J,不:G)->[I;M]{let mut 乊=[I::MAX;M];let 乕=l.map().valid;let mut 乌=不&乕;let mut 乄=乌;let mut d=0u8;while!乌.is_empty(){for
q in 乌.iter(){乊[q]=d;}let 丂=(乌.丐()&乕).丘(乄);乄|=丂;乌=丂;d=d.saturating_add(1);}乊}fn 俤(l:&J)->I{let 乊=仆(l,l.丞(0));l.丞(1).iter().map(|q|乊[q]).min().unwrap_or(I::MAX)}struct
亊<'a>{乥:&'a Y,丁:F,theirs:[I;M],亼:[I;M],free:G,乏:G,ours:G,丱:G,七:G,}impl 亊<'_>{fn new<'a>(l:&J,乥:&'a Y)->亊<'a>{let 丁=l.丝();let 丷=仆(l,l.丞(丁));let 亄=仆(l,l.丞(1-丁));let
亼=仆(l,G::q(Z[1-丁]));let 乭=l.map().valid.丘(l.丞(0)|l.丞(1));let mut 乏=G::T;for q in 乭.iter(){if 丷[q]<=亄[q]{乏.set(q);}}亊{乥,丁,theirs:亄,亼,free:乭,乏,ours:l.丞(丁),丱:l.map().丱,七:l.七(丁),}}fn
value(&self,q:F)->C{let j=self;let p=j.乥;let 丿=C::from((G::q(q).丐()&j.free).count());let 佒=p.佗;let 仧=C::from(佒)-C::from(j.theirs[q].min(佒));let 享=if p.传>0{C::from(j.亼[q].min(30))/p.传}else{0};let
专=C::from((G::q(q).丐()&j.ours).count());let line=if 专<=1{p.亻}else{0};let 丸=if j.丱.test(q){p.丸}else{0};let 倓=if 丿==0{p.亭}else{0};let 亓=p.亓*(22-C::from(j.亼[q].min(22)));let
仄=p.仄*C::from((G::q(q).丐()&j.七).count());p.open*丿+p.仧*仧-享+p.亪*专-line+丸-倓+亓-仄}fn 下(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
伔(&mut self,q:F){let j=self;j.free.clear(q);j.乏.clear(q);j.ours.set(q);j.七.set(q);}}fn 仓(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 佊=l.丬(三);if 佊{丆.push(三);}佊}fn
业(l:&mut J,不:F,乱:F,丆:&mut Vec<K>)->Q{let Some((丑,_))=l.仚(不)else{return false;};let(x,y)=仜(乱);仓(l,K::乵{丑,x,y},丆)}fn 倠(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-佇}#[derive(Clone,Copy)]enum
久{仡,亇{丠:I,七:I},}fn 侈(l:&J,乥:&Y,二:久)->L<K>{let mut 丮=l.clone();let mut 上=亊::new(l,乥);let mut 丆=L::new();乿(&mut 丮,&mut 上,&mut 丆);match 二{久::仡=>俻(&mut 丮,&mut
上,&mut 丆),久::亇{丠,七}=>{俢(&mut 丮,&mut 上,丠,七,&mut 丆)}}丆}fn 乿(l:&mut J,上:&mut 亊<'_>,丆:&mut Vec<K>){let 丁=上.丁;let mut 乼:L<F> =L::new();loop{let 丣=l.举(丁,1);let
mut 亷:P<(並,F)> =None;for 不 in l.于().iter(){if 乼.contains(&不){continue;}let 亁=(G::q(不).丐()&上.free&丣).count();if 亁==0{乼.push(不);}else if 亷.is_none_or(|(n,_)|亁<n){亷=Some((亁,不));}}let
Some((_,不))=亷 else{break;};上.七.clear(不);match 上.下(G::q(不).丐()&上.free&丣){Some(乱)if 业(l,不,乱,丆)=>上.伔(乱),_=>{上.七.set(不);乼.push(不);}}}for 不 in 乼{if!l.于().test(不){continue;}let
丏=[上.乏,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().丼(不,q),q))});if let Some(丏)=丏{业(l,不,丏,丆);}}}fn 俻(l:&mut J,上:&mut 亊<'_>,丆:&mut
Vec<K>){let 丁=上.丁;let p=上.乥;let h=(p.伟-C::from(l.乡())).max(p.佫);let 候=(C::from(俤(l))-p.佖).max(0)/p.何.max(1)+p.伋;loop{let 丈=l.丈(丁);let 七=l.伴(丁)as C;let
倀=C::from(上.乏.count());let q=上.下(l.亚(丁)&上.free&l.举(丁,1));let 侊=倠(h,(倀/(七+1)).min(候));let 佑=q.is_some()&&丈>=佇&&侊>0;let 佬=4*h-了;let 丸=佭(l,上);let 俘=丸.is_some()&&丈>=了&&佬>0;if
俘&&(!佑||佬>=侊){let(x,y)=仜(丸.unwrap_or_default());if 仓(l,K::乲{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佑)else{break;};let(x,y)=仜(q);if!仓(l,K::乐{一:1,x,y},丆){break;}上.伔(q);}}fn
佭(l:&J,上:&亊<'_>)->P<F>{let 丁=上.丁;(l.map().丱&l.丄(丁)&l.产()).iter().filter(|&丸|上.theirs[丸]>上.乥.伯).min_by_key(|&丸|(l.map().丼(Z[丁],丸),丸))}fn 俢(l:&mut J,上:&mut
亊<'_>,丠:I,七:I,丆:&mut Vec<K>,){let 丁=上.丁;for _ in 0..丠{let Some(丸)=佭(l,上)else{break;};let(x,y)=仜(丸);if!仓(l,K::乲{x,y},丆){break;}}for _ in 0..七{let Some(q)=上.下(l.亚(丁)&上.free&l.举(丁,1))else{break;};let(x,y)=仜(q);if!仓(l,K::乐{一:1,x,y},丆){break;}上.伔(q);}}fn
件(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.丝();let 丈=l.丈(丁);let 丰=交::new(l,丁,true);l.丞(1-丁).iter().any(|q|丰.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亞:I,二:久,}const 伵:I=I::MAX;const 侐:[久;8]=[久::仡,久::亇{丠:0,七:0},久::亇{丠:0,七:1},久::亇{丠:0,七:2},久::亇{丠:0,七:伵,},久::亇{丠:1,七:0},久::亇{丠:1,七:1},久::亇{丠:1,七:伵,},];struct
伹<'a>{丛:&'a[Y],从:Y,rest:丢,乸:P<乖>,倁:Settings,}impl 伹<'_>{fn 乥(&self,业:丢)->&Y{&self.丛[F::from(业.亞)%self.丛.len()]}fn 丅(&self,l:&J,业:丢)->L<K>{侈(l,self.乥(业),业.二)}}fn
估(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伦=|亿:U|hash=(hash^亿).wrapping_mul(0x0100000001b3);for(倔,临)in[l.丞(丁),l.七(丁),l.丠()].into_iter().enumerate(){for
q in 临.iter(){伦(((倔 as U)<<8)|q as U);}伦(0xff);}伦(l.丈(丁)as U);hash}enum 亣{仢{乀:F,乨:F,乓:亠,},}struct 争{亂:亣,丛:L<Y>,从:Y,丕:L<丢>,仪:L<U>,亸:F,丯:並,仒:Q,}impl 争{fn
new(亂:亣,丛:L<Y>,从:Y)->争{争{亂,丛,从,丕:L::new(),仪:L::new(),亸:0,丯:0,仒:false,}}fn 俹(&mut self,l:&J)->Q{let j=self;let 倌=matches!(j.亂,亣::仢{..});let 俺=!倌&&j.丯>0&&j.亸>=j.仪.len();if!j.仒&&(俺||件(l)){j.仒=true;}!j.仒}fn
end(&mut self){self.仒=true;}fn 上(&mut self,l:&J,乸:P<乖>)->L<K>{let j=self;let 余=伹{丛:&j.丛,从:j.从,rest:丢{亞:0,二:久::仡,},乸,倁:Settings::DEFAULT,};let 丁=l.丝();let
俳=j.仪.get(j.亸)==Some(&估(l,丁));if!俳{let 之=j.丯==0;j.丕=match j.亂{亣::仢{乀,乨,乓,}=>{let 乓=&乓;let 九=侻;let mut 乇=丕(l,&j.丛,九,乀,乨,乸,乓,);乇.resize(乀,九);乇}};j.仪=match
j.亂{亣::仢{..}=>倍(l,&j.丛,&j.丕),};j.亸=0;j.丯+=1;}let 业=j.丕.get(j.亸).copied().unwrap_or(余.rest);j.亸+=1;余.丅(l,业)}}fn 倍(l:&J,丛:&[Y],丕:&[丢])->L<U>{let 丁=l.丝();let
mut 佨=L::with_capacity(丕.len());let mut 书=l.clone();for&业 in 丕{if 书.丗().is_some(){break;}佨.push(估(&书,丁));书=伊(&书,&丅(&书,丛,业));}佨}const 俅:C=3;struct Settings{乀:C,}impl
Settings{const DEFAULT:Settings=Settings{乀:俅};}fn 乇(l:&J,不:G)->[I;M]{let 乕=l.map().valid;let mut 乊=[I::MAX;M];let mut 乌=不&乕;let mut 乄=乌;let mut d=0u8;while!乌.is_empty(){for
q in 乌.iter(){乊[q]=d;}let 丂=(乌.丐()&乕).丘(乄);乄|=丂;乌=丂;d=d.saturating_add(1);}乊}fn 倢(l:&J,z:F)->C{let 仐=乇(l,l.丄(z));let 亄=乇(l,l.丄(1-z));l.map().valid.iter().map(|q|match
仐[q].cmp(&亄[q]){_ if 仐[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 侽:[Y;27]=丛();const
侻:丢=丢{亞:16,二:久::仡,};const fn 丛()->[Y;27]{let 亪=[0,2,4];let 亻=[0,3,6];let 亓=[0,1,2];let mut 丆=[Y::伈;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亭=if g>0||t>0{4}else{0};丆[i]=Y::伈.倧(亪[g],亻[t],0).倦(亭,亓[w]);i+=1;}丆}#[derive(Default)]struct 伇{丌:C,且:C,丈:C,乽:C,丟:C,仙:C,丰:C,仛:C,}#[derive(Clone,Copy)]struct
亠{丌:C,且:C,丈:C,丟:C,乽:C,仙:C,丰:C,仛:C,}fn 倂(l:&J,z:F)->[C;M]{struct 什{丄:G,乆:[u16;M],丵:[u16;M],size:[C;M],丟:[C;M],互:u16,}fn 亝(d:&mut 什,v:F){d.互+=1;d.乆[v]=d.互;d.丵[v]=d.互;d.size[v]=1;for
w in(G::q(v).丐()&d.丄).iter(){if d.乆[w]==0{亝(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.乆[v]{d.丟[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.乆[w]);}}}let
mut d=什{丄:l.丄(z),乆:[0;M],丵:[0;M],size:[0;M],丟:[0;M],互:0,};let 买=Z[z];if d.丄.test(买){亝(&mut d,买);}d.丟[买]=0;d.丟}fn 倄(l:&J,z:F)->伇{let 丄=l.丄(z);let 丟=倂(l,z);let
亼=仆(l,G::q(Z[1-z]));let mut 丆=伇{丌:C::from(丄.count()),且:l.且(z),丈:l.丈(z),丰:C::MAX,仛:倢(l,z)/2,..伇::default()};for q in 丄.iter(){if 丟[q]>0{丆.乽+=1;丆.丟=丆.丟.max(丟[q]);}if
q!=Z[z]&&(G::q(q).丐()&丄).count()<=1{丆.仙+=1;}丆.丰=丆.丰.min(C::from(亼[q]));}丆}fn 东(l:&J,z:F,乓:&亠)->C{let s=倄(l,z);let o=乓;o.丌*s.丌+o.且*s.且+o.丈*s.丈-o.丟*s.丟-o.乽*s.乽-o.仙*s.仙-o.丰*s.丰.min(30)+o.仛*s.仛}fn
伊(l:&J,丅:&[K])->J{let mut 丂=l.clone();丂.佶(丅);if 丂.丗().is_none(){丂.佶(&[]);}丂}fn 丅(l:&J,丛:&[Y],业:丢)->L<K>{侈(l,&丛[F::from(业.亞)%丛.len()],业.二,)}fn 併(l:&J,丛:&[Y],丕:&[丢],九:丢,乀:F,)->J{let
mut 书=l.clone();for 上 in 0..乀{if 书.丗().is_some(){break;}let 业=丕.get(上).copied().unwrap_or(九);书=伊(&书,&丅(&书,丛,业));}书}fn 們(l:&J,丛:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亞 in 0..丛.len().min(F::from(I::MAX)){for&二 in&侐{let 业=丢{亞:亞 as I,二,};let c=丅(l,丛,业);if!丆.iter().any(|(_,个)|*个==c){丆.push((业,c));}}}丆}fn
俐(l:&J,丛:&[Y],九:丢,乀:F,乨:F,乸:P<乖>,乓:&亠,)->L<丢>{let 丁=l.丝();let 仑=||乸.is_some_and(|t|乖::now()>=t);let 仔=|丕:&[丢]|东(&併(l,丛,丕,九,乀),丁,乓);let mut 下:(C,L<丢>)=(仔(&[]),L::new());let
mut 乌:L<(J,L<丢>)> =vec![(l.clone(),L::new())];for 俜 in 0..乀{let mut 乷:L<(C,J,L<丢>)> =L::new();let mut 乄:HashMap<U,F> =HashMap::new();for(佯,丕)in&乌{if 仑(){break;}for(业,c)in
們(佯,丛){let 丂=伊(佯,&c);let mut 伓=丕.clone();伓.push(业);let v=东(&丂,丁,乓);let 乍=估(&丂,丁);match 乄.get(&乍){Some(&主)if 乷[主].0>=v=>{}Some(&主)=>乷[主]=(v,丂,伓),None=>{乄.insert(乍,乷.len());乷.push((v,丂,伓));}}}}if
乷.is_empty(){break;}乷.sort_by(|a,b|b.0 .cmp(&a.0));乷.truncate(乨);let 乾=俜+1==乀||仑();for(i,(_,_,丕))in 乷.iter().take(if 乾{乨}else{1}).enumerate(){if i>0&&仑(){break;}let
v=仔(丕);if v>下.0{下=(v,丕.clone());}}if 乾{break;}乌=乷.into_iter().map(|(_,s,p)|(s,p)).collect();}下.1}fn 丕(l:&J,丛:&[Y],九:丢,乀:F,修:F,乸:P<乖>,乓:&亠,)->L<丢>{let 丁=l.丝();let
仔=|丕:&[丢]|东(&併(l,丛,丕,九,乀),丁,乓);let mut 下:(C,L<丢>)=(仔(&[]),L::new());let mut 乨=2;loop{let 丩=俐(l,丛,九,乀,乨,乸,乓);let v=仔(&丩);if v>下.0{下=(v,丩);}let 仑=乸.is_some_and(|t|乖::now()>=t);if
仑||(乸.is_none()&&乨>=修){break;}乨*=2;}下.1}const 侚:F=4+12;fn 乀(伡:impl BufRead,mut 仵:impl Write,mut 伌:impl FnMut(&str,&str,並,乖)->串,)->Result<(),串>{let mut
严=伾{佹:伡};let Some(之)=严.line()?else{return Ok(());};let mut 京=format!("{}\n",之);严.take(丒(&之,"mine spot count")?,&mut 京)?;let mut 上=0;loop{let Some(之)=严.line()?else{return
Ok(());};let 亜=乖::now();上+=1;let mut 乞=format!("{}\n",之);严.take(侚-1,&mut 乞)?;for 丶 in["building count","unit count"]{let line=严.佻()?;let n=丒(&line,丶)?;乞.push_str(&line);乞.push('\n');严.take(n,&mut
乞)?;}let 伌=伌(&京,&乞,上,亜);writeln!(仵,"{}",伌).and_then(|()|仵.flush()).map_err(|亗|format!("turn {}: cannot write the answer: {}",上,亗))?;}}struct 伾<R>{佹:R,}impl<R:BufRead>伾<R>{fn
line(&mut self)->Result<P<串>,串>{let mut line=串::new();match self.佹.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(亗)=>Err(format!("cannot read input: {}",亗)),}}fn
佻(&mut self)->Result<串,串>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乞:&mut String)->Result<(),串>{for
_ in 0..n{乞.push_str(&self.佻()?);乞.push('\n');}Ok(())}}fn 丒(line:&str,丶:&str)->Result<F,串>{match line.trim().parse::<F>(){Ok(n)if n<=侦=>Ok(n),_=>Err(format!("bad {} {line:?}",丶)),}}const
侦:F=144;const 俁:C=3;const 価:C=2;const 侫:F=16;fn 俷(l:&J,丆:&mut Vec<K>){let z=l.丝();let mut 亂=佃{s:l.clone(),z,乹:1-z,丏:Z[1-z],们:Z[z],上:l.乡(),丆,};亂.run();}struct
佃<'a>{s:J,z:F,乹:F,丏:F,们:F,上:並,丆:&'a mut Vec<K>,}fn 份(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伧(丑:並,q:F)->K{let(x,y)=份(q);K::乵{丑,x,y}}fn 侇(一:I,q:F)->K{let(x,y)=份(q);K::乐{一,x,y}}impl
佃<'_>{fn run(&mut self){let j=self;if j.仰(){return;}j.乿();if j.仰(){return;}j.乒();j.倞();}fn 乊(&self,不:F,乱:F)->C{C::from(self.s.map().丼(不,乱))}fn 丬(&mut self,三:K)->Q{let
丽=self.s.丬(三);if 丽{self.丆.push(三);}丽}fn 亾(&self,両:&J)->Q{両.丗()==Some(义::乫(self.z))}fn 仰(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.于().iter(){if j.乊(q,j.丏)!=1{continue;}let Some((丑,_))=j.s.仚(q)else{continue;};let 三=伧(丑,j.丏);let mut 両=j.s.clone();if 両.丬(三)&&j.亾(&両){return j.丬(三);}}let
Some(丨)=j.俙()else{return false;};let mut 両=j.s.clone();if!丨.iter().all(|&三|両.丬(三))||!j.亾(&両){return false;}for 三 in 丨{j.丬(三);}true}fn 俙(&self)->P<L<K>>{let
j=self;const 伻:C=C::MAX;let 乚=j.s.两().乢();let 丣:L<G> =(1..=乚).map(|一|j.s.举(j.z,一)).collect();let 仉=|q:F|{丣.iter().position(|临|临.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 丄=j.s.丄(j.z);let mut 万=[伻;M];let mut 丧=[F::MAX;M];let mut 丽=[false;M];for q in 丄.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丽[q]&&万[q]<伻&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丽[q]=true;if q==j.丏{break;}for&乁 in&j.s.map().丐[0][q]{if 乁==习{break;}let 乁=F::from(乁);if 丽[乁]||丄.test(乁){continue;}let Some((_,亏))=仉(乁)else{continue;};if
万[q]+亏<万[乁]{万[乁]=万[q]+亏;丧[乁]=q;}}}if 万[j.丏]>j.s.丈(j.z){return None;}let mut 丨=L::new();let mut q=j.丏;while!丄.test(q){let(一,_)=仉(q)?;丨.push(侇(一,q));q=丧[q];}丨.reverse();Some(丨)}fn
乮(&self,为:&J)->C{let(丁,云)=(self.z,self.乹);let 丧=&self.s;let 丒=|临:G|C::from(临.count());let 专=丒(为.丞(丁))-丒(丧.丞(丁));let 仹=丒(丧.丞(云))-丒(为.丞(云));let 俫=丒(丧.丄(云))-丒(为.丄(云));let
仭=丧.亱(云)-为.亱(云);let 丠=丒(丧.丠()&丧.丞(云))-丒(为.丠()&为.丞(云));12*专+20*仹+8*(俫-仹).max(0)+5*仭+25*丠}fn 侀(&self,q:F)->C{let j=self;let 丸=j.s.map().丱.test(q);if 丸&&j.s.两().令()&&!j.s.丞(j.z).test(q){20}else{0}}fn
佞(&self,乂:C)->Q{let j=self;j.s.七(j.乹).iter().any(|q|j.乊(q,j.们)<=乂)}fn 乿(&mut self){let j=self;let mut 七:L<(C,F)> =j.s.于().iter().map(|q|(j.乊(q,j.丏),q)).collect();七.sort_unstable();for(仫,q)in
七{if j.s.丗().is_some(){return;}let Some((丑,一))=j.s.仚(q)else{continue;};let 丣=j.s.举(j.z,一);let 乁=j.s.map().丐[j.z][q];let mut 下:P<(C,F)> =None;for&丂 in 乁.iter().take_while(|&&丂|丂!=习){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.丬(伧(丑,丂)){continue;}let 东=if j.亾(&両){100000}else{j.乮(&両)+10*(仫-j.乊(丂,j.丏))+j.侀(丂)};if 下.is_none_or(|(亅,_)|东>亅){下=Some((东,丂));}}if
let Some((东,丂))=下{if 东>0{j.丬(伧(丑,丂));}}}}fn 乒(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.佞(俁){j.倛();}else{j.丠();}}fn 倛(&mut self){let j=self;if!j.s.两().以()||j.s.丈(j.z)<了{return;}let
乁=j.s.map().丐[j.z][j.们];let 丌:L<F> =乁.iter().take_while(|&&q|q!=习).map(|&q|F::from(q)).collect();let 丷=j.s.丞(j.z);if 丌.iter().any(|&q|丷.test(q)&&j.s.乔().test(q)){return;}for
q in 丌{let(x,y)=份(q);if j.丬(K::乳{x,y}){return;}}}fn 丠(&mut self){let j=self;if!j.s.两().令(){return;}while j.s.丈(j.z)>=了{let 乭=j.s.map().丱&j.s.丞(j.z)&j.s.丄(j.z)&j.s.产();let
Some(q)=乭.iter().min_by_key(|&q|(j.乊(q,j.们),q))else{return;};let(x,y)=份(q);if!j.丬(K::乲{x,y}){return;}}}fn 倞(&mut self){let j=self;let 佼=if j.上<=3{0}else{5};for
_ in 0..侫{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俪=j.佞(価);let 亲=j.s.亚(j.z);let mut 下:P<(C,K)> =None;let mut 伲:P<(C,K)> =None;let
mut 住=G::T;for 一 in 1..=j.s.两().乢(){let 丣=j.s.举(j.z,一);let 亁=(亲&丣).丘(住);住=丣;let 亏=10*C::from(一);if 亏>丈{continue;}for q in 亁.iter(){let 三=侇(一,q);let mut
両=j.s.clone();if!両.丬(三){continue;}let 伣=両.丈(j.z);let 且=両.且(j.z);let 仭=j.s.亱(j.乹)-両.亱(j.乹);let 俌=if 仭>0&&俪{伣+且>=0}else{且>=0&&(伣>=佼||仭>0)};if!俌{continue;}let
乮=j.乮(&両);let 东=乮+j.侀(q)-6*j.乊(q,j.丏)-10*(C::from(一)-1);if 乮>0{if 下.is_none_or(|(亅,_)|东>亅){下=Some((东,三));}}else if 且>=3&&伣>=佼+10&&一==1{if 伲.is_none_or(|(亅,_)|东>亅){伲=Some((东,三));}}}}let
Some((_,三))=下.or(伲)else{return;};if!j.丬(三){return;}}}}fn 俧(l:&J,丆:&mut Vec<K>){丆.clear();俷(l,丆);}const 事:C=C::MAX;const 侪:F=8;struct 交{万:[C;M],丧:[I;M],}fn
众(l:&J,z:F,乿:Q,丏:F)->C{佘(l,z,亙(l,z,乿),丏)}fn 佘(l:&J,z:F,亙:G,丏:F)->C{let mut 万=事;佡(l,z,亙,|业,new|{if new.test(丏){万=10*业 as C;}万!=事});万}fn 俼(l:&J,z:F,亙:G,中:C)->G{let
mut 件=G::T;佡(l,z,亙,|业,new|{if 10*业 as C>中{return true;}件|=new;false});件.丘(l.丄(z))}fn 亙(l:&J,z:F,乿:Q)->G{if 乿&&z==l.丝(){l.于()}else{G::T}}fn 佡(l:&J,z:F,亙:G,mut
亝:impl FnMut(F,G)->Q,){let 乚=l.两().乢();let mut 仉=[G::T;3];let mut 乄=G::T;for 一 in 1..=乚{let 临=l.举(z,一);仉[F::from(一)-1]=临.丘(乄);乄|=临;}let mut 丿=[l.丄(z),G::T,G::T,G::T];for
q in(亙&l.七(z)).iter(){if let Some(一)=l.五(q).filter(|&一|一<=乚){丿[0]|=G::q(q).丐()&l.举(z,一);}}let mut 丽=G::T;for 业 in 0..{let new=丿[业%4].丘(丽);丿[业%4]=G::T;if
亝(业,new){return;}丽|=new;let 丂=new.丐().丘(丽);for(俍,丌)in 仉.iter().enumerate(){丿[(业+俍+1)%4]|=丂&*丌;}if 丿.iter().all(|临|临.丘(丽).is_empty()){break;}}}impl 交{fn
new(l:&J,z:F,乿:Q)->交{let 乚=F::from(l.两().乢());let mut 丣=[G::T;3];for(与,临)in 丣.iter_mut().enumerate().take(乚){*临=l.举(z,与 as I+1);}let 丄=l.丄(z);let 丐=&l.map().丐[0];let
mut 万=[事;M];let mut 丧=[习;M];let mut 丿=[0u8;M];let mut 丒=0;let mut 佾=G::T;for q in 丄.iter(){万[q]=0;丿[丒]=q as I;丒+=1;}if 乿&&z==l.丝(){for 不 in l.于().iter(){let
Some(一)=l.五(不)else{continue;};let 乇=G::q(不).丐()&丣[F::from(一)-1];for q in 乇.iter(){if 万[q]!=0{万[q]=0;丿[丒]=q as I;丒+=1;}}}}while 丒>0{let mut 下=0;for 与 in
1..丒{let(a,b)=(F::from(丿[与]),F::from(丿[下]));if(万[a],a)<(万[b],b){下=与;}}let q=F::from(丿[下]);丒-=1;丿[下]=丿[丒];佾.set(q);for&丂 in&丐[q]{if 丂==习{break;}let 丂=F::from(丂);if
佾.test(丂)||丄.test(丂){continue;}let Some(一)=丣[..乚].iter().position(|临|临.test(丂))else{continue;};let 來=万[q]+10*(一 as C+1);if 來<万[丂]{if 万[丂]==事{丿[丒]=丂 as
I;丒+=1;}万[丂]=來;丧[丂]=q as I;}}}交{万,丧}}fn path(&self,丏:F)->L<F>{let mut path=L::new();if 丏>=M||self.万[丏]==事{return path;}let mut q=丏;while self.丧[q]!=习{path.push(q);q=F::from(self.丧[q]);}path.reverse();path}}struct
仟{丅:L<K>,万:C,}fn 俭(l:&J,z:F,q:F)->P<I>{(1..=l.两().乢()).find(|&一|l.举(z,一).test(q))}fn 俿(丮:&mut J,丁:F,亩:F,中:C,丅:&mut Vec<K>,乧:&mut i64,)->Q{loop{if 丮.丗()==Some(义::乫(丁)){return
true;}if 丮.丞(丁).test(亩)&&丮.丄(丁).test(亩){return true;}let 丰=交::new(丮,丁,false);let 万=丰.万[亩];if 万==事||*乧+万>中{return false;}let Some(&q)=丰.path(亩).first()else{return
false;};let Some(一)=俭(丮,丁,q)else{return false;};let 三=K::乐{一,x:(q%O)as I,y:(q/O)as I,};if!丮.丬(三){return false;}*乧+=10*C::from(一);丅.push(三);}}fn 位(l:&J,丁:F,俦:&[F],中:C)->P<仟>{let
mut 丮=l.clone();let mut 丅=L::new();let mut 乧=0;for&亩 in 俦{if!俿(&mut 丮,丁,亩,中,&mut 丅,&mut 乧){return None;}}(乧<=中).then_some(仟{丅,万:乧,})}fn 伒(l:&J,丏:F,中:C)->P<仟>{if
中<0||l.丗().is_some()||丏>=M||l.map().丹.test(丏){return None;}let 丁=l.丝();if l.丞(丁).test(丏)&&l.丄(丁).test(丏){return Some(仟{丅:L::new(),万:0,});}let mut 下=位(l,丁,&[丏],中);let
之=交::new(l,丁,false);let mut 乔:L<F> =(l.乔()&l.丞(1-丁)).iter().filter(|&么|么!=丏&&之.万[么]!=事).collect();乔.sort_by_key(|&么|(l.map().丼(么,丏),么));乔.truncate(侪);for
么 in 乔{if let Some(丨)=位(l,丁,&[么,丏],中){if 下.as_ref().is_none_or(|亅|丨.万<亅.万){下=Some(丨);}}}下}const 侨:F=20000;struct 仠{丅:L<K>,万:C,丌:C,七:C,乒:C,且:C,丙:C,}struct
伃{丌:C,七:C,乒:C,}fn 佥(l:&J,z:F)->伃{let 丄=l.丄(z);let 七=l.七(z)&丄;let 东=七.iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum();伃{丌:C::from(丄.count()),七:东,乒:C::from(((l.丠()|l.乔())&丄).count()),}}struct
佅{丁:F,仾:G,丧:伃,且:C,亰:F,中:C,亵:F,path:L<K>,丩:L<仠>,index:HashMap<仂,F,BuildHasherDefault<DefaultHasher>>,}type 仂=[U;3];fn 倥(mut 乍:仂,q:F)->仂{乍[q/64]|=1<<(q%64);乍}impl
佅{fn 作(&mut self,丮:&J,乾:P<F>,仹:仂,万:C,)->Q{let j=self;let 丁=j.丁;let 亁=match 乾{None=>丮.亚(丁),Some(q)=>G::q(q).丐(),}&j.仾;let 亁=亁.丘(丮.丞(丁));let mut 丣=[G::T;3];for
一 in 1..=丮.两().乢(){丣[F::from(一)-1]=丮.举(丁,一);}for q in 亁.iter(){let 乚=F::from(丮.两().乢());let Some(与)=丣[..乚].iter().position(|临|临.test(q))else{continue;};let
一=与 as I+1;let 亏=10*C::from(一);if 万+亏>j.中{continue;}if j.亵>=侨{return false;}j.亵+=1;let mut 丂=丮.clone();let 三=K::乐{一,x:(q%O)as I,y:(q/O)as I,};if!丂.丬(三){continue;}j.path.push(三);let
人=倥(仹,q);j.俽(&丂,人,万+亏);let 俛=j.path.len()<j.亰&&丂.丗().is_none();let 俥=!俛||j.作(&丂,Some(q),人,万+亏);j.path.pop();if!俥{return false;}}true}fn 俽(&mut self,为:&J,人:仂,万:C){let
j=self;let 仏=1-j.丁;let 伩=佥(为,仏);let 丌=j.丧.丌-伩.丌;let 七=j.丧.七-伩.七;if 丌<=0&&七<=0{return;}let 乒=j.丧.乒-伩.乒;let 且=为.且(j.丁)-j.且;let 倨=为.丗()==Some(义::乫(j.丁));let
丙=if 倨{C::MAX/2}else{3*丌+七+10*乒+2*且-万};if let Some(&主)=j.index.get(&人){if 万>=j.丩[主].万{return;}}let 丟=仠{丅:j.path.clone(),万,丌,七,乒,且,丙,};match j.index.get(&人){Some(&主)=>j.丩[主]=丟,None=>{j.index.insert(人,j.丩.len());j.丩.push(丟);}}}}fn
佚(l:&J,亰:F,中:C,乂:F)->L<仠>{乎(l,亰,中,乂).0}fn 乎(l:&J,亰:F,中:C,乂:F)->(L<仠>,F){if l.丗().is_some()||亰==0||乂==0{return(L::new(),0);}let 丁=l.丝();let 仏=1-丁;let mut
伮=佅{丁,仾:l.丞(仏)|l.丞(仏).丐(),丧:佥(l,仏),且:l.且(丁),亰,中,亵:0,path:L::new(),丩:L::new(),index:HashMap::default(),};伮.作(l,None,[0;3],0);let 亵=伮.亵;let mut 丩=伮.丩;丩.sort_by(|a,b|b.丙.cmp(&a.丙).then(a.万.cmp(&b.万)));丩.truncate(乂);(丩,亵)}const
代:C=1000000;const 侩:C=200;fn 乏(l:&J,z:F)->C{let 丄=l.丄(z);let 七:C=(l.七(z)&丄).iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.且(z)+4*C::from(丄.count())+七+10*C::from((l.乔()&丄).count())}fn
佪(l:&J,z:F,乛:G)->C{俣(l,z,佘(l,z,乛,Z[1-z]),)}fn 俣(l:&J,z:F,万:C)->C{if 万==事{return C::MIN;}l.丈(z)+l.且(z)-万}fn 佷(仲:C)->C{仲.clamp(-侩,0)}fn 信(l:&J,z:F,乛:[G;2],)->(C,[C;2]){let
个=1-z;match l.丗(){Some(义::乫(値))=>{return(if 値==z{代}else{-代},[0,0])}Some(义::伺)=>return(0,[0,0]),None=>{}}let mut 丙=乏(l,z)-乏(l,个);if z==l.丝(){let mut 丂=l.clone();丂.仇();if
丂.丗().is_none()&&众(&丂,个,true,Z[z])<=丂.丈(个){丙-=代/2;}}let 亯=[佪(l,z,乛[0]),佪(l,个,乛[1]),];(丙+佷(亯[0])-佷(亯[1]),亯)}
