// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 丧=u32;type U=u64;type 仞=i32;type C=i64;type 亟=f64;type Q=bool;type 丳=String;type 乖=Instant;type L<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侣:Q=true;fn 俲(l:&J,q:F,仚:C,並:G,丩:C)->Q{let
丁=l.丞();let 乻=1-丁;let 佬=侣&&丩<=l.万(乻)+2*l.且(乻).max(0);let 仁=G::q(q)|G::q(q).与();if!佬&&(仁&並).is_empty(){return false;}let(x,y)=乴(q);let mut 主=l.clone();if!主.中(K::乪{x,y}){return
false;}if 倡(&主,丁)<仚{return true;}佬&&优(&主,乻,false,Z[丁])>丩}const 俐:Q=false;const 侾:I=2;const 但:I=1;const 侪:F=4;const 伹:F=5;#[derive(Clone)]enum 丱{亡(K),仢(L<K>),}impl
丱{fn 丅(&self)->&[K]{match self{丱::亡(三)=>std::slice::from_ref(三),丱::仢(丅)=>丅,}}}fn 乴(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 佧(l:&J,丆:&mut Vec<丱>){丆.clear();if
l.丗().is_some(){return;}let 丁=l.丞();let 乚=l.两().乣();let mut 両=[G::T;4];for 一 in 1..=乚{両[F::from(一)]=l.丿(丁,一);}for 下 in l.亁().iter(){let Some((丏,一))=l.亽(下)else{continue;};let
乁=G::q(下).与()&両[F::from(一)];for 也 in 乁.iter(){let(x,y)=乴(也);丆.push(丱::亡(K::乭{丏,x,y}));}}let 万=l.万(丁);let 七=l.七(丁);let 亲=l.亚(丁);let 倨=match 侾{0=>亲,1=>{let
伏=l.map().valid.丘(七).与()&七;亲.丘(七)|伏}_=>亲.丘(七),};for 一 in 1..=乚{let 丈=10*C::from(一);if 万<丈{break;}let mut 不=倨&両[F::from(一)];for 俶 in 1..一{不=不.丘(両[F::from(俶)]);}for
q in 不.iter(){let(x,y)=乴(q);丆.push(丱::亡(K::乌{一,x,y}));}}if 万>=乵{let 买=l.丙(丁)&亲&l.产();if l.两().仦(){for q in(买&l.map().丰).iter(){let(x,y)=乴(q);丆.push(丱::亡(K::乶{x,y}));}}if
l.两().仧(){let 伏=l.map().valid.丘(l.丙(丁)).与();let 不=(买&伏).丘(l.map().丰);let 並=並(l,1-丁);let 仚=C::from((並&l.七(丁)).count());let 丩=优(l,1-丁,false,Z[丁]);let 丶=if
俐{let(仭,俱)=乀(l,丁);仭.丘(俱)}else{G::T};for q in 不.iter().filter(|&q|{let 仁=G::q(q)|G::q(q).与();!(仁&丶).is_empty()||俲(l,q,仚,並,丩)}){let(x,y)=乴(q);丆.push(丱::亡(K::乪{x,y}));}}}let
俰=match 但{1=>乵,2=>10,_=>C::MAX,};if 万>=俰{let 並=並(l,1-丁);let 俼=l.map().valid.丘(l.丙(丁)|l.丙(1-丁));for 下 in(l.亁()&並).iter(){let Some((丏,一))=l.亽(下)else{continue;};let
乁=G::q(下).与()&両[F::from(一)]&俼;let(佥,佦)=乴(下);let 俯=if 但==1{if l.map().丰.test(下){continue;}K::乪{x:佥,y:佦}}else{K::乌{一:1,x:佥,y:佦,}};for 也 in 乁.iter(){let(x,y)=乴(也);丆.push(丱::仢(vec![K::乭{丏,x,y},俯]));}}}for
丟 in 佝(l,伹,万,侪){if 丟.丅.len()>1{丆.push(丱::仢(丟.丅));}}if let Some(丩)=伓(l,Z[1-丁],万){if 丩.丅.len()>1{丆.push(丱::仢(丩.丅));}}}const 侶:I=2;const 俈:C=5;fn 乛(l:&J,z:F)->G{伩(l,z,侶)}fn
伩(l:&J,z:F,下:I)->G{let mut 乛=G::T;if 下>3{return 乛;}for q in(l.丄(z)&l.七(z)).iter(){if l.井(q).is_some_and(|一|一>=下){乛|=G::q(q);}}乛}fn 並(l:&J,乑:F)->G{let 丮=l.万(乑)+l.且(乑);倇(l,乑,乛(l,乑),丮)}fn
仌(l:&J,z:F)->G{並(l,1-z)&l.七(z)}fn 倡(l:&J,z:F)->C{C::from(仌(l,z).count())}const 佁:C=4;fn 俹(不:C)->C{不/佁}fn 丠(l:&J,z:F,仌:G)->C{let 七=l.七(z);let 丠=(l.丠()&七).丘(仌);let
不=C::from(七.count());let 俛=if l.乢()<=侚{不/侙+侘}else{俹(不)};俛*C::from(丠.count())}const 侚:丧=15;const 侘:C=6;const 侙:C=佁;const 俄:C=2;fn 侒(l:&J,z:F)->C{let 七=l.七(z);let[a,b,c]=七.仾();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 丐=(hash as F)&(佋-1);俑.with(|休|{let mut 休=休.borrow_mut();let
任=&mut 休[丐];if 任.0==七&&任.1==z as I+1{return 任.2;}let 丝=倮(七,Z[z]);*任=(七,z as I+1,丝);丝})}const 佋:F=1<<12;thread_local!{static 俑:std::cell::RefCell<L<(G,I,C)>>
=std::cell::RefCell::new(vec![(G::T,0,0);佋]);}fn 倮(七:G,乳:F)->C{struct 仂{七:G,义:[I;M],丵:[I;M],size:[I;M],亓:I,乿:C,件:C,}fn 亝(d:&mut 仂,v:F)->I{d.亓+=1;let 义=d.亓;d.义[v]=义;let
mut 丵=义;let mut 伲=1u8;let mut 丟=0u8;for w in(G::q(v).与()&d.七).iter(){if d.义[w]==0{let 侉=亝(d,w);伲+=侉;丵=丵.min(d.丵[w]);if d.丵[w]>=义{丟+=侉;}}else{丵=丵.min(d.义[w]);}}d.丵[v]=丵;d.size[v]=伲;if
丟>0&&d.义[v]>1{d.乿+=1;d.件=d.件.max(C::from(丟));}伲}if!七.test(乳){return 0;}let mut d=仂{七,义:[0;M],丵:[0;M],size:[0;M],亓:0,乿:0,件:0,};亝(&mut d,乳);d.乿+d.件}fn 倎(l:&J,z:F)->C{俄*(侒(l,1-z)-侒(l,z))}fn
佞(l:&J,z:F)->C{if l.万(z)+l.且(z)>=0{return 0;}(l.丄(z)&l.七(z)).iter().filter_map(|q|l.井(q)).map(|一|10*C::from(一)).sum()}const 侩:C=15;const 俎:[C;2]=[3,1];const
俌:[C;2]=[4,20];fn 倝(l:&J,z:F)->[C;2]{let mut 丒=[0;2];for q in(l.丄(z)&l.七(z)).iter(){match l.井(q){Some(2)=>丒[0]+=1,Some(3)=>丒[1]+=1,_=>{}}}丒}fn 侈(l:&J,z:F)->C{let[侎,倢]=倝(l,z);let
佑=|一:F|(3-俎[一])*俌[一];-侩*侎+佑(0)*侎+佑(1)*倢}const 侢:C=0;const 侥:C=2;const 侤:C=100;fn 來(仵:C)->C{if 仵<0{return 0;}侢+侥*仵.min(侤)/2}const 佀:I=2;const 伾:C=2;const
伽:C=4;const 侦:I=3;fn 乾(l:&J,z:F)->C{if 伾==0{return 0;}let 乕=l.map().valid;let mut 丄=G::T;for q in(l.丄(z)&l.七(z)).iter(){if l.井(q).is_some_and(|一|一<=侦){丄|=G::q(q);}}let
mut 份=乕.丘(l.丙(z));let mut 会=G::T;let mut 乁=0;for 丽 in 1..=伽{份=(份|份.与())&乕;let 仮=(丄&份).丘(会);乁+=(丽-1)*C::from(仮.count());会|=仮;}乁+=伽*C::from(丄.丘(会).count());伾*乁}const
佊:C=8;fn 乀(l:&J,z:F)->(G,G){let 七=l.七(z);let 仭=七&l.丙(1-z).与();let 乂=l.乂()&七;(仭,仭&(乂|乂.与()))}fn 侑(l:&J,z:F)->C{if 佊==0{return 0;}佊*C::from(乀(l,z).1 .count())}fn
伜(l:&J,z:F)->C{let 乛=[伩(l,z,佀),伩(l,1-z,佀),];let(业,亯)=俨(l,z,乛);let 业=业+來(亯[0])-來(亯[1]);if l.丗().is_some(){return 业;}let 仒=仌(l,z);let 亇=仌(l,1-z);let 仚=C::from(亇.count())-C::from(仒.count());业-2*佞(l,z)+2*佞(l,1-z)+倎(l,z)+侈(l,z)-侈(l,1-z)-乾(l,z)+乾(l,1-z)+俈*仚/4+丠(l,z,仒)-丠(l,1-z,亇)+侑(l,z)-侑(l,1-z)}const
侳:F=8;const 便:F=3;const 侫:F=32;const 俀:U=300;const 侗:亟=0.75;const 侟:P<丧> =Some(7);const 依:Q=false;const 侜:C=10;const 侞:丧=6;struct 亊{了:P<乖>,亘:P<乖>,仏:U,}#[derive(Default)]struct
佈{丅:L<K>,众:F,乼:U,世:F,串:F,佖:F,乽:F,乲:Q,}const 俏:Q=true;#[derive(Clone)]struct 乘{丅:L<K>,伛:C,}struct 亢{丅:L<K>,end:J,first:P<U>,专:P<L<乘>>,}type 仃=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
伇{丁:F,limit:亊,乼:U,从:仃,些:P<L<丱>>,}const 伸:Q=true;const 侔:Q=false;const 伷:P<I> =None;fn 俙(l:&J,世:&[乘],丆:&mut Vec<丱>){if!伸&&伷.is_none(){return;}let 付=世.iter().map(|乜|乜.丅.as_slice());俚(l,付,(伸,侔,伷),丆,);}fn
俚<'a>(l:&J,付:impl Iterator<Item=&'a[K]>,(乂,与,一):(Q,Q,P<I>),丆:&mut Vec<丱>,){let 丁=l.丞();let 丸=l.七(丁);let mut 今=G::T;for 乜 in 付{for 三 in 乜{if let K::乌{x,y,..}|K::乭{x,y,..}=*三{let
q=F::from(y)*O+F::from(x);if 丸.test(q){今|=G::q(q);}}}}if 今.is_empty(){return;}let 买=l.产()&丸;let 万=l.万(丁);let 佽=|三:K,丆:&mut Vec<丱>|{if!丆.iter().any(|a|a.丅()==[三]){丆.push(丱::亡(三));}};if
乂&&万>=乵&&l.两().仧(){let mut 不=今;if 与{不|=今.与();}for q in(不&买).丘(l.map().丰).iter(){let(x,y)=乴(q);佽(K::乪{x,y},丆);}}if let Some(一)=一.filter(|&一|一<=l.两().乣()){if
万>=10*C::from(一){for q in(今&买).iter(){let(x,y)=乴(q);佽(K::乌{一,x,y},丆);}}}}fn 伎(l:&mut J,丅:&[K])->Q{丅.iter().all(|&三|l.中(三))}fn 乏(丅:&[K])->U{let mut hash:U=0xcbf29ce484222325;for
三 in 丅{let(仱,a,b,c)=match*三{K::乭{丏,x,y}=>(1,U::from(丏),x,y),K::乌{一,x,y}=>(2,U::from(一),x,y),K::乶{x,y}=>(3,0,x,y),K::乪{x,y}=>(4,0,x,y),};let 什=仱|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^什).wrapping_mul(0x0100000001b3);}hash}fn
低(乙:&J,乜:&[K])->J{let mut 丂=乙.clone();丂.仉();for&三 in 乜{丂.中(三);}丂.仉();丂}fn 仳(l:&J)->P<L<K>>{let 丁=l.丞();let 丑=Z[1-丁];let 亿=|丢:&J|丢.丗()==Some(之::乮(丁));for
q in l.亁().iter(){if l.map().丽(q,丑)!=1{continue;}let Some((丏,_))=l.亽(q)else{continue;};let(x,y)=乴(丑);let 三=K::乭{丏,x,y};let mut 丢=l.clone();if 丢.中(三)&&亿(&丢){return
Some(vec![三]);}}let 丩=伓(l,丑,l.万(丁))?;let mut 丢=l.clone();(伎(&mut 丢,&丩.丅)&&亿(&丢)).then_some(丩.丅)}impl 伇{fn 乨(&self)->Q{self.乼>=self.limit.仏||self.limit.了.is_some_and(|t|乖::now()>=t)}fn
业(&mut self,乙:&J,世:&[乘],伐:C)->C{self.乼+=1;self.伭(乙,世,伐)}fn 伭(&self,乙:&J,世:&[乘],伐:C)->C{if 乙.丗()==Some(之::乮(self.丁)){return 以;}if 世.is_empty(){return 伜(乙,self.丁);}let
mut 仿=C::MAX;for 乜 in 世{仿=仿.min(伜(&低(乙,&乜.丅),self.丁));if 仿<=伐{break;}}仿}#[allow(clippy::too_many_arguments)]fn 倁(&mut self,乯:&J,亦:C,乒:&[丱],亶:&[(C,F)],举:&mut[Q],世:&[乘],亨:&mut
仃,伣:Q,)->(P<(F,J,C)>,Q){let mut 丌=None;let mut 仨=亦;for&(价,丐)in 亶{if 举[丐]{continue;}if 伣&&价!=C::MAX&&价<=仨-亦{return(丌,true);}if self.乨(){return(丌,false);}举[丐]=true;let
丅=乒[丐].丅();let mut 丂=乯.clone();if!伎(&mut 丂,丅){亨.insert(乏(丅),C::MIN/4);continue;}let 业=self.业(&丂,世,仨);亨.insert(乏(丅),业-亦);if 业>仨{仨=业;丌=Some((丐,丂,业));}}(丌,false)}fn
伖(&mut self,乳:&J,世:&[乘],仅:&[U],俞:Q)->亢{let j=self;let mut 乎=None;let mut 乯=乳.clone();let mut 丅=L::new();let mut 亦=j.业(&乯,世,C::MIN);let mut 亨=仃::default();let
mut 乒=L::new();let mut 亶=L::new();while 乯.丗().is_none()&&!j.乨(){if 丅.is_empty()&&俞{match&j.些{Some(些)=>乒.clone_from(些),None=>{佧(&乯,&mut 乒);j.些=Some(乒.clone());}}}else{佧(&乯,&mut
乒);}俙(&乯,世,&mut 乒);亶.clear();for(丐,乸)in 乒.iter().enumerate(){let k=乏(乸.丅());let 价=亨.get(&k).or(j.从.get(&k)).copied();亶.push((价.unwrap_or(C::MAX),丐));}亶.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 举=vec![false;乒.len()];if 丅.is_empty(){for(丐,乸)in 乒.iter().enumerate(){举[丐]=仅.contains(&乏(乸.丅()));}}let mut 丌=None;for
伣 in[true,false]{let(个,倖)=j.倁(&乯,亦,&乒,&亶,&mut 举,世,&mut 亨,伣,);丌=个;if 丌.is_some()||!倖{break;}}let Some((丐,丂,业))=丌 else{break;};if 丅.is_empty(){乎=Some(乏(乒[丐].丅()));}丅.extend_from_slice(乒[丐].丅());乯=丂;亦=业;}for(k,乱)in
亨{let 佩=j.从.entry(k).or_insert(乱);*佩=(*佩).max(乱);}亢{丅,end:乯,first:乎,专:None,}}fn 付(&self,乙:&J)->L<L<K>>{let mut 丂=乙.clone();丂.仉();let mut 丆=L::new();if
丂.丗().is_some(){return 丆;}let 乤=丂.丞();let 万=丂.万(乤);if let Some(丩)=伓(&丂,Z[self.丁],万){丆.push(丩.丅);}for 丟 in 佝(&丂,伹,万,便){丆.push(丟.丅);}let mut 佨=L::new();修(&丂,&mut
佨);丆.push(佨);let mut 亇=伇{丁:乤,limit:亊{仏:俀,..self.limit},乼:0,从:仃::default(),些:None,};丆.push(亇.伖(&丂,&[],&[],true).丅);丆.retain(|丅|!丅.is_empty());丆}fn 住(&self,乙:&J)->L<乘>{self.付(乙).into_iter().map(|丅|{let
伛=伜(&低(乙,&丅),self.丁);乘{丅,伛}}).collect()}fn 乽(&mut self,乳:&J,乸:&丱,世:&[乘])->P<亢>{let mut 主=乳.clone();if!伎(&mut 主,乸.丅()){return None;}self.乼+=1;let 九=self.伖(&主,世,&[],false);let
mut 丅=乸.丅().to_vec();丅.extend_from_slice(&九.丅);Some(亢{丅,end:九.end,first:Some(乏(乸.丅())),专:None,})}fn 佢(&self,丕:&亢,世:&[乘])->C{let 业=self.伭(&丕.end,世,C::MIN);match&丕.专{Some(专)if!专.is_empty()=>业.min(self.伭(&丕.end,专,C::MIN)),_=>业,}}}fn
倉(个:&[乘],世:&mut Vec<乘>)->Q{let 丨:L<_> =世.iter().map(|r|r.丅.clone()).collect();for 乜 in 个{if 世.iter().any(|r|r.丅==乜.丅){continue;}世.push(乜.clone());}世.sort_by_key(|乜|乜.伛);世.truncate(侳);世.iter().map(|r|&r.丅).ne(&丨)}fn
倠(l:&J,久:亊,乲:&mut 事)->佈{let 丁=l.丞();let mut 丬=佈::default();if l.丗().is_some(){return 丬;}if let Some(丅)=仳(l){丬.丅=丅;return 丬;}let 丼=|倐:亟|match(久.了,久.亘){(Some(了),Some(亘))=>Some(了+(亘-了).mul_f64(倐)),_=>None,};if
侟.is_some_and(|上|l.乢()>=上){乲.end();}if 乲.倄(l){let 丅=乲.上(l,丼(0.4));let 保=丅.iter().any(|三|{matches!(三,K::乌{..}|K::乶{..}|K::乪{..})});let 乾=依&&!保&&l.乢()>=侞&&l.万(丁)>=侜;if!乾{丬.乲=true;丬.丅=丅;return
丬;}乲.end();}let mut 乐=伇{丁,limit:亊{了:丼(侗),..久},乼:0,从:仃::default(),些:None,};let mut 世=L::new();let mut 串:L<亢> =L::new();let mut 仅=L::new();let mut 仫=false;let
mut 乽:L<丱> =L::new();while 丬.众<侫&&(串.is_empty()||!乐.乨()){let 丕=if 仫{let Some(乸)=乽.pop()else{break;};match 乐.乽(l,&乸,&世){Some(丕)=>丕,None=>continue,}}else{乐.伖(l,&世,&仅,true)};丬.众+=1;let
侂=串.iter().any(|p|p.丅==丕.丅);let 乎=丕.first;let 侇=乐.乨();let mut 丕=丕;let mut 佳=false;if!侇&&!侂{let 个=乐.住(&丕.end);佳=倉(&个,&mut 世);丕.专=Some(个);}if!侂{串.push(丕);}if
侇{break;}if!佳&&!仫{match 乎{Some(乎)=>仅.push(乎),None=>{仫=true;乽=乐.些.clone().unwrap_or_default();乽.retain(|a|!仅.contains(&乏(a.丅())));let 乱=|a:&丱|{乐.从.get(&乏(a.丅())).copied().unwrap_or(C::MIN)};乽.sort_by_key(乱);}}}}丬.乽=F::from(仫);丬.乼=乐.乼;丬.世=世.len();丬.串=串.len();let
佮=||久.亘.is_some_and(|t|乖::now()>=t);let mut 亹:L<P<C>> =vec![None;串.len()];for 丐 in(0..串.len()).rev(){if 丐+1<串.len()&&佮(){break;}亹[丐]=Some(乐.佢(&串[丐],&世));}let
亷=|亹:&[P<C>]|{let mut 丌:P<(C,F)> =None;for(丐,业)in 亹.iter().enumerate().rev(){if let Some(业)=*业{if 丌.is_none_or(|(予,_)|业>予){丌=Some((业,丐));}}}丌};let mut
丌=亷(&亹);乐.limit.了=久.亘;while let Some((_,丐))=丌{if!俏||串[丐].专.is_some()||佮(){break;}let 个=乐.住(&串[丐].end);串[丐].专=Some(个);丬.佖+=1;亹[丐]=Some(乐.佢(&串[丐],&世));丌=亷(&亹);}if
let Some((_,丐))=丌{let mut 丯=l.clone();for&三 in&串[丐].丅{if 丯.中(三){丬.丅.push(三);}}}丬}fn 倃(久:&dyn Fn(乖,丧)->亊,mut 乲:事,伢:impl BufRead,仸:impl Write,)->Result<(),丳>{乃(伢,仸,|京,乞,上,亜|match
J::read(京,乞,乫::伂,上){Ok(l)=>{let 丬=倠(&l,久(亜,上),&mut 乲);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,丬.众,丬.乼,丬.世,丬.佖,丬.乽,亜.elapsed());俖(&丬.丅)}Err(亗)=>{eprintln!("plan: cannot read the input: {}",亗);"WAIT".to_string()}},)}const
侕:亟=45.0;const 供:亟=800.0;const 俅:亟=0.6;fn 久(亜:乖,上:丧)->亊{if let Some(仏)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亊{了:None,亘:None,仏,};}let 仺=倣().min(100.0);let 俻=if 上==1{供}else{侕};let 丮=Duration::from_secs_f64(俻/1000.0*仺);亊{了:Some(亜+丮.mul_f64(俅)),亘:Some(亜+丮),仏:U::MAX,}}const
侽:F=9;const 侷:亠=亠{不:3,且:1,万:1,丟:2,乿:2,仛:2,並:6,仜:1,};const 侼:I=I::MAX;const 侹:C=2;const 侺:C=18;const 侸:C=0;const 侻:F=32;fn 乲()->事{俿(侻)}fn 俿(乩:F)->事{let
亅=亣::令{乃:侽,乩,乔:侷,};let 东=促.iter().map(|&书|Y{估:侼,仆:侹,传:侺,伌:侸,..书}).collect();事::new(亅,东,Y::伉)}fn main(){let 倚=io::stdin();let 倛=io::stdout();if let Err(亗)=倃(&久,乲(),倚.lock(),倛.lock()){eprintln!("plan: {}",亗);std::process::exit(1);}}const
俉:&str="CG_TIME_SCALE";fn 倣()->亟{倀(std::env::var(俉).ok().as_deref())}fn 倀(丝:P<&str>)->亟{丝.and_then(|乞|乞.trim().parse::<亟>().ok()).filter(|仺|仺.is_finite()&&*仺>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乗:F=4*O;const 乬:U=(1<<乗)-1;const 仟:U=1|1<<12|1<<24|1<<36;const 伃:U=仟<<11;const 佅:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 侓:G=G([乬;3]);#[inline]fn q(q:F)->G{let mut 临=G::T;临.set(q);临}#[inline]fn test(self,q:F)->Q{(self.0[q/乗]>>(q%乗))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乗]|=1<<(q%乗);}#[inline]fn clear(&mut self,q:F){self.0[q/乗]&=!(1<<(q%乗));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仾(self)->[U;3]{self.0}#[inline]fn count(self)->丧{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丘(self,丫:G)->G{G([self.0[0]&!丫.0[0],self.0[1]&!丫.0[1],self.0[2]&!丫.0[2],])}#[inline]fn
iter(self)->企{企{伦:self.0,丼:0,}}#[inline]fn 倕(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&佅)<<(乗-O)),(b>>O)|((c&佅)<<(乗-O)),c>>O,])}#[inline]fn 倒(self)->G{let[a,b,c]=self.0;G([(a<<O)&乬,((b<<O)&乬)|(a>>(乗-O)),((c<<O)&乬)|(b>>(乗-O)),])}#[inline]fn
倓(self)->G{let[a,b,c]=self.0;G([(a>>1)&!伃&乬,(b>>1)&!伃&乬,(c>>1)&!伃&乬,])}#[inline]fn 倔(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仟&乬,(b<<1)&!仟&乬,(c<<1)&!仟&乬,])}#[inline]fn
与(self)->G{let j=self;j.倕()|j.倒()|j.倓()|j.倔()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丫:G)->G{G([self.0[0]&丫.0[0],self.0[1]&丫.0[1],self.0[2]&丫.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丫:G)->G{G([self.0[0]|丫.0[0],self.0[1]|丫.0[1],self.0[2]|丫.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丫:G){*self=*self|丫;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侓.丘(self)}}struct 企{伦:[U;3],丼:F,}impl Iterator for 企{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.丼<3{let 仴=j.伦[j.丼];if 仴!=0{j.伦[j.丼]=仴&(仴-1);return Some(j.丼*乗+仴.trailing_zeros()as F);}j.丼+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
K{乭{丏:丧,x:I,y:I},乌{一:I,x:I,y:I},乶{x:I,y:I},乪{x:I,y:I},}impl K{fn 倥(&self)->丳{match*self{K::乭{丏,x,y}=>format!("MOVE {} {x} {y}",丏),K::乌{一,x,y}=>format!("TRAIN {} {x} {y}",一),K::乶{x,y}=>format!("BUILD MINE {x} {y}"),K::乪{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
俖(丅:&[K])->丳{if 丅.is_empty(){return "WAIT".to_string();}let mut line=丳::with_capacity(16*丅.len());for(丐,三)in 丅.iter().enumerate(){if 丐>0{line.push(';');}line.push_str(&三.倥());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乫{佌,倴,俒,#[default]伂,}impl 乫{fn 乣(self)->I{match self{乫::佌=>1,_=>3,}}fn 仦(self)->Q{matches!(self,乫::俒|乫::伂)}fn 仧(self)->Q{self==乫::伂}}const 乡:I=I::MAX;struct
乷{为:G,valid:G,丰:G,丽:Box<[I]>,与:[[[I;4];M];2],}thread_local!{static 侨:RefCell<P<Arc<乷>>> =const{RefCell::new(None)};}impl 乷{fn 們(为:G,丰:G)->Arc<乷>{侨.with(|亀|{let
mut 亀=亀.borrow_mut();if let Some(亮)=亀.as_ref(){if 亮.为==为&&亮.丰==丰{return Arc::clone(亮);}}let 亮=Arc::new(乷::build(为,丰));*亀=Some(Arc::clone(&亮));亮})}fn build(为:G,丰:G)->乷{let
乕=!为;const 俆:[[(仞,仞);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 与=[[[乡;4];M];2];for(z,乁)in 俆.iter().enumerate(){for q in
乕.iter(){let(x,y)=((q%O)as 仞,(q/O)as 仞);let mut 丒=0;for&(俥,俦)in 乁{let(併,佶)=(x+俥,y+俦);if(0..O as 仞).contains(&併)&&(0..O as 仞).contains(&佶){let 丂=佶 as F*O+併
as F;if 乕.test(丂){与[z][q][丒]=丂 as I;丒+=1;}}}}}let mut 丽=vec![乡;M*M].into_boxed_slice();let mut 伬=[0u8;M];for 下 in 乕.iter(){let 云=&mut 丽[下*M..(下+1)*M];云[下]=0;伬[0]=下
as I;let(mut 伞,mut 伴)=(0,1);while 伞<伴{let q=伬[伞]as F;伞+=1;for&丂 in&与[0][q]{if 丂!=乡&&云[丂 as F]==乡{云[丂 as F]=云[q]+1;伬[伴]=丂;伴+=1;}}}}乷{为,valid:乕,丰,丽,与,}}#[inline]fn
丽(&self,下:F,也:F)->I{self.丽[下*M+也]}}const 侲:丧=100;const 乵:C=15;const 俍:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 之{乮(F),伻,}enum
习{伆,佂,佃,}#[derive(Clone)]struct J{map:Arc<乷>,两:乫,专:[G;2],七:[G;2],丸:G,乊:G,丄:[G;2],一:[G;3],亏:G,亾:[丧;M],万:[C;2],且:[C;2],丞:F,乢:丧,亳:丧,丗:P<之>,}fn 仍(专:G,享:F)->G{if!专.test(享){return
G::T;}let mut 仗=G::q(享);loop{let 丂=(仗|仗.与())&专;if 丂==仗{return 仗;}仗=丂;}}impl J{fn 丞(&self)->F{self.丞}fn 乢(&self)->丧{self.乢}fn 丗(&self)->P<之>{self.丗}fn 两(&self)->乫{self.两}fn
亱(&self,z:F)->C{let j=self;let 丄=j.丄[z];j.万[z]+10*C::from((丄&j.一[0]).count())+20*C::from((丄&j.一[1]).count())+30*C::from((丄&j.一[2]).count())}fn 万(&self,z:F)->C{self.万[z]}fn
且(&self,z:F)->C{self.且[z]}fn 伵(&self,z:F)->F{self.丄[z].count()as F}fn map(&self)->&乷{&self.map}fn 丙(&self,z:F)->G{self.专[z]}fn 七(&self,z:F)->G{self.七[z]}fn
丄(&self,z:F)->G{self.丄[z]}fn 亽(&self,q:F)->P<(丧,I)>{let j=self;(j.丄[0]|j.丄[1]).test(q).then(||(j.亾[q],j.伥(q)))}fn 亁(&self)->G{self.丄[self.丞]&self.亏}fn
丠(&self)->G{self.丸}fn 乂(&self)->G{self.乊}fn 井(&self,q:F)->P<I>{(q<M&&(self.丄[0]|self.丄[1]).test(q)).then(||self.伥(q))}fn 候(&mut self,z:F){let j=self;j.七=[仍(j.专[0],Z[0]),仍(j.专[1],Z[1])];let
信=j.丄[z].丘(j.七[z]);j.伮(信);j.佗();j.万[z]+=j.且[z];if j.万[z]<0{j.万[z]=0;j.伮(j.丄[z]);}j.亏|=j.丄[z];}fn 佗(&mut self){let j=self;for z in 0..2{let 七=j.七[z];let
丄=七&j.丄[z];let mut 且=C::from(七.count())+4*C::from((七&j.丸).count());for(一,倬)in 俍.iter().enumerate(){且-=倬*C::from((丄&j.一[一]).count());}j.且[z]=且;}}fn 仉(&mut
self){let j=self;if j.丗.is_some(){return;}if j.丞==1{if j.乢>=侲{j.丗=Some(match j.亱(0).cmp(&j.亱(1)){std::cmp::Ordering::Greater=>之::乮(0),std::cmp::Ordering::Less=>之::乮(1),std::cmp::Ordering::Equal=>之::伻,});return;}j.乢+=1;}j.丞=1-j.丞;j.候(j.丞);}fn
佻(&mut self,丅:&[K]){let j=self;if j.丗.is_some(){return;}let 伝=j.亳;for&三 in 丅{if j.俤(三,伝){continue;}j.中(三);if j.丗.is_some(){return;}}j.仉();}fn 俤(&self,三:K,伝:丧)->Q{let
j=self;match 三{K::乭{丏,x,y}=>{j.亥(x,y).is_none()||丏>=伝||j.佣(j.丞,丏).is_none()}K::乌{一,x,y}=>{j.亥(x,y).is_none()||一==0||一>j.两.乣()}K::乶{x,y}|K::乪{x,y}=>{j.亥(x,y).is_none()}}}#[inline]fn
亥(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.为.test(y*O+x)).then_some(y*O+x)}fn 佣(&self,z:F,丏:丧)->P<F>{self.丄[z].iter().find(|&q|self.亾[q]==丏)}#[inline]fn
伥(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 俘(&self,乤:F)->G{let 乂=self.乊&self.专[乤];let 七=self.七[乤];乂|(七&(七&乂).与())}fn
丿(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乤=1-z;let 丄=j.丄[0]|j.丄[1];let 俧=j.产();let 倪=match
一{1 if j.两==乫::佌=>j.一[1]|j.一[2],1=>丄,2=>j.一[1]|j.一[2],_=>G::T,};let 丿=俧|j.专[乤].丘(倪);if 一<3{丿.丘(j.俘(乤))}else{丿}}fn 亚(&self,z:F)->G{self.七[z]|self.七[z].与()}fn
产(&self)->G{let j=self;let 丄=j.丄[0]|j.丄[1];let 俳=G::q(Z[0])|G::q(Z[1]);j.map.valid.丘(丄|j.丸|j.乊|俳)}fn 中(&mut self,三:K)->Q{let j=self;if j.丗.is_some(){return
false;}let z=j.丞;let 亖=match 三{K::乌{一,x,y}=>j.倧(z,一,x,y),K::乭{丏,x,y}=>j.俺(z,丏,x,y),K::乶{x,y}=>j.build(z,true,x,y),K::乪{x,y}=>j.build(z,false,x,y),};let
Some(亖)=亖 else{return false;};match 亖{习::伆=>{}习::佂=>j.七[z]=仍(j.专[z],Z[z]),习::佃=>{j.七=[仍(j.专[0],Z[0]),仍(j.专[1],Z[1])];}}j.佗();if!j.专[0].test(Z[0]){j.丗=Some(之::乮(1));}else
if!j.专[1].test(Z[1]){j.丗=Some(之::乮(0));}true}fn 伮(&mut self,不:G){let j=self;j.丄[0]=j.丄[0].丘(不);j.丄[1]=j.丄[1].丘(不);for 一 in&mut j.一{*一=一.丘(不);}j.亏=j.亏.丘(不);}fn
侊(&mut self,z:F,q:F)->习{let j=self;j.伮(G::q(q));j.丸.clear(q);j.乊.clear(q);let 乤=1-z;if j.专[乤].test(q){j.专[乤].clear(q);j.专[z].set(q);习::佃}else if j.专[z].test(q){习::伆}else{j.专[z].set(q);习::佂}}fn
倧(&mut self,z:F,一:I,x:I,y:I)->P<习>{let j=self;if 一==0||一>j.两.乣(){return None;}let q=j.亥(x,y)?;let 丈=10*C::from(一);if j.万[z]<丈||!j.亚(z).test(q)||!j.丿(z,一).test(q){return
None;}let 亖=j.侊(z,q);j.佹(z,一,q,j.亳);j.亳=j.亳.saturating_add(1);j.万[z]-=丈;Some(亖)}fn 佹(&mut self,z:F,一:I,q:F,丏:丧){self.丄[z].set(q);self.一[F::from(一)-1].set(q);self.亾[q]=丏;}fn
俺(&mut self,z:F,丏:丧,x:I,y:I)->P<习>{let j=self;let 丑=j.亥(x,y)?;let 下=j.佣(z,丏)?;if!j.亏.test(下){return None;}let 一=j.伥(下);let 両=j.丿(z,一);let 乄=j.map.与[z][下];if
乄.contains(&(丑 as I))&&!両.test(丑){return None;}let mut 丌=j.map.丽(下,丑);let mut 仇=None;for&丂 in 乄.iter().take_while(|&&丂|丂!=乡){let 丂=F::from(丂);let 丽=j.map.丽(丂,丑);if
丽<丌&&両.test(丂){丌=丽;仇=Some(丂);}}let 仇=仇?;let 亖=j.侊(z,仇);j.丄[z].clear(下);j.一[F::from(一)-1].clear(下);j.亏.clear(下);j.佹(z,一,仇,丏);Some(亖)}fn build(&mut self,z:F,丸:Q,x:I,y:I)->P<习>{let
j=self;let 俕=if 丸{j.两.仦()}else{j.两.仧()};let q=j.亥(x,y)?;let 俽=j.map.丰.test(q);if!俕||j.万[z]<乵||俽!=丸||!j.专[z].test(q)||!j.亚(z).test(q)||!j.产().test(q){return
None;}if 丸{j.丸.set(q);}else{j.乊.set(q);}j.万[z]-=乵;Some(习::伆)}}const 侭:C=1000000000;const 伄:C=0;const 侴:C=1;const 俊:C=2;impl J{fn read(京:&str,上:&str,两:乫,乢:丧,)->Result<J,丳>{let
mut 丰=L::new();let mut 严=京.lines();let 丒=伙(&mut 严,"mine spot count")?;for _ in 0..丒{let[x,y]=亃(亴(&mut 严,"a mine spot")?,"a mine spot")?;丰.push((仪(x)?,仪(y)?));}你(严,"the initial input")?;let
mut 严=上.lines();let mut 伟=[0i64;4];for(丝,丷)in 伟.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伫]=亃(亴(&mut 严,丷)?,丷)?;if 伫.abs()>侭{return
Err(format!("{}: {} is out of range",丷,伫));}*丝=伫;}let mut 为=G::T;let mut 不=[G::T;2];let mut 七=[G::T;2];for y in 0..O{let 云=亴(&mut 严,"a map row")?.trim();if
云.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,云));}for(x,仙)in 云.bytes().enumerate(){let q=y*O+x;match 仙{b'#'=>为.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乆=F::from(仙==b'X'||仙==b'x');不[乆].set(q);if 仙.is_ascii_uppercase(){七[乆].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仙 as char))}}}}let
俜=伙(&mut 严,"building count")?;let mut 乓=L::new();for _ in 0..俜{let 亃=亃(亴(&mut 严,"a building")?,"a building")?;乓.push(亃);}let 伵=伙(&mut 严,"unit count")?;let
mut 丄=L::new();for _ in 0..伵{let 亃=亃(亴(&mut 严,"a unit")?,"a unit")?;丄.push(亃);}你(严,"the turn's input")?;let 佸=乓.iter().find(|体|体[0]==0&&体[1]==伄).ok_or("the bot's own HQ is not among the buildings")?;let
佫=伒(佸[2],佸[3],为)?;let z=Z.iter().position(|&享|享==佫).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佫))?;let mut 侄=G::T;for(x,y)in 丰{if
为.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}侄.set(y*O+x);}let 伊=|乆:F|if 乆==0{z}else{1-z};let mut l=J{map:乷::們(为,侄),两,专:[G::T;2],七:[G::T;2],丸:G::T,乊:G::T,丄:[G::T;2],一:[G::T;3],亏:G::T,亾:[0;M],万:[0;2],且:[0;2],丞:z,乢,亳:1,丗:None,};for
乆 in 0..2{let 乥=伊(乆);l.专[乥]=不[乆];l.七[乥]=七[乆];l.万[乥]=伟[2*乆];l.且[乥]=伟[2*乆+1];}let mut 佷=G::T;for[乥,仱,x,y]in 乓{let 乆=使(乥)?;let q=伒(x,y,为)?;if!不[乆].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佷.test(q){return Err(format!("two buildings at ({x}, {y})"));}佷.set(q);match 仱{伄
if q==Z[伊(乆)]=>{}伄=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侴=>l.丸.set(q),俊=>l.乊.set(q),_=>return Err(format!("unknown building type {}",仱)),}}let
mut 乇=G::T;let mut 仲=0u32;for[乥,丏,一,x,y]in 丄{let 乆=使(乥)?;let q=伒(x,y,为)?;if!不[乆].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乇.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丏=丧::try_from(丏).map_err(|_|format!("unit id {} is out of range",丏))?;if
乇.iter().any(|丫|l.亾[丫]==丏){return Err(format!("two units have id {}",丏));}乇.set(q);l.丄[伊(乆)].set(q);l.一[一 as F-1].set(q);l.亾[q]=丏;仲=仲.max(丏);}if!乇.is_empty(){l.亳=仲.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仲))?;}l.亏=l.丄[z];Ok(l)}}fn
亴<'a>(严:&mut Lines<'a>,丷:&str)->Result<&'a str,丳>{严.next().ok_or_else(||format!("the text ends before {}",丷))}fn 你(mut 严:Lines<'_>,丷:&str)->Result<(),丳>{match
严.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丷)),None=>Ok(()),}}fn 亃<const N:F>(line:&str,丷:&str)->Result<[C;N],丳>{let
mut 仾=line.split_whitespace();let mut 侐=[0i64;N];for 丝 in&mut 侐{let 什=仾.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丷))?;*丝=什.parse().map_err(|_|format!("{}: {:?} is not an integer",丷,什))?;}if
仾.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丷));}Ok(侐)}fn 伙(严:&mut Lines<'_>,丷:&str)->Result<F,丳>{let[丒]=亃(亴(严,丷)?,丷)?;F::try_from(丒).ok().filter(|&丒|丒<=M).ok_or_else(||format!("{}: {} is out of range",丷,丒))}fn
仪(丝:C)->Result<F,丳>{F::try_from(丝).ok().filter(|&丝|丝<O).ok_or_else(||format!("coordinate {} is off the map",丝))}fn 伒(x:C,y:C,为:G)->Result<F,丳>{let q=仪(y)?*O+仪(x)?;if
为.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 使(乥:C)->Result<F,丳>{match 乥{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",乥)),}}const
価:C=15;const 侵:C=6;const 佉:C=10;const 係:I=3;#[derive(Clone,Copy)]struct Y{open:C,仩:C,佚:I,伡:C,传:C,佰:C,估:I,余:C,佘:C,伌:C,亪:C,亻:C,丹:C,亭:C,五:C,仆:C,}impl Y{const
伉:Y=Y{open:3,仩:2,佚:12,伡:4,传:価,佰:侵,估:係,余:2,佘:2,伌:0,亪:0,亻:0,丹:0,亭:0,五:0,仆:0,};const fn 倲(self,亪:C,亻:C,丹:C)->Y{Y{亪,亻,丹,..self}}const fn 倱(self,亭:C,五:C)->Y{Y{亭,五,..self}}}fn
仝(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仈(l:&J,下:G)->[I;M]{let mut 乍=[I::MAX;M];let 乕=l.map().valid;let mut 乀=下&乕;let mut 乇=乀;let mut d=0u8;while!乀.is_empty(){for
q in 乀.iter(){乍[q]=d;}let 丂=(乀.与()&乕).丘(乇);乇|=丂;乀=丂;d=d.saturating_add(1);}乍}fn 俫(l:&J)->I{let 乍=仈(l,l.丙(0));l.丙(1).iter().map(|q|乍[q]).min().unwrap_or(I::MAX)}struct
二<'a>{书:&'a Y,丁:F,theirs:[I;M],亼:[I;M],free:G,乑:G,ours:G,丰:G,丄:G,}impl 二<'_>{fn new<'a>(l:&J,书:&'a Y)->二<'a>{let 丁=l.丞();let 丸=仈(l,l.丙(丁));let 亇=仈(l,l.丙(1-丁));let
亼=仈(l,G::q(Z[1-丁]));let 买=l.map().valid.丘(l.丙(0)|l.丙(1));let mut 乑=G::T;for q in 买.iter(){if 丸[q]<=亇[q]{乑.set(q);}}二{书,丁,theirs:亇,亼,free:买,乑,ours:l.丙(丁),丰:l.map().丰,丄:l.丄(丁),}}fn
value(&self,q:F)->C{let j=self;let p=j.书;let 丶=C::from((G::q(q).与()&j.free).count());let 何=p.佚;let 仩=C::from(何)-C::from(j.theirs[q].min(何));let 享=if p.伡>0{C::from(j.亼[q].min(30))/p.伡}else{0};let
专=C::from((G::q(q).与()&j.ours).count());let line=if 专<=1{p.亻}else{0};let 丹=if j.丰.test(q){p.丹}else{0};let 倞=if 丶==0{p.亭}else{0};let 五=p.五*(22-C::from(j.亼[q].min(22)));let
仆=p.仆*C::from((G::q(q).与()&j.丄).count());p.open*丶+p.仩*仩-享+p.亪*专-line+丹-倞+五-仆}fn 丌(&self,不:G)->P<F>{不.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
伕(&mut self,q:F){let j=self;j.free.clear(q);j.乑.clear(q);j.ours.set(q);j.丄.set(q);}}fn 仕(l:&mut J,三:K,丆:&mut Vec<K>)->Q{let 位=l.中(三);if 位{丆.push(三);}位}fn
丛(l:&mut J,下:F,也:F,丆:&mut Vec<K>)->Q{let Some((丏,_))=l.亽(下)else{return false;};let(x,y)=仝(也);仕(l,K::乭{丏,x,y},丆)}fn 倫(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-佉}#[derive(Clone,Copy)]enum
么{代,争{丠:I,丄:I},}fn 侍(l:&J,书:&Y,于:么)->L<K>{let mut 丯=l.clone();let mut 上=二::new(l,书);let mut 丆=L::new();亂(&mut 丯,&mut 上,&mut 丆);match 于{么::代=>倆(&mut 丯,&mut
上,&mut 丆),么::争{丠,丄}=>{俩(&mut 丯,&mut 上,丠,丄,&mut 丆)}}丆}fn 亂(l:&mut J,上:&mut 二<'_>,丆:&mut Vec<K>){let 丁=上.丁;let mut 乾:L<F> =L::new();loop{let 両=l.丿(丁,1);let
mut 亷:P<(丧,F)> =None;for 下 in l.亁().iter(){if 乾.contains(&下){continue;}let 亄=(G::q(下).与()&上.free&両).count();if 亄==0{乾.push(下);}else if 亷.is_none_or(|(n,_)|亄<n){亷=Some((亄,下));}}let
Some((_,下))=亷 else{break;};上.丄.clear(下);match 上.丌(G::q(下).与()&上.free&両){Some(也)if 丛(l,下,也,丆)=>上.伕(也),_=>{上.丄.set(下);乾.push(下);}}}for 下 in 乾{if!l.亁().test(下){continue;}let
丑=[上.乑,上.free].into_iter().find_map(|不|{不.iter().min_by_key(|&q|(l.map().丽(下,q),q))});if let Some(丑)=丑{丛(l,下,丑,丆);}}}fn 倆(l:&mut J,上:&mut 二<'_>,丆:&mut
Vec<K>){let 丁=上.丁;let p=上.书;let h=(p.传-C::from(l.乢())).max(p.佰);let 値=(C::from(俫(l))-p.余).max(0)/p.佘.max(1)+p.伌;loop{let 万=l.万(丁);let 丄=l.伵(丁)as C;let
個=C::from(上.乑.count());let q=上.丌(l.亚(丁)&上.free&l.丿(丁,1));let 侏=倫(h,(個/(丄+1)).min(値));let 佔=q.is_some()&&万>=佉&&侏>0;let 佱=4*h-乵;let 丹=佲(l,上);let 俟=丹.is_some()&&万>=乵&&佱>0;if
俟&&(!佔||佱>=侏){let(x,y)=仝(丹.unwrap_or_default());if 仕(l,K::乶{x,y},丆){continue;}break;}let Some(q)=q.filter(|_|佔)else{break;};let(x,y)=仝(q);if!仕(l,K::乌{一:1,x,y},丆){break;}上.伕(q);}}fn
佲(l:&J,上:&二<'_>)->P<F>{let 丁=上.丁;(l.map().丰&l.七(丁)&l.产()).iter().filter(|&丹|上.theirs[丹]>上.书.估).min_by_key(|&丹|(l.map().丽(Z[丁],丹),丹))}fn 俩(l:&mut J,上:&mut
二<'_>,丠:I,丄:I,丆:&mut Vec<K>,){let 丁=上.丁;for _ in 0..丠{let Some(丹)=佲(l,上)else{break;};let(x,y)=仝(丹);if!仕(l,K::乶{x,y},丆){break;}}for _ in 0..丄{let Some(q)=上.丌(l.亚(丁)&上.free&l.丿(丁,1))else{break;};let(x,y)=仝(q);if!仕(l,K::乌{一:1,x,y},丆){break;}上.伕(q);}}fn
仹(l:&J)->Q{if l.丗().is_some(){return false;}let 丁=l.丞();let 万=l.万(丁);let 並=交::new(l,丁,true);l.丙(1-丁).iter().any(|q|並.丈[q]<=万)}#[derive(Clone,Copy)]struct
丣{亞:I,于:么,}const 伶:I=I::MAX;const 侖:[么;8]=[么::代,么::争{丠:0,丄:0},么::争{丠:0,丄:1},么::争{丠:0,丄:2},么::争{丠:0,丄:伶,},么::争{丠:1,丄:0},么::争{丠:1,丄:1},么::争{丠:1,丄:伶,},];struct
伺<'a>{东:&'a[Y],仐:Y,rest:丣,乺:P<乖>,倌:Settings,}impl 伺<'_>{fn 书(&self,丛:丣)->&Y{&self.东[F::from(丛.亞)%self.东.len()]}fn 丅(&self,l:&J,丛:丣)->L<K>{侍(l,self.书(丛),丛.于)}}fn
伱(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伧=|什:U|hash=(hash^什).wrapping_mul(0x0100000001b3);for(借,临)in[l.丙(丁),l.丄(丁),l.丠()].into_iter().enumerate(){for
q in 临.iter(){伧(((借 as U)<<8)|q as U);}伧(0xff);}伧(l.万(丁)as U);hash}enum 亣{令{乃:F,乩:F,乔:亠,},}struct 事{亅:亣,东:L<Y>,仐:Y,丕:L<丣>,们:L<U>,亸:F,串:丧,仔:Q,}impl 事{fn
new(亅:亣,东:L<Y>,仐:Y)->事{事{亅,东,仐,丕:L::new(),们:L::new(),亸:0,串:0,仔:false,}}fn 倄(&mut self,l:&J)->Q{let j=self;let 倗=matches!(j.亅,亣::令{..});let 倅=!倗&&j.串>0&&j.亸>=j.们.len();if!j.仔&&(倅||仹(l)){j.仔=true;}!j.仔}fn
end(&mut self){self.仔=true;}fn 上(&mut self,l:&J,乺:P<乖>)->L<K>{let j=self;let 作=伺{东:&j.东,仐:j.仐,rest:丣{亞:0,于:么::代,},乺,倌:Settings::DEFAULT,};let 丁=l.丞();let
俾=j.们.get(j.亸)==Some(&伱(l,丁));if!俾{let 乎=j.串==0;j.丕=match j.亅{亣::令{乃,乩,乔,}=>{let 乔=&乔;let 九=俁;let mut 乁=丕(l,&j.东,九,乃,乩,乺,乔,);乁.resize(乃,九);乁}};j.们=match
j.亅{亣::令{..}=>倘(l,&j.东,&j.丕),};j.亸=0;j.串+=1;}let 丛=j.丕.get(j.亸).copied().unwrap_or(作.rest);j.亸+=1;作.丅(l,丛)}}fn 倘(l:&J,东:&[Y],丕:&[丣])->L<U>{let 丁=l.丞();let
mut 佭=L::with_capacity(丕.len());let mut 乧=l.clone();for&丛 in 丕{if 乧.丗().is_some(){break;}佭.push(伱(&乧,丁));乧=伋(&乧,&丅(&乧,东,丛));}佭}const 俋:C=3;struct Settings{乃:C,}impl
Settings{const DEFAULT:Settings=Settings{乃:俋};}fn 乁(l:&J,下:G)->[I;M]{let 乕=l.map().valid;let mut 乍=[I::MAX;M];let mut 乀=下&乕;let mut 乇=乀;let mut d=0u8;while!乀.is_empty(){for
q in 乀.iter(){乍[q]=d;}let 丂=(乀.与()&乕).丘(乇);乇|=丂;乀=丂;d=d.saturating_add(1);}乍}fn 倭(l:&J,z:F)->C{let 仒=乁(l,l.七(z));let 亇=乁(l,l.七(1-z));l.map().valid.iter().map(|q|match
仒[q].cmp(&亇[q]){_ if 仒[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 促:[Y;27]=东();const
俁:丣=丣{亞:16,于:么::代,};const fn 东()->[Y;27]{let 亪=[0,2,4];let 亻=[0,3,6];let 五=[0,1,2];let mut 丆=[Y::伉;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亭=if g>0||t>0{4}else{0};丆[i]=Y::伉.倲(亪[g],亻[t],0).倱(亭,五[w]);i+=1;}丆}#[derive(Default)]struct 伈{不:C,且:C,万:C,乿:C,丟:C,仛:C,並:C,仜:C,}#[derive(Clone,Copy)]struct
亠{不:C,且:C,万:C,丟:C,乿:C,仛:C,並:C,仜:C,}fn 倍(l:&J,z:F)->[C;M]{struct 仂{七:G,义:[u16;M],丵:[u16;M],size:[C;M],丟:[C;M],亓:u16,}fn 亝(d:&mut 仂,v:F){d.亓+=1;d.义[v]=d.亓;d.丵[v]=d.亓;d.size[v]=1;for
w in(G::q(v).与()&d.七).iter(){if d.义[w]==0{亝(d,w);d.size[v]+=d.size[w];d.丵[v]=d.丵[v].min(d.丵[w]);if d.丵[w]>=d.义[v]{d.丟[v]+=d.size[w];}}else{d.丵[v]=d.丵[v].min(d.义[w]);}}}let
mut d=仂{七:l.七(z),义:[0;M],丵:[0;M],size:[0;M],丟:[0;M],亓:0,};let 乳=Z[z];if d.七.test(乳){亝(&mut d,乳);}d.丟[乳]=0;d.丟}fn 倏(l:&J,z:F)->伈{let 七=l.七(z);let 丟=倍(l,z);let
亼=仈(l,G::q(Z[1-z]));let mut 丆=伈{不:C::from(七.count()),且:l.且(z),万:l.万(z),並:C::MAX,仜:倭(l,z)/2,..伈::default()};for q in 七.iter(){if 丟[q]>0{丆.乿+=1;丆.丟=丆.丟.max(丟[q]);}if
q!=Z[z]&&(G::q(q).与()&七).count()<=1{丆.仛+=1;}丆.並=丆.並.min(C::from(亼[q]));}丆}fn 丝(l:&J,z:F,乔:&亠)->C{let s=倏(l,z);let o=乔;o.不*s.不+o.且*s.且+o.万*s.万-o.丟*s.丟-o.乿*s.乿-o.仛*s.仛-o.並*s.並.min(30)+o.仜*s.仜}fn
伋(l:&J,丅:&[K])->J{let mut 丂=l.clone();丂.佻(丅);if 丂.丗().is_none(){丂.佻(&[]);}丂}fn 丅(l:&J,东:&[Y],丛:丣)->L<K>{侍(l,&东[F::from(丛.亞)%东.len()],丛.于,)}fn 佺(l:&J,东:&[Y],丕:&[丣],九:丣,乃:F,)->J{let
mut 乧=l.clone();for 上 in 0..乃{if 乧.丗().is_some(){break;}let 丛=丕.get(上).copied().unwrap_or(九);乧=伋(&乧,&丅(&乧,东,丛));}乧}fn 倜(l:&J,东:&[Y])->L<(丣,L<K>)>{let mut
丆:L<(丣,L<K>)> =L::new();for 亞 in 0..东.len().min(F::from(I::MAX)){for&于 in&侖{let 丛=丣{亞:亞 as I,于,};let c=丅(l,东,丛);if!丆.iter().any(|(_,丫)|*丫==c){丆.push((丛,c));}}}丆}fn
俗(l:&J,东:&[Y],九:丣,乃:F,乩:F,乺:P<乖>,乔:&亠,)->L<丣>{let 丁=l.丞();let 仓=||乺.is_some_and(|t|乖::now()>=t);let 他=|丕:&[丣]|丝(&佺(l,东,丕,九,乃),丁,乔);let mut 丌:(C,L<丣>)=(他(&[]),L::new());let
mut 乀:L<(J,L<丣>)> =vec![(l.clone(),L::new())];for 俣 in 0..乃{let mut 乹:L<(C,J,L<丣>)> =L::new();let mut 乇:HashMap<U,F> =HashMap::new();for(佴,丕)in&乀{if 仓(){break;}for(丛,c)in
倜(佴,东){let 丂=伋(佴,&c);let mut 伔=丕.clone();伔.push(丛);let v=丝(&丂,丁,乔);let 乏=伱(&丂,丁);match 乇.get(&乏){Some(&丼)if 乹[丼].0>=v=>{}Some(&丼)=>乹[丼]=(v,丂,伔),None=>{乇.insert(乏,乹.len());乹.push((v,丂,伔));}}}}if
乹.is_empty(){break;}乹.sort_by(|a,b|b.0 .cmp(&a.0));乹.truncate(乩);let 亀=俣+1==乃||仓();for(i,(_,_,丕))in 乹.iter().take(if 亀{乩}else{1}).enumerate(){if i>0&&仓(){break;}let
v=他(丕);if v>丌.0{丌=(v,丕.clone());}}if 亀{break;}乀=乹.into_iter().map(|(_,s,p)|(s,p)).collect();}丌.1}fn 丕(l:&J,东:&[Y],九:丣,乃:F,俸:F,乺:P<乖>,乔:&亠,)->L<丣>{let 丁=l.丞();let
他=|丕:&[丣]|丝(&佺(l,东,丕,九,乃),丁,乔);let mut 丌:(C,L<丣>)=(他(&[]),L::new());let mut 乩=2;loop{let 个=俗(l,东,九,乃,乩,乺,乔);let v=他(&个);if v>丌.0{丌=(v,个);}let 仓=乺.is_some_and(|t|乖::now()>=t);if
仓||(乺.is_none()&&乩>=俸){break;}乩*=2;}丌.1}const 侠:F=4+12;fn 乃(伢:impl BufRead,mut 仸:impl Write,mut 伍:impl FnMut(&str,&str,丧,乖)->丳,)->Result<(),丳>{let mut
严=伿{佾:伢};let Some(乎)=严.line()?else{return Ok(());};let mut 京=format!("{}\n",乎);严.take(丒(&乎,"mine spot count")?,&mut 京)?;let mut 上=0;loop{let Some(乎)=严.line()?else{return
Ok(());};let 亜=乖::now();上+=1;let mut 乞=format!("{}\n",乎);严.take(侠-1,&mut 乞)?;for 丷 in["building count","unit count"]{let line=严.侀()?;let n=丒(&line,丷)?;乞.push_str(&line);乞.push('\n');严.take(n,&mut
乞)?;}let 伍=伍(&京,&乞,上,亜);writeln!(仸,"{}",伍).and_then(|()|仸.flush()).map_err(|亗|format!("turn {}: cannot write the answer: {}",上,亗))?;}}struct 伿<R>{佾:R,}impl<R:BufRead>伿<R>{fn
line(&mut self)->Result<P<丳>,丳>{let mut line=丳::new();match self.佾.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(亗)=>Err(format!("cannot read input: {}",亗)),}}fn
侀(&mut self)->Result<丳,丳>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乞:&mut String)->Result<(),丳>{for
_ in 0..n{乞.push_str(&self.侀()?);乞.push('\n');}Ok(())}}fn 丒(line:&str,丷:&str)->Result<F,丳>{match line.trim().parse::<F>(){Ok(n)if n<=侬=>Ok(n),_=>Err(format!("bad {} {line:?}",丷)),}}const
侬:F=144;const 俇:C=3;const 侧:C=2;const 侱:F=16;fn 倂(l:&J,丆:&mut Vec<K>){let z=l.丞();let mut 亅=佄{s:l.clone(),z,乻:1-z,丑:Z[1-z],仯:Z[z],上:l.乢(),丆,};亅.run();}struct
佄<'a>{s:J,z:F,乻:F,丑:F,仯:F,上:丧,丆:&'a mut Vec<K>,}fn 伀(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伨(丏:丧,q:F)->K{let(x,y)=伀(q);K::乭{丏,x,y}}fn 侌(一:I,q:F)->K{let(x,y)=伀(q);K::乌{一,x,y}}impl
佄<'_>{fn run(&mut self){let j=self;if j.仳(){return;}j.亂();if j.仳(){return;}j.乓();j.倩();}fn 乍(&self,下:F,也:F)->C{C::from(self.s.map().丽(下,也))}fn 中(&mut self,三:K)->Q{let
举=self.s.中(三);if 举{self.丆.push(三);}举}fn 亿(&self,丢:&J)->Q{丢.丗()==Some(之::乮(self.z))}fn 仳(&mut self)->Q{let j=self;if j.s.丗().is_some(){return false;}for
q in j.s.亁().iter(){if j.乍(q,j.丑)!=1{continue;}let Some((丏,_))=j.s.亽(q)else{continue;};let 三=伨(丏,j.丑);let mut 丢=j.s.clone();if 丢.中(三)&&j.亿(&丢){return j.中(三);}}let
Some(丩)=j.俠()else{return false;};let mut 丢=j.s.clone();if!丩.iter().all(|&三|丢.中(三))||!j.亿(&丢){return false;}for 三 in 丩{j.中(三);}true}fn 俠(&self)->P<L<K>>{let
j=self;const 似:C=C::MAX;let 乚=j.s.两().乣();let 両:L<G> =(1..=乚).map(|一|j.s.丿(j.z,一)).collect();let 介=|q:F|{両.iter().position(|临|临.test(q)).map(|丐|(丐 as I+1,10*(丐
as C+1)))};let 七=j.s.七(j.z);let mut 丈=[似;M];let mut 丨=[F::MAX;M];let mut 举=[false;M];for q in 七.iter(){丈[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!举[q]&&丈[q]<似&&丂.is_none_or(|n|丈[q]<丈[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};举[q]=true;if q==j.丑{break;}for&乄 in&j.s.map().与[0][q]{if 乄==乡{break;}let 乄=F::from(乄);if 举[乄]||七.test(乄){continue;}let Some((_,亐))=介(乄)else{continue;};if
丈[q]+亐<丈[乄]{丈[乄]=丈[q]+亐;丨[乄]=q;}}}if 丈[j.丑]>j.s.万(j.z){return None;}let mut 丩=L::new();let mut q=j.丑;while!七.test(q){let(一,_)=介(q)?;丩.push(侌(一,q));q=丨[q];}丩.reverse();Some(丩)}fn
乱(&self,主:&J)->C{let(丁,互)=(self.z,self.乻);let 丨=&self.s;let 丒=|临:G|C::from(临.count());let 专=丒(主.丙(丁))-丒(丨.丙(丁));let 仼=丒(丨.丙(互))-丒(主.丙(互));let 俵=丒(丨.七(互))-丒(主.七(互));let
仰=丨.亱(互)-主.亱(互);let 丠=丒(丨.丠()&丨.丙(互))-丒(主.丠()&主.丙(互));12*专+20*仼+8*(俵-仼).max(0)+5*仰+25*丠}fn 侅(&self,q:F)->C{let j=self;let 丹=j.s.map().丰.test(q);if 丹&&j.s.两().仦()&&!j.s.丙(j.z).test(q){20}else{0}}fn
佡(&self,久:C)->Q{let j=self;j.s.丄(j.乻).iter().any(|q|j.乍(q,j.仯)<=久)}fn 亂(&mut self){let j=self;let mut 丄:L<(C,F)> =j.s.亁().iter().map(|q|(j.乍(q,j.丑),q)).collect();丄.sort_unstable();for(仮,q)in
丄{if j.s.丗().is_some(){return;}let Some((丏,一))=j.s.亽(q)else{continue;};let 両=j.s.丿(j.z,一);let 乄=j.s.map().与[j.z][q];let mut 丌:P<(C,F)> =None;for&丂 in 乄.iter().take_while(|&&丂|丂!=乡){let
丂=F::from(丂);if!両.test(丂){continue;}let mut 丢=j.s.clone();if!丢.中(伨(丏,丂)){continue;}let 丝=if j.亿(&丢){100000}else{j.乱(&丢)+10*(仮-j.乍(丂,j.丑))+j.侅(丂)};if 丌.is_none_or(|(予,_)|丝>予){丌=Some((丝,丂));}}if
let Some((丝,丂))=丌{if 丝>0{j.中(伨(丏,丂));}}}}fn 乓(&mut self){let j=self;if j.s.丗().is_some(){return;}if j.佡(俇){j.倦();}else{j.丠();}}fn 倦(&mut self){let j=self;if!j.s.两().仧()||j.s.万(j.z)<乵{return;}let
乄=j.s.map().与[j.z][j.仯];let 不:L<F> =乄.iter().take_while(|&&q|q!=乡).map(|&q|F::from(q)).collect();let 丸=j.s.丙(j.z);if 不.iter().any(|&q|丸.test(q)&&j.s.乂().test(q)){return;}for
q in 不{let(x,y)=伀(q);if j.中(K::乪{x,y}){return;}}}fn 丠(&mut self){let j=self;if!j.s.两().仦(){return;}while j.s.万(j.z)>=乵{let 买=j.s.map().丰&j.s.丙(j.z)&j.s.七(j.z)&j.s.产();let
Some(q)=买.iter().min_by_key(|&q|(j.乍(q,j.仯),q))else{return;};let(x,y)=伀(q);if!j.中(K::乶{x,y}){return;}}}fn 倩(&mut self){let j=self;let 侁=if j.上<=3{0}else{5};for
_ in 0..侱{if j.s.丗().is_some(){return;}let 万=j.s.万(j.z);if 万<10{return;}let 俴=j.佡(侧);let 亲=j.s.亚(j.z);let mut 丌:P<(C,K)> =None;let mut 伳:P<(C,K)> =None;let
mut 佒=G::T;for 一 in 1..=j.s.两().乣(){let 両=j.s.丿(j.z,一);let 亄=(亲&両).丘(佒);佒=両;let 亐=10*C::from(一);if 亐>万{continue;}for q in 亄.iter(){let 三=侌(一,q);let mut
丢=j.s.clone();if!丢.中(三){continue;}let 伤=丢.万(j.z);let 且=丢.且(j.z);let 仰=j.s.亱(j.乻)-丢.亱(j.乻);let 俓=if 仰>0&&俴{伤+且>=0}else{且>=0&&(伤>=侁||仰>0)};if!俓{continue;}let
乱=j.乱(&丢);let 丝=乱+j.侅(q)-6*j.乍(q,j.丑)-10*(C::from(一)-1);if 乱>0{if 丌.is_none_or(|(予,_)|丝>予){丌=Some((丝,三));}}else if 且>=3&&伤>=侁+10&&一==1{if 伳.is_none_or(|(予,_)|丝>予){伳=Some((丝,三));}}}}let
Some((_,三))=丌.or(伳)else{return;};if!j.中(三){return;}}}}fn 修(l:&J,丆:&mut Vec<K>){丆.clear();倂(l,丆);}const 亍:C=C::MAX;const 侰:F=8;struct 交{丈:[C;M],丨:[I;M],}fn
优(l:&J,z:F,亂:Q,丑:F)->C{佛(l,z,亙(l,z,亂),丑)}fn 佛(l:&J,z:F,亙:G,丑:F)->C{let mut 丈=亍;佤(l,z,亙,|丛,new|{if new.test(丑){丈=10*丛 as C;}丈!=亍});丈}fn 倇(l:&J,z:F,亙:G,丮:C)->G{let
mut 仹=G::T;佤(l,z,亙,|丛,new|{if 10*丛 as C>丮{return true;}仹|=new;false});仹.丘(l.七(z))}fn 亙(l:&J,z:F,亂:Q)->G{if 亂&&z==l.丞(){l.亁()}else{G::T}}fn 佤(l:&J,z:F,亙:G,mut
亝:impl FnMut(F,G)->Q,){let 乚=l.两().乣();let mut 介=[G::T;3];let mut 乇=G::T;for 一 in 1..=乚{let 临=l.丿(z,一);介[F::from(一)-1]=临.丘(乇);乇|=临;}let mut 丶=[l.七(z),G::T,G::T,G::T];for
q in(亙&l.丄(z)).iter(){if let Some(一)=l.井(q).filter(|&一|一<=乚){丶[0]|=G::q(q).与()&l.丿(z,一);}}let mut 举=G::T;for 丛 in 0..{let new=丶[丛%4].丘(举);丶[丛%4]=G::T;if
亝(丛,new){return;}举|=new;let 丂=new.与().丘(举);for(俔,不)in 介.iter().enumerate(){丶[(丛+俔+1)%4]|=丂&*不;}if 丶.iter().all(|临|临.丘(举).is_empty()){break;}}}impl 交{fn
new(l:&J,z:F,亂:Q)->交{let 乚=F::from(l.两().乣());let mut 両=[G::T;3];for(丐,临)in 両.iter_mut().enumerate().take(乚){*临=l.丿(z,丐 as I+1);}let 七=l.七(z);let 与=&l.map().与[0];let
mut 丈=[亍;M];let mut 丨=[乡;M];let mut 丶=[0u8;M];let mut 丒=0;let mut 侃=G::T;for q in 七.iter(){丈[q]=0;丶[丒]=q as I;丒+=1;}if 亂&&z==l.丞(){for 下 in l.亁().iter(){let
Some(一)=l.井(下)else{continue;};let 乁=G::q(下).与()&両[F::from(一)-1];for q in 乁.iter(){if 丈[q]!=0{丈[q]=0;丶[丒]=q as I;丒+=1;}}}}while 丒>0{let mut 丌=0;for 丐 in
1..丒{let(a,b)=(F::from(丶[丐]),F::from(丶[丌]));if(丈[a],a)<(丈[b],b){丌=丐;}}let q=F::from(丶[丌]);丒-=1;丶[丌]=丶[丒];侃.set(q);for&丂 in&与[q]{if 丂==乡{break;}let 丂=F::from(丂);if
侃.test(丂)||七.test(丂){continue;}let Some(一)=両[..乚].iter().position(|临|临.test(丂))else{continue;};let 例=丈[q]+10*(一 as C+1);if 例<丈[丂]{if 丈[丂]==亍{丶[丒]=丂 as
I;丒+=1;}丈[丂]=例;丨[丂]=q as I;}}}交{丈,丨}}fn path(&self,丑:F)->L<F>{let mut path=L::new();if 丑>=M||self.丈[丑]==亍{return path;}let mut q=丑;while self.丨[q]!=乡{path.push(q);q=F::from(self.丨[q]);}path.reverse();path}}struct
仠{丅:L<K>,丈:C,}fn 俷(l:&J,z:F,q:F)->P<I>{(1..=l.两().乣()).find(|&一|l.丿(z,一).test(q))}fn 倊(丯:&mut J,丁:F,亩:F,丮:C,丅:&mut Vec<K>,乨:&mut i64,)->Q{loop{if 丯.丗()==Some(之::乮(丁)){return
true;}if 丯.丙(丁).test(亩)&&丯.七(丁).test(亩){return true;}let 並=交::new(丯,丁,false);let 丈=並.丈[亩];if 丈==亍||*乨+丈>丮{return false;}let Some(&q)=並.path(亩).first()else{return
false;};let Some(一)=俷(丯,丁,q)else{return false;};let 三=K::乌{一,x:(q%O)as I,y:(q/O)as I,};if!丯.中(三){return false;}*乨+=10*C::from(一);丅.push(三);}}fn 佐(l:&J,丁:F,俭:&[F],丮:C)->P<仠>{let
mut 丯=l.clone();let mut 丅=L::new();let mut 乨=0;for&亩 in 俭{if!倊(&mut 丯,丁,亩,丮,&mut 丅,&mut 乨){return None;}}(乨<=丮).then_some(仠{丅,丈:乨,})}fn 伓(l:&J,丑:F,丮:C)->P<仠>{if
丮<0||l.丗().is_some()||丑>=M||l.map().为.test(丑){return None;}let 丁=l.丞();if l.丙(丁).test(丑)&&l.七(丁).test(丑){return Some(仠{丅:L::new(),丈:0,});}let mut 丌=佐(l,丁,&[丑],丮);let
乎=交::new(l,丁,false);let mut 乂:L<F> =(l.乂()&l.丙(1-丁)).iter().filter(|&乊|乊!=丑&&乎.丈[乊]!=亍).collect();乂.sort_by_key(|&乊|(l.map().丽(乊,丑),乊));乂.truncate(侰);for
乊 in 乂{if let Some(丩)=佐(l,丁,&[乊,丑],丮){if 丌.as_ref().is_none_or(|予|丩.丈<予.丈){丌=Some(丩);}}}丌}const 侮:F=20000;struct 仡{丅:L<K>,丈:C,不:C,丄:C,乓:C,且:C,业:C,}struct
伅{不:C,丄:C,乓:C,}fn 佪(l:&J,z:F)->伅{let 七=l.七(z);let 丄=l.丄(z)&七;let 丝=丄.iter().filter_map(|q|l.井(q)).map(|一|10*C::from(一)).sum();伅{不:C::from(七.count()),丄:丝,乓:C::from(((l.丠()|l.乂())&七).count()),}}struct
佇{丁:F,仁:G,丨:伅,且:C,亰:F,丮:C,亵:F,path:L<K>,个:L<仡>,index:HashMap<仄,F,BuildHasherDefault<DefaultHasher>>,}type 仄=[U;3];fn 倰(mut 乏:仄,q:F)->仄{乏[q/64]|=1<<(q%64);乏}impl
佇{fn 佟(&mut self,丯:&J,亀:P<F>,仼:仄,丈:C,)->Q{let j=self;let 丁=j.丁;let 亄=match 亀{None=>丯.亚(丁),Some(q)=>G::q(q).与(),}&j.仁;let 亄=亄.丘(丯.丙(丁));let mut 両=[G::T;3];for
一 in 1..=丯.两().乣(){両[F::from(一)-1]=丯.丿(丁,一);}for q in 亄.iter(){let 乚=F::from(丯.两().乣());let Some(丐)=両[..乚].iter().position(|临|临.test(q))else{continue;};let
一=丐 as I+1;let 亐=10*C::from(一);if 丈+亐>j.丮{continue;}if j.亵>=侮{return false;}j.亵+=1;let mut 丂=丯.clone();let 三=K::乌{一,x:(q%O)as I,y:(q/O)as I,};if!丂.中(三){continue;}j.path.push(三);let
人=倰(仼,q);j.倈(&丂,人,丈+亐);let 俢=j.path.len()<j.亰&&丂.丗().is_none();let 俬=!俢||j.佟(&丂,Some(q),人,丈+亐);j.path.pop();if!俬{return false;}}true}fn 倈(&mut self,主:&J,人:仄,丈:C){let
j=self;let 仑=1-j.丁;let 伪=佪(主,仑);let 不=j.丨.不-伪.不;let 丄=j.丨.丄-伪.丄;if 不<=0&&丄<=0{return;}let 乓=j.丨.乓-伪.乓;let 且=主.且(j.丁)-j.且;let 倳=主.丗()==Some(之::乮(j.丁));let
业=if 倳{C::MAX/2}else{3*不+丄+10*乓+2*且-丈};if let Some(&丼)=j.index.get(&人){if 丈>=j.个[丼].丈{return;}}let 丟=仡{丅:j.path.clone(),丈,不,丄,乓,且,业,};match j.index.get(&人){Some(&丼)=>j.个[丼]=丟,None=>{j.index.insert(人,j.个.len());j.个.push(丟);}}}}fn
佝(l:&J,亰:F,丮:C,久:F)->L<仡>{乐(l,亰,丮,久).0}fn 乐(l:&J,亰:F,丮:C,久:F)->(L<仡>,F){if l.丗().is_some()||亰==0||久==0{return(L::new(),0);}let 丁=l.丞();let 仑=1-丁;let mut
伯=佇{丁,仁:l.丙(仑)|l.丙(仑).与(),丨:佪(l,仑),且:l.且(丁),亰,丮,亵:0,path:L::new(),个:L::new(),index:HashMap::default(),};伯.佟(l,None,[0;3],0);let 亵=伯.亵;let mut 个=伯.个;个.sort_by(|a,b|b.业.cmp(&a.业).then(a.丈.cmp(&b.丈)));个.truncate(久);(个,亵)}const
以:C=1000000;const 侯:C=200;fn 乑(l:&J,z:F)->C{let 七=l.七(z);let 丄:C=(l.丄(z)&七).iter().filter_map(|q|l.井(q)).map(|一|10*C::from(一)).sum();l.万(z)+3*l.且(z)+4*C::from(七.count())+丄+10*C::from((l.乂()&七).count())}fn
佯(l:&J,z:F,乛:G)->C{俪(l,z,佛(l,z,乛,Z[1-z]),)}fn 俪(l:&J,z:F,丈:C)->C{if 丈==亍{return C::MIN;}l.万(z)+l.且(z)-丈}fn 佼(仵:C)->C{仵.clamp(-侯,0)}fn 俨(l:&J,z:F,乛:[G;2],)->(C,[C;2]){let
丫=1-z;match l.丗(){Some(之::乮(倯))=>{return(if 倯==z{以}else{-以},[0,0])}Some(之::伻)=>return(0,[0,0]),None=>{}}let mut 业=乑(l,z)-乑(l,丫);if z==l.丞(){let mut 丂=l.clone();丂.仉();if
丂.丗().is_none()&&优(&丂,丫,true,Z[z])<=丂.万(丫){业-=以/2;}}let 亯=[佯(l,z,乛[0]),佯(l,丫,乛[1]),];(业+佼(亯[0])-佼(亯[1]),亯)}
