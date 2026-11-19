// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 並=u32;type U=u64;type 仞=i32;type C=i64;type 亟=f64;type Q=bool;type 串=String;type 乖=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侠:Q=true;fn 俭(l:&J,q:F,仙:C,丰:G,丨:C)->Q{let
丁=l.丞();let 乹=1-丁;let 佩=侠&&丨<=l.丈(乹)+2*l.且(乹).max(0);let 什=G::q(q)|G::q(q).与();if!佩&&(什&丰).is_empty(){return false;}let(x,y)=井(q);let mut 主=l.clone();if!主.丬(K::乳{x,y}){return
false;}if 倛(&主,丁)<仙{return true;}佩&&优(&主,乹,false,Z[丁])>丨}const 俍:Q=false;const 侻:I=2;const 侧:F=4;const 伹:F=5;#[derive(Clone)]enum 丳{亡(K),伅(L<K>),}impl
丳{fn 丅(&self)->&[K]{match self{丳::亡(三)=>std::slice::from_ref(三),丳::伅(丅)=>丅,}}}fn 井(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 佤(l:&J,丆:&mut Vec<丳>){丆.clear();if
l.丗().is_some(){return;}let 丁=l.丞();let 乚=l.两().乢();let mut 丣=[G::T;4];for 一 in 1..=乚{丣[F::from(一)]=l.丿(丁,一);}for 不 in l.于().iter(){let Some((丑,一))=l.仛(不)else{continue;};let
义=G::q(不).与()&丣[F::from(一)];for 乱 in 义.iter(){let(x,y)=井(乱);丆.push(丳::亡(K::乵{丑,x,y}));}}let 丈=l.丈(丁);let 七=l.七(丁);let 亲=l.亚(丁);let 倢=match 侻{0=>亲,1=>{let
伏=l.map().valid.丘(七).与()&七;亲.丘(七)|伏}_=>亲.丘(七),};for 一 in 1..=乚{let 万=10*C::from(一);if 丈<万{break;}let mut 丌=倢&丣[F::from(一)];for 俱 in 1..一{丌=丌.丘(丣[F::from(俱)]);}for
q in 丌.iter(){let(x,y)=井(q);丆.push(丳::亡(K::乑{一,x,y}));}}if 丈>=了{let 乭=l.丝(丁)&亲&l.产();if l.两().以(){for q in(乭&l.map().丱).iter(){let(x,y)=井(q);丆.push(丳::亡(K::乲{x,y}));}}if
l.两().仦(){let 伏=l.map().valid.丘(l.丝(丁)).与();let 丌=(乭&伏).丘(l.map().丱);let 丰=丰(l,1-丁);let 仙=C::from((丰&l.七(丁)).count());let 丨=优(l,1-丁,false,Z[丁]);let 丶=if
俍{let(们,俬)=乀(l,丁);们.丘(俬)}else{G::T};for q in 丌.iter().filter(|&q|{let 什=G::q(q)|G::q(q).与();!(什&丶).is_empty()||俭(l,q,仙,丰,丨)}){let(x,y)=井(q);丆.push(丳::亡(K::乳{x,y}));}}}for
丟 in 作(l,伹,丈,侧){if 丟.丅.len()>1{丆.push(丳::伅(丟.丅));}}if let Some(丨)=伓(l,Z[1-丁],丈){if 丨.丅.len()>1{丆.push(丳::伅(丨.丅));}}}const 侳:I=2;const 俅:C=5;fn 乛(l:&J,z:F)->G{伩(l,z,侳)}fn
伩(l:&J,z:F,不:I)->G{let mut 乛=G::T;if 不>3{return 乛;}for q in(l.丄(z)&l.七(z)).iter(){if l.五(q).is_some_and(|一|一>=不){乛|=G::q(q);}}乛}fn 丰(l:&J,乐:F)->G{let 中=l.丈(乐)+l.且(乐);倁(l,乐,乛(l,乐),中)}fn
介(l:&J,z:F)->G{丰(l,1-z)&l.七(z)}fn 倛(l:&J,z:F)->C{C::from(介(l,z).count())}const 佁:C=4;fn 俴(丌:C)->C{丌/佁}fn 丠(l:&J,z:F,介:G)->C{let 七=l.七(z);let 丠=(l.丠()&七).丘(介);let
丌=C::from(七.count());let 俘=if l.乡()<=侗{丌/侖+侕}else{俴(丌)};俘*C::from(丠.count())}const 侗:並=15;const 侕:C=6;const 侖:C=佁;const 俁:C=2;fn 侏(l:&J,z:F)->C{let 七=l.七(z);let[a,b,c]=七.份();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 丏=(hash as F)&(佊-1);俎.with(|休|{let mut 休=休.borrow_mut();let
仺=&mut 休[丏];if 仺.0==七&&仺.1==z as I+1{return 仺.2;}let 东=倨(七,Z[z]);*仺=(七,z as I+1,东);东})}const 佊:F=1<<12;thread_local!{static 俎:std::cell::RefCell<L<(G,I,C)>>
=std::cell::RefCell::new(vec![(G::T,0,0);佊]);}fn 倨(七:G,买:F)->C{struct 仁{七:G,么:[I;M],丵:[I;M],size:[I;M],互:I,乽:C,仵:C,}fn 亝(d:&mut 仁,v:F)->I{d.互+=1;let 么=d.互;d.么[v]=么;let
mut 丵=么;let mut 伲=1u8;let mut 丟=0u8;for w in(G::q(v).与()&d.七).iter(){if d.么[w]==0{let 來=亝(d,w);伲+=來;丵=丵.min(d.丵[w]);if d.丵[w]>=么{丟+=來;}}else{丵=丵.min(d.么[w]);}}d.丵[v]=丵;d.size[v]=伲;if
丟>0&&d.么[v]>1{d.乽+=1;d.仵=d.仵.max(C::from(丟));}伲}if!七.test(买){return 0;}let mut d=仁{七,么:[0;M],丵:[0;M],size:[0;M],互:0,乽:0,仵:0,};亝(&mut d,买);d.乽+d.仵}fn 倈(l:&J,z:F)->C{俁*(侏(l,1-z)-侏(l,z))}fn
佝(l:&J,z:F)->C{if l.丈(z)+l.且(z)>=0{return 0;}(l.丄(z)&l.七(z)).iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum()}const 侦:C=15;const 俋:[C;2]=[3,1];const
俉:[C;2]=[4,20];fn 倗(l:&J,z:F)->[C;2]{let mut 丒=[0;2];for q in(l.丄(z)&l.七(z)).iter(){match l.五(q){Some(2)=>丒[0]+=1,Some(3)=>丒[1]+=1,_=>{}}}丒}fn 侅(l:&J,z:F)->C{let[例,倜]=倗(l,z);let
佐=|一:F|(3-俋[一])*俉[一];-侦*例+佐(0)*例+佐(1)*倜}const 侟:C=0;const 侢:C=2;const 価:C=100;fn 侃(仴:C)->C{if 仴<0{return 0;}侟+侢*仴.min(価)/2}const 佀:I=2;const 伾:C=2;const
伽:C=4;const 侣:I=3;fn 乼(l:&J,z:F)->C{if 伾==0{return 0;}let 乕=l.map().valid;let mut 丄=G::T;for q in(l.丄(z)&l.七(z)).iter(){if l.五(q).is_some_and(|一|一<=侣){丄|=G::q(q);}}let
mut 仼=乕.丘(l.丝(z));let mut 会=G::T;let mut 义=0;for 丽 in 1..=伽{仼=(仼|仼.与())&乕;let 仭=(丄&仼).丘(会);义+=(丽-1)*C::from(仭.count());会|=仭;}义+=伽*C::from(丄.丘(会).count());伾*义}const
佉:C=8;fn 乀(l:&J,z:F)->(G,G){let 七=l.七(z);let 们=七&l.丝(1-z).与();let 乁=l.乁()&七;(们,们&(乁|乁.与()))}fn 侎(l:&J,z:F)->C{if 佉==0{return 0;}佉*C::from(乀(l,z).1 .count())}fn
伜(l:&J,z:F)->C{let 乛=[伩(l,z,佀),伩(l,1-z,佀),];let(丙,亯)=俥(l,z,乛);let 丙=丙+侃(亯[0])-侃(亯[1]);if l.丗().is_some(){return 丙;}let 仑=介(l,z);let 亄=介(l,1-z);let 仙=C::from(亄.count())-C::from(仑.count());丙-2*佝(l,z)+2*佝(l,1-z)+倈(l,z)+侅(l,z)-侅(l,1-z)-乼(l,z)+乼(l,1-z)+俅*仙/4+丠(l,z,仑)-丠(l,1-z,亄)+侎(l,z)-侎(l,1-z)}const
侰:F=8;const 侼:F=3;const 侨:F=32;const 侽:U=300;const 侔:亟=0.75;const 侜:P<並> =Some(7);const 侚:Q=false;const 侙:C=10;const 供:並=6;struct 予{亃:P<乖>,亘:P<乖>,从:U,}#[derive(Default)]struct
佇{丅:L<K>,众:F,乺:U,世:F,丯:F,何:F,乻:F,乯:Q,}const 俌:Q=true;#[derive(Clone)]struct 乘{丅:L<K>,伛:C,}struct 亢{丅:L<K>,end:J,first:P<U>,专:P<L<乘>>,}type 仂=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
伇{丁:F,limit:予,乺:U,仍:仂,些:P<L<丳>>,}const 伸:Q=true;const 侑:Q=false;const 伷:P<I> =None;fn 俖(l:&J,世:&[乘],丆:&mut Vec<丳>){if!伸&&伷.is_none(){return;}let 仗=世.iter().map(|乜|乜.丅.as_slice());俗(l,仗,(伸,侑,伷),丆,);}fn
俗<'a>(l:&J,仗:impl Iterator<Item=&'a[K]>,(乁,与,一):(Q,Q,P<I>),丆:&mut Vec<丳>,){let 丁=l.丞();let 丸=l.七(丁);let mut 仉=G::T;for 乜 in 仗{for 三 in 乜{if let K::乑{x,y,..}|K::乵{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丸.test(q){仉|=G::q(q);}}}}if 仉.is_empty(){return;}let 乭=l.产()&丸;let 丈=l.丈(丁);let 佺=|三:K,丆:&mut Vec<丳>|{if!丆.iter().any(|a|a.丅()==[三]){丆.push(丳::亡(三));}};if
乁&&丈>=了&&l.两().仦(){let mut 丌=仉;if 与{丌|=仉.与();}for q in(丌&乭).丘(l.map().丱).iter(){let(x,y)=井(q);佺(K::乳{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().乢()){if
丈>=10*C::from(一){for q in(仉&乭).iter(){let(x,y)=井(q);佺(K::乑{一,x,y},丆);}}}}fn 伎(l:&mut J,丅:&[K])->Q{丅.iter().all(|&三|l.丬(三))}fn 乎(丅:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丅{let(仰,a,b,c)=match*三{K::乵{丑,x,y}=>(1,U::from(丑),x,y),K::乑{一,x,y}=>(2,U::from(一),x,y),K::乲{x,y}=>(3,0,x,y),K::乳{x,y}=>(4,0,x,y),};let 亿=仰|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亿).wrapping_mul(0x0100000001b3);}hash}fn
位(乙:&J,乜:&[K])->J{let mut 丂=乙.clone();丂.仈();for&三 in 乜{丂.丬(三);}丂.仈();丂}fn 仲(l:&J)->P<L<K>>{let 丁=l.丞();let 丐=Z[1-丁];let 亾=|両:&J|両.丗()==Some(之::乫(丁));for
q in l.于().iter(){if l.map().丽(q,丐)!=1{continue;}let Some((丑,_))=l.仛(q)else{continue;};let(x,y)=井(丐);let 三=K::乵{丑,x,y};let mut 両=l.clone();if 両.丬(三)&&亾(&両){return
Some(vec![三]);}}let 丨=伓(l,丐,l.丈(丁))?;let mut 両=l.clone();(伎(&mut 両,&丨.丅)&&亾(&両)).then_some(丨.丅)}impl 伇{fn 乧(&self)->Q{self.乺>=self.limit.从||self.limit.亃.is_some_and(|t|乖::now()>=t)}fn
丙(&mut self,乙:&J,世:&[乘],伐:C)->C{self.乺+=1;self.伭(乙,世,伐)}fn 伭(&self,乙:&J,世:&[乘],伐:C)->C{if 乙.丗()==Some(之::乫(self.丁)){return 令;}if 世.is_empty(){return 伜(乙,self.丁);}let
mut 仾=C::MAX;for 乜 in 世{仾=仾.min(伜(&位(乙,&乜.丅),self.丁));if 仾<=伐{break;}}仾}#[allow(clippy::too_many_arguments)]fn 俻(&mut self,乬:&J,亦:C,乒:&[丳],亶:&[(C,F)],举:&mut[Q],世:&[乘],亨:&mut
仂,伣:Q,)->(P<(F,J,C)>,Q){let mut 下=None;let mut 仧=亦;for&(件,丏)in 亶{if 举[丏]{continue;}if 伣&&件!=C::MAX&&件<=仧-亦{return(下,true);}if self.乧(){return(下,false);}举[丏]=true;let
丅=乒[丏].丅();let mut 丂=乬.clone();if!伎(&mut 丂,丅){亨.insert(乎(丅),C::MIN/4);continue;}let 丙=self.丙(&丂,世,仧);亨.insert(乎(丅),丙-亦);if 丙>仧{仧=丙;下=Some((丏,丂,丙));}}(下,false)}fn
伖(&mut self,买:&J,世:&[乘],仄:&[U],俛:Q)->亢{let j=self;let mut 乍=None;let mut 乬=买.clone();let mut 丅=L::new();let mut 亦=j.丙(&乬,世,C::MIN);let mut 亨=仂::default();let
mut 乒=L::new();let mut 亶=L::new();while 乬.丗().is_none()&&!j.乧(){if 丅.is_empty()&&俛{match&j.些{Some(些)=>乒.clone_from(些),None=>{佤(&乬,&mut 乒);j.些=Some(乒.clone());}}}else{佤(&乬,&mut
乒);}俖(&乬,世,&mut 乒);亶.clear();for(丏,乶)in 乒.iter().enumerate(){let k=乎(乶.丅());let 件=亨.get(&k).or(j.仍.get(&k)).copied();亶.push((件.unwrap_or(C::MAX),丏));}亶.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 举=vec![false;乒.len()];if 丅.is_empty(){for(丏,乶)in 乒.iter().enumerate(){举[丏]=仄.contains(&乎(乶.丅()));}}let mut 下=None;for
伣 in[true,false]{let(丩,倐)=j.俻(&乬,亦,&乒,&亶,&mut 举,世,&mut 亨,伣,);下=丩;if 下.is_some()||!倐{break;}}let Some((丏,丂,丙))=下 else{break;};if 丅.is_empty(){乍=Some(乎(乒[丏].丅()));}丅.extend_from_slice(乒[丏].丅());乬=丂;亦=丙;}for(k,乮)in
亨{let 佦=j.仍.entry(k).or_insert(乮);*佦=(*佦).max(乮);}亢{丅,end:乬,first:乍,专:None,}}fn 仗(&self,乙:&J)->L<L<K>>{let mut 丂=乙.clone();丂.仈();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 乣=丂.丞();let 丈=丂.丈(乣);if let Some(丨)=伓(&丂,Z[self.丁],丈){丆.push(丨.丅);}for 丟 in 作(&丂,伹,丈,侼){丆.push(丟.丅);}let mut 佥=L::new();俫(&丂,&mut
佥);丆.push(佥);let mut 亄=伇{丁:乣,limit:予{从:侽,..self.limit},乺:0,仍:仂::default(),些:None,};丆.push(亄.伖(&丂,&[],&[],true).丅);丆.retain(|丅|!丅.is_empty());丆}fn 低(&self,乙:&J)->L<乘>{self.仗(乙).into_iter().map(|丅|{let
伛=伜(&位(乙,&丅),self.丁);乘{丅,伛}}).collect()}fn 乻(&mut self,买:&J,乶:&丳,世:&[乘])->P<亢>{let mut 主=买.clone();if!伎(&mut 主,乶.丅()){return None;}self.乺+=1;let 九=self.伖(&主,世,&[],false);let
mut 丅=乶.丅().to_vec();丅.extend_from_slice(&九.丅);Some(亢{丅,end:九.end,first:Some(乎(乶.丅())),专:None,})}fn 佡(&self,丕:&亢,世:&[乘])->C{let 丙=self.伭(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>丙.min(self.伭(&丕.end,专,C::MIN)),_=>丙,}}}fn
倃(丩:&[乘],世:&mut Vec<乘>)->Q{let 丧:L<_> =世.iter().map(|r|r.丅.clone()).collect();for 乜 in 丩{if 世.iter().any(|r|r.丅==乜.丅){continue;}世.push(乜.clone());}世.sort_by_key(|乜|乜.伛);世.truncate(侰);世.iter().map(|r|&r.丅).ne(&丧)}fn
倚(l:&J,乄:予,乯:&mut 争)->佇{let 丁=l.丞();let mut 丫=佇::default();if l.丗().is_some(){return 丫;}if let Some(丅)=仲(l){丫.丅=丅;return 丫;}let 丼=|倊:亟|match(乄.亃,乄.亘){(Some(亃),Some(亘))=>Some(亃+(亘-亃).mul_f64(倊)),_=>None,};if
侜.is_some_and(|上|l.乡()>=上){乯.end();}if 乯.俾(l){let 丅=乯.上(l,丼(0.4));let 俚=丅.iter().any(|三|{matches!(三,K::乑{..}|K::乲{..}|K::乳{..})});let 乼=侚&&!俚&&l.乡()>=供&&l.丈(丁)>=侙;if!乼{丫.乯=true;丫.丅=丅;return
丫;}乯.end();}let mut 乏=伇{丁,limit:予{亃:丼(侔),..乄},乺:0,仍:仂::default(),些:None,};let mut 世=L::new();let mut 丯:L<亢> =L::new();let mut 仄=L::new();let mut 仪=false;let
mut 乻:L<丳> =L::new();while 丫.众<侨&&(丯.is_empty()||!乏.乧()){let 丕=if 仪{let Some(乶)=乻.pop()else{break;};match 乏.乻(l,&乶,&世){Some(丕)=>丕,None=>continue,}}else{乏.伖(l,&世,&仄,true)};丫.众+=1;let
使=丯.iter().any(|p|p.丅==丕.丅);let 乍=丕.first;let 侄=乏.乧();let mut 丕=丕;let mut 佰=false;if!侄&&!使{let 丩=乏.低(&丕.end);佰=倃(&丩,&mut 世);丕.专=Some(丩);}if!使{丯.push(丕);}if
侄{break;}if!佰&&!仪{match 乍{Some(乍)=>仄.push(乍),None=>{仪=true;乻=乏.些.clone().unwrap_or_default();乻.retain(|a|!仄.contains(&乎(a.丅())));let 乮=|a:&丳|{乏.仍.get(&乎(a.丅())).copied().unwrap_or(C::MIN)};乻.sort_by_key(乮);}}}}丫.乻=F::from(仪);丫.乺=乏.乺;丫.世=世.len();丫.丯=丯.len();let
佫=||乄.亘.is_some_and(|t|乖::now()>=t);let mut 亹:L<P<C>> =vec![None;丯.len()];for 丏 in(0..丯.len()).rev(){if 丏+1<丯.len()&&佫(){break;}亹[丏]=Some(乏.佡(&丯[丏],&世));}let
亷=|亹:&[P<C>]|{let mut 下:P<(C,F)> =None;for(丏,丙)in 亹.iter().enumerate().rev(){if let Some(丙)=*丙{if 下.is_none_or(|(亅,_)|丙>亅){下=Some((丙,丏));}}}下};let mut
下=亷(&亹);乏.limit.亃=乄.亘;while let Some((_,丏))=下{if!俌||丯[丏].专.is_some()||佫(){break;}let 丩=乏.低(&丯[丏].end);丯[丏].专=Some(丩);丫.何+=1;亹[丏]=Some(乏.佡(&丯[丏],&世));下=亷(&亹);}if
let Some((_,丏))=下{let mut 丮=l.clone();for&三 in&丯[丏].丅{if 丮.丬(三){丫.丅.push(三);}}}丫}fn 俽(乄:&dyn Fn(乖,並)->予,mut 乯:争,伢:impl BufRead,价:impl Write,)->Result<(),串>{乂(伢,价,|京,乞,上,亜|match
J::read(京,乞,乩::企,上){Ok(l)=>{let 丫=倚(&l,乄(亜,上),&mut 乯);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,丫.众,丫.乺,丫.世,丫.何,丫.乻,亜.elapsed());俓(&丫.丅)}Err(亗)=>{eprintln!("plan: cannot read the input: {}",亗);"WAIT".to_string()}},)}const
侒:亟=45.0;const 侘:亟=800.0;const 係:亟=0.6;fn 乄(亜:乖,上:並)->予{if let Some(从)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
予{亃:None,亘:None,从,};}let 仹=倝().min(100.0);let 俶=if 上==1{侘}else{侒};let 中=Duration::from_secs_f64(俶/1000.0*仹);予{亃:Some(亜+中.mul_f64(係)),亘:Some(亜+中),从:U::MAX,}}const
侺:F=9;const 侴:亠=亠{丌:3,且:1,丈:1,丟:2,乽:2,仚:2,丰:6,仜:1,};const 侹:I=I::MAX;const 侶:C=2;const 侷:C=18;const 侵:C=0;const 侸:F=32;fn 乯()->争{俹(侸)}fn 俹(乨:F)->争{let
亂=亣::代{乂:侺,乨,乔:侴,};let 丛=俀.iter().map(|&乥|Y{估:侹,仅:侶,传:侷,伌:侵,..乥}).collect();争::new(亂,丛,Y::伉)}fn main(){let 倔=io::stdin();let 倕=io::stdout();if let Err(亗)=俽(&乄,乯(),倔.lock(),倕.lock()){eprintln!("plan: {}",亗);std::process::exit(1);}}const
俆:&str="CG_TIME_SCALE";fn 倝()->亟{俺(std::env::var(俆).ok().as_deref())}fn 俺(东:P<&str>)->亟{东.and_then(|乞|乞.trim().parse::<亟>().ok()).filter(|仹|仹.is_finite()&&*仹>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乗:F=4*O;const 乪:U=(1<<乗)-1;const 仟:U=1|1<<12|1<<24|1<<36;const 伂:U=仟<<11;const 佅:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侐:G=G([乪;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乗]>>(q%乗))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乗]|=1<<(q%乗);}#[inline]fn clear(&mut self,q:F){self.0[q/乗]&=!(1<<(q%乗));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
份(self)->[U;3]{self.0}#[inline]fn count(self)->並{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丘(self,个:G)->G{G([self.0[0]&!个.0[0],self.0[1]&!个.0[1],self.0[2]&!个.0[2],])}#[inline]fn
iter(self)->伀{伀{伦:self.0,丼:0,}}#[inline]fn 倏(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&佅)<<(乗-O)),(b>>O)|((c&佅)<<(乗-O)),c>>O,])}#[inline]fn 倌(self)->G{let[a,b,c]=self.0;G([(a<<O)&乪,((b<<O)&乪)|(a>>(乗-O)),((c<<O)&乪)|(b>>(乗-O)),])}#[inline]fn
倍(self)->G{let[a,b,c]=self.0;G([(a>>1)&!伂&乪,(b>>1)&!伂&乪,(c>>1)&!伂&乪,])}#[inline]fn 倎(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仟&乪,(b<<1)&!仟&乪,(c<<1)&!仟&乪,])}#[inline]fn
与(self)->G{let j=self;j.倏()|j.倌()|j.倍()|j.倎()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,个:G)->G{G([self.0[0]&个.0[0],self.0[1]&个.0[1],self.0[2]&个.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,个:G)->G{G([self.0[0]|个.0[0],self.0[1]|个.0[1],self.0[2]|个.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,个:G){*self=*self|个;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侐.丘(self)}}struct 伀{伦:[U;3],丼:F,}impl Iterator for 伀{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.丼<3{let 仳=j.伦[j.丼];if 仳!=0{j.伦[j.丼]=仳&(仳-1);return Some(j.丼*乗+仳.trailing_zeros()as F);}j.丼+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乵{丑:並,x:I,y:I},乑{一:I,x:I,y:I},乲{x:I,y:I},乳{x:I,y:I},}impl K{fn 借(&self)->串{match*self{K::乵{丑,x,y}=>format!("MOVE {} {x} {y}",丑),K::乑{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::乲{x,y}=>format!("BUILD MINE {x} {y}"),K::乳{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
俓(丅:&[K])->串{if 丅.is_empty(){return "WAIT".to_string();}let mut line=串::with_capacity(16*丅.len());for(丏,三)in 丅.iter().enumerate(){if 丏>0{line.push(';');}line.push_str(&三.借());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乩{佋,倮,俏,#[default]企,}impl 乩{fn 乢(self)->I{match self{乩::佋=>1,_=>3,}}fn 以(self)->Q{matches!(self,乩::俏|乩::企)}fn 仦(self)->Q{self==乩::企}}const 习:I=I::MAX;struct
乴{为:G,valid:G,丱:G,丽:Box<[I]>,与:[[[I;4];M];2],}thread_local!{static 侥:RefCell<P<Arc<乴>>> =const{RefCell::new(None)};}impl 乴{fn 個(为:G,丱:G)->Arc<乴>{侥.with(|乾|{let
mut 乾=乾.borrow_mut();if let Some(亮)=乾.as_ref(){if 亮.为==为&&亮.丱==丱{return Arc::clone(亮);}}let 亮=Arc::new(乴::build(为,丱));*乾=Some(Arc::clone(&亮));亮})}fn build(为:G,丱:G)->乴{let
乕=!为;const 促:[[(仞,仞);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 与=[[[习;4];M];2];for(z,义)in 促.iter().enumerate(){for q in
乕.iter(){let(x,y)=((q%O)as 仞,(q/O)as 仞);let mut 丒=0;for&(俢,俣)in 义{let(佲,佳)=(x+俢,y+俣);if(0..O as 仞).contains(&佲)&&(0..O as 仞).contains(&佳){let 丂=佳 as F*O+佲
as F;if 乕.test(丂){与[z][q][丒]=丂 as I;丒+=1;}}}}}let mut 丽=vec![习;M*M].into_boxed_slice();let mut 伬=[0u8;M];for 不 in 乕.iter(){let 亐=&mut 丽[不*M..(不+1)*M];亐[不]=0;伬[0]=不
as I;let(mut 伞,mut 伴)=(0,1);while 伞<伴{let q=伬[伞]as F;伞+=1;for&丂 in&与[0][q]{if 丂!=习&&亐[丂 as F]==习{亐[丂 as F]=亐[q]+1;伬[伴]=丂;伴+=1;}}}}乴{为,valid:乕,丱,丽,与,}}#[inline]fn
丽(&self,不:F,乱:F)->I{self.丽[不*M+乱]}}const 侯:並=100;const 了:C=15;const 俊:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 之{乫(F),伻,}enum
也{伆,佂,佃,}#[derive(Clone)]struct J{map:Arc<乴>,两:乩,专:[G;2],七:[G;2],丸:G,乊:G,丄:[G;2],一:[G;3],亍:G,亽:[並;M],丈:[C;2],且:[C;2],丞:F,乡:並,亳:並,丗:P<之>,}fn 仌(专:G,享:F)->G{if!专.test(享){return
G::T;}let mut 他=G::q(享);loop{let 丂=(他|他.与())&专;if 丂==他{return 他;}他=丂;}}impl J{fn 丞(&self)->F{self.丞}fn 乡(&self)->並{self.乡}fn 丗(&self)->P<之>{self.丗}fn 两(&self)->乩{self.两}fn
亱(&self,z:F)->C{let j=self;let 丄=j.丄[z];j.丈[z]+10*C::from((丄&j.一[0]).count())+20*C::from((丄&j.一[1]).count())+30*C::from((丄&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伵(&self,z:F)->F{self.丄[z].count()as F}fn map(&self)->&乴{&self.map}fn 丝(&self,z:F)->G{self.专[z]}fn 七(&self,z:F)->G{self.七[z]}fn
丄(&self,z:F)->G{self.丄[z]}fn 仛(&self,q:F)->P<(並,I)>{let j=self;(j.丄[0]|j.丄[1]).test(q).then(||(j.亽[q],j.伥(q)))}fn 于(&self)->G{self.丄[self.丞]&self.亍}fn
丠(&self)->G{self.丸}fn 乁(&self)->G{self.乊}fn 五(&self,q:F)->P<I>{(q<M&&(self.丄[0]|self.丄[1]).test(q)).then(||self.伥(q))}fn 倓(&mut self,z:F){let j=self;j.七=[仌(j.专[0],Z[0]),仌(j.专[1],Z[1])];let
俞=j.丄[z].丘(j.七[z]);j.伮(俞);j.佖();j.丈[z]+=j.且[z];if j.丈[z]<0{j.丈[z]=0;j.伮(j.丄[z]);}j.亍|=j.丄[z];}fn 佖(&mut self){let j=self;for z in 0..2{let 七=j.七[z];let
丄=七&j.丄[z];let mut 且=C::from(七.count())+4*C::from((七&j.丸).count());for(一,倦)in 俊.iter().enumerate(){且-=倦*C::from((丄&j.一[一]).count());}j.且[z]=且;}}fn 仈(&mut
self){let j=self;if j.丗.is_some(){return;}if j.丞==1{if j.乡>=侯{j.丗=Some(match j.亱(0).cmp(&j.亱(1)){std::cmp::Ordering::Greater=>之::乫(0),std::cmp::Ordering::Less=>之::乫(1),std::cmp::Ordering::Equal=>之::伻,});return;}j.乡+=1;}j.丞=1-j.丞;j.倓(j.丞);}fn
佸(&mut self,丅:&[K]){let j=self;if j.丗.is_some(){return;}let 伝=j.亳;for&三 in 丅{if j.信(三,伝){continue;}j.丬(三);if j.丗.is_some(){return;}}j.仈();}fn 信(&self,三:K,伝:並)->Q{let
j=self;match 三{K::乵{丑,x,y}=>{j.亥(x,y).is_none()||丑>=伝||j.佢(j.丞,丑).is_none()}K::乑{一,x,y}=>{j.亥(x,y).is_none()||一==0||一>j.两.乢()}K::乲{x,y}|K::乳{x,y}=>{j.亥(x,y).is_none()}}}#[inline]fn
亥(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.为.test(y*O+x)).then_some(y*O+x)}fn 佢(&self,z:F,丑:並)->P<F>{self.丄[z].iter().find(|&q|self.亽[q]==丑)}#[inline]fn
伥(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 俕(&self,乣:F)->G{let 乁=self.乊&self.专[乣];let 七=self.七[乣];乁|(七&(七&乁).与())}fn
丿(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乣=1-z;let 丄=j.丄[0]|j.丄[1];let 俤=j.产();let 値=match
一{1 if j.两==乩::佋=>j.一[1]|j.一[2],1=>丄,2=>j.一[1]|j.一[2],_=>G::T,};let 丿=俤|j.专[乣].丘(値);if 一<3{丿.丘(j.俕(乣))}else{丿}}fn 亚(&self,z:F)->G{self.七[z]|self.七[z].与()}fn
产(&self)->G{let j=self;let 丄=j.丄[0]|j.丄[1];let 修=G::q(Z[0])|G::q(Z[1]);j.map.valid.丘(丄|j.丸|j.乊|修)}fn 丬(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.丞;let 亖=match 三{K::乑{一,x,y}=>j.倡(z,一,x,y),K::乵{丑,x,y}=>j.俵(z,丑,x,y),K::乲{x,y}=>j.build(z,true,x,y),K::乳{x,y}=>j.build(z,false,x,y),};let
Some(亖)=亖 else{return false;};match 亖{也::伆=>{}也::佂=>j.七[z]=仌(j.专[z],Z[z]),也::佃=>{j.七=[仌(j.专[0],Z[0]),仌(j.专[1],Z[1])];}}j.佖();if!j.专[0].test(Z[0]){j.丗=Some(之::乫(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(之::乫(0));}true}fn 伮(&mut self,丌:G){let j=self;j.丄[0]=j.丄[0].丘(丌);j.丄[1]=j.丄[1].丘(丌);for 一 in&mut j.一{*一=一.丘(丌);}j.亍=j.亍.丘(丌);}fn
侇(&mut self,z:F,q:F)->也{let j=self;j.伮(G::q(q));j.丸.clear(q);j.乊.clear(q);let 乣=1-z;if j.专[乣].test(q){j.专[乣].clear(q);j.专[z].set(q);也::佃}else if j.专[z].test(q){也::伆}else{j.专[z].set(q);也::佂}}fn
倡(&mut self,z:F,一:I,x:I,y:I)->P<也>{let j=self;if 一==0||一>j.两.乢(){return None;}let q=j.亥(x,y)?;let 万=10*C::from(一);if j.丈[z]<万||!j.亚(z).test(q)||!j.丿(z,一).test(q){return
None;}let 亖=j.侇(z,q);j.佶(z,一,q,j.亳);j.亳=j.亳.saturating_add(1);j.丈[z]-=万;Some(亖)}fn 佶(&mut self,z:F,一:I,q:F,丑:並){self.丄[z].set(q);self.一[F::from(一)-1].set(q);self.亽[q]=丑;}fn
俵(&mut self,z:F,丑:並,x:I,y:I)->P<也>{let j=self;let 丐=j.亥(x,y)?;let 不=j.佢(z,丑)?;if!j.亍.test(不){return None;}let 一=j.伥(不);let 丣=j.丿(z,一);let 乃=j.map.与[z][不];if
乃.contains(&(丐 as I))&&!丣.test(丐){return None;}let mut 下=j.map.丽(不,丐);let mut 仆=None;for&丂 in 乃.iter().take_while(|&&丂|丂!=习){let 丂=F::from(丂);let 丽=j.map.丽(丂,丐);if
丽<下&&丣.test(丂){下=丽;仆=Some(丂);}}let 仆=仆?;let 亖=j.侇(z,仆);j.丄[z].clear(不);j.一[F::from(一)-1].clear(不);j.亍.clear(不);j.佶(z,一,仆,丑);Some(亖)}fn build(&mut self,z:F,丸:Q,x:I,y:I)->P<也>{let
j=self;let 俒=if 丸{j.两.以()}else{j.两.仦()};let q=j.亥(x,y)?;let 俷=j.map.丱.test(q);if!俒||j.丈[z]<了||俷!=丸||!j.专[z].test(q)||!j.亚(z).test(q)||!j.产().test(q){return
None;}if 丸{j.丸.set(q);}else{j.乊.set(q);}j.丈[z]-=了;Some(也::伆)}}const 侪:C=1000000000;const 伃:C=0;const 侱:C=1;const 俇:C=2;impl J{fn read(京:&str,上:&str,两:乩,乡:並,)->Result<J,串>{let
mut 丱=L::new();let mut 严=京.lines();let 丒=伙(&mut 严,"mine spot count")?;for _ in 0..丒{let[x,y]=亀(亴(&mut 严,"a mine spot")?,"a mine spot")?;丱.push((仩(x)?,仩(y)?));}佟(严,"the initial input")?;let
mut 严=上.lines();let mut 伟=[0i64;4];for(东,丷)in 伟.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伫]=亀(亴(&mut 严,丷)?,丷)?;if 伫.abs()>侪{return
Err(format!("{}: {} is out of range",丷,伫));}*东=伫;}let mut 为=G::T;let mut 丌=[G::T;2];let mut 七=[G::T;2];for y in 0..O{let 亐=亴(&mut 严,"a map row")?.trim();if
亐.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,亐));}for(x,付)in 亐.bytes().enumerate(){let q=y*O+x;match 付{b'#'=>为.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
久=F::from(付==b'X'||付==b'x');丌[久].set(q);if 付.is_ascii_uppercase(){七[久].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",付 as char))}}}}let
俙=伙(&mut 严,"building count")?;let mut 乓=L::new();for _ in 0..俙{let 亀=亀(亴(&mut 严,"a building")?,"a building")?;乓.push(亀);}let 伵=伙(&mut 严,"unit count")?;let
mut 丄=L::new();for _ in 0..伵{let 亀=亀(亴(&mut 严,"a unit")?,"a unit")?;丄.push(亀);}佟(严,"the turn's input")?;let 併=乓.iter().find(|佒|佒[0]==0&&佒[1]==伃).ok_or("the bot's own HQ is not among the buildings")?;let
佨=伒(併[2],併[3],为)?;let z=Z.iter().position(|&享|享==佨).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佨))?;let mut 侁=G::T;for(x,y)in 丱{if
为.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}侁.set(y*O+x);}let 伊=|久:F|if 久==0{z}else{1-z};let mut l=J{map:乴::個(为,侁),两,专:[G::T;2],七:[G::T;2],丸:G::T,乊:G::T,丄:[G::T;2],一:[G::T;3],亍:G::T,亽:[0;M],丈:[0;2],且:[0;2],丞:z,乡,亳:1,丗:None,};for
久 in 0..2{let 乤=伊(久);l.专[乤]=丌[久];l.七[乤]=七[久];l.丈[乤]=伟[2*久];l.且[乤]=伟[2*久+1];}let mut 佴=G::T;for[乤,仰,x,y]in 乓{let 久=佼(乤)?;let q=伒(x,y,为)?;if!丌[久].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佴.test(q){return Err(format!("two buildings at ({x}, {y})"));}佴.set(q);match 仰{伃
if q==Z[伊(久)]=>{}伃=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侱=>l.丸.set(q),俇=>l.乊.set(q),_=>return Err(format!("unknown building type {}",仰)),}}let
mut 乆=G::T;let mut 仱=0u32;for[乤,丑,一,x,y]in 丄{let 久=佼(乤)?;let q=伒(x,y,为)?;if!丌[久].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乆.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丑=並::try_from(丑).map_err(|_|format!("unit id {} is out of range",丑))?;if
乆.iter().any(|个|l.亽[个]==丑){return Err(format!("two units have id {}",丑));}乆.set(q);l.丄[伊(久)].set(q);l.一[一 as F-1].set(q);l.亽[q]=丑;仱=仱.max(丑);}if!乆.is_empty(){l.亳=仱.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仱))?;}l.亍=l.丄[z];Ok(l)}}fn
亴<'a>(严:&mut Lines<'a>,丷:&str)->Result<&'a str,串>{严.next().ok_or_else(||format!("the text ends before {}",丷))}fn 佟(mut 严:Lines<'_>,丷:&str)->Result<(),串>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丷)),None=>Ok(()),}}fn 亀<const N:F>(line:&str,丷:&str)->Result<[C;N],串>{let
mut 份=line.split_whitespace();let mut 侍=[0i64;N];for 东 in&mut 侍{let 亿=份.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丷))?;*东=亿.parse().map_err(|_|format!("{}: {:?} is not an integer",丷,亿))?;}if
份.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丷));}Ok(侍)}fn 伙(严:&mut Lines<'_>,丷:&str)->Result<F,串>{let[丒]=亀(亴(严,丷)?,丷)?;F::try_from(丒).ok().filter(|&丒|丒<=M).ok_or_else(||format!("{}: {} is out of range",丷,丒))}fn
仩(东:C)->Result<F,串>{F::try_from(东).ok().filter(|&东|东<O).ok_or_else(||format!("coordinate {} is off the map",东))}fn 伒(x:C,y:C,为:G)->Result<F,串>{let q=仩(y)?*O+仩(x)?;if
为.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佼(乤:C)->Result<F,串>{match 乤{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乤)),}}const
侞:C=15;const 侲:C=6;const 佈:C=10;const 便:I=3;#[derive(Clone,Copy)]struct Y{open:C,仨:C,余:I,伡:C,传:C,佭:C,估:I,佘:C,佗:C,伌:C,亪:C,亻:C,丹:C,亭:C,亓:C,仅:C,}impl Y{const
伉:Y=Y{open:3,仨:2,余:12,伡:4,传:侞,佭:侲,估:便,佘:2,佗:2,伌:0,亪:0,亻:0,丹:0,亭:0,亓:0,仅:0,};const fn 倬(self,亪:C,亻:C,丹:C)->Y{Y{亪,亻,丹,..self}}const fn 倫(self,亭:C,亓:C)->Y{Y{亭,亓,..self}}}fn
仝(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仇(l:&J,不:G)->[I;M]{let mut 乌=[I::MAX;M];let 乕=l.map().valid;let mut 乀=不&乕;let mut 乆=乀;let mut d=0u8;while!乀.is_empty(){for
q in 乀.iter(){乌[q]=d;}let 丂=(乀.与()&乕).丘(乆);乆|=丂;乀=丂;d=d.saturating_add(1);}乌}fn 俨(l:&J)->I{let 乌=仇(l,l.丝(0));l.丝(1).iter().map(|q|乌[q]).min().unwrap_or(I::MAX)}struct
亊<'a>{乥:&'a Y,丁:F,theirs:[I;M],亼:[I;M],free:G,乐:G,ours:G,丱:G,丄:G,}impl 亊<'_>{fn new<'a>(l:&J,乥:&'a Y)->亊<'a>{let 丁=l.丞();let 丸=仇(l,l.丝(丁));let 亄=仇(l,l.丝(1-丁));let
亼=仇(l,G::q(Z[1-丁]));let 乭=l.map().valid.丘(l.丝(0)|l.丝(1));let mut 乐=G::T;for q in 乭.iter(){if 丸[q]<=亄[q]{乐.set(q);}}亊{乥,丁,theirs:亄,亼,free:乭,乐,ours:l.丝(丁),丱:l.map().丱,丄:l.丄(丁),}}fn
value(&self,q:F)->C{let j=self;let p=j.乥;let 丶=C::from((G::q(q).与()&j.free).count());let 佔=p.余;let 仨=C::from(佔)-C::from(j.theirs[q].min(佔));let 享=if p.伡>0{C::from(j.亼[q].min(30))/p.伡}else{0};let
专=C::from((G::q(q).与()&j.ours).count());let line=if 专<=1{p.亻}else{0};let 丹=if j.丱.test(q){p.丹}else{0};let 倘=if 丶==0{p.亭}else{0};let 亓=p.亓*(22-C::from(j.亼[q].min(22)));let
仅=p.仅*C::from((G::q(q).与()&j.丄).count());p.open*丶+p.仨*仨-享+p.亪*专-line+丹-倘+亓-仅}fn 下(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
伕(&mut self,q:F){let j=self;j.free.clear(q);j.乐.clear(q);j.ours.set(q);j.丄.set(q);}}fn 仔(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 佌=l.丬(三);if 佌{丆.push(三);}佌}fn
业(l:&mut J,不:F,乱:F,丆:&mut Vec<K>)->Q{let Some((丑,_))=l.仛(不)else{return false;};let(x,y)=仝(乱);仔(l,K::乵{丑,x,y},丆)}fn 倥(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-佈}#[derive(Clone,Copy)]enum
乇{仢,亇{丠:I,丄:I},}fn 侊(l:&J,乥:&Y,二:乇)->L<K>{let mut 丮=l.clone();let mut 上=亊::new(l,乥);let mut 丆=L::new();乿(&mut 丮,&mut 上,&mut 丆);match 二{乇::仢=>倀(&mut 丮,&mut
上,&mut 丆),乇::亇{丠,丄}=>{俦(&mut 丮,&mut 上,丠,丄,&mut 丆)}}丆}fn 乿(l:&mut J,上:&mut 亊<'_>,丆:&mut Vec<K>){let 丁=上.丁;let mut 乼:L<F> =L::new();loop{let 丣=l.丿(丁,1);let
mut 亷:P<(並,F)> =None;for 不 in l.于().iter(){if 乼.contains(&不){continue;}let 亁=(G::q(不).与()&上.free&丣).count();if 亁==0{乼.push(不);}else if 亷.is_none_or(|(n,_)|亁<n){亷=Some((亁,不));}}let
Some((_,不))=亷 else{break;};上.丄.clear(不);match 上.下(G::q(不).与()&上.free&丣){Some(乱)if 业(l,不,乱,丆)=>上.伕(乱),_=>{上.丄.set(不);乼.push(不);}}}for 不 in 乼{if!l.于().test(不){continue;}let
丐=[上.乐,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().丽(不,q),q))});if let Some(丐)=丐{业(l,不,丐,丆);}}}fn 倀(l:&mut J,上:&mut 亊<'_>,丆:&mut
Vec<K>){let 丁=上.丁;let p=上.乥;let h=(p.传-C::from(l.乡())).max(p.佭);let 倞=(C::from(俨(l))-p.佘).max(0)/p.佗.max(1)+p.伌;loop{let 丈=l.丈(丁);let 丄=l.伵(丁)as C;let
倅=C::from(上.乐.count());let q=上.下(l.亚(丁)&上.free&l.丿(丁,1));let 侌=倥(h,(倅/(丄+1)).min(倞));let 体=q.is_some()&&丈>=佈&&侌>0;let 佮=4*h-了;let 丹=佯(l,上);let 俜=丹.is_some()&&丈>=了&&佮>0;if
俜&&(!体||佮>=侌){let(x,y)=仝(丹.unwrap_or_default());if 仔(l,K::乲{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|体)else{break;};let(x,y)=仝(q);if!仔(l,K::乑{一:1,x,y},丆){break;}上.伕(q);}}fn
佯(l:&J,上:&亊<'_>)->P<F>{let 丁=上.丁;(l.map().丱&l.七(丁)&l.产()).iter().filter(|&丹|上.theirs[丹]>上.乥.估).min_by_key(|&丹|(l.map().丽(Z[丁],丹),丹))}fn 俦(l:&mut J,上:&mut
亊<'_>,丠:I,丄:I,丆:&mut Vec<K>,){let 丁=上.丁;for _ in 0..丠{let Some(丹)=佯(l,上)else{break;};let(x,y)=仝(丹);if!仔(l,K::乲{x,y},丆){break;}}for _ in 0..丄{let Some(q)=上.下(l.亚(丁)&上.free&l.丿(丁,1))else{break;};let(x,y)=仝(q);if!仔(l,K::乑{一:1,x,y},丆){break;}上.伕(q);}}fn
仸(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.丞();let 丈=l.丈(丁);let 丰=交::new(l,丁,true);l.丝(1-丁).iter().any(|q|丰.万[q]<=丈)}#[derive(Clone,Copy)]struct
丢{亞:I,二:乇,}const 伶:I=I::MAX;const 侓:[乇;8]=[乇::仢,乇::亇{丠:0,丄:0},乇::亇{丠:0,丄:1},乇::亇{丠:0,丄:2},乇::亇{丠:0,丄:伶,},乇::亇{丠:1,丄:0},乇::亇{丠:1,丄:1},乇::亇{丠:1,丄:伶,},];struct
伺<'a>{丛:&'a[Y],仏:Y,rest:丢,乸:P<乖>,倆:Settings,}impl 伺<'_>{fn 乥(&self,业:丢)->&Y{&self.丛[F::from(业.亞)%self.丛.len()]}fn 丅(&self,l:&J,业:丢)->L<K>{侊(l,self.乥(业),业.二)}}fn
伱(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伧=|亿:U|hash=(hash^亿).wrapping_mul(0x0100000001b3);for(候,临)in[l.丝(丁),l.丄(丁),l.丠()].into_iter().enumerate(){for
q in 临.iter(){伧(((候 as U)<<8)|q as U);}伧(0xff);}伧(l.丈(丁)as U);hash}enum 亣{代{乂:F,乨:F,乔:亠,},}struct 争{亂:亣,丛:L<Y>,仏:Y,丕:L<丢>,仫:L<U>,亸:F,丯:並,仓:Q,}impl 争{fn
new(亂:亣,丛:L<Y>,仏:Y)->争{争{亂,丛,仏,丕:L::new(),仫:L::new(),亸:0,丯:0,仓:false,}}fn 俾(&mut self,l:&J)->Q{let j=self;let 們=matches!(j.亂,亣::代{..});let 俿=!們&&j.丯>0&&j.亸>=j.仫.len();if!j.仓&&(俿||仸(l)){j.仓=true;}!j.仓}fn
end(&mut self){self.仓=true;}fn 上(&mut self,l:&J,乸:P<乖>)->L<K>{let j=self;let 佛=伺{丛:&j.丛,仏:j.仏,rest:丢{亞:0,二:乇::仢,},乸,倆:Settings::DEFAULT,};let 丁=l.丞();let
俸=j.仫.get(j.亸)==Some(&伱(l,丁));if!俸{let 乍=j.丯==0;j.丕=match j.亂{亣::代{乂,乨,乔,}=>{let 乔=&乔;let 九=侾;let mut 义=丕(l,&j.丛,九,乂,乨,乸,乔,);义.resize(乂,九);义}};j.仫=match
j.亂{亣::代{..}=>倒(l,&j.丛,&j.丕),};j.亸=0;j.丯+=1;}let 业=j.丕.get(j.亸).copied().unwrap_or(佛.rest);j.亸+=1;佛.丅(l,业)}}fn 倒(l:&J,丛:&[Y],丕:&[丢])->L<U>{let 丁=l.丞();let
mut 佪=L::with_capacity(丕.len());let mut 书=l.clone();for&业 in 丕{if 书.丗().is_some(){break;}佪.push(伱(&书,丁));书=伋(&书,&丅(&书,丛,业));}佪}const 俈:C=3;struct Settings{乂:C,}impl
Settings{const DEFAULT:Settings=Settings{乂:俈};}fn 义(l:&J,不:G)->[I;M]{let 乕=l.map().valid;let mut 乌=[I::MAX;M];let mut 乀=不&乕;let mut 乆=乀;let mut d=0u8;while!乀.is_empty(){for
q in 乀.iter(){乌[q]=d;}let 丂=(乀.与()&乕).丘(乆);乆|=丂;乀=丂;d=d.saturating_add(1);}乌}fn 倧(l:&J,z:F)->C{let 仑=义(l,l.七(z));let 亄=义(l,l.七(1-z));l.map().valid.iter().map(|q|match
仑[q].cmp(&亄[q]){_ if 仑[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 俀:[Y;27]=丛();const
侾:丢=丢{亞:16,二:乇::仢,};const fn 丛()->[Y;27]{let 亪=[0,2,4];let 亻=[0,3,6];let 亓=[0,1,2];let mut 丆=[Y::伉;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亭=if g>0||t>0{4}else{0};丆[i]=Y::伉.倬(亪[g],亻[t],0).倫(亭,亓[w]);i+=1;}丆}#[derive(Default)]struct 伈{丌:C,且:C,丈:C,乽:C,丟:C,仚:C,丰:C,仜:C,}#[derive(Clone,Copy)]struct
亠{丌:C,且:C,丈:C,丟:C,乽:C,仚:C,丰:C,仜:C,}fn 倇(l:&J,z:F)->[C;M]{struct 仁{七:G,么:[u16;M],丵:[u16;M],size:[C;M],丟:[C;M],互:u16,}fn 亝(d:&mut 仁,v:F){d.互+=1;d.么[v]=d.互;d.丵[v]=d.互;d.size[v]=1;for
w in(G::q(v).与()&d.七).iter(){if d.么[w]==0{亝(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.么[v]{d.丟[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.么[w]);}}}let
mut d=仁{七:l.七(z),么:[0;M],丵:[0;M],size:[0;M],丟:[0;M],互:0,};let 买=Z[z];if d.七.test(买){亝(&mut d,买);}d.丟[买]=0;d.丟}fn 倉(l:&J,z:F)->伈{let 七=l.七(z);let 丟=倇(l,z);let
亼=仇(l,G::q(Z[1-z]));let mut 丆=伈{丌:C::from(七.count()),且:l.且(z),丈:l.丈(z),丰:C::MAX,仜:倧(l,z)/2,..伈::default()};for q in 七.iter(){if 丟[q]>0{丆.乽+=1;丆.丟=丆.丟.max(丟[q]);}if
q!=Z[z]&&(G::q(q).与()&七).count()<=1{丆.仚+=1;}丆.丰=丆.丰.min(C::from(亼[q]));}丆}fn 东(l:&J,z:F,乔:&亠)->C{let s=倉(l,z);let o=乔;o.丌*s.丌+o.且*s.且+o.丈*s.丈-o.丟*s.丟-o.乽*s.乽-o.仚*s.仚-o.丰*s.丰.min(30)+o.仜*s.仜}fn
伋(l:&J,丅:&[K])->J{let mut 丂=l.clone();丂.佸(丅);if 丂.丗().is_none(){丂.佸(&[]);}丂}fn 丅(l:&J,丛:&[Y],业:丢)->L<K>{侊(l,&丛[F::from(业.亞)%丛.len()],业.二,)}fn 佷(l:&J,丛:&[Y],丕:&[丢],九:丢,乂:F,)->J{let
mut 书=l.clone();for 上 in 0..乂{if 书.丗().is_some(){break;}let 业=丕.get(上).copied().unwrap_or(九);书=伋(&书,&丅(&书,丛,业));}书}fn 倖(l:&J,丛:&[Y])->L<(丢,L<K>)>{let mut
丆:L<(丢,L<K>)> =L::new();for 亞 in 0..丛.len().min(F::from(I::MAX)){for&二 in&侓{let 业=丢{亞:亞 as I,二,};let c=丅(l,丛,业);if!丆.iter().any(|(_,个)|*个==c){丆.push((业,c));}}}丆}fn
俔(l:&J,丛:&[Y],九:丢,乂:F,乨:F,乸:P<乖>,乔:&亠,)->L<丢>{let 丁=l.丞();let 仒=||乸.is_some_and(|t|乖::now()>=t);let 仕=|丕:&[丢]|东(&佷(l,丛,丕,九,乂),丁,乔);let mut 下:(C,L<丢>)=(仕(&[]),L::new());let
mut 乀:L<(J,L<丢>)> =vec![(l.clone(),L::new())];for 俠 in 0..乂{let mut 乷:L<(C,J,L<丢>)> =L::new();let mut 乆:HashMap<U,F> =HashMap::new();for(佱,丕)in&乀{if 仒(){break;}for(业,c)in
倖(佱,丛){let 丂=伋(佱,&c);let mut 伔=丕.clone();伔.push(业);let v=东(&丂,丁,乔);let 乎=伱(&丂,丁);match 乆.get(&乎){Some(&丼)if 乷[丼].0>=v=>{}Some(&丼)=>乷[丼]=(v,丂,伔),None=>{乆.insert(乎,乷.len());乷.push((v,丂,伔));}}}}if
乷.is_empty(){break;}乷.sort_by(|a,b|b.0 .cmp(&a.0));乷.truncate(乨);let 乾=俠+1==乂||仒();for(i,(_,_,丕))in 乷.iter().take(if 乾{乨}else{1}).enumerate(){if i>0&&仒(){break;}let
v=仕(丕);if v>下.0{下=(v,丕.clone());}}if 乾{break;}乀=乷.into_iter().map(|(_,s,p)|(s,p)).collect();}下.1}fn 丕(l:&J,丛:&[Y],九:丢,乂:F,俳:F,乸:P<乖>,乔:&亠,)->L<丢>{let 丁=l.丞();let
仕=|丕:&[丢]|东(&佷(l,丛,丕,九,乂),丁,乔);let mut 下:(C,L<丢>)=(仕(&[]),L::new());let mut 乨=2;loop{let 丩=俔(l,丛,九,乂,乨,乸,乔);let v=仕(&丩);if v>下.0{下=(v,丩);}let 仒=乸.is_some_and(|t|乖::now()>=t);if
仒||(乸.is_none()&&乨>=俳){break;}乨*=2;}下.1}const 依:F=4+12;fn 乂(伢:impl BufRead,mut 价:impl Write,mut 伍:impl FnMut(&str,&str,並,乖)->串,)->Result<(),串>{let mut
严=伿{佻:伢};let Some(乍)=严.line()?else{return Ok(());};let mut 京=format!("{}\n",乍);严.take(丒(&乍,"mine spot count")?,&mut 京)?;let mut 上=0;loop{let Some(乍)=严.line()?else{return
Ok(());};let 亜=乖::now();上+=1;let mut 乞=format!("{}\n",乍);严.take(依-1,&mut 乞)?;for 丷 in["building count","unit count"]{let line=严.佽()?;let n=丒(&line,丷)?;乞.push_str(&line);乞.push('\n');严.take(n,&mut
乞)?;}let 伍=伍(&京,&乞,上,亜);writeln!(价,"{}",伍).and_then(|()|价.flush()).map_err(|亗|format!("turn {}: cannot write the answer: {}",上,亗))?;}}struct 伿<R>{佻:R,}impl<R:BufRead>伿<R>{fn
line(&mut self)->Result<P<串>,串>{let mut line=串::new();match self.佻.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(亗)=>Err(format!("cannot read input: {}",亗)),}}fn
佽(&mut self)->Result<串,串>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乞:&mut String)->Result<(),串>{for
_ in 0..n{乞.push_str(&self.佽()?);乞.push('\n');}Ok(())}}fn 丒(line:&str,丷:&str)->Result<F,串>{match line.trim().parse::<F>(){Ok(n)if n<=侩=>Ok(n),_=>Err(format!("bad {} {line:?}",丷)),}}const
侩:F=144;const 俄:C=3;const 侤:C=2;const 侮:F=16;fn 俼(l:&J,丆:&mut Vec<K>){let z=l.丞();let mut 亂=佄{s:l.clone(),z,乹:1-z,丐:Z[1-z],仮:Z[z],上:l.乡(),丆,};亂.run();}struct
佄<'a>{s:J,z:F,乹:F,丐:F,仮:F,上:並,丆:&'a mut Vec<K>,}fn 仿(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伨(丑:並,q:F)->K{let(x,y)=仿(q);K::乵{丑,x,y}}fn 侉(一:I,q:F)->K{let(x,y)=仿(q);K::乑{一,x,y}}impl
佄<'_>{fn run(&mut self){let j=self;if j.仲(){return;}j.乿();if j.仲(){return;}j.乓();j.倣();}fn 乌(&self,不:F,乱:F)->C{C::from(self.s.map().丽(不,乱))}fn 丬(&mut self,三:K)->Q{let
举=self.s.丬(三);if 举{self.丆.push(三);}举}fn 亾(&self,両:&J)->Q{両.丗()==Some(之::乫(self.z))}fn 仲(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.于().iter(){if j.乌(q,j.丐)!=1{continue;}let Some((丑,_))=j.s.仛(q)else{continue;};let 三=伨(丑,j.丐);let mut 両=j.s.clone();if 両.丬(三)&&j.亾(&両){return j.丬(三);}}let
Some(丨)=j.保()else{return false;};let mut 両=j.s.clone();if!丨.iter().all(|&三|両.丬(三))||!j.亾(&両){return false;}for 三 in 丨{j.丬(三);}true}fn 保(&self)->P<L<K>>{let
j=self;const 似:C=C::MAX;let 乚=j.s.两().乢();let 丣:L<G> =(1..=乚).map(|一|j.s.丿(j.z,一)).collect();let 今=|q:F|{丣.iter().position(|临|临.test(q)).map(|丏|(丏 as I+1,10*(丏
as C+1)))};let 七=j.s.七(j.z);let mut 万=[似;M];let mut 丧=[F::MAX;M];let mut 举=[false;M];for q in 七.iter(){万[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!举[q]&&万[q]<似&&丂.is_none_or(|n|万[q]<万[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};举[q]=true;if q==j.丐{break;}for&乃 in&j.s.map().与[0][q]{if 乃==习{break;}let 乃=F::from(乃);if 举[乃]||七.test(乃){continue;}let Some((_,亏))=今(乃)else{continue;};if
万[q]+亏<万[乃]{万[乃]=万[q]+亏;丧[乃]=q;}}}if 万[j.丐]>j.s.丈(j.z){return None;}let mut 丨=L::new();let mut q=j.丐;while!七.test(q){let(一,_)=今(q)?;丨.push(侉(一,q));q=丧[q];}丨.reverse();Some(丨)}fn
乮(&self,主:&J)->C{let(丁,云)=(self.z,self.乹);let 丧=&self.s;let 丒=|临:G|C::from(临.count());let 专=丒(主.丝(丁))-丒(丧.丝(丁));let 任=丒(丧.丝(云))-丒(主.丝(云));let 俰=丒(丧.七(云))-丒(主.七(云));let
仯=丧.亱(云)-主.亱(云);let 丠=丒(丧.丠()&丧.丝(云))-丒(主.丠()&主.丝(云));12*专+20*任+8*(俰-任).max(0)+5*仯+25*丠}fn 侂(&self,q:F)->C{let j=self;let 丹=j.s.map().丱.test(q);if 丹&&j.s.两().以()&&!j.s.丝(j.z).test(q){20}else{0}}fn
你(&self,乄:C)->Q{let j=self;j.s.丄(j.乹).iter().any(|q|j.乌(q,j.仮)<=乄)}fn 乿(&mut self){let j=self;let mut 丄:L<(C,F)> =j.s.于().iter().map(|q|(j.乌(q,j.丐),q)).collect();丄.sort_unstable();for(仭,q)in
丄{if j.s.丗().is_some(){return;}let Some((丑,一))=j.s.仛(q)else{continue;};let 丣=j.s.丿(j.z,一);let 乃=j.s.map().与[j.z][q];let mut 下:P<(C,F)> =None;for&丂 in 乃.iter().take_while(|&&丂|丂!=习){let
丂=F::from(丂);if!丣.test(丂){continue;}let mut 両=j.s.clone();if!両.丬(伨(丑,丂)){continue;}let 东=if j.亾(&両){100000}else{j.乮(&両)+10*(仭-j.乌(丂,j.丐))+j.侂(丂)};if 下.is_none_or(|(亅,_)|东>亅){下=Some((东,丂));}}if
let Some((东,丂))=下{if 东>0{j.丬(伨(丑,丂));}}}}fn 乓(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.你(俄){j.倠();}else{j.丠();}}fn 倠(&mut self){let j=self;if!j.s.两().仦()||j.s.丈(j.z)<了{return;}let
乃=j.s.map().与[j.z][j.仮];let 丌:L<F> =乃.iter().take_while(|&&q|q!=习).map(|&q|F::from(q)).collect();let 丸=j.s.丝(j.z);if 丌.iter().any(|&q|丸.test(q)&&j.s.乁().test(q)){return;}for
q in 丌{let(x,y)=仿(q);if j.丬(K::乳{x,y}){return;}}}fn 丠(&mut self){let j=self;if!j.s.两().以(){return;}while j.s.丈(j.z)>=了{let 乭=j.s.map().丱&j.s.丝(j.z)&j.s.七(j.z)&j.s.产();let
Some(q)=乭.iter().min_by_key(|&q|(j.乌(q,j.仮),q))else{return;};let(x,y)=仿(q);if!j.丬(K::乲{x,y}){return;}}}fn 倣(&mut self){let j=self;let 佾=if j.上<=3{0}else{5};for
_ in 0..侮{if j.s.丗().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俯=j.你(侤);let 亲=j.s.亚(j.z);let mut 下:P<(C,K)> =None;let mut 伳:P<(C,K)> =None;let
mut 佑=G::T;for 一 in 1..=j.s.两().乢(){let 丣=j.s.丿(j.z,一);let 亁=(亲&丣).丘(佑);佑=丣;let 亏=10*C::from(一);if 亏>丈{continue;}for q in 亁.iter(){let 三=侉(一,q);let mut
両=j.s.clone();if!両.丬(三){continue;}let 伤=両.丈(j.z);let 且=両.且(j.z);let 仯=j.s.亱(j.乹)-両.亱(j.乹);let 俐=if 仯>0&&俯{伤+且>=0}else{且>=0&&(伤>=佾||仯>0)};if!俐{continue;}let
乮=j.乮(&両);let 东=乮+j.侂(q)-6*j.乌(q,j.丐)-10*(C::from(一)-1);if 乮>0{if 下.is_none_or(|(亅,_)|东>亅){下=Some((东,三));}}else if 且>=3&&伤>=佾+10&&一==1{if 伳.is_none_or(|(亅,_)|东>亅){伳=Some((东,三));}}}}let
Some((_,三))=下.or(伳)else{return;};if!j.丬(三){return;}}}}fn 俫(l:&J,丆:&mut Vec<K>){丆.clear();俼(l,丆);}const 事:C=C::MAX;const 侭:F=8;struct 交{万:[C;M],丧:[I;M],}fn
优(l:&J,z:F,乿:Q,丐:F)->C{佚(l,z,亙(l,z,乿),丐)}fn 佚(l:&J,z:F,亙:G,丐:F)->C{let mut 万=事;佣(l,z,亙,|业,new|{if new.test(丐){万=10*业 as C;}万!=事});万}fn 倁(l:&J,z:F,亙:G,中:C)->G{let
mut 仸=G::T;佣(l,z,亙,|业,new|{if 10*业 as C>中{return true;}仸|=new;false});仸.丘(l.七(z))}fn 亙(l:&J,z:F,乿:Q)->G{if 乿&&z==l.丞(){l.于()}else{G::T}}fn 佣(l:&J,z:F,亙:G,mut
亝:impl FnMut(F,G)->Q,){let 乚=l.两().乢();let mut 今=[G::T;3];let mut 乆=G::T;for 一 in 1..=乚{let 临=l.丿(z,一);今[F::from(一)-1]=临.丘(乆);乆|=临;}let mut 丶=[l.七(z),G::T,G::T,G::T];for
q in(亙&l.丄(z)).iter(){if let Some(一)=l.五(q).filter(|&一|一<=乚){丶[0]|=G::q(q).与()&l.丿(z,一);}}let mut 举=G::T;for 业 in 0..{let new=丶[业%4].丘(举);丶[业%4]=G::T;if
亝(业,new){return;}举|=new;let 丂=new.与().丘(举);for(俑,丌)in 今.iter().enumerate(){丶[(业+俑+1)%4]|=丂&*丌;}if 丶.iter().all(|临|临.丘(举).is_empty()){break;}}}impl 交{fn
new(l:&J,z:F,乿:Q)->交{let 乚=F::from(l.两().乢());let mut 丣=[G::T;3];for(丏,临)in 丣.iter_mut().enumerate().take(乚){*临=l.丿(z,丏 as I+1);}let 七=l.七(z);let 与=&l.map().与[0];let
mut 万=[事;M];let mut 丧=[习;M];let mut 丶=[0u8;M];let mut 丒=0;let mut 侀=G::T;for q in 七.iter(){万[q]=0;丶[丒]=q as I;丒+=1;}if 乿&&z==l.丞(){for 不 in l.于().iter(){let
Some(一)=l.五(不)else{continue;};let 义=G::q(不).与()&丣[F::from(一)-1];for q in 义.iter(){if 万[q]!=0{万[q]=0;丶[丒]=q as I;丒+=1;}}}}while 丒>0{let mut 下=0;for 丏 in
1..丒{let(a,b)=(F::from(丶[丏]),F::from(丶[下]));if(万[a],a)<(万[b],b){下=丏;}}let q=F::from(丶[下]);丒-=1;丶[下]=丶[丒];侀.set(q);for&丂 in&与[q]{if 丂==习{break;}let 丂=F::from(丂);if
侀.test(丂)||七.test(丂){continue;}let Some(一)=丣[..乚].iter().position(|临|临.test(丂))else{continue;};let 侈=万[q]+10*(一 as C+1);if 侈<万[丂]{if 万[丂]==事{丶[丒]=丂 as
I;丒+=1;}万[丂]=侈;丧[丂]=q as I;}}}交{万,丧}}fn path(&self,丐:F)->L<F>{let mut path=L::new();if 丐>=M||self.万[丐]==事{return path;}let mut q=丐;while self.丧[q]!=习{path.push(q);q=F::from(self.丧[q]);}path.reverse();path}}struct
仠{丅:L<K>,万:C,}fn 俲(l:&J,z:F,q:F)->P<I>{(1..=l.两().乢()).find(|&一|l.丿(z,一).test(q))}fn 倄(丮:&mut J,丁:F,亩:F,中:C,丅:&mut Vec<K>,乧:&mut i64,)->Q{loop{if 丮.丗()==Some(之::乫(丁)){return
true;}if 丮.丝(丁).test(亩)&&丮.七(丁).test(亩){return true;}let 丰=交::new(丮,丁,false);let 万=丰.万[亩];if 万==事||*乧+万>中{return false;}let Some(&q)=丰.path(亩).first()else{return
false;};let Some(一)=俲(丮,丁,q)else{return false;};let 三=K::乑{一,x:(q%O)as I,y:(q/O)as I,};if!丮.丬(三){return false;}*乧+=10*C::from(一);丅.push(三);}}fn 住(l:&J,丁:F,俪:&[F],中:C)->P<仠>{let
mut 丮=l.clone();let mut 丅=L::new();let mut 乧=0;for&亩 in 俪{if!倄(&mut 丮,丁,亩,中,&mut 丅,&mut 乧){return None;}}(乧<=中).then_some(仠{丅,万:乧,})}fn 伓(l:&J,丐:F,中:C)->P<仠>{if
中<0||l.丗().is_some()||丐>=M||l.map().为.test(丐){return None;}let 丁=l.丞();if l.丝(丁).test(丐)&&l.七(丁).test(丐){return Some(仠{丅:L::new(),万:0,});}let mut 下=住(l,丁,&[丐],中);let
乍=交::new(l,丁,false);let mut 乁:L<F> =(l.乁()&l.丝(1-丁)).iter().filter(|&乊|乊!=丐&&乍.万[乊]!=事).collect();乁.sort_by_key(|&乊|(l.map().丽(乊,丐),乊));乁.truncate(侭);for
乊 in 乁{if let Some(丨)=住(l,丁,&[乊,丐],中){if 下.as_ref().is_none_or(|亅|丨.万<亅.万){下=Some(丨);}}}下}const 侫:F=20000;struct 仡{丅:L<K>,万:C,丌:C,丄:C,乓:C,且:C,丙:C,}struct
伄{丌:C,丄:C,乓:C,}fn 佧(l:&J,z:F)->伄{let 七=l.七(z);let 丄=l.丄(z)&七;let 东=丄.iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum();伄{丌:C::from(七.count()),丄:东,乓:C::from(((l.丠()|l.乁())&七).count()),}}struct
但{丁:F,什:G,丧:伄,且:C,亰:F,中:C,亵:F,path:L<K>,丩:L<仡>,index:HashMap<仃,F,BuildHasherDefault<DefaultHasher>>,}type 仃=[U;3];fn 倪(mut 乎:仃,q:F)->仃{乎[q/64]|=1<<(q%64);乎}impl
但{fn 佞(&mut self,丮:&J,乾:P<F>,任:仃,万:C,)->Q{let j=self;let 丁=j.丁;let 亁=match 乾{None=>丮.亚(丁),Some(q)=>G::q(q).与(),}&j.什;let 亁=亁.丘(丮.丝(丁));let mut 丣=[G::T;3];for
一 in 1..=丮.两().乢(){丣[F::from(一)-1]=丮.丿(丁,一);}for q in 亁.iter(){let 乚=F::from(丮.两().乢());let Some(丏)=丣[..乚].iter().position(|临|临.test(q))else{continue;};let
一=丏 as I+1;let 亏=10*C::from(一);if 万+亏>j.中{continue;}if j.亵>=侫{return false;}j.亵+=1;let mut 丂=丮.clone();let 三=K::乑{一,x:(q%O)as I,y:(q/O)as I,};if!丂.丬(三){continue;}j.path.push(三);let
人=倪(任,q);j.倂(&丂,人,万+亏);let 俟=j.path.len()<j.亰&&丂.丗().is_none();let 俩=!俟||j.佞(&丂,Some(q),人,万+亏);j.path.pop();if!俩{return false;}}true}fn 倂(&mut self,主:&J,人:仃,万:C){let
j=self;let 仐=1-j.丁;let 伪=佧(主,仐);let 丌=j.丧.丌-伪.丌;let 丄=j.丧.丄-伪.丄;if 丌<=0&&丄<=0{return;}let 乓=j.丧.乓-伪.乓;let 且=主.且(j.丁)-j.且;let 倭=主.丗()==Some(之::乫(j.丁));let
丙=if 倭{C::MAX/2}else{3*丌+丄+10*乓+2*且-万};if let Some(&丼)=j.index.get(&人){if 万>=j.丩[丼].万{return;}}let 丟=仡{丅:j.path.clone(),万,丌,丄,乓,且,丙,};match j.index.get(&人){Some(&丼)=>j.丩[丼]=丟,None=>{j.index.insert(人,j.丩.len());j.丩.push(丟);}}}}fn
作(l:&J,亰:F,中:C,乄:F)->L<仡>{乏(l,亰,中,乄).0}fn 乏(l:&J,亰:F,中:C,乄:F)->(L<仡>,F){if l.丗().is_some()||亰==0||乄==0{return(L::new(),0);}let 丁=l.丞();let 仐=1-丁;let mut
伯=但{丁,什:l.丝(仐)|l.丝(仐).与(),丧:佧(l,仐),且:l.且(丁),亰,中,亵:0,path:L::new(),丩:L::new(),index:HashMap::default(),};伯.佞(l,None,[0;3],0);let 亵=伯.亵;let mut 丩=伯.丩;丩.sort_by(|a,b|b.丙.cmp(&a.丙).then(a.万.cmp(&b.万)));丩.truncate(乄);(丩,亵)}const
令:C=1000000;const 侬:C=200;fn 乐(l:&J,z:F)->C{let 七=l.七(z);let 丄:C=(l.丄(z)&七).iter().filter_map(|q|l.五(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.且(z)+4*C::from(七.count())+丄+10*C::from((l.乁()&七).count())}fn
佬(l:&J,z:F,乛:G)->C{俧(l,z,佚(l,z,乛,Z[1-z]),)}fn 俧(l:&J,z:F,万:C)->C{if 万==事{return C::MIN;}l.丈(z)+l.且(z)-万}fn 佹(仴:C)->C{仴.clamp(-侬,0)}fn 俥(l:&J,z:F,乛:[G;2],)->(C,[C;2]){let
个=1-z;match l.丗(){Some(之::乫(倩))=>{return(if 倩==z{令}else{-令},[0,0])}Some(之::伻)=>return(0,[0,0]),None=>{}}let mut 丙=乐(l,z)-乐(l,个);if z==l.丞(){let mut 丂=l.clone();丂.仈();if
丂.丗().is_none()&&优(&丂,个,true,Z[z])<=丂.丈(个){丙-=令/2;}}let 亯=[佬(l,z,乛[0]),佬(l,个,乛[1]),];(丙+佹(亯[0])-佹(亯[1]),亯)}
