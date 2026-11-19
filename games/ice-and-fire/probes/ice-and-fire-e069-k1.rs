// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type J=u8;type 並=u32;type U=u64;type 仞=i32;type C=i64;type 亟=f64;type Q=bool;type 串=String;type 乗=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侥:Q=true;fn 俲(l:&I,q:F,仙:C,丰:G,丨:C)->Q{let
丁=l.东();let 乺=1-丁;let 佭=侥&&丨<=l.丈(乺)+2*l.专(乺).max(0);let 仁=G::q(q)|G::q(q).与();if!佭&&(仁&丰).is_empty(){return false;}let(x,y)=井(q);let mut 丹=l.clone();if!丹.丬(K::乳{x,y}){return
false;}if 伸(&丹,丁)<仙{return true;}佭&&仪(&丹,乺,false,Z[丁])>丨}const 俒:Q=false;const 俀:J=2;const 侬:F=4;const 伽:F=5;#[derive(Clone)]enum 丳{亡(K),伆(L<K>),}impl
丳{fn 丅(&self)->&[K]{match self{丳::亡(三)=>std::slice::from_ref(三),丳::伆(丅)=>丅,}}}fn 井(q:F)->(J,J){((q%O)as J,(q/O)as J)}fn 佨(l:&I,丆:&mut Vec<丳>){丆.clear();if
l.且().is_some(){return;}let 丁=l.东();let 乚=l.两().乣();let mut 丣=[G::T;4];for 一 in 1..=乚{丣[F::from(一)]=l.乀(丁,一);}for 不 in l.于().iter(){let Some((丑,一))=l.仛(不)else{continue;};let
之=G::q(不).与()&丣[F::from(一)];for 乱 in 之.iter(){let(x,y)=井(乱);丆.push(丳::亡(K::乵{丑,x,y}));}}let 丈=l.丈(丁);let 七=l.七(丁);let 亳=l.亚(丁);let 倧=match 俀{0=>亳,1=>{let
休=l.map().valid.丘(七).与()&七;亳.丘(七)|休}_=>亳.丘(七),};for 一 in 1..=乚{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=倧&丣[F::from(一)];for 俶 in 1..一{丌=丌.丘(丣[F::from(俶)]);}for
q in 丌.iter(){let(x,y)=井(q);丆.push(丳::亡(K::乒{一,x,y}));}}if 丈>=亇{let 乭=l.丞(丁)&亳&l.产();if l.两().以(){for q in(乭&l.map().丱).iter(){let(x,y)=井(q);丆.push(丳::亡(K::乲{x,y}));}}if
l.两().仦(){let 休=l.map().valid.丘(l.丞(丁)).与();let 丌=(乭&休).丘(l.map().丱);let 丰=丰(l,1-丁);let 仙=C::from((丰&l.七(丁)).count());let 丨=仪(l,1-丁,false,Z[丁]);let 丶=if
俒{let(仭,俱)=乁(l,丁);仭.丘(俱)}else{G::T};for q in 丌.iter().filter(|&q|{let 仁=G::q(q)|G::q(q).与();!(仁&丶).is_empty()||俲(l,q,仙,丰,丨)}){let(x,y)=井(q);丆.push(丳::亡(K::乳{x,y}));}}}for
丟 in 你(l,伽,丈,侬){if 丟.丅.len()>1{丆.push(丳::伆(丟.丅));}}if let Some(丨)=伕(l,Z[1-丁],丈){if 丨.丅.len()>1{丆.push(丳::伆(丨.丅));}}}const 侸:J=2;const 俊:C=5;fn 乛(l:&I,z:F)->G{伫(l,z,侸)}fn
伫(l:&I,z:F,不:J)->G{let mut 乛=G::T;if 不>3{return 乛;}for q in(l.丄(z)&l.七(z)).iter(){if l.五(q).is_some_and(|一|一>=不){乛|=G::q(q);}}乛}fn 丰(l:&I,乑:F)->G{let 中=l.丈(乑)+l.专(乑);倆(l,乑,乛(l,乑),中)}fn
介(l:&I,z:F)->G{丰(l,1-z)&l.七(z)}fn 伸(l:&I,z:F)->C{C::from(介(l,z).count())}const 佅:C=4;fn 俹(丌:C)->C{丌/佅}fn 丠(l:&I,z:F,介:G)->C{let 七=l.七(z);let 丠=(l.丠()&七).丘(介);let
丌=C::from(七.count());let 保=if l.乢()<=侜{丌/供+侚}else{俹(丌)};保*C::from(丠.count())}const 侜:並=15;const 侚:C=6;const 供:C=佅;const 俆:C=2;fn 侔(l:&I,z:F)->C{let 七=l.七(z);let[a,b,c]=七.仾();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 丏=(hash as F)&(低-1);俓.with(|伓|{let mut 伓=伓.borrow_mut();let
任=&mut 伓[丏];if 任.0==七&&任.1==z as J+1{return 任.2;}let 丝=倭(七,Z[z]);*任=(七,z as J+1,丝);丝})}const 低:F=1<<12;thread_local!{static 俓:std::cell::RefCell<L<(G,J,C)>>
=std::cell::RefCell::new(vec![(G::T,0,0);低]);}fn 倭(七:G,买:F)->C{struct 仂{七:G,义:[J;M],丵:[J;M],size:[J;M],互:J,乾:C,件:C,}fn 亝(d:&mut 仂,v:F)->J{d.互+=1;let 义=d.互;d.义[v]=义;let
mut 丵=义;let mut 伴=1u8;let mut 丟=0u8;for w in(G::q(v).与()&d.七).iter(){if d.义[w]==0{let 例=亝(d,w);伴+=例;丵=丵.min(d.丵[w]);if d.丵[w]>=义{丟+=例;}}else{丵=丵.min(d.义[w]);}}d.丵[v]=丵;d.size[v]=伴;if
丟>0&&d.义[v]>1{d.乾+=1;d.件=d.件.max(C::from(丟));}伴}if!七.test(买){return 0;}let mut d=仂{七,义:[0;M],丵:[0;M],size:[0;M],互:0,乾:0,件:0,};亝(&mut d,买);d.乾+d.件}fn 倍(l:&I,z:F)->C{俆*(侔(l,1-z)-侔(l,z))}fn
佡(l:&I,z:F)->C{if l.丈(z)+l.专(z)>=0{return 0;}(l.丄(z)&l.七(z)).iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum()}const 侫:C=15;const 俐:[C;2]=[3,1];const
俎:[C;2]=[4,20];fn 倝(l:&I,z:F)->[C;2]{let mut 丒=[0;2];for q in(l.丄(z)&l.七(z)).iter(){match l.五(q){Some(2)=>丒[0]+=1,Some(3)=>丒[1]+=1,_=>{}}}丒}fn 侊(l:&I,z:F)->C{let[侐,倡]=倝(l,z);let
佔=|一:F|(3-俐[一])*俎[一];-侫*侐+佔(0)*侐+佔(1)*倡}const 侤:C=0;const 侧:C=2;const 侦:C=100;fn 侇(仵:C)->C{if 仵<0{return 0;}侤+侧*仵.min(侦)/2}const 佄:J=2;const 佂:C=2;const
佁:C=4;const 侨:J=3;fn 乽(l:&I,z:F)->C{if 佂==0{return 0;}let 乖=l.map().valid;let mut 丄=G::T;for q in(l.丄(z)&l.七(z)).iter(){if l.五(q).is_some_and(|一|一<=侨){丄|=G::q(q);}}let
mut 份=乖.丘(l.丞(z));let mut 伜=G::T;let mut 之=0;for 举 in 1..=佁{份=(份|份.与())&乖;let 仮=(丄&份).丘(伜);之+=(举-1)*C::from(仮.count());伜|=仮;}之+=佁*C::from(丄.丘(伜).count());佂*之}const
位:C=8;fn 乁(l:&I,z:F)->(G,G){let 七=l.七(z);let 仭=七&l.丞(1-z).与();let 乂=l.乂()&七;(仭,仭&(乂|乂.与()))}fn 侓(l:&I,z:F)->C{if 位==0{return 0;}位*C::from(乁(l,z).1 .count())}fn
伞(l:&I,z:F)->C{let 乛=[伫(l,z,佄),伫(l,1-z,佄),];let(丙,亰)=俪(l,z,乛);let 丙=丙+侇(亰[0])-侇(亰[1]);if l.且().is_some(){return 丙;}let 仑=介(l,z);let 亅=介(l,1-z);let 仙=C::from(亅.count())-C::from(仑.count());丙-2*佡(l,z)+2*佡(l,1-z)+倍(l,z)+侊(l,z)-侊(l,1-z)-乽(l,z)+乽(l,1-z)+俊*仙/4+丠(l,z,仑)-丠(l,1-z,亅)+侓(l,z)-侓(l,1-z)}const
侵:F=8;const 俁:F=3;const 侭:F=32;const 係:U=300;const 侙:亟=0.75;const 価:P<並> =Some(7);const 侟:Q=false;const 侞:C=10;const 侠:並=6;struct 争{亄:P<乗>,亘:P<乗>,从:U,}#[derive(Default)]struct
佋{丅:L<K>,伙:F,乻:U,丗:F,丯:F,佚:F,乼:F,乯:Q,}const 俑:Q=true;#[derive(Clone)]struct 乙{丅:L<K>,伝:C,}struct 亢{丅:L<K>,end:I,first:P<U>,丕:P<L<乙>>,}type 仃=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
伉{丁:F,limit:争,乻:U,仍:仃,些:P<L<丳>>,伶:Q,}const 伇:C=1;const 似:Q=true;const 侖:Q=false;const 伻:P<J> =None;fn 俛(l:&I,丗:&[乙],丆:&mut Vec<丳>){if!似&&伻.is_none(){return;}let
仗=丗.iter().map(|乜|乜.丅.as_slice());俜(l,仗,(似,侖,伻),丆,);}fn 俜<'a>(l:&I,仗:impl Iterator<Item=&'a[K]>,(乂,与,一):(Q,Q,P<J>),丆:&mut Vec<丳>,){let 丁=l.东();let 为=l.七(丁);let
mut 仉=G::T;for 乜 in 仗{for 三 in 乜{if let K::乒{x,y,..}|K::乵{x,y,..}=*三{let q=F::from(y)*O+F::from(x);if 为.test(q){仉|=G::q(q);}}}}if 仉.is_empty(){return;}let
乭=l.产()&为;let 丈=l.丈(丁);let 佾=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丅()==[三]){丆.push(丳::亡(三));}};if 乂&&丈>=亇&&l.两().仦(){let mut 丌=仉;if 与{丌|=仉.与();}for
q in(丌&乭).丘(l.map().丱).iter(){let(x,y)=井(q);佾(K::乳{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().乣()){if 丈>=10*C::from(一){for q in(仉&乭).iter(){let(x,y)=井(q);佾(K::乒{一,x,y},丆);}}}}fn
伐(l:&mut I,丅:&[K])->Q{丅.iter().all(|&三|l.丬(三))}fn 乏(丅:&[K])->U{let mut hash:U=0xcbf29ce484222325;for 三 in 丅{let(仱,a,b,c)=match*三{K::乵{丑,x,y}=>(1,U::from(丑),x,y),K::乒{一,x,y}=>(2,U::from(一),x,y),K::乲{x,y}=>(3,0,x,y),K::乳{x,y}=>(4,0,x,y),};let
什=仱|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^什).wrapping_mul(0x0100000001b3);}hash}fn 佑(乊:&I,乜:&[K])->I{let mut 丂=乊.clone();丂.亨();for&三 in 乜{丂.丬(三);}丂.亨();丂}fn
仳(l:&I)->P<L<K>>{let 丁=l.东();let 丐=Z[1-丁];let 亿=|両:&I|両.且()==Some(么::乡(丁));for q in l.于().iter(){if l.map().举(q,丐)!=1{continue;}let Some((丑,_))=l.仛(q)else{continue;};let(x,y)=井(丐);let
三=K::乵{丑,x,y};let mut 両=l.clone();if 両.丬(三)&&亿(&両){return Some(vec![三]);}}let 丨=伕(l,丐,l.丈(丁))?;let mut 両=l.clone();(伐(&mut 両,&丨.丅)&&亿(&両)).then_some(丨.丅)}fn
侉(l:&I,z:F,丷:C)->Q{let 万=会(l,z,l.丄(z),Z[1-z]);万!=乶&&l.丈(z)+丷*l.专(z)>=万}fn 倜(l:&I)->Q{let 丁=l.东();if 伇==0||l.且().is_some(){return false;}let mut 丂=l.clone();丂.亨();let
伸=丂.且().is_none()&&仪(&丂,1-丁,true,Z[丁])<=丂.丈(1-丁);!伸&&侉(l,丁,伇)}impl 伉{fn 佖(&self,乊:&I)->Q{self.伶&&乊.且()!=Some(么::乡(self.丁))&&!侉(乊,self.丁,伇)}fn 乨(&self)->Q{self.乻>=self.limit.从||self.limit.亄.is_some_and(|t|乗::now()>=t)}fn
丙(&mut self,乊:&I,丗:&[乙],伒:C)->C{self.乻+=1;self.伯(乊,丗,伒)}fn 伯(&self,乊:&I,丗:&[乙],伒:C)->C{if 乊.且()==Some(么::乡(self.丁)){return 令;}if 丗.is_empty(){return 伞(乊,self.丁);}let
mut 仿=C::MAX;for 乜 in 丗{仿=仿.min(伞(&佑(乊,&乜.丅),self.丁));if 仿<=伒{break;}}仿}#[allow(clippy::too_many_arguments)]fn 倀(&mut self,乬:&I,亦:C,乓:&[丳],亷:&[(C,F)],丿:&mut[Q],丗:&[乙],亩:&mut
仃,伥:Q,)->(P<(F,I,C)>,Q){let mut 下=None;let mut 仧=亦;for&(价,丏)in 亷{if 丿[丏]{continue;}if 伥&&价!=C::MAX&&价<=仧-亦{return(下,true);}if self.乨(){return(下,false);}丿[丏]=true;let
丅=乓[丏].丅();let mut 丂=乬.clone();if!伐(&mut 丂,丅)||self.佖(&丂){亩.insert(乏(丅),C::MIN/4);continue;}let 丙=self.丙(&丂,丗,仧);亩.insert(乏(丅),丙-亦);if 丙>仧{仧=丙;下=Some((丏,丂,丙));}}(下,false)}fn
优(&mut self,买:&I,丗:&[乙],仅:&[U],俠:Q)->亢{let j=self;let mut 乎=None;let mut 乬=买.clone();let mut 丅=L::new();let mut 亦=j.丙(&乬,丗,C::MIN);let mut 亩=仃::default();let
mut 乓=L::new();let mut 亷=L::new();while 乬.且().is_none()&&!j.乨(){if 丅.is_empty()&&俠{match&j.些{Some(些)=>乓.clone_from(些),None=>{佨(&乬,&mut 乓);j.些=Some(乓.clone());}}}else{佨(&乬,&mut
乓);}俛(&乬,丗,&mut 乓);亷.clear();for(丏,乷)in 乓.iter().enumerate(){let k=乏(乷.丅());let 价=亩.get(&k).or(j.仍.get(&k)).copied();亷.push((价.unwrap_or(C::MAX),丏));}亷.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丿=vec![false;乓.len()];if 丅.is_empty(){for(丏,乷)in 乓.iter().enumerate(){丿[丏]=仅.contains(&乏(乷.丅()));}}let mut 下=None;for
伥 in[true,false]{let(丩,倕)=j.倀(&乬,亦,&乓,&亷,&mut 丿,丗,&mut 亩,伥,);下=丩;if 下.is_some()||!倕{break;}}let Some((丏,丂,丙))=下 else{break;};if 丅.is_empty(){乎=Some(乏(乓[丏].丅()));}丅.extend_from_slice(乓[丏].丅());乬=丂;亦=丙;}for(k,乮)in
亩{let 佪=j.仍.entry(k).or_insert(乮);*佪=(*佪).max(乮);}亢{丅,end:乬,first:乎,丕:None,}}fn 仗(&self,乊:&I)->L<L<K>>{let mut 丂=乊.clone();丂.亨();let mut 丆=L::new();if
丂.且().is_some(){return 丆;}let 乤=丂.东();let 丈=丂.丈(乤);if let Some(丨)=伕(&丂,Z[self.丁],丈){丆.push(丨.丅);}for 丟 in 你(&丂,伽,丈,俁){丆.push(丟.丅);}let mut 佩=L::new();俰(&丂,&mut
佩);丆.push(佩);let mut 亅=伉{丁:乤,limit:争{从:係,..self.limit},乻:0,仍:仃::default(),些:None,伶:false,};丆.push(亅.优(&丂,&[],&[],true).丅);丆.retain(|丅|!丅.is_empty());丆}fn
佒(&self,乊:&I)->L<乙>{self.仗(乊).into_iter().map(|丅|{let 伝=伞(&佑(乊,&丅),self.丁);乙{丅,伝}}).collect()}fn 乼(&mut self,买:&I,乷:&丳,丗:&[乙])->P<亢>{let mut 丹=买.clone();if!伐(&mut
丹,乷.丅())||self.佖(&丹){return None;}self.乻+=1;let 九=self.优(&丹,丗,&[],false);let mut 丅=乷.丅().to_vec();丅.extend_from_slice(&九.丅);Some(亢{丅,end:九.end,first:Some(乏(乷.丅())),丕:None,})}fn
佥(&self,世:&亢,丗:&[乙])->C{let 丙=self.伯(&世.end,丗,C::MIN);match&世.丕{Some(丕)if!丕.is_empty()=>丙.min(self.伯(&世.end,丕,C::MIN)),_=>丙,}}}fn 倈(丩:&[乙],丗:&mut Vec<乙>)->Q{let
丧:L<_> =丗.iter().map(|r|r.丅.clone()).collect();for 乜 in 丩{if 丗.iter().any(|r|r.丅==乜.丅){continue;}丗.push(乜.clone());}丗.sort_by_key(|乜|乜.伝);丗.truncate(侵);丗.iter().map(|r|&r.丅).ne(&丧)}fn
倠(l:&I,乄:争,乯:&mut 亊)->佋{let 丁=l.东();let mut 丫=佋::default();if l.且().is_some(){return 丫;}if let Some(丅)=仳(l){丫.丅=丅;return 丫;}let 丽=|倏:亟|match(乄.亄,乄.亘){(Some(亄),Some(亘))=>Some(亄+(亘-亄).mul_f64(倏)),_=>None,};if
価.is_some_and(|上|l.乢()>=上){乯.end();}if 乯.倃(l){let 丅=乯.上(l,丽(0.4));let 俟=丅.iter().any(|三|{matches!(三,K::乒{..}|K::乲{..}|K::乳{..})});let 乽=侟&&!俟&&l.乢()>=侠&&l.丈(丁)>=侞;if!乽{丫.乯=true;丫.丅=丅;return
丫;}乯.end();}let mut 乐=伉{丁,limit:争{亄:丽(侙),..乄},乻:0,仍:仃::default(),些:None,伶:倜(l),};let mut 丗=L::new();let mut 丯:L<亢> =L::new();let mut 仅=L::new();let mut
仫=false;let mut 乼:L<丳> =L::new();while 丫.伙<侭&&(丯.is_empty()||!乐.乨()){let 世=if 仫{let Some(乷)=乼.pop()else{break;};match 乐.乼(l,&乷,&丗){Some(世)=>世,None=>continue,}}else{乐.优(l,&丗,&仅,true)};丫.伙+=1;let
侃=丯.iter().any(|p|p.丅==世.丅);let 乎=世.first;let 侈=乐.乨();let mut 世=世;let mut 佴=false;if!侈&&!侃{let 丩=乐.佒(&世.end);佴=倈(&丩,&mut 丗);世.丕=Some(丩);}if!侃{丯.push(世);}if
侈{break;}if!佴&&!仫{match 乎{Some(乎)=>仅.push(乎),None=>{仫=true;乼=乐.些.clone().unwrap_or_default();乼.retain(|a|!仅.contains(&乏(a.丅())));let 乮=|a:&丳|{乐.仍.get(&乏(a.丅())).copied().unwrap_or(C::MIN)};乼.sort_by_key(乮);}}}}丫.乼=F::from(仫);丫.乻=乐.乻;丫.丗=丗.len();丫.丯=丯.len();let
佯=||乄.亘.is_some_and(|t|乗::now()>=t);let mut 人:L<P<C>> =vec![None;丯.len()];for 丏 in(0..丯.len()).rev(){if 丏+1<丯.len()&&佯(){break;}人[丏]=Some(乐.佥(&丯[丏],&丗));}let
亸=|人:&[P<C>]|{let mut 下:P<(C,F)> =None;for(丏,丙)in 人.iter().enumerate().rev(){if let Some(丙)=*丙{if 下.is_none_or(|(了,_)|丙>了){下=Some((丙,丏));}}}下};let mut
下=亸(&人);乐.limit.亄=乄.亘;while let Some((_,丏))=下{if!俑||丯[丏].丕.is_some()||佯(){break;}let 丩=乐.佒(&丯[丏].end);丯[丏].丕=Some(丩);丫.佚+=1;人[丏]=Some(乐.佥(&丯[丏],&丗));下=亸(&人);}if
let Some((_,丏))=下{let mut 丮=l.clone();for&三 in&丯[丏].丅{if 丮.丬(三){丫.丅.push(三);}}}丫}fn 倂(乄:&dyn Fn(乗,並)->争,mut 乯:亊,伤:impl BufRead,仸:impl Write,)->Result<(),串>{丷(伤,仸,|亭,乞,上,亜|match
I::read(亭,乞,乪::伂,上){Ok(l)=>{let 丫=倠(&l,乄(亜,上),&mut 乯);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,丫.伙,丫.乻,丫.丗,丫.佚,丫.乼,亜.elapsed());俘(&丫.丅)}Err(亗)=>{eprintln!("plan: cannot read the input: {}",亗);"WAIT".to_string()}},)}const
侗:亟=45.0;const 依:亟=800.0;const 俇:亟=0.6;fn 乄(亜:乗,上:並)->争{if let Some(从)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
争{亄:None,亘:None,从,};}let 仺=倢().min(100.0);let 俻=if 上==1{依}else{侗};let 中=Duration::from_secs_f64(俻/1000.0*仺);争{亄:Some(亜+中.mul_f64(俇)),亘:Some(亜+中),从:U::MAX,}}const
便:F=9;const 侹:亠=亠{丌:3,专:1,丈:1,丟:2,乾:2,仚:2,丰:6,仜:1,};const 侾:J=J::MAX;const 侻:C=2;const 侼:C=18;const 侺:C=0;const 侽:F=32;fn 乯()->亊{俾(侽)}fn 俾(乩:F)->亊{let
亃=亣::代{丷:便,乩,乕:侹,};let 丛=俅.iter().map(|&书|Y{伲:侾,仆:侻,伢:侼,伎:侺,..书}).collect();亊::new(亃,丛,Y::伋)}fn main(){let 候=io::stdin();let 倚=io::stdout();if let Err(亗)=倂(&乄,乯(),候.lock(),倚.lock()){eprintln!("plan: {}",亗);std::process::exit(1);}}const
俋:&str="CG_TIME_SCALE";fn 倢()->亟{俿(std::env::var(俋).ok().as_deref())}fn 俿(丝:P<&str>)->亟{丝.and_then(|乞|乞.trim().parse::<亟>().ok()).filter(|仺|仺.is_finite()&&*仺>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乘:F=4*O;const 乫:U=(1<<乘)-1;const 仟:U=1|1<<12|1<<24|1<<36;const 伃:U=仟<<11;const 佉:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侕:G=G([乫;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乘]>>(q%乘))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乘]|=1<<(q%乘);}#[inline]fn clear(&mut self,q:F){self.0[q/乘]&=!(1<<(q%乘));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仾(self)->[U;3]{self.0}#[inline]fn count(self)->並{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丘(self,个:G)->G{G([self.0[0]&!个.0[0],self.0[1]&!个.0[1],self.0[2]&!个.0[2],])}#[inline]fn
iter(self)->企{企{伨:self.0,丽:0,}}#[inline]fn 倔(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&佉)<<(乘-O)),(b>>O)|((c&佉)<<(乘-O)),c>>O,])}#[inline]fn 們(self)->G{let[a,b,c]=self.0;G([(a<<O)&乫,((b<<O)&乫)|(a>>(乘-O)),((c<<O)&乫)|(b>>(乘-O)),])}#[inline]fn
倒(self)->G{let[a,b,c]=self.0;G([(a>>1)&!伃&乫,(b>>1)&!伃&乫,(c>>1)&!伃&乫,])}#[inline]fn 倓(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仟&乫,(b<<1)&!仟&乫,(c<<1)&!仟&乫,])}#[inline]fn
与(self)->G{let j=self;j.倔()|j.們()|j.倒()|j.倓()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,个:G)->G{G([self.0[0]&个.0[0],self.0[1]&个.0[1],self.0[2]&个.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,个:G)->G{G([self.0[0]|个.0[0],self.0[1]|个.0[1],self.0[2]|个.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,个:G){*self=*self|个;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侕.丘(self)}}struct 企{伨:[U;3],丽:F,}impl Iterator for 企{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.丽<3{let 仴=j.伨[j.丽];if 仴!=0{j.伨[j.丽]=仴&(仴-1);return Some(j.丽*乘+仴.trailing_zeros()as F);}j.丽+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乵{丑:並,x:J,y:J},乒{一:J,x:J,y:J},乲{x:J,y:J},乳{x:J,y:J},}impl K{fn 値(&self)->串{match*self{K::乵{丑,x,y}=>format!("MOVE {} {x} {y}",丑),K::乒{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::乲{x,y}=>format!("BUILD MINE {x} {y}"),K::乳{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
俘(丅:&[K])->串{if 丅.is_empty(){return "WAIT".to_string();}let mut line=串::with_capacity(16*丅.len());for(丏,三)in 丅.iter().enumerate(){if 丏>0{line.push(';');}line.push_str(&三.値());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乪{住,倳,俔,#[default]伂,}impl 乪{fn 乣(self)->J{match self{乪::住=>1,_=>3,}}fn 以(self)->Q{matches!(self,乪::俔|乪::伂)}fn 仦(self)->Q{self==乪::伂}}const 习:J=J::MAX;struct
乴{丼:G,valid:G,丱:G,举:Box<[J]>,与:[[[J;4];M];2],}thread_local!{static 侪:RefCell<P<Arc<乴>>> =const{RefCell::new(None)};}impl 乴{fn 倐(丼:G,丱:G)->Arc<乴>{侪.with(|乿|{let
mut 乿=乿.borrow_mut();if let Some(亯)=乿.as_ref(){if 亯.丼==丼&&亯.丱==丱{return Arc::clone(亯);}}let 亯=Arc::new(乴::build(丼,丱));*乿=Some(Arc::clone(&亯));亯})}fn build(丼:G,丱:G)->乴{let
乖=!丼;const 俈:[[(仞,仞);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 与=[[[习;4];M];2];for(z,之)in 俈.iter().enumerate(){for q in
乖.iter(){let(x,y)=((q%O)as 仞,(q/O)as 仞);let mut 丒=0;for&(俧,俨)in 之{let(佶,佷)=(x+俧,y+俨);if(0..O as 仞).contains(&佶)&&(0..O as 仞).contains(&佷){let 丂=佷 as F*O+佶
as F;if 乖.test(丂){与[z][q][丒]=丂 as J;丒+=1;}}}}}let mut 举=vec![习;M*M].into_boxed_slice();let mut 伮=[0u8;M];for 不 in 乖.iter(){let 亐=&mut 举[不*M..(不+1)*M];亐[不]=0;伮[0]=不
as J;let(mut 传,mut 伷)=(0,1);while 传<伷{let q=伮[传]as F;传+=1;for&丂 in&与[0][q]{if 丂!=习&&亐[丂 as F]==习{亐[丂 as F]=亐[q]+1;伮[伷]=丂;伷+=1;}}}}乴{丼,valid:乖,丱,举,与,}}#[inline]fn
举(&self,不:F,乱:F)->J{self.举[不*M+乱]}}const 侴:並=100;const 亇:C=15;const 俏:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乡(F),伿,}enum
也{伈,但,佇,}#[derive(Clone)]struct I{map:Arc<乴>,两:乪,丕:[G;2],七:[G;2],为:G,乌:G,丄:[G;2],一:[G;3],亍:G,亾:[並;M],丈:[C;2],专:[C;2],东:F,乢:並,亴:並,且:P<么>,}fn 仌(丕:G,京:F)->G{if!丕.test(京){return
G::T;}let mut 他=G::q(京);loop{let 丂=(他|他.与())&丕;if 丂==他{return 他;}他=丂;}}impl I{fn 东(&self)->F{self.东}fn 乢(&self)->並{self.乢}fn 且(&self)->P<么>{self.且}fn 两(&self)->乪{self.两}fn
亲(&self,z:F)->C{let j=self;let 丄=j.丄[z];j.丈[z]+10*C::from((丄&j.一[0]).count())+20*C::from((丄&j.一[1]).count())+30*C::from((丄&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
专(&self,z:F)->C{self.专[z]}fn 伹(&self,z:F)->F{self.丄[z].count()as F}fn map(&self)->&乴{&self.map}fn 丞(&self,z:F)->G{self.丕[z]}fn 七(&self,z:F)->G{self.七[z]}fn
丄(&self,z:F)->G{self.丄[z]}fn 仛(&self,q:F)->P<(並,J)>{let j=self;(j.丄[0]|j.丄[1]).test(q).then(||(j.亾[q],j.伧(q)))}fn 于(&self)->G{self.丄[self.东]&self.亍}fn
丠(&self)->G{self.为}fn 乂(&self)->G{self.乌}fn 五(&self,q:F)->P<J>{(q<M&&(self.丄[0]|self.丄[1]).test(q)).then(||self.伧(q))}fn 倘(&mut self,z:F){let j=self;j.七=[仌(j.丕[0],Z[0]),仌(j.丕[1],Z[1])];let
俣=j.丄[z].丘(j.七[z]);j.估(俣);j.佛();j.丈[z]+=j.专[z];if j.丈[z]<0{j.丈[z]=0;j.估(j.丄[z]);}j.亍|=j.丄[z];}fn 佛(&mut self){let j=self;for z in 0..2{let 七=j.七[z];let
丄=七&j.丄[z];let mut 专=C::from(七.count())+4*C::from((七&j.为).count());for(一,倫)in 俏.iter().enumerate(){专-=倫*C::from((丄&j.一[一]).count());}j.专[z]=专;}}fn 亨(&mut
self){let j=self;if j.且.is_some(){return;}if j.东==1{if j.乢>=侴{j.且=Some(match j.亲(0).cmp(&j.亲(1)){std::cmp::Ordering::Greater=>么::乡(0),std::cmp::Ordering::Less=>么::乡(1),std::cmp::Ordering::Equal=>么::伿,});return;}j.乢+=1;}j.东=1-j.东;j.倘(j.东);}fn
佼(&mut self,丅:&[K]){let j=self;if j.且.is_some(){return;}let 伟=j.亴;for&三 in 丅{if j.俦(三,伟){continue;}j.丬(三);if j.且.is_some(){return;}}j.亨();}fn 俦(&self,三:K,伟:並)->Q{let
j=self;match 三{K::乵{丑,x,y}=>{j.亥(x,y).is_none()||丑>=伟||j.佦(j.东,丑).is_none()}K::乒{一,x,y}=>{j.亥(x,y).is_none()||一==0||一>j.两.乣()}K::乲{x,y}|K::乳{x,y}=>{j.亥(x,y).is_none()}}}#[inline]fn
亥(&self,x:J,y:J)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丼.test(y*O+x)).then_some(y*O+x)}fn 佦(&self,z:F,丑:並)->P<F>{self.丄[z].iter().find(|&q|self.亾[q]==丑)}#[inline]fn
伧(&self,q:F)->J{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 俚(&self,乤:F)->G{let 乂=self.乌&self.丕[乤];let 七=self.七[乤];乂|(七&(七&乂).与())}fn
乀(&self,z:F,一:J)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乤=1-z;let 丄=j.丄[0]|j.丄[1];let 俩=j.产();let 倩=match
一{1 if j.两==乪::住=>j.一[1]|j.一[2],1=>丄,2=>j.一[1]|j.一[2],_=>G::T,};let 乀=俩|j.丕[乤].丘(倩);if 一<3{乀.丘(j.俚(乤))}else{乀}}fn 亚(&self,z:F)->G{self.七[z]|self.七[z].与()}fn
产(&self)->G{let j=self;let 丄=j.丄[0]|j.丄[1];let 俳=G::q(Z[0])|G::q(Z[1]);j.map.valid.丘(丄|j.为|j.乌|俳)}fn 丬(&mut self,三:K)->Q{let j=self;if j.且.is_some(){return
false;}let z=j.东;let 亖=match 三{K::乒{一,x,y}=>j.倦(z,一,x,y),K::乵{丑,x,y}=>j.俺(z,丑,x,y),K::乲{x,y}=>j.build(z,true,x,y),K::乳{x,y}=>j.build(z,false,x,y),};let
Some(亖)=亖 else{return false;};match 亖{也::伈=>{}也::但=>j.七[z]=仌(j.丕[z],Z[z]),也::佇=>{j.七=[仌(j.丕[0],Z[0]),仌(j.丕[1],Z[1])];}}j.佛();if!j.丕[0].test(Z[0]){j.且=Some(么::乡(1));}else
if!j.丕[1].test(Z[1]){j.且=Some(么::乡(0));}true}fn 估(&mut self,丌:G){let j=self;j.丄[0]=j.丄[0].丘(丌);j.丄[1]=j.丄[1].丘(丌);for 一 in&mut j.一{*一=一.丘(丌);}j.亍=j.亍.丘(丌);}fn
侌(&mut self,z:F,q:F)->也{let j=self;j.估(G::q(q));j.为.clear(q);j.乌.clear(q);let 乤=1-z;if j.丕[乤].test(q){j.丕[乤].clear(q);j.丕[z].set(q);也::佇}else if j.丕[z].test(q){也::伈}else{j.丕[z].set(q);也::但}}fn
倦(&mut self,z:F,一:J,x:J,y:J)->P<也>{let j=self;if 一==0||一>j.两.乣(){return None;}let q=j.亥(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亚(z).test(q)||!j.乀(z,一).test(q){return
None;}let 亖=j.侌(z,q);j.佺(z,一,q,j.亴);j.亴=j.亴.saturating_add(1);j.丈[z]-=万;Some(亖)}fn 佺(&mut self,z:F,一:J,q:F,丑:並){self.丄[z].set(q);self.一[F::from(一)-1].set(q);self.亾[q]=丑;}fn
俺(&mut self,z:F,丑:並,x:J,y:J)->P<也>{let j=self;let 丐=j.亥(x,y)?;let 不=j.佦(z,丑)?;if!j.亍.test(不){return None;}let 一=j.伧(不);let 丣=j.乀(z,一);let 乃=j.map.与[z][不];if
乃.contains(&(丐 as J))&&!丣.test(丐){return None;}let mut 下=j.map.举(不,丐);let mut 仇=None;for&丂 in 乃.iter().take_while(|&&丂|丂!=习){let 丂=F::from(丂);let 举=j.map.举(丂,丐);if
举<下&&丣.test(丂){下=举;仇=Some(丂);}}let 仇=仇?;let 亖=j.侌(z,仇);j.丄[z].clear(不);j.一[F::from(一)-1].clear(不);j.亍.clear(不);j.佺(z,一,仇,丑);Some(亖)}fn build(&mut self,z:F,为:Q,x:J,y:J)->P<也>{let
j=self;let 俗=if 为{j.两.以()}else{j.两.仦()};let q=j.亥(x,y)?;let 俼=j.map.丱.test(q);if!俗||j.丈[z]<亇||俼!=为||!j.丕[z].test(q)||!j.亚(z).test(q)||!j.产().test(q){return
None;}if 为{j.为.set(q);}else{j.乌.set(q);}j.丈[z]-=亇;Some(也::伈)}}const 侯:C=1000000000;const 伄:C=0;const 侶:C=1;const 俌:C=2;impl I{fn read(亭:&str,上:&str,两:乪,乢:並,)->Result<I,串>{let
mut 丱=L::new();let mut 严=亭.lines();let 丒=伛(&mut 严,"mine spot count")?;for _ in 0..丒{let[x,y]=亁(亵(&mut 严,"a mine spot")?,"a mine spot")?;丱.push((仩(x)?,仩(y)?));}佣(严,"the initial input")?;let
mut 严=上.lines();let mut 伡=[0i64;4];for(丝,丸)in 伡.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伭]=亁(亵(&mut 严,丸)?,丸)?;if 伭.abs()>侯{return
Err(format!("{}: {} is out of range",丸,伭));}*丝=伭;}let mut 丼=G::T;let mut 丌=[G::T;2];let mut 七=[G::T;2];for y in 0..O{let 亐=亵(&mut 严,"a map row")?.trim();if
亐.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,亐));}for(x,付)in 亐.bytes().enumerate(){let q=y*O+x;match 付{b'#'=>丼.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
久=F::from(付==b'X'||付==b'x');丌[久].set(q);if 付.is_ascii_uppercase(){七[久].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",付 as char))}}}}let
俞=伛(&mut 严,"building count")?;let mut 乔=L::new();for _ in 0..俞{let 亁=亁(亵(&mut 严,"a building")?,"a building")?;乔.push(亁);}let 伹=伛(&mut 严,"unit count")?;let
mut 丄=L::new();for _ in 0..伹{let 亁=亁(亵(&mut 严,"a unit")?,"a unit")?;丄.push(亁);}佣(严,"the turn's input")?;let 佹=乔.iter().find(|佗|佗[0]==0&&佗[1]==伄).ok_or("the bot's own HQ is not among the buildings")?;let
佬=伔(佹[2],佹[3],丼)?;let z=Z.iter().position(|&京|京==佬).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佬))?;let mut 侅=G::T;for(x,y)in 丱{if
丼.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}侅.set(y*O+x);}let 伌=|久:F|if 久==0{z}else{1-z};let mut l=I{map:乴::倐(丼,侅),两,丕:[G::T;2],七:[G::T;2],为:G::T,乌:G::T,丄:[G::T;2],一:[G::T;3],亍:G::T,亾:[0;M],丈:[0;2],专:[0;2],东:z,乢,亴:1,且:None,};for
久 in 0..2{let 乥=伌(久);l.丕[乥]=丌[久];l.七[乥]=七[久];l.丈[乥]=伡[2*久];l.专[乥]=伡[2*久+1];}let mut 佸=G::T;for[乥,仱,x,y]in 乔{let 久=侀(乥)?;let q=伔(x,y,丼)?;if!丌[久].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佸.test(q){return Err(format!("two buildings at ({x}, {y})"));}佸.set(q);match 仱{伄
if q==Z[伌(久)]=>{}伄=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侶=>l.为.set(q),俌=>l.乌.set(q),_=>return Err(format!("unknown building type {}",仱)),}}let
mut 乆=G::T;let mut 仲=0u32;for[乥,丑,一,x,y]in 丄{let 久=侀(乥)?;let q=伔(x,y,丼)?;if!丌[久].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乆.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丑=並::try_from(丑).map_err(|_|format!("unit id {} is out of range",丑))?;if
乆.iter().any(|个|l.亾[个]==丑){return Err(format!("two units have id {}",丑));}乆.set(q);l.丄[伌(久)].set(q);l.一[一 as F-1].set(q);l.亾[q]=丑;仲=仲.max(丑);}if!乆.is_empty(){l.亴=仲.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仲))?;}l.亍=l.丄[z];Ok(l)}}fn
亵<'a>(严:&mut Lines<'a>,丸:&str)->Result<&'a str,串>{严.next().ok_or_else(||format!("the text ends before {}",丸))}fn 佣(mut 严:Lines<'_>,丸:&str)->Result<(),串>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丸)),None=>Ok(()),}}fn 亁<const N:F>(line:&str,丸:&str)->Result<[C;N],串>{let
mut 仾=line.split_whitespace();let mut 侒=[0i64;N];for 丝 in&mut 侒{let 什=仾.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丸))?;*丝=什.parse().map_err(|_|format!("{}: {:?} is not an integer",丸,什))?;}if
仾.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丸));}Ok(侒)}fn 伛(严:&mut Lines<'_>,丸:&str)->Result<F,串>{let[丒]=亁(亵(严,丸)?,丸)?;F::try_from(丒).ok().filter(|&丒|丒<=M).ok_or_else(||format!("{}: {} is out of range",丸,丒))}fn
仩(丝:C)->Result<F,串>{F::try_from(丝).ok().filter(|&丝|丝<O).ok_or_else(||format!("coordinate {} is off the map",丝))}fn 伔(x:C,y:C,丼:G)->Result<F,串>{let q=仩(y)?*O+仩(x)?;if
丼.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 侀(乥:C)->Result<F,串>{match 乥{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乥)),}}const
侣:C=15;const 侷:C=6;const 佌:C=10;const 俄:J=3;#[derive(Clone,Copy)]struct Y{open:C,仨:C,佞:J,伣:C,伢:C,佱:C,伲:J,佝:C,作:C,伎:C,享:C,亼:C,主:C,亮:C,亓:C,仆:C,}impl Y{const
伋:Y=Y{open:3,仨:2,佞:12,伣:4,伢:侣,佱:侷,伲:俄,佝:2,作:2,伎:0,享:0,亼:0,主:0,亮:0,亓:0,仆:0,};const fn 倱(self,享:C,亼:C,主:C)->Y{Y{享,亼,主,..self}}const fn 倰(self,亮:C,亓:C)->Y{Y{亮,亓,..self}}}fn
仝(q:F)->(J,J){((q%O)as J,(q/O)as J)}fn 仈(l:&I,不:G)->[J;M]{let mut 乍=[J::MAX;M];let 乖=l.map().valid;let mut 乁=不&乖;let mut 乆=乁;let mut d=0u8;while!乁.is_empty(){for
q in 乁.iter(){乍[q]=d;}let 丂=(乁.与()&乖).丘(乆);乆|=丂;乁=丂;d=d.saturating_add(1);}乍}fn 俭(l:&I)->J{let 乍=仈(l,l.丞(0));l.丞(1).iter().map(|q|乍[q]).min().unwrap_or(J::MAX)}struct
事<'a>{书:&'a Y,丁:F,theirs:[J;M],亽:[J;M],free:G,乑:G,ours:G,丱:G,丄:G,}impl 事<'_>{fn new<'a>(l:&I,书:&'a Y)->事<'a>{let 丁=l.东();let 为=仈(l,l.丞(丁));let 亅=仈(l,l.丞(1-丁));let
亽=仈(l,G::q(Z[1-丁]));let 乭=l.map().valid.丘(l.丞(0)|l.丞(1));let mut 乑=G::T;for q in 乭.iter(){if 为[q]<=亅[q]{乑.set(q);}}事{书,丁,theirs:亅,亽,free:乭,乑,ours:l.丞(丁),丱:l.map().丱,丄:l.丄(丁),}}fn
value(&self,q:F)->C{let j=self;let p=j.书;let 丶=C::from((G::q(q).与()&j.free).count());let 余=p.佞;let 仨=C::from(余)-C::from(j.theirs[q].min(余));let 京=if p.伣>0{C::from(j.亽[q].min(30))/p.伣}else{0};let
丕=C::from((G::q(q).与()&j.ours).count());let line=if 丕<=1{p.亼}else{0};let 主=if j.丱.test(q){p.主}else{0};let 倞=if 丶==0{p.亮}else{0};let 亓=p.亓*(22-C::from(j.亽[q].min(22)));let
仆=p.仆*C::from((G::q(q).与()&j.丄).count());p.open*丶+p.仨*仨-京+p.享*丕-line+主-倞+亓-仆}fn 下(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
众(&mut self,q:F){let j=self;j.free.clear(q);j.乑.clear(q);j.ours.set(q);j.丄.set(q);}}fn 仔(l:&mut I,三:K,丆:&mut Vec<K>)->Q{let 佐=l.丬(三);if 佐{丆.push(三);}佐}fn
业(l:&mut I,不:F,乱:F,丆:&mut Vec<K>)->Q{let Some((丑,_))=l.仛(不)else{return false;};let(x,y)=仝(乱);仔(l,K::乵{丑,x,y},丆)}fn 倪(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-佌}#[derive(Clone,Copy)]enum
乇{仢,予{丠:J,丄:J},}fn 侏(l:&I,书:&Y,二:乇)->L<K>{let mut 丮=l.clone();let mut 上=事::new(l,书);let mut 丆=L::new();亀(&mut 丮,&mut 上,&mut 丆);match 二{乇::仢=>倅(&mut 丮,&mut
上,&mut 丆),乇::予{丠,丄}=>{俫(&mut 丮,&mut 上,丠,丄,&mut 丆)}}丆}fn 亀(l:&mut I,上:&mut 事<'_>,丆:&mut Vec<K>){let 丁=上.丁;let mut 乽:L<F> =L::new();loop{let 丣=l.乀(丁,1);let
mut 亸:P<(並,F)> =None;for 不 in l.于().iter(){if 乽.contains(&不){continue;}let 亂=(G::q(不).与()&上.free&丣).count();if 亂==0{乽.push(不);}else if 亸.is_none_or(|(n,_)|亂<n){亸=Some((亂,不));}}let
Some((_,不))=亸 else{break;};上.丄.clear(不);match 上.下(G::q(不).与()&上.free&丣){Some(乱)if 业(l,不,乱,丆)=>上.众(乱),_=>{上.丄.set(不);乽.push(不);}}}for 不 in 乽{if!l.于().test(不){continue;}let
丐=[上.乑,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().举(不,q),q))});if let Some(丐)=丐{业(l,不,丐,丆);}}}fn 倅(l:&mut I,上:&mut 事<'_>,丆:&mut
Vec<K>){let 丁=上.丁;let p=上.书;let h=(p.伢-C::from(l.乢())).max(p.佱);let 倣=(C::from(俭(l))-p.佝).max(0)/p.作.max(1)+p.伎;loop{let 丈=l.丈(丁);let 丄=l.伹(丁)as C;let
倊=C::from(上.乑.count());let q=上.下(l.亚(丁)&上.free&l.乀(丁,1));let 侑=倪(h,(倊/(丄+1)).min(倣));let 佘=q.is_some()&&丈>=佌&&侑>0;let 佲=4*h-亇;let 主=佳(l,上);let 信=主.is_some()&&丈>=亇&&佲>0;if
信&&(!佘||佲>=侑){let(x,y)=仝(主.unwrap_or_default());if 仔(l,K::乲{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佘)else{break;};let(x,y)=仝(q);if!仔(l,K::乒{一:1,x,y},丆){break;}上.众(q);}}fn
佳(l:&I,上:&事<'_>)->P<F>{let 丁=上.丁;(l.map().丱&l.七(丁)&l.产()).iter().filter(|&主|上.theirs[主]>上.书.伲).min_by_key(|&主|(l.map().举(Z[丁],主),主))}fn 俫(l:&mut I,上:&mut
事<'_>,丠:J,丄:J,丆:&mut Vec<K>,){let 丁=上.丁;for _ in 0..丠{let Some(主)=佳(l,上)else{break;};let(x,y)=仝(主);if!仔(l,K::乲{x,y},丆){break;}}for _ in 0..丄{let Some(q)=上.下(l.亚(丁)&上.free&l.乀(丁,1))else{break;};let(x,y)=仝(q);if!仔(l,K::乒{一:1,x,y},丆){break;}上.众(q);}}fn
仹(l:&I)->Q{if l.且().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丰=交::new(l,丁,true);l.丞(1-丁).iter().any(|q|丰.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亞:J,二:乇,}const 伺:J=J::MAX;const 侘:[乇;8]=[乇::仢,乇::予{丠:0,丄:0},乇::予{丠:0,丄:1},乇::予{丠:0,丄:2},乇::予{丠:0,丄:伺,},乇::予{丠:1,丄:0},乇::予{丠:1,丄:1},乇::予{丠:1,丄:伺,},];struct
伾<'a>{丛:&'a[Y],仏:Y,rest:丢,乹:P<乗>,個:Settings,}impl 伾<'_>{fn 书(&self,业:丢)->&Y{&self.丛[F::from(业.亞)%self.丛.len()]}fn 丅(&self,l:&I,业:丢)->L<K>{侏(l,self.书(业),业.二)}}fn
伳(l:&I,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伩=|什:U|hash=(hash^什).wrapping_mul(0x0100000001b3);for(借,临)in[l.丞(丁),l.丄(丁),l.丠()].into_iter().enumerate(){for
q in 临.iter(){伩(((借 as U)<<8)|q as U);}伩(0xff);}伩(l.丈(丁)as U);hash}enum 亣{代{丷:F,乩:F,乕:亠,},}struct 亊{亃:亣,丛:L<Y>,仏:Y,世:L<丢>,们:L<U>,亹:F,丯:並,仓:Q,}impl 亊{fn
new(亃:亣,丛:L<Y>,仏:Y)->亊{亊{亃,丛,仏,世:L::new(),们:L::new(),亹:0,丯:0,仓:false,}}fn 倃(&mut self,l:&I)->Q{let j=self;let 倖=matches!(j.亃,亣::代{..});let 倄=!倖&&j.丯>0&&j.亹>=j.们.len();if!j.仓&&(倄||仹(l)){j.仓=true;}!j.仓}fn
end(&mut self){self.仓=true;}fn 上(&mut self,l:&I,乹:P<乗>)->L<K>{let j=self;let 佟=伾{丛:&j.丛,仏:j.仏,rest:丢{亞:0,二:乇::仢,},乹,個:Settings::DEFAULT,};let 丁=l.东();let
俽=j.们.get(j.亹)==Some(&伳(l,丁));if!俽{let 乎=j.丯==0;j.世=match j.亃{亣::代{丷,乩,乕,}=>{let 乕=&乕;let 九=促;let mut 之=世(l,&j.丛,九,丷,乩,乹,乕,);之.resize(丷,九);之}};j.们=match
j.亃{亣::代{..}=>倗(l,&j.丛,&j.世),};j.亹=0;j.丯+=1;}let 业=j.世.get(j.亹).copied().unwrap_or(佟.rest);j.亹+=1;佟.丅(l,业)}}fn 倗(l:&I,丛:&[Y],世:&[丢])->L<U>{let 丁=l.东();let
mut 佮=L::with_capacity(世.len());let mut 乧=l.clone();for&业 in 世{if 乧.且().is_some(){break;}佮.push(伳(&乧,丁));乧=伍(&乧,&丅(&乧,丛,业));}佮}const 俍:C=3;struct Settings{丷:C,}impl
Settings{const DEFAULT:Settings=Settings{丷:俍};}fn 之(l:&I,不:G)->[J;M]{let 乖=l.map().valid;let mut 乍=[J::MAX;M];let mut 乁=不&乖;let mut 乆=乁;let mut d=0u8;while!乁.is_empty(){for
q in 乁.iter(){乍[q]=d;}let 丂=(乁.与()&乖).丘(乆);乆|=丂;乁=丂;d=d.saturating_add(1);}乍}fn 倬(l:&I,z:F)->C{let 仑=之(l,l.七(z));let 亅=之(l,l.七(1-z));l.map().valid.iter().map(|q|match
仑[q].cmp(&亅[q]){_ if 仑[q]==J::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 俅:[Y;27]=丛();const
促:丢=丢{亞:16,二:乇::仢,};const fn 丛()->[Y;27]{let 享=[0,2,4];let 亼=[0,3,6];let 亓=[0,1,2];let mut 丆=[Y::伋;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亮=if g>0||t>0{4}else{0};丆[i]=Y::伋.倱(享[g],亼[t],0).倰(亮,亓[w]);i+=1;}丆}#[derive(Default)]struct 伊{丌:C,专:C,丈:C,乾:C,丟:C,仚:C,丰:C,仜:C,}#[derive(Clone,Copy)]struct
亠{丌:C,专:C,丈:C,丟:C,乾:C,仚:C,丰:C,仜:C,}fn 倌(l:&I,z:F)->[C;M]{struct 仂{七:G,义:[u16;M],丵:[u16;M],size:[C;M],丟:[C;M],互:u16,}fn 亝(d:&mut 仂,v:F){d.互+=1;d.义[v]=d.互;d.丵[v]=d.互;d.size[v]=1;for
w in(G::q(v).与()&d.七).iter(){if d.义[w]==0{亝(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.义[v]{d.丟[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.义[w]);}}}let
mut d=仂{七:l.七(z),义:[0;M],丵:[0;M],size:[0;M],丟:[0;M],互:0,};let 买=Z[z];if d.七.test(买){亝(&mut d,买);}d.丟[买]=0;d.丟}fn 倎(l:&I,z:F)->伊{let 七=l.七(z);let 丟=倌(l,z);let
亽=仈(l,G::q(Z[1-z]));let mut 丆=伊{丌:C::from(七.count()),专:l.专(z),丈:l.丈(z),丰:C::MAX,仜:倬(l,z)/2,..伊::default()};for q in 七.iter(){if 丟[q]>0{丆.乾+=1;丆.丟=丆.丟.max(丟[q]);}if
q!=Z[z]&&(G::q(q).与()&七).count()<=1{丆.仚+=1;}丆.丰=丆.丰.min(C::from(亽[q]));}丆}fn 丝(l:&I,z:F,乕:&亠)->C{let s=倎(l,z);let o=乕;o.丌*s.丌+o.专*s.专+o.丈*s.丈-o.丟*s.丟-o.乾*s.乾-o.仚*s.仚-o.丰*s.丰.min(30)+o.仜*s.仜}fn
伍(l:&I,丅:&[K])->I{let mut 丂=l.clone();丂.佼(丅);if 丂.且().is_none(){丂.佼(&[]);}丂}fn 丅(l:&I,丛:&[Y],业:丢)->L<K>{侏(l,&丛[F::from(业.亞)%丛.len()],业.二,)}fn 佻(l:&I,丛:&[Y],世:&[丢],九:丢,丷:F,)->I{let
mut 乧=l.clone();for 上 in 0..丷{if 乧.且().is_some(){break;}let 业=世.get(上).copied().unwrap_or(九);乧=伍(&乧,&丅(&乧,丛,业));}乧}fn 倛(l:&I,丛:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亞 in 0..丛.len().min(F::from(J::MAX)){for&二 in&侘{let 业=丢{亞:亞 as J,二,};let c=丅(l,丛,业);if!丆.iter().any(|(_,个)|*个==c){丆.push((业,c));}}}丆}fn
俙(l:&I,丛:&[Y],九:丢,丷:F,乩:F,乹:P<乗>,乕:&亠,)->L<丢>{let 丁=l.东();let 仒=||乹.is_some_and(|t|乗::now()>=t);let 仕=|世:&[丢]|丝(&佻(l,丛,世,九,丷),丁,乕);let mut 下:(C,L<丢>)=(仕(&[]),L::new());let
mut 乁:L<(I,L<丢>)> =vec![(l.clone(),L::new())];for 俥 in 0..丷{let mut 乸:L<(C,I,L<丢>)> =L::new();let mut 乆:HashMap<U,F> =HashMap::new();for(併,世)in&乁{if 仒(){break;}for(业,c)in
倛(併,丛){let 丂=伍(併,&c);let mut 伖=世.clone();伖.push(业);let v=丝(&丂,丁,乕);let 乏=伳(&丂,丁);match 乆.get(&乏){Some(&丽)if 乸[丽].0>=v=>{}Some(&丽)=>乸[丽]=(v,丂,伖),None=>{乆.insert(乏,乸.len());乸.push((v,丂,伖));}}}}if
乸.is_empty(){break;}乸.sort_by(|a,b|b.0 .cmp(&a.0));乸.truncate(乩);let 乿=俥+1==丷||仒();for(i,(_,_,世))in 乸.iter().take(if 乿{乩}else{1}).enumerate(){if i>0&&仒(){break;}let
v=仕(世);if v>下.0{下=(v,世.clone());}}if 乿{break;}乁=乸.into_iter().map(|(_,s,p)|(s,p)).collect();}下.1}fn 世(l:&I,丛:&[Y],九:丢,丷:F,俸:F,乹:P<乗>,乕:&亠,)->L<丢>{let 丁=l.东();let
仕=|世:&[丢]|丝(&佻(l,丛,世,九,丷),丁,乕);let mut 下:(C,L<丢>)=(仕(&[]),L::new());let mut 乩=2;loop{let 丩=俙(l,丛,九,丷,乩,乹,乕);let v=仕(&丩);if v>下.0{下=(v,丩);}let 仒=乹.is_some_and(|t|乗::now()>=t);if
仒||(乹.is_none()&&乩>=俸){break;}乩*=2;}下.1}const 侢:F=4+12;fn 丷(伤:impl BufRead,mut 仸:impl Write,mut 伏:impl FnMut(&str,&str,並,乗)->串,)->Result<(),串>{let mut
严=佃{使:伤};let Some(乎)=严.line()?else{return Ok(());};let mut 亭=format!("{}\n",乎);严.take(丒(&乎,"mine spot count")?,&mut 亭)?;let mut 上=0;loop{let Some(乎)=严.line()?else{return
Ok(());};let 亜=乗::now();上+=1;let mut 乞=format!("{}\n",乎);严.take(侢-1,&mut 乞)?;for 丸 in["building count","unit count"]{let line=严.侁()?;let n=丒(&line,丸)?;乞.push_str(&line);乞.push('\n');严.take(n,&mut
乞)?;}let 伏=伏(&亭,&乞,上,亜);writeln!(仸,"{}",伏).and_then(|()|仸.flush()).map_err(|亗|format!("turn {}: cannot write the answer: {}",上,亗))?;}}struct 佃<R>{使:R,}impl<R:BufRead>佃<R>{fn
line(&mut self)->Result<P<串>,串>{let mut line=串::new();match self.使.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(亗)=>Err(format!("cannot read input: {}",亗)),}}fn
侁(&mut self)->Result<串,串>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乞:&mut String)->Result<(),串>{for
_ in 0..n{乞.push_str(&self.侁()?);乞.push('\n');}Ok(())}}fn 丒(line:&str,丸:&str)->Result<F,串>{match line.trim().parse::<F>(){Ok(n)if n<=侮=>Ok(n),_=>Err(format!("bad {} {line:?}",丸)),}}const
侮:F=144;const 俉:C=3;const 侩:C=2;const 侳:F=16;fn 倁(l:&I,丆:&mut Vec<K>){let z=l.东();let mut 亃=佈{s:l.clone(),z,乺:1-z,丐:Z[1-z],仯:Z[z],上:l.乢(),丆,};亃.run();}struct
佈<'a>{s:I,z:F,乺:F,丐:F,仯:F,上:並,丆:&'a mut Vec<K>,}fn 伀(q:F)->(J,J){((q%O)as J,(q/O)as J)}fn 伪(丑:並,q:F)->K{let(x,y)=伀(q);K::乵{丑,x,y}}fn 侎(一:J,q:F)->K{let(x,y)=伀(q);K::乒{一,x,y}}impl
佈<'_>{fn run(&mut self){let j=self;if j.仳(){return;}j.亀();if j.仳(){return;}j.乔();j.倨();}fn 乍(&self,不:F,乱:F)->C{C::from(self.s.map().举(不,乱))}fn 丬(&mut self,三:K)->Q{let
丿=self.s.丬(三);if 丿{self.丆.push(三);}丿}fn 亿(&self,両:&I)->Q{両.且()==Some(么::乡(self.z))}fn 仳(&mut self)->Q{let j=self;if j.s.且().is_some(){return false;}for
q in j.s.于().iter(){if j.乍(q,j.丐)!=1{continue;}let Some((丑,_))=j.s.仛(q)else{continue;};let 三=伪(丑,j.丐);let mut 両=j.s.clone();if 両.丬(三)&&j.亿(&両){return j.丬(三);}}let
Some(丨)=j.俢()else{return false;};let mut 両=j.s.clone();if!丨.iter().all(|&三|両.丬(三))||!j.亿(&両){return false;}for 三 in 丨{j.丬(三);}true}fn 俢(&self)->P<L<K>>{let
j=self;const 佀:C=C::MAX;let 乚=j.s.两().乣();let 丣:L<G> =(1..=乚).map(|一|j.s.乀(j.z,一)).collect();let 今=|q:F|{丣.iter().position(|临|临.test(q)).map(|丏|(丏 as J+1,10*(丏
as C+1)))};let 七=j.s.七(j.z);let mut 万=[佀;M];let mut 丧=[F::MAX;M];let mut 丿=[false;M];for q in 七.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丿[q]&&万[q]<佀&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丿[q]=true;if q==j.丐{break;}for&乃 in&j.s.map().与[0][q]{if 乃==习{break;}let 乃=F::from(乃);if 丿[乃]||七.test(乃){continue;}let Some((_,亏))=今(乃)else{continue;};if
万[q]+亏<万[乃]{万[乃]=万[q]+亏;丧[乃]=q;}}}if 万[j.丐]>j.s.丈(j.z){return None;}let mut 丨=L::new();let mut q=j.丐;while!七.test(q){let(一,_)=今(q)?;丨.push(侎(一,q));q=丧[q];}丨.reverse();Some(丨)}fn
乮(&self,丹:&I)->C{let(丁,云)=(self.z,self.乺);let 丧=&self.s;let 丒=|临:G|C::from(临.count());let 丕=丒(丹.丞(丁))-丒(丧.丞(丁));let 仼=丒(丧.丞(云))-丒(丹.丞(云));let 俵=丒(丧.七(云))-丒(丹.七(云));let
仰=丧.亲(云)-丹.亲(云);let 丠=丒(丧.丠()&丧.丞(云))-丒(丹.丠()&丹.丞(云));12*丕+20*仼+8*(俵-仼).max(0)+5*仰+25*丠}fn 來(&self,q:F)->C{let j=self;let 主=j.s.map().丱.test(q);if 主&&j.s.两().以()&&!j.s.丞(j.z).test(q){20}else{0}}fn
佤(&self,乄:C)->Q{let j=self;j.s.丄(j.乺).iter().any(|q|j.乍(q,j.仯)<=乄)}fn 亀(&mut self){let j=self;let mut 丄:L<(C,F)> =j.s.于().iter().map(|q|(j.乍(q,j.丐),q)).collect();丄.sort_unstable();for(仮,q)in
丄{if j.s.且().is_some(){return;}let Some((丑,一))=j.s.仛(q)else{continue;};let 丣=j.s.乀(j.z,一);let 乃=j.s.map().与[j.z][q];let mut 下:P<(C,F)> =None;for&丂 in 乃.iter().take_while(|&&丂|丂!=习){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.丬(伪(丑,丂)){continue;}let 丝=if j.亿(&両){100000}else{j.乮(&両)+10*(仮-j.乍(丂,j.丐))+j.來(丂)};if 下.is_none_or(|(了,_)|丝>了){下=Some((丝,丂));}}if
let Some((丝,丂))=下{if 丝>0{j.丬(伪(丑,丂));}}}}fn 乔(&mut self){let j=self;if j.s.且().is_some(){return;}if j.佤(俉){j.倥();}else{j.丠();}}fn 倥(&mut self){let j=self;if!j.s.两().仦()||j.s.丈(j.z)<亇{return;}let
乃=j.s.map().与[j.z][j.仯];let 丌:L<F> =乃.iter().take_while(|&&q|q!=习).map(|&q|F::from(q)).collect();let 为=j.s.丞(j.z);if 丌.iter().any(|&q|为.test(q)&&j.s.乂().test(q)){return;}for
q in 丌{let(x,y)=伀(q);if j.丬(K::乳{x,y}){return;}}}fn 丠(&mut self){let j=self;if!j.s.两().以(){return;}while j.s.丈(j.z)>=亇{let 乭=j.s.map().丱&j.s.丞(j.z)&j.s.七(j.z)&j.s.产();let
Some(q)=乭.iter().min_by_key(|&q|(j.乍(q,j.仯),q))else{return;};let(x,y)=伀(q);if!j.丬(K::乲{x,y}){return;}}}fn 倨(&mut self){let j=self;let 侂=if j.上<=3{0}else{5};for
_ in 0..侳{if j.s.且().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俴=j.佤(侩);let 亳=j.s.亚(j.z);let mut 下:P<(C,K)> =None;let mut 伵:P<(C,K)> =None;let
mut 何=G::T;for 一 in 1..=j.s.两().乣(){let 丣=j.s.乀(j.z,一);let 亂=(亳&丣).丘(何);何=丣;let 亏=10*C::from(一);if 亏>丈{continue;}for q in 亂.iter(){let 三=侎(一,q);let mut
両=j.s.clone();if!両.丬(三){continue;}let 伦=両.丈(j.z);let 专=両.专(j.z);let 仰=j.s.亲(j.乺)-両.亲(j.乺);let 俕=if 仰>0&&俴{伦+专>=0}else{专>=0&&(伦>=侂||仰>0)};if!俕{continue;}let
乮=j.乮(&両);let 丝=乮+j.來(q)-6*j.乍(q,j.丐)-10*(C::from(一)-1);if 乮>0{if 下.is_none_or(|(了,_)|丝>了){下=Some((丝,三));}}else if 专>=3&&伦>=侂+10&&一==1{if 伵.is_none_or(|(了,_)|丝>了){伵=Some((丝,三));}}}}let
Some((_,三))=下.or(伵)else{return;};if!j.丬(三){return;}}}}fn 俰(l:&I,丆:&mut Vec<K>){丆.clear();倁(l,丆);}const 乶:C=C::MAX;const 侲:F=8;struct 交{万:[C;M],丧:[J;M],}fn
仪(l:&I,z:F,亀:Q,丐:F)->C{会(l,z,亙(l,z,亀),丐)}fn 会(l:&I,z:F,亙:G,丐:F)->C{let mut 万=乶;佧(l,z,亙,|业,new|{if new.test(丐){万=10*业 as C;}万!=乶});万}fn 倆(l:&I,z:F,亙:G,中:C)->G{let
mut 仹=G::T;佧(l,z,亙,|业,new|{if 10*业 as C>中{return true;}仹|=new;false});仹.丘(l.七(z))}fn 亙(l:&I,z:F,亀:Q)->G{if 亀&&z==l.东(){l.于()}else{G::T}}fn 佧(l:&I,z:F,亙:G,mut
亝:impl FnMut(F,G)->Q,){let 乚=l.两().乣();let mut 今=[G::T;3];let mut 乆=G::T;for 一 in 1..=乚{let 临=l.乀(z,一);今[F::from(一)-1]=临.丘(乆);乆|=临;}let mut 丶=[l.七(z),G::T,G::T,G::T];for
q in(亙&l.丄(z)).iter(){if let Some(一)=l.五(q).filter(|&一|一<=乚){丶[0]|=G::q(q).与()&l.乀(z,一);}}let mut 丿=G::T;for 业 in 0..{let new=丶[业%4].丘(丿);丶[业%4]=G::T;if
亝(业,new){return;}丿|=new;let 丂=new.与().丘(丿);for(俖,丌)in 今.iter().enumerate(){丶[(业+俖+1)%4]|=丂&*丌;}if 丶.iter().all(|临|临.丘(丿).is_empty()){break;}}}impl 交{fn
new(l:&I,z:F,亀:Q)->交{let 乚=F::from(l.两().乣());let mut 丣=[G::T;3];for(丏,临)in 丣.iter_mut().enumerate().take(乚){*临=l.乀(z,丏 as J+1);}let 七=l.七(z);let 与=&l.map().与[0];let
mut 万=[乶;M];let mut 丧=[习;M];let mut 丶=[0u8;M];let mut 丒=0;let mut 侄=G::T;for q in 七.iter(){万[q]=0;丶[丒]=q as J;丒+=1;}if 亀&&z==l.东(){for 不 in l.于().iter(){let
Some(一)=l.五(不)else{continue;};let 之=G::q(不).与()&丣[F::from(一)-1];for q in 之.iter(){if 万[q]!=0{万[q]=0;丶[丒]=q as J;丒+=1;}}}}while 丒>0{let mut 下=0;for 丏 in
1..丒{let(a,b)=(F::from(丶[丏]),F::from(丶[下]));if(万[a],a)<(万[b],b){下=丏;}}let q=F::from(丶[下]);丒-=1;丶[下]=丶[丒];侄.set(q);for&丂 in&与[q]{if 丂==习{break;}let 丂=F::from(丂);if
侄.test(丂)||七.test(丂){continue;}let Some(一)=丣[..乚].iter().position(|临|临.test(丂))else{continue;};let 侍=万[q]+10*(一 as C+1);if 侍<万[丂]{if 万[丂]==乶{丶[丒]=丂 as
J;丒+=1;}万[丂]=侍;丧[丂]=q as J;}}}交{万,丧}}fn path(&self,丐:F)->L<F>{let mut path=L::new();if 丐>=M||self.万[丐]==乶{return path;}let mut q=丐;while self.丧[q]!=习{path.push(q);q=F::from(self.丧[q]);}path.reverse();path}}struct
仠{丅:L<K>,万:C,}fn 俷(l:&I,z:F,q:F)->P<J>{(1..=l.两().乣()).find(|&一|l.乀(z,一).test(q))}fn 倉(丮:&mut I,丁:F,亪:F,中:C,丅:&mut Vec<K>,乨:&mut i64,)->Q{loop{if 丮.且()==Some(么::乡(丁)){return
true;}if 丮.丞(丁).test(亪)&&丮.七(丁).test(亪){return true;}let 丰=交::new(丮,丁,false);let 万=丰.万[亪];if 万==乶||*乨+万>中{return false;}let Some(&q)=丰.path(亪).first()else{return
false;};let Some(一)=俷(丮,丁,q)else{return false;};let 三=K::乒{一,x:(q%O)as J,y:(q/O)as J,};if!丮.丬(三){return false;}*乨+=10*C::from(一);丅.push(三);}}fn 体(l:&I,丁:F,俯:&[F],中:C)->P<仠>{let
mut 丮=l.clone();let mut 丅=L::new();let mut 乨=0;for&亪 in 俯{if!倉(&mut 丮,丁,亪,中,&mut 丅,&mut 乨){return None;}}(乨<=中).then_some(仠{丅,万:乨,})}fn 伕(l:&I,丐:F,中:C)->P<仠>{if
中<0||l.且().is_some()||丐>=M||l.map().丼.test(丐){return None;}let 丁=l.东();if l.丞(丁).test(丐)&&l.七(丁).test(丐){return Some(仠{丅:L::new(),万:0,});}let mut 下=体(l,丁,&[丐],中);let
乎=交::new(l,丁,false);let mut 乂:L<F> =(l.乂()&l.丞(1-丁)).iter().filter(|&乌|乌!=丐&&乎.万[乌]!=乶).collect();乂.sort_by_key(|&乌|(l.map().举(乌,丐),乌));乂.truncate(侲);for
乌 in 乂{if let Some(丨)=体(l,丁,&[乌,丐],中){if 下.as_ref().is_none_or(|了|丨.万<了.万){下=Some(丨);}}}下}const 侰:F=20000;struct 仡{丅:L<K>,万:C,丌:C,丄:C,乔:C,专:C,丙:C,}struct
伅{丌:C,丄:C,乔:C,}fn 佫(l:&I,z:F)->伅{let 七=l.七(z);let 丄=l.丄(z)&七;let 丝=丄.iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum();伅{丌:C::from(七.count()),丄:丝,乔:C::from(((l.丠()|l.乂())&七).count()),}}struct
佊{丁:F,仁:G,丧:伅,专:C,亱:F,中:C,亶:F,path:L<K>,丩:L<仡>,index:HashMap<仄,F,BuildHasherDefault<DefaultHasher>>,}type 仄=[U;3];fn 倯(mut 乏:仄,q:F)->仄{乏[q/64]|=1<<(q%64);乏}impl
佊{fn 佢(&mut self,丮:&I,乿:P<F>,仼:仄,万:C,)->Q{let j=self;let 丁=j.丁;let 亂=match 乿{None=>丮.亚(丁),Some(q)=>G::q(q).与(),}&j.仁;let 亂=亂.丘(丮.丞(丁));let mut 丣=[G::T;3];for
一 in 1..=丮.两().乣(){丣[F::from(一)-1]=丮.乀(丁,一);}for q in 亂.iter(){let 乚=F::from(丮.两().乣());let Some(丏)=丣[..乚].iter().position(|临|临.test(q))else{continue;};let
一=丏 as J+1;let 亏=10*C::from(一);if 万+亏>j.中{continue;}if j.亶>=侰{return false;}j.亶+=1;let mut 丂=丮.clone();let 三=K::乒{一,x:(q%O)as J,y:(q/O)as J,};if!丂.丬(三){continue;}j.path.push(三);let
亻=倯(仼,q);j.倇(&丂,亻,万+亏);let 俤=j.path.len()<j.亱&&丂.且().is_none();let 修=!俤||j.佢(&丂,Some(q),亻,万+亏);j.path.pop();if!修{return false;}}true}fn 倇(&mut self,丹:&I,亻:仄,万:C){let
j=self;let 仐=1-j.丁;let 伬=佫(丹,仐);let 丌=j.丧.丌-伬.丌;let 丄=j.丧.丄-伬.丄;if 丌<=0&&丄<=0{return;}let 乔=j.丧.乔-伬.乔;let 专=丹.专(j.丁)-j.专;let 倲=丹.且()==Some(么::乡(j.丁));let
丙=if 倲{C::MAX/2}else{3*丌+丄+10*乔+2*专-万};if let Some(&丽)=j.index.get(&亻){if 万>=j.丩[丽].万{return;}}let 丟=仡{丅:j.path.clone(),万,丌,丄,乔,专,丙,};match j.index.get(&亻){Some(&丽)=>j.丩[丽]=丟,None=>{j.index.insert(亻,j.丩.len());j.丩.push(丟);}}}}fn
你(l:&I,亱:F,中:C,乄:F)->L<仡>{乐(l,亱,中,乄).0}fn 乐(l:&I,亱:F,中:C,乄:F)->(L<仡>,F){if l.且().is_some()||亱==0||乄==0{return(L::new(),0);}let 丁=l.东();let 仐=1-丁;let mut
伱=佊{丁,仁:l.丞(仐)|l.丞(仐).与(),丧:佫(l,仐),专:l.专(丁),亱,中,亶:0,path:L::new(),丩:L::new(),index:HashMap::default(),};伱.佢(l,None,[0;3],0);let 亶=伱.亶;let mut 丩=伱.丩;丩.sort_by(|a,b|b.丙.cmp(&a.丙).then(a.万.cmp(&b.万)));丩.truncate(乄);(丩,亶)}const
令:C=1000000;const 侱:C=200;fn 乑(l:&I,z:F)->C{let 七=l.七(z);let 丄:C=(l.丄(z)&七).iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.专(z)+4*C::from(七.count())+丄+10*C::from((l.乂()&七).count())}fn
佰(l:&I,z:F,乛:G)->C{俬(l,z,会(l,z,乛,Z[1-z]),)}fn 俬(l:&I,z:F,万:C)->C{if 万==乶{return C::MIN;}l.丈(z)+l.专(z)-万}fn 佽(仵:C)->C{仵.clamp(-侱,0)}fn 俪(l:&I,z:F,乛:[G;2],)->(C,[C;2]){let
个=1-z;match l.且(){Some(么::乡(倮))=>{return(if 倮==z{令}else{-令},[0,0])}Some(么::伿)=>return(0,[0,0]),None=>{}}let mut 丙=乑(l,z)-乑(l,个);if z==l.东(){let mut 丂=l.clone();丂.亨();if
丂.且().is_none()&&仪(&丂,个,true,Z[z])<=丂.丈(个){丙-=令/2;}}let 亰=[佰(l,z,乛[0]),佰(l,个,乛[1]),];(丙+佽(亰[0])-佽(亰[1]),亰)}
