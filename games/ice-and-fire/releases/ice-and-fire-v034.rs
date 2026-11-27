// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伺=u16;type Q=u32;type 丮=u64;type 伹=i32;type C=i64;type 亾=f64;type O=bool;type 乘=String;type 了=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
偼:O=true;fn 僰(l:&J,q:F,伴:C,丩:G,丝:C)->O{let 丂=l.东();let 丷=1-丂;let 倛=偼&&丝<=l.下(丷)+2*l.丟(丷).max(0);let 伍=G::q(q)|G::q(q).丏();if!倛&&(伍&丩).is_empty(){return
false;}let(x,y)=乔(q);let mut 乞=l.clone();if!乞.乄(L::乿{x,y}){return false;}if 伳(&乞,丂)<伴{return true;}倛&&佋(&乞,丷,false,T[丂])>丝}const 傹:O=false;const 傘:I=2;const
俤:I=1;const 俎:I=2;const 佁:F=0;const 佹:F=2;const 傟:O=false;const 傡:O=false;const 傠:O=false;const 傞:O=false;const 傃:F=4;const 俋:F=5;#[derive(Clone)]enum
丫{仒(L),亏(K<L>),}impl 丫{fn 丈(&self)->&[L]{match self{丫::仒(丒)=>std::slice::from_ref(丒),丫::亏(丈)=>丈,}}}fn 乔(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 倓(l:&J,丅:&mut
Vec<丫>){丅.clear();if l.丢().is_some(){return;}let 丂=l.东();let 事=l.丽().亖();let mut 丨=[G::U;4];for 丆 in 1..=事{丨[F::from(丆)]=l.乊(丂,丆);}for 三 in l.乺().iter(){let
Some((上,丆))=l.亼(三)else{continue;};let 义=G::q(三).丏()&丨[F::from(丆)];for 两 in 义.iter(){let(x,y)=乔(两);丅.push(丫::仒(L::乶{上,x,y}));}}let 下=l.下(丂);let 七=l.七(丂);let
乖=l.仉(丂);let 乱=match 傘{0=>乖,1=>{let 伙=l.map().valid.丕(七).丏()&七;乖.丕(七)|伙}_=>乖.丕(七),};for 丆 in 1..=事{let 与=10*C::from(丆);if 下<与{break;}let mut 丑=乱&丨[F::from(丆)];for
僼 in 1..丆{丑=丑.丕(丨[F::from(僼)]);}for q in 丑.iter(){let(x,y)=乔(q);丅.push(丫::仒(L::丼{丆,x,y}));}}if 下>=于{let 丧=l.丐(丂)&乖&l.仗();if l.丽().但(){for q in(丧&l.map().为).iter(){let(x,y)=乔(q);丅.push(丫::仒(L::京{x,y}));}}if
l.丽().仰(){let 伙=l.map().valid.丕(l.丐(丂)).丏();let 丑=(丧&伙).丕(l.map().为);let 丩=丩(l,1-丂);let 伴=C::from((丩&l.七(丂)).count());let 丝=佋(l,1-丂,false,T[丂]);let 両=if
傹{let(产,佒)=乆(l,丂);产.丕(佒)}else{G::U};for q in 丑.iter().filter(|&q|{let 伍=G::q(q)|G::q(q).丏();!(伍&両).is_empty()||僰(l,q,伴,丩,丝)}){let(x,y)=乔(q);丅.push(丫::仒(L::乿{x,y}));}}}let
僯=match 俤{1=>于,2=>10,_=>C::MAX,};if 下>=僯{let 丩=丩(l,1-丂);let 亗=l.map().valid.丕(l.丐(丂)|l.丐(1-丂));for 三 in(l.乺()&丩).iter(){let Some((上,丆))=l.亼(三)else{continue;};let
义=G::q(三).丏()&丨[F::from(丆)]&亗;let(仛,仜)=乔(三);let 依=if 俤==1{if l.map().为.test(三){continue;}L::乿{x:仛,y:仜}}else{L::丼{丆:1,x:仛,y:仜,}};for 两 in 义.iter(){let(x,y)=乔(两);丅.push(丫::亏(vec![L::乶{上,x,y},依]));}}}if
俎!=0&&下>=于&&l.丽().仰(){僤(l,&丨,丅);}if 佁!=0&&下>=20{償(l,丨[1],下,丅);}if 傞{僘(l,丨[1],下,丅);}for 串 in 倇(l,俋,下,傃){if 串.丈.len()>1{丅.push(丫::亏(串.丈));}}if let Some(丝)=侌(l,T[1-丂],下){if
丝.丈.len()>1{丅.push(丫::亏(丝.丈));}}}fn 僤(l:&J,丨:&[G;4],丅:&mut Vec<丫>){let 丂=l.东();let 丷=1-丂;let 丩=丩(l,丷);let 严=l.丐(丂);let 丗=l.丐(丷);let 伙=丗.丏();let 为=l.map().为;for
三 in(l.乺()&丩&l.七(丂)).iter(){if 为.test(三){continue;}let Some((上,丆))=l.亼(三)else{continue;};let 僁=l.map().丘(三,T[丷]);let 义=G::q(三).丏()&丨[F::from(丆)];let(仛,仜)=乔(三);for
两 in 义.iter(){let 侄=match 俎{1=>严.test(两)&&l.map().丘(两,T[丷])>僁,2=>严.test(两)&&伙.test(两),_=>严.test(两)||(丗.test(两)&&两!=T[丷]),};if!侄{continue;}let(x,y)=乔(两);丅.push(丫::亏(vec![L::乶{上,x,y},L::乿{x:仛,y:仜},]));}}}fn
償(l:&J,侥:G,下:C,丅:&mut Vec<丫>){let 丂=l.东();let 並=l.map();let 亗=並.valid.丕(l.丐(丂)|l.丐(1-丂));let 両=亗&侥;let 乱:K<F> =(l.七(丂).丏()&両).iter().collect();if 乱.is_empty(){return;}let
侭=|不:F|{乱.iter().copied().min_by_key(|&q|(並.丘(q,不),q))};let mut 侍:K<(F,F)> =K::new();if 傟{for 世 in 儠(l,&乱){if let Some(举)=侭(世){侍.push((举,世));}}}else{let
不=T[1-丂];let mut 侺=乱.clone();侺.sort_by_key(|&q|(並.丘(q,不),q));侺.truncate(佹);侍.extend(侺.into_iter().map(|举|(举,不)));}let 亩=F::try_from(下/10).unwrap_or(0).min(佁);for(举,不)in
侍{let mut 丝=vec![举];let mut 享=G::q(举);while 丝.len()<亩{let 乹=丝[丝.len()-1];if 乹==不{break;}let 丄=(G::q(乹).丏()&両).丕(享).iter().min_by_key(|&q|(並.丘(q,不),q));let
Some(丄)=丄 else{break;};丝.push(丄);享.set(丄);let 丈=丝.iter().map(|&q|{let(x,y)=乔(q);L::丼{丆:1,x,y}}).collect();丅.push(丫::亏(丈));}if 傠{價(l,&丝,丅);}}}fn 價(l:&J,丝:&[F],丅:&mut
Vec<丫>){let 丂=l.东();let 举=丝[0];let 乻=l.乺()&l.万(丂)&G::q(举).丏();for 三 in 乻.iter(){let Some((上,丆))=l.亼(三)else{continue;};if!l.乊(丂,丆).test(举){continue;}let(x,y)=乔(举);let
mut 丈=vec![L::乶{上,x,y}];for&q in&丝[1..]{let(x,y)=乔(q);丈.push(L::丼{丆:1,x,y});丅.push(丫::亏(丈.clone()));}}}fn 儠(l:&J,乱:&[F])->K<F>{let 丂=l.东();let 並=l.map();let(久,中)=佑(l);let
严:K<F> =l.万(丂).iter().collect();let 丗:K<F> =l.万(1-丂).iter().collect();let 佐=|世:F|{乱.iter().map(|&q|並.丘(q,世)).min().unwrap_or(I::MAX)};let 僐=|世:F|{let 丩=佐(世);严.iter().any(|&亻|{let
d=並.丘(亻,世);d<=丩&&丗.iter().all(|&丷|d<並.丘(丷,世))})};let 侨=|世:F|{傡&&(l.丐(1-丂).test(世)||丗.iter().any(|&丷|並.丘(丷,世)<佐(世)))};let 両=|丑:G|->K<F>{let mut 亃:K<F> =丑.丕(l.丐(丂)).iter().filter(|&世|佐(世)!=I::MAX&&!僐(世)&&!侨(世)).collect();亃.sort_by_key(|&世|(佐(世),世));亃};let
mut 亃=両(中);if 亃.is_empty(){亃=両(久);}亃.truncate(佹);亃}fn 僒(l:&J)->K<F>{let 丂=l.东();let 並=l.map();let(久,中)=佑(l);let 七=l.七(丂);let 严=l.丐(丂);let 丩=丩(l,1-丂);let
丗:K<F> =l.万(1-丂).iter().collect();let 們=|q:F|{七.iter().map(|a|並.丘(a,q)).min().unwrap_or(I::MAX)};let mut 丑:K<F> =(久|中).iter().filter(|&q|{if 严.test(q){七.test(q)&&丩.test(q)}else{!l.丐(1-丂).test(q)&&丗.iter().any(|&e|並.丘(e,q)<們(q))}}).collect();丑.sort_by_key(|&q|(I::from(!中.test(q)),們(q),q));丑.truncate(
2*佹);丑}fn 僘(l:&J,侥:G,下:C,丅:&mut Vec<丫>){let 丂=l.东();let 並=l.map();let 亗=並.valid.丕(l.丐(丂)|l.丐(1-丂));let 両=亗&侥;let 七=l.七(丂);let 丧=七&l.仗();let 僻=if 佁==0{3}else{佁};for
q in 僒(l){if!l.丐(丂).test(q){let 举=(七.丏()&両).iter().min_by_key(|&c|(並.丘(c,q),c));if let Some(举)=举{let mut 丝=vec![举];let mut 享=G::q(举);while 丝[丝.len()-1]!=q&&丝.len()<僻{let
乹=丝[丝.len()-1];let 丄=(G::q(乹).丏()&両).丕(享).iter().min_by_key(|&c|(並.丘(c,q),c));let Some(丄)=丄 else{break;};丝.push(丄);享.set(丄);}let 与=10*C::try_from(丝.len()).unwrap_or(C::MAX);if
丝.len()>=2&&丝[丝.len()-1]==q&&与<=下{let 丈=丝.iter().map(|&c|{let(x,y)=乔(c);L::丼{丆:1,x,y}}).collect();丅.push(丫::亏(丈));}}}if 下>=于&&l.丽().仰(){let 为=(G::q(q)|G::q(q).丏())&丧;for
乫 in 为.丕(並.为).iter(){let(x,y)=乔(乫);丅.push(丫::仒(L::乿{x,y}));}}}}const 傐:I=2;const 傭:C=5;fn 乻(l:&J,丁:F)->G{侬(l,丁,傐)}fn 侬(l:&J,丁:F,三:I)->G{let mut 乻=G::U;if
三>3{return 乻;}for q in(l.万(丁)&l.七(丁)).iter(){if l.亟(q).is_some_and(|丆|丆>=三){乻|=G::q(q);}}乻}fn 丩(l:&J,丶:F)->G{let 乍=l.下(丶)+l.丟(丶);儘(l,丶,乻(l,丶),乍)}fn 亥(l:&J,丁:F)->G{丩(l,1-丁)&l.七(丁)}fn
伳(l:&J,丁:F)->C{C::from(亥(l,丁).count())}const 俛:C=4;fn 儀(丑:C)->C{丑/俛}fn 丹(l:&J,丁:F,亥:G)->C{let 七=l.七(丁);let 丹=(l.丹()&七).丕(亥);let 丑=C::from(七.count());let
侈=if l.书()<=偩{丑/偨+偧}else{儀(丑)};侈*C::from(丹.count())}const 偩:Q=15;const 偧:C=6;const 偨:C=俛;const 傦:C=2;const 佂:I=1;const 俣:I=3;const 傧:I=6;fn 僸(仆:I,乖:I,仙:I,d:I)->I{match
仆{0=>4,1 if d<=乖=>4,1=>0,_ if d<=乖=>4,_ if d>=仙=>0,_=>(4*Q::from(仙-d)/Q::from(仙-乖))as I,}}fn 偙(l:&J,丁:F)->C{let 七=l.七(丁);let 丷=if 佂==0{G::U}else{l.七(1-丁)};let[a,b,c]=七.伷();let[e,f,g]=丷.伷();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丁 as 丮).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丠=(hash as F)&(俭-1);傺.with(|侊|{let mut 侊=侊.borrow_mut();let 仌=&mut 侊[丠];if 仌.0==七&&仌.1==丷&&仌.2==丁 as I+1{return 仌.3;}let 丬=if 佂==0{克(七,T[丁])}else if 佂==1{let
丰=l.map().valid;let mut 乖=丷&丰;for _ in 0..俣{乖=(乖|乖.丏())&丰;}兊(七,T[丁],乖)}else{let d=丳(l,丷);let mut 乐=[0u8;M];for(w,&义)in 乐.iter_mut().zip(d.iter()){*w=僸(佂,俣,傧,义);}做(七,T[丁],&乐)};*仌=(七,丷,丁
as I+1,丬);丬})}const 俭:F=1<<12;thread_local!{static 傺:std::cell::RefCell<K<(G,G,I,C)>> =std::cell::RefCell::new(vec![(G::U,G::U,0,0);俭]);}fn 儓(七:G)->G{let
n=七.佣();let s=七.佤();let e=七.倿();let w=七.偀();let 儃=e.佣();let 億=w.佣();let 儡=e.佤();let 儴=w.佤();let 僷=[n&儃&e,e&儡&s,s&儴&w,w&億&n];let 亽=|a:G,b:G|(a|b).丕(a&b);let
俯=|[a,b,c,d]:[G;4]|{let(债,俻)=(亽(a,b),a&b);let(倻,俽)=(亽(c,d),c&d);let t=债&倻;let 俼=亽(俻,俽);(亽(债,倻),亽(俼,t),(俻&俽)|(t&俼))};let(僲,倗,僳)=俯([n,e,s,w]);let(僝,倊,僞)=俯(僷);let
俶=僝.丕(僲);let 停=亽(倗,倊);let 僓=亽(停,俶);let 僂=倊.丕(倗)|俶.丕(停);let 僔=亽(亽(僳,僞),僂);七&(僓|僔)}fn 兊(七:G,乮:F,乖:G)->C{if!七.test(乮){return 0;}let 亅=七.count();let mut 亊=0i64;let
mut 亩=0i64;for v in(儓(七)&乖).iter(){if v==乮{continue;}let 乪=七.丕(G::q(v));let mut 乩=G::q(乮);loop{let 丄=(乩|乩.丏())&乪;if 丄==乩{break;}乩=丄;}let 串=C::from(亅-1-乩.count());if
串>0{亊+=4;亩=亩.max(4*串);}}(亊+亩)/4}fn 克(七:G,乮:F)->C{做(七,乮,&[4;M])}fn 做(七:G,乮:F,乐:&[I;M])->C{struct 伎<'a>{乐:&'a[I;M],七:G,乳:[I;M],乛:[I;M],size:[I;M],从:I,亊:C,亩:C,}fn
仨(d:&mut 伎<'_>,v:F)->I{d.从+=1;let 乳=d.从;d.乳[v]=乳;let mut 乛=乳;let mut 侶=1u8;let mut 串=0u8;for w in(G::q(v).丏()&d.七).iter(){if d.乳[w]==0{let 偋=仨(d,w);侶+=偋;乛=乛.min(d.乛[w]);if
d.乛[w]>=乳{串+=偋;}}else{乛=乛.min(d.乳[w]);}}d.乛[v]=乛;d.size[v]=侶;if 串>0&&d.乳[v]>1{let w=C::from(d.乐[v]);d.亊+=w;d.亩=d.亩.max(w*C::from(串));}侶}if!七.test(乮){return
0;}let mut d=伎{乐,七,乳:[0;M],乛:[0;M],size:[0;M],从:0,亊:0,亩:0,};仨(&mut d,乮);(d.亊+d.亩)/4}fn 儣(l:&J,丁:F)->C{傦*(偙(l,1-丁)-偙(l,丁))}fn 倈(l:&J,丁:F)->C{if l.下(丁)+l.丟(丁)>=0{return
0;}(l.万(丁)&l.七(丁)).iter().filter_map(|q|l.亟(q)).map(|丆|10*C::from(丆)).sum()}const 傂:C=15;const 債:[C;2]=[3,1];const 傳:[C;2]=[4,20];fn 儲(l:&J,丁:F)->[C;2]{let
mut 丙=[0;2];for q in(l.万(丁)&l.七(丁)).iter(){match l.亟(q){Some(2)=>丙[0]+=1,Some(3)=>丙[1]+=1,_=>{}}}丙}fn 偊(l:&J,丁:F)->C{let[偑,儽]=儲(l,丁);let 俷=|丆:F|(3-債[丆])*傳[丆];-傂*偑+俷(0)*偑+俷(1)*儽}const
偻:C=0;const 偾:C=2;const 偽:C=100;fn 偃(亨:C)->C{if 亨<0{return 0;}偻+偾*亨.min(偽)/2}const 俚:I=2;const 俗:C=2;const 俖:C=4;const 偿:I=3;fn 亵(l:&J,丁:F)->C{if 俗==0{return
0;}let 丰=l.map().valid;let mut 万=G::U;for q in(l.万(丁)&l.七(丁)).iter(){if l.亟(q).is_some_and(|丆|丆<=偿){万|=G::q(q);}}let mut 佩=丰.丕(l.丐(丁));let mut 仃=G::U;let
mut 义=0;for 丘 in 1..=俖{佩=(佩|佩.丏())&丰;let 仄=(万&佩).丕(仃);义+=(丘-1)*C::from(仄.count());仃|=仄;}义+=俖*C::from(万.丕(仃).count());俗*义}const 俬:C=8;fn 乆(l:&J,丁:F)->(G,G){let
七=l.七(丁);let 产=七&l.丐(1-丁).丏();let 乓=l.乓()&七;(产,产&(乓|乓.丏()))}fn 偖(l:&J,丁:F)->C{if 俬==0{return 0;}俬*C::from(乆(l,丁).1 .count())}const 佳:C=0;const 俓:I=1;const
俔:C=4;const 佴:C=0;const 俒:C=0;const 偲:Q=14;const 偱:C=2;const 俫:C=0;const 傸:Q=14;fn 仩(l:&J,丁:F)->C{if 俫==0||l.书()>傸{return 0;}let 严=丳(l,l.七(丁));let 丗=丳(l,l.七(1-丁));let
mut 仢=0;for c in l.map().valid.iter(){match 严[c].cmp(&丗[c]){std::cmp::Ordering::Less=>仢+=1,std::cmp::Ordering::Greater=>仢-=1,std::cmp::Ordering::Equal=>{}}}俫*仢}struct
俕{valid:G,久:G,中:G,}thread_local!{static 偳:std::cell::RefCell<P<俕>> =const{std::cell::RefCell::new(None)};}fn 佑(l:&J)->(G,G){偳.with(|q|{let mut 乀=q.borrow_mut();let
丰=l.map().valid;if 乀.as_ref().is_none_or(|z|z.valid!=丰){let 仦=仦(l);let mut 久=G::U;for c in 丰.iter(){if 仦[c]<=偱{久.set(c);}}*乀=Some(俕{valid:丰,久,中:仠(l),});}乀.as_ref().map_or((G::U,G::U),|z|(z.久,z.中))})}fn
佒(l:&J,丁:F)->G{let 七=l.七(丁);let 乓=l.乓()&七;(乓|乓.丏())&七}fn 世(l:&J,丁:F)->C{if(佳==0&&俔==0&&佴==0&&俒==0)||l.书()>偲{return 0;}let(久,中)=佑(l);let mut 亅=0;if 佳!=0{let
严=丳(l,l.七(丁));let 丗=丳(l,l.七(1-丁));let mut 仢=0;for c in 久.iter(){let(a,b)=(严[c],丗[c]);if a!=I::MAX&&a.saturating_add(俓)<=b{仢+=1;}else if b!=I::MAX&&b.saturating_add(俓)<=a{仢-=1;}}亅+=佳*仢;}let
丙=|乑:G|C::from(乑.count());let(丸,丗)=(l.七(丁),l.七(1-丁));亅+=俔*(丙(丸&久)-丙(丗&久));if 佴!=0{亅+=佴*(丙(佒(l,丁)&久)-丙(佒(l,1-丁)&久));}亅+=俒*(丙(丸&中)-丙(丗&中));亅}const 俑:C=4;const
俐:C=0;const 偮:C=4;const 偯:Q=12;fn 僨()->[G;2]{static 偭:std::sync::OnceLock<[G;2]> =std::sync::OnceLock::new();*偭.get_or_init(僩)}fn 僩()->[G;2]{let mut 丅=[G::U;2];for
c in 0..M{let(x,y)=((c%Y)as C,(c/Y)as C);if x-y>=俑{丅[0].set(c);}else if y-x>=俑{丅[1].set(c);}}丅}fn 偘(l:&J,丁:F)->C{let[a,b]=僨();let 七=l.七(丁);C::from((七&a).count().min((七&b).count())).min(偮)}fn
僧(l:&J,丁:F)->C{if 俐==0||l.书()>偯{return 0;}俐*(偘(l,丁)-偘(l,1-丁))}fn 僠(l:&J,丁:F)->C{let 乻=[侬(l,丁,俚),侬(l,1-丁,俚),];let(丱,仿)=僡(l,丁,乻);let 丱=丱+偃(仿[0])-偃(仿[1]);if
l.丢().is_some(){return 丱;}let 严=亥(l,丁);let 丗=亥(l,1-丁);let 伴=C::from(丗.count())-C::from(严.count());丱-2*倈(l,丁)+2*倈(l,1-丁)+儣(l,丁)+偊(l,丁)-偊(l,1-丁)-亵(l,丁)+亵(l,1-丁)+傭*伴/4+丹(l,丁,严)-丹(l,1-丁,丗)+偖(l,丁)-偖(l,1-丁)+世(l,丁)+仩(l,丁)+僧(l,丁)}const
傌:F=8;const 傛:F=3;const 傄:F=32;const 傜:丮=300;const 偦:亾=0.75;const 偸:P<Q> =Some(7);const 偶:O=false;const 偵:C=10;const 偷:Q=6;struct 亮{人:P<了>,仝:P<了>,伥:丮,}#[derive(Default)]struct
佾{丈:K<L>,侒:F,亴:丮,丞:F,之:F,倁:F,亦:F,亄:O,假:O,今:F,}const 傷:O=true;#[derive(Clone)]struct 予{丈:K<L>,侕:C,}struct 仫{丈:K<L>,end:J,first:P<丮>,丛:P<K<予>>,}type 伏=HashMap<丮,C,BuildHasherDefault<DefaultHasher>>;struct
佼{丂:F,limit:亮,亴:丮,伢:伏,仧:P<K<丫>>,侼:O,乚:P<仪>,}const 佺:C=0;const 俈:O=true;const 偡:O=false;const 俇:P<I> =None;fn 僇(l:&J,丞:&[予],丅:&mut Vec<丫>){if!俈&&俇.is_none(){return;}let
伯=丞.iter().map(|乼|乼.丈.as_slice());僈(l,伯,(俈,偡,俇),丅,);}fn 僈<'a>(l:&J,伯:impl Iterator<Item=&'a[L]>,(乓,丏,丆):(O,O,P<I>),丅:&mut Vec<丫>,){let 丂=l.东();let 丸=l.七(丂);let
mut 传=G::U;for 乼 in 伯{for 丒 in 乼{if let L::丼{x,y,..}|L::乶{x,y,..}=*丒{let q=F::from(y)*Y+F::from(x);if 丸.test(q){传|=G::q(q);}}}}if 传.is_empty(){return;}let
丧=l.仗()&丸;let 下=l.下(丂);let 倴=|丒:L,丅:&mut Vec<丫>|{if!丅.iter().any(|a|a.丈()==[丒]){丅.push(丫::仒(丒));}};if 乓&&下>=于&&l.丽().仰(){let mut 丑=传;if 丏{丑|=传.丏();}for
q in(丑&丧).丕(l.map().为).iter(){let(x,y)=乔(q);倴(L::乿{x,y},丅);}}if let Some(丆)=丆.filter(|&丆|丆<=l.丽().亖()){if 下>=10*C::from(丆){for q in(传&丧).iter(){let(x,y)=乔(q);倴(L::丼{丆,x,y},丅);}}}}fn
來(l:&mut J,丈:&[L])->O{丈.iter().all(|&丒|l.乄(丒))}fn 井(丈:&[L])->丮{let mut hash:丮=0xcbf29ce484222325;for 丒 in 丈{let(佖,a,b,c)=match*丒{L::乶{上,x,y}=>(1,丮::from(上),x,y),L::丼{丆,x,y}=>(2,丮::from(丆),x,y),L::京{x,y}=>(3,0,x,y),L::乿{x,y}=>(4,0,x,y),};let
伌=佖|a<<8|丮::from(b)<<40|丮::from(c)<<48;hash=(hash^伌).wrapping_mul(0x0100000001b3);}hash}fn 俱(乴:&J,乼:&[L])->J{let mut 丄=乴.clone();丄.件();for&丒 in 乼{丄.乄(丒);}丄.件();丄}fn
佘(l:&J)->P<K<L>>{let 丂=l.东();let 不=T[1-丂];let 伋=|丵:&J|丵.丢()==Some(乲::互(丂));for q in l.乺().iter(){if l.map().丘(q,不)!=1{continue;}let Some((上,_))=l.亼(q)else{continue;};let(x,y)=乔(不);let
丒=L::乶{上,x,y};let mut 丵=l.clone();if 丵.乄(丒)&&伋(&丵){return Some(vec![丒]);}}let 丝=侌(l,不,l.下(丂))?;let mut 丵=l.clone();(來(&mut 丵,&丝.丈)&&伋(&丵)).then_some(丝.丈)}fn
偉(l:&J,丁:F,个:C)->O{let 与=侓(l,丁,l.万(丁),T[1-丁]);与!=亱&&l.下(丁)+个*l.丟(丁)>=与}fn 儱(l:&J)->O{let 丂=l.东();if 佺==0||l.丢().is_some(){return false;}let mut 丄=l.clone();丄.件();let
伳=丄.丢().is_none()&&佋(&丄,1-丂,true,T[丂])<=丄.下(1-丂);!伳&&偉(l,丂,佺)}impl 佼{fn 俹(&self,乴:&J)->O{self.侼&&乴.丢()!=Some(乲::互(self.丂))&&!偉(乴,self.丂,佺)}fn 亜(&self)->O{self.亴>=self.limit.伥||self.limit.人.is_some_and(|t|了::now()>=t)}fn
丱(&mut self,乴:&J,丞:&[予],侉:C)->C{self.亴+=1;self.侲(乴,丞,侉)}fn 侲(&self,乴:&J,丞:&[予],侉:C)->C{if 乴.丢()==Some(乲::互(self.丂)){return 佄;}if 丞.is_empty(){return self.value(乴);}let
mut 佪=C::MAX;for 乼 in 丞{佪=佪.min(self.value(&俱(乴,&乼.丈)));if 佪<=侉{break;}}佪}fn value(&self,l:&J)->C{let 优=僠(l,self.丂);match&self.乚{Some(乚)if l.丢().is_none()=>优+乚.儸(l,self.丂),_=>优,}}fn
俰(&self,九:&mut Vec<丫>){if let Some(乚)=&self.乚{for 佝 in&乚.今{九.push(丫::亏(佝.clone()));}}}#[allow(clippy::too_many_arguments)]fn 儎(&mut self,乁:&J,仵:C,九:&[丫],习:&[(C,F)],乥:&mut[O],丞:&[予],价:&mut
伏,侣:O,)->(P<(F,J,C)>,O){let mut 专=None;let mut 佈=仵;for&(佚,丠)in 习{if 乥[丠]{continue;}if 侣&&佚!=C::MAX&&佚<=佈-仵{return(专,true);}if self.亜(){return(专,false);}乥[丠]=true;let
丈=九[丠].丈();let mut 丄=乁.clone();if!來(&mut 丄,丈)||self.俹(&丄){价.insert(井(丈),C::MIN/4);continue;}let 丱=self.丱(&丄,丞,佈);价.insert(井(丈),丱-仵);if 丱>佈{佈=丱;专=Some((丠,丄,丱));}}(专,false)}fn
侑(&mut self,乮:&J,丞:&[予],众:&[丮],僋:O)->仫{let 一=self;let mut 乸=None;let mut 乁=乮.clone();let mut 丈=K::new();let mut 仵=一.丱(&乁,丞,C::MIN);let mut 价=伏::default();let
mut 九=K::new();let mut 习=K::new();while 乁.丢().is_none()&&!一.亜(){if 丈.is_empty()&&僋{match&一.仧{Some(仧)=>九.clone_from(仧),None=>{倓(&乁,&mut 九);if let Some(乚)=一.乚.as_ref().filter(|g|g.倬){九.push(丫::亏(乚.业.clone()));}一.俰(&mut
九);一.仧=Some(九.clone());}}}else{倓(&乁,&mut 九);一.俰(&mut 九);}僇(&乁,丞,&mut 九);习.clear();for(丠,亓)in 九.iter().enumerate(){let k=井(亓.丈());let 佚=价.get(&k).or(一.伢.get(&k)).copied();习.push((佚.unwrap_or(C::MAX),丠));}习.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乥=vec![false;九.len()];if 丈.is_empty(){for(丠,亓)in 九.iter().enumerate(){乥[丠]=众.contains(&井(亓.丈()));}}let mut 专=None;for
侣 in[true,false]{let(乎,儧)=一.儎(&乁,仵,&九,&习,&mut 乥,丞,&mut 价,侣,);专=乎;if 专.is_some()||!儧{break;}}let Some((丠,丄,丱))=专 else{break;};if 丈.is_empty(){乸=Some(井(九[丠].丈()));}丈.extend_from_slice(九[丠].丈());乁=丄;仵=丱;}for(k,乏)in
价{let 倘=一.伢.entry(k).or_insert(乏);*倘=(*倘).max(乏);}仫{丈,end:乁,first:乸,丛:None,}}fn 伯(&self,乴:&J)->K<K<L>>{let mut 丄=乴.clone();丄.件();let mut 丅=K::new();if
丄.丢().is_some(){return 丅;}let 亘=丄.东();let 下=丄.下(亘);if let Some(丝)=侌(&丄,T[self.丂],下){丅.push(丝.丈);}for 串 in 倇(&丄,俋,下,傛){丅.push(串.丈);}let mut 倕=K::new();僮(&丄,&mut
倕);丅.push(倕);let mut 丗=佼{丂:亘,limit:亮{伥:傜,..self.limit},亴:0,伢:伏::default(),仧:None,侼:false,乚:None,};丅.push(丗.侑(&丄,&[],&[],true).丈);丅.retain(|丈|!丈.is_empty());丅}fn
俲(&self,乴:&J)->K<予>{self.伯(乴).into_iter().map(|丈|{let 侕=self.value(&俱(乴,&丈));予{丈,侕}}).collect()}fn 亦(&mut self,乮:&J,亓:&丫,丞:&[予])->P<仫>{let mut 乞=乮.clone();if!來(&mut
乞,亓.丈())||self.俹(&乞){return None;}self.亴+=1;let 乪=self.侑(&乞,丞,&[],false);let mut 丈=亓.丈().to_vec();丈.extend_from_slice(&乪.丈);Some(仫{丈,end:乪.end,first:Some(井(亓.丈())),丛:None,})}fn
倎(&self,业:&仫,丞:&[予])->C{let 丱=self.侲(&业.end,丞,C::MIN);match&业.丛{Some(丛)if!丛.is_empty()=>丱.min(self.侲(&业.end,丛,C::MIN)),_=>丱,}}}fn 儚(乎:&[予],丞:&mut Vec<予>)->O{let
乌:K<_> =丞.iter().map(|r|r.丈.clone()).collect();for 乼 in 乎{if 丞.iter().any(|r|r.丈==乼.丈){continue;}丞.push(乼.clone());}丞.sort_by_key(|乼|乼.侕);丞.truncate(傌);丞.iter().map(|r|&r.丈).ne(&乌)}fn
儼(l:&J,乒:亮,亄:&mut 亣)->佾{偍(l,乒,亄,None)}fn 偍(l:&J,乒:亮,亄:&mut 亣,乚:P<仪>,)->佾{let 丂=l.东();let mut 丿=佾::default();if l.丢().is_some(){return 丿;}if let Some(丈)=佘(l){丿.丈=丈;return
丿;}let 乙=|儥:亾|match(乒.人,乒.仝){(Some(人),Some(仝))=>Some(人+(仝-人).mul_f64(儥)),_=>None,};if 偸.is_some_and(|且|l.书()>=且){亄.end();}if 亄.儒(l){let 丈=亄.且(l,乙(0.4));let
僊=丈.iter().any(|丒|{matches!(丒,L::丼{..}|L::京{..}|L::乿{..})});let 亵=偶&&!僊&&l.书()>=偷&&l.下(丂)>=偵;if!亵{丿.亄=true;丿.丈=丈;return 丿;}亄.end();}let mut 乡=佼{丂,limit:亮{人:乙(偦),..乒},亴:0,伢:伏::default(),仧:None,侼:儱(l),乚,};let
mut 丞=K::new();let mut 之:K<仫> =K::new();let mut 众=K::new();let mut 低=false;let mut 亦:K<丫> =K::new();let mut 举=乡.乚.as_ref().filter(|g|g.start).map(|g|丫::亏(g.业.clone()));while
丿.侒<傄&&(之.is_empty()||!乡.亜()){let 儬=match 举.take(){Some(亓)=>乡.亦(l,&亓,&丞),None=>None,};let 业=if let Some(业)=儬{业}else if 低{let Some(亓)=亦.pop()else{break;};match
乡.亦(l,&亓,&丞){Some(业)=>业,None=>continue,}}else{乡.侑(l,&丞,&众,true)};丿.侒+=1;let 倽=之.iter().any(|p|p.丈==业.丈);let 乸=业.first;let 偆=乡.亜();let mut 业=业;let mut 倥=false;if!偆&&!倽{let
乎=乡.俲(&业.end);倥=儚(&乎,&mut 丞);业.丛=Some(乎);}if!倽{之.push(业);}if 偆{break;}if!倥&&!低{match 乸{Some(乸)=>众.push(乸),None=>{低=true;亦=乡.仧.clone().unwrap_or_default();亦.retain(|a|!众.contains(&井(a.丈())));let
乏=|a:&丫|{乡.伢.get(&井(a.丈())).copied().unwrap_or(C::MIN)};亦.sort_by_key(乏);}}}}丿.亦=F::from(低);丿.亴=乡.亴;丿.丞=丞.len();丿.之=之.len();let 仅=||乒.仝.is_some_and(|t|了::now()>=t);let
mut 伇:K<P<C>> =vec![None;之.len()];for 丠 in(0..之.len()).rev(){if 丠+1<之.len()&&仅(){break;}伇[丠]=Some(乡.倎(&之[丠],&丞));}let 伅=|伇:&[P<C>]|{let mut 专:P<(C,F)>
=None;for(丠,丱)in 伇.iter().enumerate().rev(){if let Some(丱)=*丱{if 专.is_none_or(|(亝,_)|丱>亝){专=Some((丱,丠));}}}专};let mut 专=伅(&伇);乡.limit.人=乒.仝;while let Some((_,丠))=专{if!傷||之[丠].丛.is_some()||仅(){break;}let
乎=乡.俲(&之[丠].end);之[丠].丛=Some(乎);丿.倁+=1;伇[丠]=Some(乡.倎(&之[丠],&丞));专=伅(&伇);}if let Some((_,丠))=专{丿.假=乡.乚.as_ref().is_some_and(|g|!g.业.is_empty()&&之[丠].丈.starts_with(&g.业));丿.今=乡.乚.as_ref().map_or(0,|g|{g.今.iter().filter(|p|p.iter().all(|c|之[丠].丈.contains(c))).count()});let
mut 丌=l.clone();for&丒 in&之[丠].丈{if 丌.乄(丒){丿.丈.push(丒);}}}丿}fn 儑(乒:&dyn Fn(了,Q)->亮,mut 亄:亣,侢:impl BufRead,作:impl Write,)->Result<(),乘>{let mut 亪=伒::default();个(侢,作,|仺,二,且,举|match
J::read(仺,二,亡::佭,且){Ok(l)=>{let 份=乚(&l,&mut 亪);let 丿=match 份{Some(乚)=>{偍(&l,乒(举,且),&mut 亄,Some(乚))}None=>儼(&l,乒(举,且),&mut 亄),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",且,丿.侒,丿.亴,丿.丞,丿.倁,丿.亦,I::from(丿.假),丿.今,举.elapsed());傿(&丿.丈)}Err(付)=>{eprintln!("plan: cannot read the input: {}"
,付);"WAIT".to_string()}},)}const 傓:P<C> =None;const 傶:O=false;#[derive(Clone,Copy,PartialEq)]enum 休{佯,偤,}#[derive(Clone,Copy)]struct 亰{q:F,kind:休,仞:O,value:C,}fn
乨(l:&J,丁:F)->K<亰>{儅(l,丁,傶)}fn 儅(l:&J,丁:F,儵:O)->K<亰>{let(久,中)=佑(l);let(丸,丗)=(l.七(丁),l.七(1-丁));let(伞,伟)=(丳(l,丸),丳(l,丗));let 优=伨(l,&伞,&伞,&伟);let mut 丅=K::new();for
q in(久|中).iter(){let 侖=丳(l,G::q(q));let 仞=丸.test(q);let 丬=if 儵{伨(l,&伞,&侖,&伟)-伨(l,&伟,&侖,&伞)}else if 仞{let 偛=丳(l,僪(丸,丁,q));优-伨(l,&偛,&偛,&伟)}else{伨(l,&伞,&侖,&伟)-优};丅.push(亰{q,kind:if
中.test(q){休::佯}else{休::偤},仞,value:丬,});}丅.sort_by_key(|o|(-o.value,o.kind!=休::佯,o.q));丅}fn 伨(l:&J,a:&[I;M],g:&[I;M],b:&[I;M])->C{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 仃(乨:&[亰])->K<亰>{乨.iter().filter(|o|o.value>=傓.unwrap_or(1)).copied().collect()}fn 僪(七:G,丁:F,q:F)->G{let
乪=七.丕(G::q(q));let mut 乬=G::q(T[丁])&乪;loop{let 丄=乬|(乬.丏()&乪);if 丄==乬{return 乬;}乬=丄;}}const 俦:C=10;const 傑:C=100;#[derive(Clone,Copy)]enum 伿{伕(Q),丼(F),}#[derive(Clone,Copy)]struct
乾{个:Q,侠:伿,}#[derive(Clone,Copy)]struct 伓{q:F,ours:P<乾>,佛:P<乾>,theirs:P<乾>,亨:C,}fn 儗(l:&J,丁:F,乨:&[亰])->K<伓>{let mut 丅=K::new();for 临 in 乨.iter().filter(|o|!o.仞){let
q=临.q;let(儋,佛)=俳(l,丁,q);let 严=专(儋,佛);let(儻,儺)=俳(l,1-丁,q);let 丗=专(儻,儺);let 个=|a:P<乾>|a.map_or(傑,|a|C::from(a.个));丅.push(伓{q,ours:严,佛,theirs:丗,亨:个(丗)-个(严),});}丅}fn
专(亻:P<乾>,伶:P<乾>)->P<乾>{match(亻,伶){(Some(u),Some(t))=>Some(if t.个<u.个{t}else{u}),(u,t)=>u.or(t),}}fn 俳(l:&J,丶:F,q:F)->(P<乾>,P<乾>){let 並=l.map();let mut
亻:P<乾> =None;for 三 in l.万(丶).iter(){let d=並.丘(三,q);let Some(上)=l.亍(三)else{continue;};if d!=I::MAX&&亻.is_none_or(|u|Q::from(d)<u.个){亻=Some(乾{个:Q::from(d),侠:伿::伕(上),});}}let
下=l.下(丶)+if 丶==l.东(){0}else{l.丟(丶)};let 佨=match(下,l.丟(丶)){(下,_)if 下>=俦=>Some(0),(_,丟)if 丟<=0=>None,(下,丟)=>Some((俦-下+丟-1)/丟),};let mut 伶:P<乾> =None;if let
Some(佨)=佨{let 乱=l.七(丶).丏()&並.valid&l.乊(丶,1);for 三 in 乱.iter(){let d=並.丘(三,q);if d!=I::MAX&&伶.is_none_or(|t|1+佨 as Q+Q::from(d)<t.个){伶=Some(乾{个:1+佨 as Q+Q::from(d),侠:伿::丼(三),});}}}(亻,伶)}const
俧:C=10;#[derive(Default)]struct 伒{倪:BTreeMap<Q,F>,}#[derive(Default)]struct 仁{仚:K<(Q,F)>,亞:K<(F,F)>,back:K<Q>,}fn 仱(l:&J,丁:F,乨:&[亰],令:&[伓],丞:&mut 伒,)->仁{俴(l,丁,乨,令,丞,l.万(丁))}fn
俴(l:&J,丁:F,乨:&[亰],令:&[伓],丞:&mut 伒,侗:G,)->仁{let 並=l.map();let 万:BTreeMap<Q,F> =(l.万(丁)&侗).iter().filter_map(|c|l.亍(c).map(|上|(上,c))).collect();let mut 両:K<(亰,伓)>
=K::new();for o in 仃(乨).into_iter().filter(|o|!o.仞){if let Some(r)=令.iter().find(|r|r.q==o.q){if r.ours.is_some()&&r.亨>=0{両.push((o,*r));}}}両.sort_by_key(|(o,r)|{(-o.value,o.kind!=休::佯,r.ours.map_or(Q::MAX,|a|a.个),o.q,)});let
乂=|亻:Q,世:F|Q::from(並.丘(万[&亻],世));let 倝=|五:&BTreeMap<F,Q>,伮:&BTreeSet<Q>,上:Q|{伮.contains(&上)||五.values().any(|&u|u==上)};let mut 五:BTreeMap<F,Q> =BTreeMap::new();for(&上,&世)in&丞.倪{if
万.contains_key(&上)&&両.iter().any(|(o,_)|o.q==世)&&!五.contains_key(&世){五.insert(世,上);}}let mut 伮:BTreeSet<Q> =BTreeSet::new();for(o,_)in&両{let Some(&借)=五.get(&o.q)else{continue;};let
侭=万.keys().copied().filter(|&上|!倝(&五,&伮,上)).map(|上|(乂(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();if let Some((d,上))=侭{if d<乂(借,o.q){伮.insert(借);五.insert(o.q,上);}}}let
mut 亞=K::new();for(o,r)in&両{if 五.contains_key(&o.q){continue;}let 丧=万.keys().copied().filter(|&上|!倝(&五,&伮,上)).map(|上|(乂(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();let
偓=丧.map(|(d,_)|C::from(d));let 佦=r.佛.and_then(|t|match t.侠{伿::丼(q)if l.下(丁)-俧*亞.len()as C>=俧&&!亞.iter().any(|&(c,_)|c==q)=>{Some((q,C::from(t.个)))}_=>None,});let
偐=佦.map(|(_,t)|t);let 偔=match(偓,偐){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 像=if 偔{偐}else{偓};let Some(个)=像 else{continue;};if r.theirs.map_or(100,|a|C::from(a.个))<个{continue;}match(偔,佦,丧){(true,Some((q,_)),_)=>亞.push((q,o.q)),(false,_,Some((_,上)))=>{五.insert(o.q,上);}_=>{}}}let
mut 丅=仁{亞,..仁::default()};for(o,_)in&両{if let Some(&上)=五.get(&o.q){丅.仚.push((上,o.q));}}丅.back=l.万(丁).iter().filter_map(|c|l.亍(c)).filter(|上|!丅.仚.iter().any(|&(u,_)|u==*上)).collect();丅.back.sort_unstable();丞.倪=丅.仚.iter().copied().collect();丅}const
俨:C=10;fn 俿(l:&J,丁:F,上:Q)->P<F>{l.万(丁).iter().find(|&c|l.亍(c)==Some(上))}fn 偅(l:&J,丁:F,三:F,世:F)->K<F>{let 並=l.map();let 仄=並.丘(三,世);if 仄==I::MAX||仄==0{return
K::new();}let mut 丅:K<F> =(G::q(三).丏()&並.valid).iter().filter(|&n|並.丘(n,世)<仄).collect();丅.sort_by_key(|&n|(l.丐(丁).test(n),n));丅}fn 交(丌:&mut J,丅:&mut Vec<L>,丒:L)->O{let
倭=丌.乄(丒);if 倭{丅.push(丒);}倭}#[derive(Clone,Copy)]struct 仭{侴:O,guard:I,倍:O,值:O,}fn 今(l:&J,亪:&仁,乵:仭)->K<K<L>>{let mut 丅=K::new();for&(上,世)in&亪.仚{let 儉=仁{仚:vec![(上,世)],..仁::default()};let
佝=偈(l,&儉,仭{侴:false,..乵},);if!佝.is_empty(){丅.push(佝);}}丅}fn 丣(丌:&J,上:Q,三:F,两:F,亥:O,乵:仭,)->P<K<L>>{let 丁=丌.东();let mut 丵=丌.clone();let mut 丈=K::new();let(x,y)=乔(两);if!交(&mut
丵,&mut 丈,L::乶{上,x,y}){return None;}if 亥&&乵.guard!=0{let(仛,仜)=乔(三);let 依=if 乵.guard==1{L::乿{x:仛,y:仜}}else{L::丼{丆:1,x:仛,y:仜,}};if!交(&mut 丵,&mut 丈,依){return
None;}}if 乵.值&&伳(&丵,丁)>伳(丌,丁){return None;}Some(丈)}fn 偈(l:&J,亪:&仁,乵:仭)->K<L>{let 丁=l.东();let 並=l.map();let mut 丌=l.clone();let mut 丅=K::new();let 丩=丩(l,1-丁);let
mut 仾:K<(Q,F)> =K::new();for&(上,世)in&亪.仚{let Some(三)=俿(&丌,丁,上)else{continue;};if!丌.乺().test(三){continue;}let 亥=丩.test(三);for 两 in 偅(&丌,丁,三,世){if let Some(丈)=丣(&丌,上,三,两,亥,乵){for
丒 in&丈{丌.乄(*丒);}if 亥&&乵.guard==0{仾.push((上,三));}丅.extend(丈);break;}}}for&(上,世)in&亪.仚{if 丌.下(丁)<俨{break;}let Some(乙)=俿(&丌,丁,上)else{continue;};let 僄=仾.iter().find(|&&(u,_)|u==上).map(|&(_,c)|c);let
丑=match 僄{Some(q)if 乵.倍=>vec![q],_=>偅(&丌,丁,乙,世),};for 两 in 丑{let(x,y)=乔(两);if 交(&mut 丌,&mut 丅,L::丼{丆:1,x,y}){break;}}}if 乵.侴{for&(_,世)in&亪.亞{if 丌.下(丁)<俨{break;}let
乱=丌.七(丁).丏()&並.valid;let mut 丑:K<F> =乱.iter().filter(|&c|並.丘(c,世)!=I::MAX).collect();丑.sort_by_key(|&c|(並.丘(c,世),c));for 两 in 丑{let(x,y)=乔(两);if 交(&mut
丌,&mut 丅,L::丼{丆:1,x,y}){break;}}}}丅}const 俩:I=0;const 傰:I=1;const 俜:C=0;const 傔:O=false;const 側:I=0;const 偫:O=false;const 傣:O=false;const 偠:O=false;const
偰:Q=7;const 傲:Q=14;const 傪:O=俩!=0||俜!=0;struct 仪{业:K<L>,倬:O,start:O,今:K<K<L>>,乨:K<(F,C,C)>,伆:C,}impl 仪{fn 儸(&self,l:&J,丁:F)->C{if self.伆==0{return 0;}let(丸,丗)=(l.七(丁),l.七(1-丁));let
mut 亅=0;for&(q,丬,亗)in&self.乨{亅+=if 丸.test(q){丬}else if 丗.test(q){-丬}else{亗};}self.伆*亅/4}}fn 乚(l:&J,丞:&mut 伒)->P<仪>{let 伩=l.书();if!傪||!(偰..=傲).contains(&伩)||l.丢().is_some(){return
None;}僱(l,丞,傥)}#[derive(Clone,Copy)]struct 佃{仆:I,乵:仭,侇:O,伆:C,侱:O,}const 傥:佃=佃{仆:俩,乵:仭{侴:傰==2,guard:側,倍:偫,值:傣,},侇:偠,伆:俜,侱:傔,};fn 僱(l:&J,丞:&mut 伒,乃:佃)->P<仪>{let
佃{仆,乵,侇,伆,侱,}=乃;let 丁=l.东();let 乨=乨(l,丁);let 令=儗(l,丁,&乨);let 亪=if 侇{let 侗=l.万(丁).丕(丩(l,1-丁));俴(l,丁,&乨,&令,丞,侗)}else{仱(l,丁,&乨,&令,丞)};let 业=if 仆!=0{偈(l,&亪,乵)}else{K::new()};let
仃=仃(&乨);let 儹=仃.iter().map(|o|{let 亨=令.iter().find(|r|r.q==o.q).map_or(0,|r|r.亨.clamp(-1,1));let 亗=if 侱{o.value*亨}else{0};(o.q,o.value,亗)}).collect();let
儍=if 仆&4!=0{今(l,&亪,乵)}else{K::new()};Some(仪{今:儍,倬:!业.is_empty()&&仆&1!=0,start:!业.is_empty()&&仆&2!=0,业,乨:儹,伆,})}const 偢:亾=45.0;const 偬:亾=800.0;const 储:亾=0.6;fn
乒(举:了,且:Q)->亮{if let Some(伥)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return 亮{人:None,仝:None,伥,};}let 佢=儾().min(100.0);let
儂=if 且==1{偬}else{偢};let 乍=Duration::from_secs_f64(儂/1000.0*佢);亮{人:Some(举+乍.mul_f64(储)),仝:Some(举+乍),伥:丮::MAX,}}const 傖:F=9;const 傒:亿=亿{丑:3,丟:1,下:1,串:2,亊:2,伵:2,丩:6,仩:1,中:0,仱:false,习:傗,};const
傚:Q=0;const 傗:亁=亁::佲{仲:8};const 俠:I=I::MAX;const 俞:C=2;const 俟:C=18;const 保:C=0;const 傕:F=32;const 偞:O=false;const 偟:Settings1=Settings1{侸:3,侷:1,侏:4,仅:2,侘:2,位:2,侾:true,reserve:true,侙:true,乤:什::乷,fill:true,乗:Z{佡:俠,仴:俞,佔:俟,佅:保,..Z::仓},};fn
亄()->亣{if 偞{return 亣::new(云::伻(偟),K::new(),Z::仓);}儊(傕)}fn 儊(亠:F)->亣{let 些=云::伔{个:傖,亠,临:傒,佟:傚,};let 丯=傤.iter().map(|&乗|Z{佡:俠,仴:俞,佔:俟,佅:保,..乗}).collect();亣::new(些,丯,Z::仓)}fn
main(){let 儭=io::stdin();let 儮=io::stdout();if let Err(付)=儑(&乒,亄(),儭.lock(),儮.lock()){eprintln!("plan: {}",付);std::process::exit(1);}}const 傮:&str="CG_TIME_SCALE";fn
儾()->亾{儌(std::env::var(傮).ok().as_deref())}fn 儌(丬:P<&str>)->亾{丬.and_then(|二|二.trim().parse::<亾>().ok()).filter(|佢|佢.is_finite()&&*佢>0.0).unwrap_or(1.0)}const
_:()=assert!(偪==16);const 偪:F=16;const 健:亾=0.4;const 傍:I=4;#[derive(Clone,Copy)]struct 仔{中:G,倀:G,伦:G,}impl 仔{fn of(l:&J)->仔{let(a,伛)=倰(l,T[0]);let(b,佌)=倰(l,T[1]);let
亅=a[T[1]];let 僺=伛[T[1]];let mut 乀=仔{中:G::U,倀:G::U,伦:G::U,};for q in l.map().valid.iter(){if 伛[q]==I::MAX||佌[q]==I::MAX{continue;}if 伛[q].abs_diff(佌[q])<=1{乀.倀.set(q);}if
伛[q].abs_diff(佌[q])<=傍{乀.伦.set(q);}let 儆=伺::from(伛[q])+伺::from(佌[q])==伺::from(僺);if 亅>0.0&&儆&&q!=T[0]&&q!=T[1]&&a[q]*b[q]/亅>=健{乀.中.set(q);}}乀}fn 仠(&self)->G{self.中&self.伦}}fn
儰(l:&J,優:G)->[I;M]{let 丰=l.map().valid;let mut 丅=[I::MAX;M];let mut 乬=優&丰;let mut 份=乬;let mut 丣=0u8;while!份.is_empty(){for q in 份.iter(){丅[q]=丣;}let 丄=(份.丏()&丰).丕(乬);乬|=丄;份=丄;丣=丣.saturating_add(1);}丅}fn
倰(l:&J,三:F)->([亾;M],[I;M]){let 乂=儰(l,G::q(三));let mut 习:K<F> =l.map().valid.iter().filter(|&c|乂[c]!=I::MAX).collect();习.sort_by_key(|&c|乂[c]);let mut 丙=[0.0;M];丙[三]=1.0;for
q in 习{if q==三{continue;}let 也=G::q(q).丏()&l.map().valid;丙[q]=也.iter().filter(|&n|乂[n]!=I::MAX&&乂[n]+1==乂[q]).map(|n|丙[n]).sum();}(丙,乂)}const Y:F=12;const
M:F=Y*Y;const 亇:F=4*Y;const 亢:丮=(1<<亇)-1;const 似:丮=1|1<<12|1<<24|1<<36;const 佮:丮=似<<11;const 俢:丮=(1<<Y)-1;#[derive(Clone,Copy,Debug,PartialEq)]struct G([丮;3]);impl
G{const U:G=G([0;3]);const 偝:G=G([亢;3]);#[inline]fn q(q:F)->G{let mut 乑=G::U;乑.set(q);乑}#[inline]fn test(self,q:F)->O{(self.0[q/亇]>>(q%亇))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/亇]|=1<<(q%亇);}#[inline]fn clear(&mut self,q:F){self.0[q/亇]&=!(1<<(q%亇));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伷(self)->[丮;3]{self.0}#[inline]fn count(self)->Q{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丕(self,么:G)->G{G([self.0[0]&!么.0[0],self.0[1]&!么.0[1],self.0[2]&!么.0[2],])}#[inline]fn
iter(self)->佬{佬{侧:self.0,乙:0,}}#[inline]fn 佤(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&俢)<<(亇-Y)),(b>>Y)|((c&俢)<<(亇-Y)),c>>Y,])}#[inline]fn 佣(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亢,((b<<Y)&亢)|(a>>(亇-Y)),((c<<Y)&亢)|(b>>(亇-Y)),])}#[inline]fn
倿(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佮&亢,(b>>1)&!佮&亢,(c>>1)&!佮&亢,])}#[inline]fn 偀(self)->G{let[a,b,c]=self.0;G([(a<<1)&!似&亢,(b<<1)&!似&亢,(c<<1)&!似&亢,])}#[inline]fn
丏(self)->G{let 一=self;一.佤()|一.佣()|一.倿()|一.偀()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,么:G)->G{G([self.0[0]&么.0[0],self.0[1]&么.0[1],self.0[2]&么.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,么:G)->G{G([self.0[0]|么.0[0],self.0[1]|么.0[1],self.0[2]|么.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,么:G){*self=*self|么;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::偝.丕(self)}}struct 佬{侧:[丮;3],乙:F,}impl Iterator for 佬{type Item=F;#[inline]fn
next(&mut self)->P<F>{let 一=self;while 一.乙<3{let 余=一.侧[一.乙];if 余!=0{一.侧[一.乙]=余&(余-1);return Some(一.乙*亇+余.trailing_zeros()as F);}一.乙+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乶{上:Q,x:I,y:I},丼{丆:I,x:I,y:I},京{x:I,y:I},乿{x:I,y:I},}impl L{fn 允(&self)->乘{match*self{L::乶{上,x,y}=>format!("MOVE {} {x} {y}",上),L::丼{丆,x,y}=>format!("TRAIN {} {x} {y}",丆),L::京{x,y}=>format!("BUILD MINE {x} {y}"),L::乿{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
傿(丈:&[L])->乘{if 丈.is_empty(){return "WAIT".to_string();}let mut line=乘::with_capacity(16*丈.len());for(丠,丒)in 丈.iter().enumerate(){if 丠>0{line.push(';');}line.push_str(&丒.允());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亡{修,兒,傻,#[default]佭,}impl 亡{fn 亖(self)->I{match self{亡::修=>1,_=>3,}}fn 但(self)->O{matches!(self,亡::傻|亡::佭)}fn 仰(self)->O{self==亡::佭}}const 亐:I=I::MAX;struct
亯{乣:G,valid:G,为:G,丘:Box<[I]>,丏:[[[I;4];M];2],}thread_local!{static 傁:RefCell<P<Arc<亯>>> =const{RefCell::new(None)};}impl 亯{fn 儦(乣:G,为:G)->Arc<亯>{傁.with(|乹|{let
mut 乹=乹.borrow_mut();if let Some(並)=乹.as_ref(){if 並.乣==乣&&並.为==为{return Arc::clone(並);}}let 並=Arc::new(亯::build(乣,为));*乹=Some(Arc::clone(&並));並})}fn build(乣:G,为:G)->亯{let
丰=!乣;const 傩:[[(伹,伹);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[亐;4];M];2];for(丁,义)in 傩.iter().enumerate(){for q in
丰.iter(){let(x,y)=((q%Y)as 伹,(q/Y)as 伹);let mut 丙=0;for&(僛,僜)in 义{let(倨,倩)=(x+僛,y+僜);if(0..Y as 伹).contains(&倨)&&(0..Y as 伹).contains(&倩){let 丄=倩 as F*Y+倨
as F;if 丰.test(丄){丏[丁][q][丙]=丄 as I;丙+=1;}}}}}let mut 丘=vec![亐;M*M].into_boxed_slice();let mut 侰=[0u8;M];for 三 in 丰.iter(){let 乽=&mut 丘[三*M..(三+1)*M];乽[三]=0;侰[0]=三
as I;let(mut 侞,mut 侽)=(0,1);while 侞<侽{let q=侰[侞]as F;侞+=1;for&丄 in&丏[0][q]{if 丄!=亐&&乽[丄 as F]==亐{乽[丄 as F]=乽[q]+1;侰[侽]=丄;侽+=1;}}}}亯{乣,valid:丰,为,丘,丏,}}#[inline]fn
丘(&self,三:F,两:F)->I{self.丘[三*M+两]}}const 傋:Q=100;const 于:C=15;const 傴:[C;3]=[1,4,20];const T:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 乲{互(F),俍,}enum
亀{佻,佷,佸,}#[derive(Clone)]struct J{map:Arc<亯>,丽:亡,丛:[G;2],七:[G;2],丸:G,乫:G,万:[G;2],丆:[G;3],仈:G,亍:[Q;M],下:[C;2],丟:[C;2],东:F,书:Q,伂:Q,丢:P<乲>,仟:O,}fn 倖(mut 乩:G,丛:G)->G{loop{let
丄=(乩|乩.丏())&丛;if 丄==乩{return 乩;}乩=丄;}}fn 争(丛:G,仹:F)->G{if!丛.test(仹){return G::U;}let mut 乩=G::q(仹);loop{let 丄=(乩|乩.丏())&丛;if 丄==乩{return 乩;}乩=丄;}}impl
J{fn 东(&self)->F{self.东}fn 书(&self)->Q{self.书}fn 丢(&self)->P<乲>{self.丢}fn 丽(&self)->亡{self.丽}fn 企(&self,丁:F)->C{let 一=self;let 万=一.万[丁];一.下[丁]+10*C::from((万&一.丆[0]).count())+20*C::from((万&一.丆[1]).count())+30*C::from((万&一.丆[2]).count())}fn
下(&self,丁:F)->C{self.下[丁]}fn 丟(&self,丁:F)->C{self.丟[丁]}fn 俀(&self,丁:F)->F{self.万[丁].count()as F}fn map(&self)->&亯{&self.map}fn 丐(&self,丁:F)->G{self.丛[丁]}fn
七(&self,丁:F)->G{self.七[丁]}fn 万(&self,丁:F)->G{self.万[丁]}fn 亼(&self,q:F)->P<(Q,I)>{let 一=self;(一.万[0]|一.万[1]).test(q).then(||(一.亍[q],一.侦(q)))}fn 乺(&self)->G{self.万[self.东]&self.仈}fn
丹(&self)->G{self.丸}fn 乓(&self)->G{self.乫}fn 亟(&self,q:F)->P<I>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.侦(q))}fn 亍(&self,q:F)->P<Q>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.亍[q])}fn
儫(&mut self,丁:F){let 一=self;if 一.仟{debug_assert_eq!(一.七,[争(一.丛[0],T[0]),争(一.丛[1],T[1])]);}else{一.七=[争(一.丛[0],T[0]),争(一.丛[1],T[1])];一.仟=true;}let 僕=一.万[丁].丕(一.七[丁]);一.侳(僕);一.倃();一.下[丁]+=一.丟[丁];if
一.下[丁]<0{一.下[丁]=0;一.侳(一.万[丁]);}一.仈|=一.万[丁];}fn 倃(&mut self){let 一=self;for 丁 in 0..2{let 七=一.七[丁];let 万=七&一.万[丁];let mut 丟=C::from(七.count())+4*C::from((七&一.丸).count());for(丆,兇)in
傴.iter().enumerate(){丟-=兇*C::from((万&一.丆[丆]).count());}一.丟[丁]=丟;}}fn 件(&mut self){let 一=self;if 一.丢.is_some(){return;}if 一.东==1{if 一.书>=傋{一.丢=Some(match
一.企(0).cmp(&一.企(1)){std::cmp::Ordering::Greater=>乲::互(0),std::cmp::Ordering::Less=>乲::互(1),std::cmp::Ordering::Equal=>乲::俍,});return;}一.书+=1;}一.东=1-一.东;一.儫(一.东);}fn
佞(&mut self,丈:&[L]){let 一=self;if 一.丢.is_some(){return;}let 供=一.伂;for&丒 in 丈{if 一.僚(丒,供){continue;}一.乄(丒);if 一.丢.is_some(){return;}}一.件();}fn 僚(&self,丒:L,供:Q)->O{let
一=self;match 丒{L::乶{上,x,y}=>{一.仳(x,y).is_none()||上>=供||一.倏(一.东,上).is_none()}L::丼{丆,x,y}=>{一.仳(x,y).is_none()||丆==0||丆>一.丽.亖()}L::京{x,y}|L::乿{x,y}=>{一.仳(x,y).is_none()}}}#[inline]fn
仳(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<Y&&y<Y&&!self.map.乣.test(y*Y+x)).then_some(y*Y+x)}fn 倏(&self,丁:F,上:Q)->P<F>{self.万[丁].iter().find(|&q|self.亍[q]==上)}#[inline]fn
侦(&self,q:F)->I{if self.丆[0].test(q){1}else if self.丆[1].test(q){2}else{3}}fn 僆(&self,亘:F)->G{let 乓=self.乫&self.丛[亘];let 七=self.七[亘];乓|(七&(七&乓).丏())}fn
乊(&self,丁:F,丆:I)->G{let 一=self;debug_assert!((1..=3).contains(&丆),"unit level {} is not 1 to 3",丆);let 亘=1-丁;let 万=一.万[0]|一.万[1];let 僟=一.仗();let 充=match
丆{1 if 一.丽==亡::修=>一.丆[1]|一.丆[2],1=>万,2=>一.丆[1]|一.丆[2],_=>G::U,};let 乊=僟|一.丛[亘].丕(充);if 丆<3{乊.丕(一.僆(亘))}else{乊}}fn 仉(&self,丁:F)->G{self.七[丁]|self.七[丁].丏()}fn
仗(&self)->G{let 一=self;let 万=一.万[0]|一.万[1];let 僴=G::q(T[0])|G::q(T[1]);一.map.valid.丕(万|一.丸|一.乫|僴)}fn 乄(&mut self,丒:L)->O{let 一=self;if 一.丢.is_some(){return
false;}let 丁=一.东;let 仕=match 丒{L::丼{丆,x,y}=>一.佦(丁,丆,x,y),L::乶{上,x,y}=>一.儁(丁,上,x,y),L::京{x,y}=>一.build(丁,true,x,y),L::乿{x,y}=>一.build(丁,false,x,y),};let
Some(仕)=仕 else{return false;};match 仕{亀::佻=>{}亀::佷 if 一.仟=>{一.七[丁]=倖(一.七[丁],一.丛[丁]);}亀::佸 if 一.仟=>{一.七[丁]=倖(一.七[丁],一.丛[丁]);一.七[1-丁]=争(一.丛[1-丁],T[1-丁]);}亀::佷=>一.七[丁]=争(一.丛[丁],T[丁]),亀::佸=>{一.七=[争(一.丛[0],T[0]),争(一.丛[1],T[1])];}}debug_assert!(!一.仟||一.七==[争(一.丛[0],T[0]),争(一.丛[1],T[1])]);一.倃();if!一.丛[0].test
(T[0]){一.丢=Some(乲::互(1));}else if!一.丛[1].test(T[1]){一.丢=Some(乲::互(0));}true}fn 侳(&mut self,丑:G){let 一=self;一.万[0]=一.万[0].丕(丑);一.万[1]=一.万[1].丕(丑);for 丆
in&mut 一.丆{*丆=丆.丕(丑);}一.仈=一.仈.丕(丑);}fn 偌(&mut self,丁:F,q:F)->亀{let 一=self;一.侳(G::q(q));一.丸.clear(q);一.乫.clear(q);let 亘=1-丁;if 一.丛[亘].test(q){一.丛[亘].clear(q);一.丛[丁].set(q);亀::佸}else
if 一.丛[丁].test(q){亀::佻}else{一.丛[丁].set(q);亀::佷}}fn 佦(&mut self,丁:F,丆:I,x:I,y:I)->P<亀>{let 一=self;if 丆==0||丆>一.丽.亖(){return None;}let q=一.仳(x,y)?;let 与=10*C::from(丆);if
一.下[丁]<与||!一.仉(丁).test(q)||!一.乊(丁,丆).test(q){return None;}let 仕=一.偌(丁,q);一.倱(丁,丆,q,一.伂);一.伂=一.伂.saturating_add(1);一.下[丁]-=与;Some(仕)}fn 倱(&mut self,丁:F,丆:I,q:F,上:Q){self.万[丁].set(q);self.丆[F::from(丆)-1].set(q);self.亍[q]=上;}fn
儁(&mut self,丁:F,上:Q,x:I,y:I)->P<亀>{let 一=self;let 不=一.仳(x,y)?;let 三=一.倏(丁,上)?;if!一.仈.test(三){return None;}let 丆=一.侦(三);let 丨=一.乊(丁,丆);let 也=一.map.丏[丁][三];if
也.contains(&(不 as I))&&!丨.test(不){return None;}let mut 专=一.map.丘(三,不);let mut 伝=None;for&丄 in 也.iter().take_while(|&&丄|丄!=亐){let 丄=F::from(丄);let 丘=一.map.丘(丄,不);if
丘<专&&丨.test(丄){专=丘;伝=Some(丄);}}let 伝=伝?;let 仕=一.偌(丁,伝);一.万[丁].clear(三);一.丆[F::from(丆)-1].clear(三);一.仈.clear(三);一.倱(丁,丆,伝,上);Some(仕)}fn build(&mut self,丁:F,丸:O,x:I,y:I)->P<亀>{let
一=self;let 侄=if 丸{一.丽.但()}else{一.丽.仰()};let q=一.仳(x,y)?;let 儇=一.map.为.test(q);if!侄||一.下[丁]<于||儇!=丸||!一.丛[丁].test(q)||!一.仉(丁).test(q)||!一.仗().test(q){return
None;}if 丸{一.丸.set(q);}else{一.乫.set(q);}一.下[丁]-=于;Some(亀::佻)}}const 傆:C=1000000000;const 併:C=0;const 傎:C=1;const 傯:C=2;impl J{fn read(仺:&str,且:&str,丽:亡,书:Q,)->Result<J,乘>{let
mut 为=K::new();let mut 乇=仺.lines();let 丙=侔(&mut 乇,"mine spot count")?;for _ in 0..丙{let[x,y]=亸(伃(&mut 乇,"a mine spot")?,"a mine spot")?;为.push((佊(x)?,佊(y)?));}個(乇,"the initial input")?;let
mut 乇=且.lines();let mut 侟=[0i64;4];for(丬,乜)in 侟.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[侮]=亸(伃(&mut 乇,乜)?,乜)?;if 侮.abs()>傆{return
Err(format!("{}: {} is out of range",乜,侮));}*丬=侮;}let mut 乣=G::U;let mut 丑=[G::U;2];let mut 七=[G::U;2];for y in 0..Y{let 乽=伃(&mut 乇,"a map row")?.trim();if
乽.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,乽));}for(x,伱)in 乽.bytes().enumerate(){let q=y*Y+x;match 伱{b'#'=>乣.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乭=F::from(伱==b'X'||伱==b'x');丑[乭].set(q);if 伱.is_ascii_uppercase(){七[乭].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伱 as char))}}}}let
僉=侔(&mut 乇,"building count")?;let mut 亂=K::new();for _ in 0..僉{let 亸=亸(伃(&mut 乇,"a building")?,"a building")?;亂.push(亸);}let 俀=侔(&mut 乇,"unit count")?;let
mut 万=K::new();for _ in 0..俀{let 亸=亸(伃(&mut 乇,"a unit")?,"a unit")?;万.push(亸);}個(乇,"the turn's input")?;let 倮=亂.iter().find(|俺|俺[0]==0&&俺[1]==併).ok_or("the bot's own HQ is not among the buildings")?;let
倚=例(倮[2],倮[3],乣)?;let 丁=T.iter().position(|&仹|仹==倚).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",倚))?;let mut 偁=G::U;for(x,y)in 为{if
乣.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}偁.set(y*Y+x);}let 侂=|乭:F|if 乭==0{丁}else{1-丁};let mut l=J{map:亯::儦(乣,偁),丽,丛:[G::U;2],七:[G::U;2],丸:G::U,乫:G::U,万:[G::U;2],丆:[G::U;3],仈:G::U,亍:[0;M],下:[0;2],丟:[0;2],东:丁,书,伂:1,丢:None,仟:false,};for
乭 in 0..2{let 亚=侂(乭);l.丛[亚]=丑[乭];l.七[亚]=七[乭];l.下[亚]=侟[2*乭];l.丟[亚]=侟[2*乭+1];}let mut 倫=G::U;for[亚,佖,x,y]in 亂{let 乭=倷(亚)?;let q=例(x,y,乣)?;if!丑[乭].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 倫.test(q){return Err(format!("two buildings at ({x}, {y})"));}倫.set(q);match 佖{併
if q==T[侂(乭)]=>{}併=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}傎=>l.丸.set(q),傯=>l.乫.set(q),_=>return Err(format!("unknown building type {}",佖)),}}let
mut 乯=G::U;let mut 佗=0u32;for[亚,上,丆,x,y]in 万{let 乭=倷(亚)?;let q=例(x,y,乣)?;if!丑[乭].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乯.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丆){return Err(format!("unit level {} is not 1, 2 or 3",丆));}let 上=Q::try_from(上).map_err(|_|format!("unit id {} is out of range",上))?;if
乯.iter().any(|么|l.亍[么]==上){return Err(format!("two units have id {}",上));}乯.set(q);l.万[侂(乭)].set(q);l.丆[丆 as F-1].set(q);l.亍[q]=上;佗=佗.max(上);}if!乯.is_empty(){l.伂=佗.checked_add(1).ok_or_else(||format!("unit id {} is out of range",佗))?;}l.仈=l.万[丁];l.仟=l.七==[争(l.丛[0],T[0]),争(l.丛[1],T[1])];Ok(l)}}fn
伃<'a>(乇:&mut Lines<'a>,乜:&str)->Result<&'a str,乘>{乇.next().ok_or_else(||format!("the text ends before {}",乜))}fn 個(mut 乇:Lines<'_>,乜:&str)->Result<(),乘>{match
乇.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乜)),None=>Ok(()),}}fn 亸<const N:F>(line:&str,乜:&str)->Result<[C;N],乘>{let
mut 伷=line.split_whitespace();let mut 偕=[0i64;N];for 丬 in&mut 偕{let 伌=伷.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乜))?;*丬=伌.parse().map_err(|_|format!("{}: {:?} is not an integer",乜,伌))?;}if
伷.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乜));}Ok(偕)}fn 侔(乇:&mut Lines<'_>,乜:&str)->Result<F,乘>{let[丙]=亸(伃(乇,乜)?,乜)?;F::try_from(丙).ok().filter(|&丙|丙<=M).ok_or_else(||format!("{}: {} is out of range",乜,丙))}fn
佊(丬:C)->Result<F,乘>{F::try_from(丬).ok().filter(|&丬|丬<Y).ok_or_else(||format!("coordinate {} is off the map",丬))}fn 例(x:C,y:C,乣:G)->Result<F,乘>{let q=佊(y)?*Y+佊(x)?;if
乣.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 倷(亚:C)->Result<F,乘>{match 亚{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亚)),}}#[derive(Clone,Copy,PartialEq)]enum
什{乷,俆,}#[derive(Clone,Copy)]struct Settings1{侸:I,侷:I,侏:C,仅:C,侘:F,位:I,侾:O,reserve:O,侙:O,乤:什,fill:O,乗:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{侸:3,侷:1,侏:4,仅:2,侘:2,位:2,侾:true,reserve:true,侙:true,乤:什::乷,fill:true,乗:Z::仓,};}const
侁:C=1000;const 侀:C=10;fn 促(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侯(l:&mut J,丒:L,丅:&mut Vec<L>)->O{let 交=l.乄(丒);if 交{丅.push(丒);}交}fn 侻(l:&mut J,三:F,两:F,丅:&mut
Vec<L>)->O{let Some((上,_))=l.亼(三)else{return false;};let(x,y)=促(两);侯(l,L::乶{上,x,y},丅)}fn 儷(l:&J,乀:&仔,乃:&Settings1,乒:F,)->K<F>{let 丂=l.东();let 严=丳(l,l.丐(丂));let
丗=丳(l,l.丐(1-丂));let 亗=l.map().valid.丕(l.丐(0)|l.丐(1));let 倵=|c:F|严[c]!=I::MAX&&丗[c]!=I::MAX;let mut 中:K<F> =乀.仠().丕(l.丐(丂)).iter().filter(|&c|倵(c)).collect();中.sort_by_key(|&c|(严[c],c));let
mut 乆:K<F> =亗.iter().filter(|&c|倵(c)&&严[c].abs_diff(丗[c])<=乃.侷).collect();乆.sort_by_key(|&c|(严[c].abs_diff(丗[c]),严[c],c));let mut 丅:K<F> =K::new();for
q in 中.into_iter().chain(乆){if 丅.len()>=乒{break;}let 仙=丅.iter().all(|&t|l.map().丘(t,q)>=乃.侸);let 僎=乀.仠().test(q);if!丅.contains(&q)&&(僎||仙){丅.push(q);}}丅}fn
僵(与:&[K<C>])->K<F>{let n=与.len();let 倞=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 偗=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亶=0usize;let mut 伧=vec![倞;n+1];let mut 俁=vec![false;n+1];loop{俁[亶]=true;let 倜=p[亶];let mut 伜=倞;let mut 伣=0usize;for j in 1..=n{if!俁[j]{let
乁=与[倜-1][j-1]-u[倜]-v[j];if 乁<伧[j]{伧[j]=乁;偗[j]=亶;}if 伧[j]<伜{伜=伧[j];伣=j;}}}for j in 0..=n{if 俁[j]{u[p[j]]+=伜;v[j]-=伜;}else{伧[j]-=伜;}}亶=伣;if p[亶]==0{break;}}loop{let
伣=偗[亶];p[亶]=p[伣];亶=伣;if 亶==0{break;}}}let mut 丅=vec![0usize;n];for j in 1..=n{if p[j]>0{丅[p[j]-1]=j-1;}}丅}#[derive(Clone,Copy)]enum 仐{伕(F),使,}fn 乱(l:&J)->G{let
丂=l.东();(l.仉(丂)&l.乊(丂,1)).丕(l.丐(丂))}fn 儏(l:&J,乗:&Z)->F{佧(l,乗,乕::乷).iter().filter(|c|matches!(c,L::丼{..})).count()}fn 両(q:F,丧:G)->Q{(G::q(q).丏()&丧).count()}fn
倒(l:&J,乏:G,不:F,侤:I)->K<[C;M]>{let 丰=l.map().valid;let mut 专=[C::MIN;M];专[不]=0;let mut 乧=vec![专];for _ in 0..侤{let mut 丄=专;for q in 丰.iter(){for n in(G::q(q).丏()&丰).iter(){if
专[n]>C::MIN{let 丬=专[n]+C::from(乏.test(n));if 丬>丄[q]{丄[q]=丬;}}}}专=丄;乧.push(专);}乧}fn 倯(l:&J,乧:&[[C;M]],乏:G,三:F,k:F,)->G{let 丰=l.map().valid;let mut 丅=G::U;let
mut q=三;let mut k=k.min(乧.len()-1);while k>0&&乧[k][q]>0{let Some(丄)=(G::q(q).丏()&丰).iter().filter(|&n|乧[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(乏.test(n))+乧[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乏.test(丄){丅.set(丄);}q=丄;k-=1;}丅}fn 且(l:&J,乃:&Settings1)->K<L>{let 丂=l.东();let 乀=仔::of(l);let mut 丌=l.clone();let mut 丅=K::new();let 丗=丳(l,l.丐(1-丂));let
万:K<F> =l.乺().iter().collect();let 倔=(l.下(丂)/侀).max(0)as F;let 亞=match 乃.乤{什::乷=>儏(l,&乃.乗).min(倔),什::俆=>倔,};let mut 伖:K<仐> =万.iter().map(|&c|仐::伕(c)).collect();伖.extend(std::iter::repeat_n(仐::使,亞));let
亃=儷(l,&乀,乃,伖.len()+乃.侘);let 元=乱(l);let 中=乀.仠();let n=伖.len().max(亃.len());let mut 与=vec![vec![0i64;n];n];for(乽,&不)in 亃.iter().enumerate(){for(会,仌)in 与[乽].iter_mut().enumerate(){let
Some(&傾)=伖.get(会)else{*仌=侁;continue;};let 佇=match 傾{仐::伕(q)=>{let d=l.map().丘(q,不);if d==I::MAX{None}else{Some(C::from(d)-1)}}仐::使=>元.iter().map(|c|l.map().丘(c,不)).filter(|&d|d!=I::MAX).min().map(C::from),};*仌=match
佇{None=>侁,Some(佇)=>{let 丷=C::from(丗[不])-1;let 仅=乃.仅*(佇-丷).max(0);let 侈=if 中.test(不){乃.侏}else{0};(佇+仅-侈).max(0)}};}}let 僀=僵(&与);let mut 侜:K<(F,F)> =K::new();let
mut 便:K<(C,F)> =K::new();for(乽,&不)in 亃.iter().enumerate(){let 会=僀[乽];if 与[乽][会]>=侁{continue;}match 伖.get(会){Some(仐::伕(q))=>侜.push((*q,不)),Some(仐::使)=>便.push((与[乽][会],不)),None=>{}}}let
mut 丧=l.map().valid.丕(l.丐(0)|l.丐(1));let mut 倦:K<F> =K::new();let mut 你=G::U;let 亙=|q:F|{let 丆=l.亟(q).unwrap_or(1);let 伲=丧|l.丐(1-丂);(G::q(q).丏()&伲&l.乊(丂,丆)).count()};侜.sort_by_key(|&(q,不)|{let
僥=if 乃.侙{亙(q)}else{0};(僥,l.map().丘(q,不),q)});for&(三,不)in&侜{let Some(丆)=丌.亟(三)else{continue;};let 丨=丌.乊(丂,丆);let d=丌.map().丘(三,不);let 乏=(丧|丌.丐(1-丂)).丕(你);let
侤=d.saturating_add(乃.位);let 乧=倒(&丌,乏,不,侤.saturating_sub(1));let 仯=乧[乧.len()-1];let 丄=(G::q(三).丏()&丨).iter().filter(|&c|仯[c]>C::MIN).max_by_key(|&c|{let
僑=d.saturating_sub(丌.map().丘(c,不));let 伲=丧.test(c)||丌.丐(1-丂).test(c);(伲&&乃.侾,C::from(乏.test(c))+仯[c],僑,両(c,丧),std::cmp::Reverse(c),)}).filter(|&c|{丧.test(c)||丌.丐(1-丂).test(c)||丌.map().丘(c,不)<d});if
let Some(两)=丄{if 侻(&mut 丌,三,两,&mut 丅){丧.clear(两);倦.push(三);if 乃.reserve{你|=倯(&丌,&乧,乏,两,乧.len()-1);}}}}let 严=丳(l,l.丐(丂));let mut 丶=G::U;for c in 丧.iter(){if
严[c]<=丗[c]{丶.set(c);}}for&三 in&万{if 倦.contains(&三)||!丌.乺().test(三){continue;}let Some(丆)=丌.亟(三)else{continue;};let 丨=丌.乊(丂,丆);let 伲=(G::q(三).丏()&丧&丨).iter().max_by_key(|&c|(両(c,丧),丗[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(两)=伲{if 侻(&mut 丌,三,两,&mut 丅){丧.clear(两);}continue;}let 世=[丶&丧,丧].into_iter().find_map(|丑|{丑.iter().min_by_key(|&c|(丌.map().丘(三,c),c))});if let
Some(世)=世{let d=丌.map().丘(三,世);let 丄=(G::q(三).丏()&丨).iter().filter(|&c|丌.map().丘(c,世)<d).min_by_key(|&c|c);if let Some(两)=丄{侻(&mut 丌,三,两,&mut 丅);}}}便.sort();let
mut 侩=0usize;for&(_,不)in&便{if 丌.下(丂)<侀{break;}let 丑=乱(&丌);let Some(乖)=丑.iter().map(|c|丌.map().丘(c,不)).min()else{break;};let 乏=(丧|丌.丐(1-丂)).丕(你);let 乧=倒(&丌,乏,不,乖.saturating_add(乃.位));let
仯=乧[乧.len()-1];let q=丑.iter().filter(|&c|丌.map().丘(c,不)==乖).max_by_key(|&c|(仯[c],両(c,丧),std::cmp::Reverse(c)));let Some(q)=q else{break;};let(x,y)=促(q);if
侯(&mut 丌,L::丼{丆:1,x,y},&mut 丅){丧.clear(q);侩+=1;if 乃.reserve{你|=倯(&丌,&乧,乏,q,乧.len()-1);}}}if 乃.fill&&乃.乤==什::乷{while 侩<亞&&丌.下(丂)>=侀{let q=(乱(&丌)&丧).iter().max_by_key(|&c|{(両(c,丧),std::cmp::Reverse(丗[c]),std::cmp::Reverse(c),)});let
Some(q)=q else{break;};let(x,y)=促(q);if!侯(&mut 丌,L::丼{丆:1,x,y},&mut 丅){break;}丧.clear(q);侩+=1;}}丅}const 偺:C=15;const 傏:C=6;const 俪:C=10;const 傢:I=3;#[derive(Clone,Copy)]struct
Z{open:C,佉:C,倆:I,価:C,佔:C,倢:C,佡:I,倅:C,倄:C,佅:C,仸:C,伉:C,乢:C,任:C,仏:C,仴:C,}impl Z{const 仓:Z=Z{open:3,佉:2,倆:12,価:4,佔:偺,倢:傏,佡:傢,倅:2,倄:2,佅:0,仸:0,伉:0,乢:0,任:0,仏:0,仴:0,};const
fn 児(self,仸:C,伉:C,乢:C)->Z{Z{仸,伉,乢,..self}}const fn 兏(self,任:C,仏:C)->Z{Z{任,仏,..self}}}fn 伸(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 丳(l:&J,三:G)->[I;M]{let mut
乂=[I::MAX;M];let 丰=l.map().valid;let mut 乆=三&丰;let mut 乯=乆;let mut d=0u8;while!乆.is_empty(){for q in 乆.iter(){乂[q]=d;}let 丄=(乆.丏()&丰).丕(乯);乯|=丄;乆=丄;d=d.saturating_add(1);}乂}fn
僬(l:&J)->I{let 乂=丳(l,l.丐(0));l.丐(1).iter().map(|q|乂[q]).min().unwrap_or(I::MAX)}struct 仂<'a>{乗:&'a Z,丂:F,theirs:[I;M],伊:[I;M],free:G,丶:G,ours:G,为:G,万:G,}impl
仂<'_>{fn new<'a>(l:&J,乗:&'a Z)->仂<'a>{let 丂=l.东();let 丸=丳(l,l.丐(丂));let 丗=丳(l,l.丐(1-丂));let 伊=丳(l,G::q(T[1-丂]));let 丧=l.map().valid.丕(l.丐(0)|l.丐(1));let
mut 丶=G::U;for q in 丧.iter(){if 丸[q]<=丗[q]{丶.set(q);}}仂{乗,丂,theirs:丗,伊,free:丧,丶,ours:l.丐(丂),为:l.map().为,万:l.万(丂),}}fn value(&self,q:F)->C{let 一=self;let
p=一.乗;let 両=C::from((G::q(q).丏()&一.free).count());let 仲=p.倆;let 佉=C::from(仲)-C::from(一.theirs[q].min(仲));let 仹=if p.価>0{C::from(一.伊[q].min(30))/p.価}else{0};let
丛=C::from((G::q(q).丏()&一.ours).count());let line=if 丛<=1{p.伉}else{0};let 乢=if 一.为.test(q){p.乢}else{0};let 儳=if 両==0{p.任}else{0};let 仏=p.仏*(22-C::from(一.伊[q].min(22)));let
仴=p.仴*C::from((G::q(q).丏()&一.万).count());p.open*両+p.佉*佉-仹+p.仸*丛-line+乢-儳+仏-仴}fn 专(&self,丑:G)->P<F>{丑.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
侐(&mut self,q:F){let 一=self;一.free.clear(q);一.丶.clear(q);一.ours.set(q);一.万.set(q);}}fn 伬(l:&mut J,丒:L,丅:&mut Vec<L>)->O{let 交=l.乄(丒);if 交{丅.push(丒);}交}fn
偄(l:&mut J,三:F,两:F,丅:&mut Vec<L>)->O{let Some((上,_))=l.亼(三)else{return false;};let(x,y)=伸(两);伬(l,L::乶{上,x,y},丅)}fn 兆(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-俪}fn
儞(l:&J,乗:&Z)->K<L>{佧(l,乗,乕::乷)}#[derive(Clone,Copy,PartialEq)]enum 乕{乷,亭{丹:I,万:I},}fn 佧(l:&J,乗:&Z,乤:乕)->K<L>{let mut 丌=l.clone();let mut 且=仂::new(l,乗);let
mut 丅=K::new();亷(&mut 丌,&mut 且,&mut 丅);match 乤{乕::乷=>儖(&mut 丌,&mut 且,&mut 丅),乕::亭{丹,万}=>{僣(&mut 丌,&mut 且,丹,万,&mut 丅)}}丅}fn 亷(l:&mut J,且:&mut 仂<'_>,丅:&mut
Vec<L>){let 丂=且.丂;let mut 亵:K<F> =K::new();loop{let 丨=l.乊(丂,1);let mut 伅:P<(Q,F)> =None;for 三 in l.乺().iter(){if 亵.contains(&三){continue;}let 亙=(G::q(三).丏()&且.free&丨).count();if
亙==0{亵.push(三);}else if 伅.is_none_or(|(n,_)|亙<n){伅=Some((亙,三));}}let Some((_,三))=伅 else{break;};且.万.clear(三);match 且.专(G::q(三).丏()&且.free&丨){Some(两)if
偄(l,三,两,丅)=>且.侐(两),_=>{且.万.set(三);亵.push(三);}}}for 三 in 亵{if!l.乺().test(三){continue;}let 不=[且.丶,且.free].into_iter().find_map(|丑|{丑.iter().min_by_key(|&q|(l.map().丘(三,q),q))});if
let Some(不)=不{偄(l,三,不,丅);}}}fn 儖(l:&mut J,且:&mut 仂<'_>,丅:&mut Vec<L>){let 丂=且.丂;let p=且.乗;let h=(p.佔-C::from(l.书())).max(p.倢);let 儿=(C::from(僬(l))-p.倅).max(0)/p.倄.max(1)+p.佅;loop{let
下=l.下(丂);let 万=l.俀(丂)as C;let 儝=C::from(且.丶.count());let q=且.专(l.仉(丂)&且.free&l.乊(丂,1));let 偒=兆(h,(儝/(万+1)).min(儿));let 俾=q.is_some()&&下>=俪&&偒>0;let 倣=4*h-于;let
乢=値(l,且);let 僌=乢.is_some()&&下>=于&&倣>0;if 僌&&(!俾||倣>=偒){let(x,y)=伸(乢.unwrap_or_default());if 伬(l,L::京{x,y},丅){continue;}break;}let Some(q)=q.filter(|_|俾)else{break;};let(x,y)=伸(q);if!伬(l,L::丼{丆:1,x,y},丅){break;}且.侐(q);}}fn
値(l:&J,且:&仂<'_>)->P<F>{let 丂=且.丂;(l.map().为&l.七(丂)&l.仗()).iter().filter(|&乢|且.theirs[乢]>且.乗.佡).min_by_key(|&乢|(l.map().丘(T[丂],乢),乢))}fn 僣(l:&mut J,且:&mut
仂<'_>,丹:I,万:I,丅:&mut Vec<L>,){let 丂=且.丂;for _ in 0..丹{let Some(乢)=値(l,且)else{break;};let(x,y)=伸(乢);if!伬(l,L::京{x,y},丅){break;}}for _ in 0..万{let Some(q)=且.专(l.仉(丂)&且.free&l.乊(丂,1))else{break;};let(x,y)=伸(q);if!伬(l,L::丼{丆:1,x,y},丅){break;}且.侐(q);}}fn
乬(l:&J)->O{if l.丢().is_some(){return false;}let 丂=l.东();let 下=l.下(丂);let 丩=们::new(l,丂,true);l.丐(1-丂).iter().any(|q|丩.与[q]<=下)}#[derive(Clone,Copy)]struct
主{乐:I,乤:乕,}const 俄:I=I::MAX;const 偣:[乕;8]=[乕::乷,乕::亭{丹:0,万:0},乕::亭{丹:0,万:1},乕::亭{丹:0,万:2},乕::亭{丹:0,万:俄,},乕::亭{丹:1,万:0},乕::亭{丹:1,万:1},乕::亭{丹:1,万:俄,},];struct
佰<'a>{丯:&'a[Z],仇:Z,rest:主,亳:P<了>,乃:Settings2,}impl 佰<'_>{fn 乗(&self,丣:主)->&Z{&self.丯[F::from(丣.乐)%self.丯.len()]}fn 丈(&self,l:&J,丣:主)->K<L>{佧(l,self.乗(丣),丣.乤)}}fn
傼(l:&J,丈:&[L],仇:&Z)->(J,O){let mut 丄=l.clone();丄.佞(丈);if 丄.丢().is_some()||乬(&丄){return(丄,true);}let 乼=儞(&丄,仇);丄.佞(&乼);let 亹=丄.丢().is_some()||乬(&丄)||丄.书()>俊;(丄,亹)}fn
儕(l:&J,业:&[主],他:&佰<'_>)->K<丮>{let 丂=l.东();let mut 伤=vec![估(l,丂)];let mut 买=l.clone();for 且 in 0..俊 as F{let 丣=业.get(且).copied().unwrap_or(他.rest);let 丈=他.丈(&买,丣);let(丄,亹)=傼(&买,&丈,&他.仇);if
亹{break;}买=丄;伤.push(估(&买,丂));}伤}fn 估(l:&J,丂:F)->丮{let mut hash:丮=0xcbf29ce484222325;let mut 侪=|伌:丮|hash=(hash^伌).wrapping_mul(0x0100000001b3);for(儶,乑)in[l.丐(丂),l.万(丂),l.丹()].into_iter().enumerate(){for
q in 乑.iter(){侪(((儶 as 丮)<<8)|q as 丮);}侪(0xff);}侪(l.下(丂)as 丮);hash}enum 云{伔{个:F,亠:F,临:亿,佟:Q,},伻(Settings1),}struct 亣{些:云,丯:K<Z>,仇:Z,业:K<主>,住:K<丮>,代:F,之:Q,亹:O,}impl
亣{fn new(些:云,丯:K<Z>,仇:Z)->亣{亣{些,丯,仇,业:K::new(),住:K::new(),代:0,之:0,亹:false,}}fn 儒(&mut self,l:&J)->O{let 一=self;let 儨=matches!(一.些,云::伔{..}|云::伻(_));let
儔=!儨&&一.之>0&&一.代>=一.住.len();if!一.亹&&(儔||乬(l)){一.亹=true;}!一.亹}fn end(&mut self){self.亹=true;}fn 且(&mut self,l:&J,亳:P<了>)->K<L>{let 一=self;let 他=佰{丯:&一.丯,仇:一.仇,rest:主{乐:0,乤:乕::乷,},亳,乃:Settings2::DEFAULT,};if
let 云::伻(乃)=一.些{一.之+=1;一.代+=1;return 且(l,&乃);}let 丂=l.东();let 儈=一.住.get(一.代)==Some(&估(l,丂));let 僢=matches!(一.些,云::伔{佟,..}if 佟>0&&l.书()>=佟);if!儈||僢{let
乸=一.之==0;一.业=match 一.些{云::伔{个,亠,临,..}=>{let 临=&临;let 乪=傝;let mut 义=业(l,&一.丯,乪,个,亠,亳,临,);义.resize(个,乪);义}云::伻(_)=>K::new(),};一.住=match 一.些{云::伔{..}=>儩(l,&一.丯,&一.业),_=>儕(l,&一.业,&他),};一.代=0;一.之+=1;}let
丣=一.业.get(一.代).copied().unwrap_or(他.rest);一.代+=1;他.丈(l,丣)}}fn 儩(l:&J,丯:&[Z],业:&[主])->K<丮>{let 丂=l.东();let mut 伤=K::with_capacity(业.len());let mut 买=l.clone();for&丣
in 业{if 买.丢().is_some(){break;}伤.push(估(&买,丂));买=侃(&买,&丈(&买,丯,丣));}伤}const 俊:Q=20;const 傱:C=3;#[derive(Clone,Copy)]struct Settings2{个:C,}impl Settings2{const
DEFAULT:Settings2=Settings2{个:傱};}fn 义(l:&J,三:G)->[I;M]{let 丰=l.map().valid;let mut 乂=[I::MAX;M];let mut 乆=三&丰;let mut 乯=乆;let mut d=0u8;while!乆.is_empty(){for
q in 乆.iter(){乂[q]=d;}let 丄=(乆.丏()&丰).丕(乯);乯|=丄;乆=丄;d=d.saturating_add(1);}乂}fn 光(l:&J,丁:F)->C{let 严=义(l,l.七(丁));let 丗=义(l,l.七(1-丁));l.map().valid.iter().map(|q|match
严[q].cmp(&丗[q]){_ if 严[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 傤:[Z;27]=丯();const
傝:主=主{乐:16,乤:乕::乷,};const fn 丯()->[Z;27]{let 仸=[0,2,4];let 伉=[0,3,6];let 仏=[0,1,2];let mut 丅=[Z::仓;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
任=if g>0||t>0{4}else{0};丅[i]=Z::仓.児(仸[g],伉[t],0).兏(任,仏[w]);i+=1;}丅}#[derive(Default)]struct 佽{丑:C,丟:C,下:C,亊:C,串:C,伵:C,丩:C,仩:C,}#[derive(Clone,Copy)]struct
亿{丑:C,丟:C,下:C,串:C,亊:C,伵:C,丩:C,仩:C,中:C,仱:O,习:亁,}#[derive(Clone,Copy,PartialEq)]enum 亁{俘,俌(佱),佲{仲:C},仑(仑),俉,}fn 兀(l:&J,丁:F)->[I;M]{let 严=丳(l,G::q(T[丁]));let
丗=丳(l,G::q(T[1-丁]));let mut 产=G::U;let mut 丶=G::U;for c in l.map().valid.iter(){if 严[c]==I::MAX||丗[c]==I::MAX{continue;}if 严[c]==丗[c]{产.set(c);}if 严[c]<=丗[c]{丶.set(c);}}let
d=丳(l,产);let mut 丅=[I::MAX;M];for c in 丶.iter(){丅[c]=d[c];}丅}fn 僅(l:&J,丁:F,产:&[I;M])->C{let d=丳(l,l.七(丁));(0..M).filter(|&c|产[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&产[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 仑{備,}impl
仑{fn 係(self,a:C,b:C,d:C)->C{let 以=(a-b).abs()/2;match self{仑::備=>a.min(d/2),}}}fn 僦(l:&J,丁:F,侚:仑)->[C;M]{let 严=丳(l,G::q(T[丁]));let 丗=丳(l,G::q(T[1-丁]));let
d=C::from(严[T[1-丁]]);let mut 丅=[0;M];for c in 0..M{if 严[c]!=I::MAX&&丗[c]!=I::MAX{丅[c]=侚.係(C::from(严[c]),C::from(丗[c]),d);}}丅}#[derive(Clone,Copy,PartialEq)]enum
佱{傫(C),}impl 佱{fn 係(self,以:C)->C{match self{佱::傫(亝)=>(亝-以).max(0).pow(2),}}}type 佀=[C;4];fn 儢(l:&J,丁:F)->[C;M]{struct 伎{七:G,乳:[伺;M],乛:[伺;M],size:[C;M],串:[C;M],从:伺,}fn
仨(d:&mut 伎,v:F){d.从+=1;d.乳[v]=d.从;d.乛[v]=d.从;d.size[v]=1;for w in(G::q(v).丏()&d.七).iter(){if d.乳[w]==0{仨(d,w);d.size[v]+=d.size[w];d.乛[v]=d.乛[v].min(d.乛[w]);if
d.乛[w]>=d.乳[v]{d.串[v]+=d.size[w];}}else{d.乛[v]=d.乛[v].min(d.乳[w]);}}}let mut d=伎{七:l.七(丁),乳:[0;M],乛:[0;M],size:[0;M],串:[0;M],从:0,};let 乮=T[丁];if d.七.test(乮){仨(&mut
d,乮);}d.串[乮]=0;d.串}fn 儤(l:&J,丁:F)->佽{let 七=l.七(丁);let 串=儢(l,丁);let 伊=丳(l,G::q(T[1-丁]));let mut 丅=佽{丑:C::from(七.count()),丟:l.丟(丁),下:l.下(丁),丩:C::MAX,仩:光(l,丁)/2,..佽::default()};for
q in 七.iter(){if 串[q]>0{丅.亊+=1;丅.串=丅.串.max(串[q]);}if q!=T[丁]&&(G::q(q).丏()&七).count()<=1{丅.伵+=1;}丅.丩=丅.丩.min(C::from(伊[q]));}丅}fn 先(l:&J,丁:F,临:&亿,伦:G)->C{let
仞=C::from((l.丐(丁)&伦).count());let s=儤(l,丁);let o=临;o.丑*s.丑+o.丟*s.丟+o.下*s.下-o.串*s.串-o.亊*s.亊-o.伵*s.伵-o.丩*s.丩.min(30)+o.仩*s.仩+o.中*仞}fn 仠(l:&J)->G{仔::of(l).仠()}fn
僿(l:&J)->G{let 严=丳(l,G::q(T[0]));let 丗=丳(l,G::q(T[1]));let mut 久=G::U;for c in l.map().valid.iter(){if 严[c]!=I::MAX&&丗[c]!=I::MAX&&严[c].abs_diff(丗[c])<=2{久.set(c);}}久}fn
仦(l:&J)->[C;M]{let a=丳(l,G::q(T[0]));let b=丳(l,G::q(T[1]));let mut 丅=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丅[c]=C::from(a[c].abs_diff(b[c]))/2;}}丅}#[derive(Clone,Copy)]struct
仮{中:G,久:G,乐:[C;M],仦:[C;M],乆:G,front:[I;M],}impl 仮{fn of(l:&J,临:&亿)->仮{let 倠=matches!(临.习,亁::俘);let mut 乐=[0;M];if let 亁::俌(僖)=临.习{for(w,以)in 乐.iter_mut().zip(仦(l)){*w=if
以==C::MAX{0}else{僖.係(以)};}}if let 亁::仑(侚)=临.习{乐=僦(l,l.东(),侚);}let(儜,乆)=match 临.习{亁::佲{..}=>{let r=仦(l);let mut f=G::U;for(c,&以)in r.iter().enumerate(){if
以==0{f.set(c);}}(r,f)}_=>([C::MAX;M],G::U),};仮{乐,仦:儜,乆,front:if 临.习==亁::俉{兀(l,l.东())}else{[I::MAX;M]},中:if 临.中!=0||倠{仔::of(l).仠()}else{G::U},久:if 倠{僿(l)}else{G::U},}}}fn
仼(l:&J,丁:F,临:&亿,乀:仮)->佀{let 佥=先(l,丁,临,乀.中);match 临.习{亁::俌(_)|亁::仑(_)=>{let 七=l.七(丁);let 兌:C=七.iter().map(|c|乀.乐[c]).sum();[C::from(七.count()),兌,佥,0]}亁::佲{仲}=>{let
七=l.七(丁);let d=丳(l,七);let 仙:C=乀.乆.iter().map(|c|C::from(d[c]).min(仲)).sum();[C::from(七.count()),-仙,佥,0]}亁::俉=>[C::from(l.七(丁).count()),僅(l,丁,&乀.front),佥,0,],亁::俘=>{let
丐=l.丐(丁);let 中=C::from((丐&乀.中).count());let 久=C::from((丐&乀.久).count());match 临.习{_=>[C::from(l.七(丁).count()),中,久,佥],}}}}fn 侃(l:&J,丈:&[L])->J{let mut 丄=l.clone();丄.佞(丈);if
丄.丢().is_none(){丄.佞(&[]);}丄}const 俅:I=I::MAX;fn 丈(l:&J,丯:&[Z],丣:主)->K<L>{if 丣.乐==俅{let 乃=Settings1{乗:丯.first().copied().unwrap_or(Z::仓),乤:if 丣.乤==乕::乷{什::乷}else{什::俆},..Settings1::DEFAULT};return
且(l,&乃);}佧(l,&丯[F::from(丣.乐)%丯.len()],丣.乤,)}fn 倲(l:&J,丯:&[Z],业:&[主],乪:主,个:F,)->J{let mut 买=l.clone();for 且 in 0..个{if 买.丢().is_some(){break;}let 丣=业.get(且).copied().unwrap_or(乪);买=侃(&买,&丈(&买,丯,丣));}买}fn
儯(l:&J,丯:&[Z],仱:O)->K<(主,K<L>)>{let mut 丅:K<(主,K<L>)> =K::new();if 仱{for 乤 in[乕::乷,乕::亭{丹:0,万:I::MAX,},]{let 丣=主{乐:俅,乤,};let c=丈(l,丯,丣);if!丅.iter().any(|(_,么)|*么==c){丅.push((丣,c));}}}for
乐 in 0..丯.len().min(F::from(I::MAX)){for&乤 in&偣{let 丣=主{乐:乐 as I,乤,};let c=丈(l,丯,丣);if!丅.iter().any(|(_,么)|*么==c){丅.push((丣,c));}}}丅}fn 僃(l:&J,丯:&[Z],乪:主,个:F,亠:F,亳:P<了>,临:&亿,)->K<主>{let
丂=l.东();let 伫=||亳.is_some_and(|t|了::now()>=t);let 乀=仮::of(l,临);let 伭=|业:&[主]|仼(&倲(l,丯,业,乪,个),丂,临,乀);let mut 专:(佀,K<主>)=(伭(&[]),K::new());let mut 乆:K<(J,K<主>)>
=vec![(l.clone(),K::new())];for 僙 in 0..个{let mut 亲:K<(佀,J,K<主>)> =K::new();let mut 乯:HashMap<丮,F> =HashMap::new();for(倧,业)in&乆{if 伫(){break;}for(丣,c)in
儯(倧,丯,临.仱){let 丄=侃(倧,&c);let mut 侎=业.clone();侎.push(丣);let v=仼(&丄,丂,临,乀);let 仼=估(&丄,丂);match 乯.get(&仼){Some(&乙)if 亲[乙].0>=v=>{}Some(&乙)=>亲[乙]=(v,丄,侎),None=>{乯.insert(仼,亲.len());亲.push((v,丄,侎));}}}}if
亲.is_empty(){break;}亲.sort_by(|a,b|b.0 .cmp(&a.0));亲.truncate(亠);let 乹=僙+1==个||伫();for(i,(_,_,业))in 亲.iter().take(if 乹{亠}else{1}).enumerate(){if i>0&&伫(){break;}let
v=伭(业);if v>专.0{专=(v,业.clone());}}if 乹{break;}乆=亲.into_iter().map(|(_,s,p)|(s,p)).collect();}专.1}fn 业(l:&J,丯:&[Z],乪:主,个:F,僾:F,亳:P<了>,临:&亿,)->K<主>{let 丂=l.东();let
乀=仮::of(l,临);let 伭=|业:&[主]|仼(&倲(l,丯,业,乪,个),丂,临,乀);let mut 专:(佀,K<主>)=(伭(&[]),K::new());let mut 亠=2;loop{let 乎=僃(l,丯,乪,个,亠,亳,临);let v=伭(&乎);if v>专.0{专=(v,乎);}let
伫=亳.is_some_and(|t|了::now()>=t);if 伫||(亳.is_none()&&亠>=僾){break;}亠*=2;}专.1}const 偹:F=4+12;fn 个(侢:impl BufRead,mut 作:impl Write,mut 侅:impl FnMut(&str,&str,Q,了)->乘,)->Result<(),乘>{let
mut 乇=俙{倶:侢};let Some(乸)=乇.line()?else{return Ok(());};let mut 仺=format!("{}\n",乸);乇.take(丙(&乸,"mine spot count")?,&mut 仺)?;let mut 且=0;loop{let Some(乸)=乇.line()?else{return
Ok(());};let 举=了::now();且+=1;let mut 二=format!("{}\n",乸);乇.take(偹-1,&mut 二)?;for 乜 in["building count","unit count"]{let line=乇.倸()?;let n=丙(&line,乜)?;二.push_str(&line);二.push('\n');乇.take(n,&mut
二)?;}let 侅=侅(&仺,&二,且,举);writeln!(作,"{}",侅).and_then(|()|作.flush()).map_err(|付|format!("turn {}: cannot write the answer: {}",且,付))?;}}struct 俙<R>{倶:R,}impl<R:BufRead>俙<R>{fn
line(&mut self)->Result<P<乘>,乘>{let mut line=乘::new();match self.倶.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(付)=>Err(format!("cannot read input: {}",付)),}}fn
倸(&mut self)->Result<乘,乘>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,二:&mut String)->Result<(),乘>{for
_ in 0..n{二.push_str(&self.倸()?);二.push('\n');}Ok(())}}fn 丙(line:&str,乜:&str)->Result<F,乘>{match line.trim().parse::<F>(){Ok(n)if n<=傅=>Ok(n),_=>Err(format!("bad {} {line:?}",乜)),}}const
傅:F=144;const 催:C=3;const 傀:C=2;const 傊:F=16;fn 儐(l:&J,丅:&mut Vec<L>){let 丁=l.东();let mut 些=信{s:l.clone(),丁,丷:1-丁,不:T[1-丁],体:T[丁],且:l.书(),丅,};些.run();}struct
信<'a>{s:J,丁:F,丷:F,不:F,体:F,且:Q,丅:&'a mut Vec<L>,}fn 佫(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侫(上:Q,q:F)->L{let(x,y)=佫(q);L::乶{上,x,y}}fn 偏(丆:I,q:F)->L{let(x,y)=佫(q);L::丼{丆,x,y}}impl
信<'_>{fn run(&mut self){let 一=self;if 一.佘(){return;}一.亷();if 一.佘(){return;}一.亂();一.亞();}fn 乂(&self,三:F,两:F)->C{C::from(self.s.map().丘(三,两))}fn 乄(&mut self,丒:L)->O{let
乥=self.s.乄(丒);if 乥{self.丅.push(丒);}乥}fn 伋(&self,丵:&J)->O{丵.丢()==Some(乲::互(self.丁))}fn 佘(&mut self)->O{let 一=self;if 一.s.丢().is_some(){return false;}for
q in 一.s.乺().iter(){if 一.乂(q,一.不)!=1{continue;}let Some((上,_))=一.s.亼(q)else{continue;};let 丒=侫(上,一.不);let mut 丵=一.s.clone();if 丵.乄(丒)&&一.伋(&丵){return 一.乄(丒);}}let
Some(丝)=一.働()else{return false;};let mut 丵=一.s.clone();if!丝.iter().all(|&丒|丵.乄(丒))||!一.伋(&丵){return false;}for 丒 in 丝{一.乄(丒);}true}fn 働(&self)->P<K<L>>{let
一=self;const 俏:C=C::MAX;let 事=一.s.丽().亖();let 丨:K<G> =(1..=事).map(|丆|一.s.乊(一.丁,丆)).collect();let 伡=|q:F|{丨.iter().position(|乑|乑.test(q)).map(|丠|(丠 as I+1,10*(丠
as C+1)))};let 七=一.s.七(一.丁);let mut 与=[俏;M];let mut 乌=[F::MAX;M];let mut 乥=[false;M];for q in 七.iter(){与[q]=0;}loop{let mut 丄:P<F> =None;for q in 0..M{if!乥[q]&&与[q]<俏&&丄.is_none_or(|n|与[q]<与[n]){丄=Some(q);}}let
Some(q)=丄 else{break;};乥[q]=true;if q==一.不{break;}for&也 in&一.s.map().丏[0][q]{if 也==亐{break;}let 也=F::from(也);if 乥[也]||七.test(也){continue;}let Some((_,介))=伡(也)else{continue;};if
与[q]+介<与[也]{与[也]=与[q]+介;乌[也]=q;}}}if 与[一.不]>一.s.下(一.丁){return None;}let mut 丝=K::new();let mut q=一.不;while!七.test(q){let(丆,_)=伡(q)?;丝.push(偏(丆,q));q=乌[q];}丝.reverse();Some(丝)}fn
乏(&self,乞:&J)->C{let(丂,仍)=(self.丁,self.丷);let 乌=&self.s;let 丙=|乑:G|C::from(乑.count());let 丛=丙(乞.丐(丂))-丙(乌.丐(丂));let 享=丙(乌.丐(仍))-丙(乞.丐(仍));let 侨=丙(乌.七(仍))-丙(乞.七(仍));let
何=乌.企(仍)-乞.企(仍);let 丹=丙(乌.丹()&乌.丐(仍))-丙(乞.丹()&乞.丐(仍));12*丛+20*享+8*(侨-享).max(0)+5*何+25*丹}fn 偂(&self,q:F)->C{let 一=self;let 乢=一.s.map().为.test(q);if 乢&&一.s.丽().但()&&!一.s.丐(一.丁).test(q){20}else{0}}fn
倌(&self,乒:C)->O{let 一=self;一.s.万(一.丷).iter().any(|q|一.乂(q,一.体)<=乒)}fn 亷(&mut self){let 一=self;let mut 万:K<(C,F)> =一.s.乺().iter().map(|q|(一.乂(q,一.不),q)).collect();万.sort_unstable();for(仄,q)in
万{if 一.s.丢().is_some(){return;}let Some((上,丆))=一.s.亼(q)else{continue;};let 丨=一.s.乊(一.丁,丆);let 也=一.s.map().丏[一.丁][q];let mut 专:P<(C,F)> =None;for&丄 in 也.iter().take_while(|&&丄|丄!=亐){let
丄=F::from(丄);if!丨.test(丄){continue;}let mut 丵=一.s.clone();if!丵.乄(侫(上,丄)){continue;}let 丬=if 一.伋(&丵){100000}else{一.乏(&丵)+10*(仄-一.乂(丄,一.不))+一.偂(丄)};if 专.is_none_or(|(亝,_)|丬>亝){专=Some((丬,丄));}}if
let Some((丬,丄))=专{if 丬>0{一.乄(侫(上,丄));}}}}fn 亂(&mut self){let 一=self;if 一.s.丢().is_some(){return;}if 一.倌(催){一.兂();}else{一.丹();}}fn 兂(&mut self){let 一=self;if!一.s.丽().仰()||一.s.下(一.丁)<于{return;}let
也=一.s.map().丏[一.丁][一.体];let 丑:K<F> =也.iter().take_while(|&&q|q!=亐).map(|&q|F::from(q)).collect();let 丸=一.s.丐(一.丁);if 丑.iter().any(|&q|丸.test(q)&&一.s.乓().test(q)){return;}for
q in 丑{let(x,y)=佫(q);if 一.乄(L::乿{x,y}){return;}}}fn 丹(&mut self){let 一=self;if!一.s.丽().但(){return;}while 一.s.下(一.丁)>=于{let 丧=一.s.map().为&一.s.丐(一.丁)&一.s.七(一.丁)&一.s.仗();let
Some(q)=丧.iter().min_by_key(|&q|(一.乂(q,一.体),q))else{return;};let(x,y)=佫(q);if!一.乄(L::京{x,y}){return;}}}fn 亞(&mut self){let 一=self;let 倹=if 一.且<=3{0}else{5};for
_ in 0..傊{if 一.s.丢().is_some(){return;}let 下=一.s.下(一.丁);if 下<10{return;}let 僶=一.倌(傀);let 乖=一.s.仉(一.丁);let mut 专:P<(C,L)> =None;let mut 侹:P<(C,L)> =None;let
mut 俸=G::U;for 丆 in 1..=一.s.丽().亖(){let 丨=一.s.乊(一.丁,丆);let 亙=(乖&丨).丕(俸);俸=丨;let 介=10*C::from(丆);if 介>下{continue;}for q in 亙.iter(){let 丒=偏(丆,q);let mut
丵=一.s.clone();if!丵.乄(丒){continue;}let 仾=丵.下(一.丁);let 丟=丵.丟(一.丁);let 何=一.s.企(一.丷)-丵.企(一.丷);let 傽=if 何>0&&僶{仾+丟>=0}else{丟>=0&&(仾>=倹||何>0)};if!傽{continue;}let
乏=一.乏(&丵);let 丬=乏+一.偂(q)-6*一.乂(q,一.不)-10*(C::from(丆)-1);if 乏>0{if 专.is_none_or(|(亝,_)|丬>亝){专=Some((丬,丒));}}else if 丟>=3&&仾>=倹+10&&丆==1{if 侹.is_none_or(|(亝,_)|丬>亝){侹=Some((丬,丒));}}}}let
Some((_,丒))=专.or(侹)else{return;};if!一.乄(丒){return;}}}}fn 僮(l:&J,丅:&mut Vec<L>){丅.clear();儐(l,丅);}const 亱:C=C::MAX;const 傉:F=8;struct 们{与:[C;M],乌:[I;M],}fn
佋(l:&J,丁:F,亷:O,不:F)->C{侓(l,丁,仡(l,丁,亷),不)}fn 侓(l:&J,丁:F,仡:G,不:F)->C{let mut 与=亱;倐(l,丁,仡,|丣,new|{if new.test(不){与=10*丣 as C;}与!=亱});与}fn 儘(l:&J,丁:F,仡:G,乍:C)->G{let
mut 乬=G::U;倐(l,丁,仡,|丣,new|{if 10*丣 as C>乍{return true;}乬|=new;false});乬.丕(l.七(丁))}fn 仡(l:&J,丁:F,亷:O)->G{if 亷&&丁==l.东(){l.乺()}else{G::U}}fn 倐(l:&J,丁:F,仡:G,mut
仨:impl FnMut(F,G)->O,){let 事=l.丽().亖();let mut 伡=[G::U;3];let mut 乯=G::U;for 丆 in 1..=事{let 乑=l.乊(丁,丆);伡[F::from(丆)-1]=乑.丕(乯);乯|=乑;}let mut 両=[l.七(丁),G::U,G::U,G::U];for
q in(仡&l.万(丁)).iter(){if let Some(丆)=l.亟(q).filter(|&丆|丆<=事){両[0]|=G::q(q).丏()&l.乊(丁,丆);}}let mut 乥=G::U;for 丣 in 0..{let new=両[丣%4].丕(乥);両[丣%4]=G::U;if
仨(丣,new){return;}乥|=new;let 丄=new.丏().丕(乥);for(仯,丑)in 伡.iter().enumerate(){両[(丣+仯+1)%4]|=丄&*丑;}if 両.iter().all(|乑|乑.丕(乥).is_empty()){break;}}}impl 们{fn
new(l:&J,丁:F,亷:O)->们{let 事=F::from(l.丽().亖());let mut 丨=[G::U;3];for(丠,乑)in 丨.iter_mut().enumerate().take(事){*乑=l.乊(丁,丠 as I+1);}let 七=l.七(丁);let 丏=&l.map().丏[0];let
mut 与=[亱;M];let mut 乌=[亐;M];let mut 両=[0u8;M];let mut 丙=0;let mut 倾=G::U;for q in 七.iter(){与[q]=0;両[丙]=q as I;丙+=1;}if 亷&&丁==l.东(){for 三 in l.乺().iter(){let
Some(丆)=l.亟(三)else{continue;};let 义=G::q(三).丏()&丨[F::from(丆)-1];for q in 义.iter(){if 与[q]!=0{与[q]=0;両[丙]=q as I;丙+=1;}}}}while 丙>0{let mut 专=0;for 丠 in
1..丙{let(a,b)=(F::from(両[丠]),F::from(両[专]));if(与[a],a)<(与[b],b){专=丠;}}let q=F::from(両[专]);丙-=1;両[专]=両[丙];倾.set(q);for&丄 in&丏[q]{if 丄==亐{break;}let 丄=F::from(丄);if
倾.test(丄)||七.test(丄){continue;}let Some(丆)=丨[..事].iter().position(|乑|乑.test(丄))else{continue;};let 偎=与[q]+10*(丆 as C+1);if 偎<与[丄]{if 与[丄]==亱{両[丙]=丄 as
I;丙+=1;}与[丄]=偎;乌[丄]=q as I;}}}们{与,乌}}fn path(&self,不:F)->K<F>{let mut path=K::new();if 不>=M||self.与[不]==亱{return path;}let mut q=不;while self.乌[q]!=亐{path.push(q);q=F::from(self.乌[q]);}path.reverse();path}}struct
伽{丈:K<L>,与:C,}fn 僽(l:&J,丁:F,q:F)->P<I>{(1..=l.丽().亖()).find(|&丆|l.乊(丁,丆).test(q))}fn 儛(丌:&mut J,丂:F,世:F,乍:C,丈:&mut Vec<L>,亜:&mut i64,)->O{loop{if 丌.丢()==Some(乲::互(丂)){return
true;}if 丌.丐(丂).test(世)&&丌.七(丂).test(世){return true;}let 丩=们::new(丌,丂,false);let 与=丩.与[世];if 与==亱||*亜+与>乍{return false;}let Some(&q)=丩.path(世).first()else{return
false;};let Some(丆)=僽(丌,丂,q)else{return false;};let 丒=L::丼{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丌.乄(丒){return false;}*亜+=10*C::from(丆);丈.push(丒);}}fn 俵(l:&J,丂:F,亃:&[F],乍:C)->P<伽>{let
mut 丌=l.clone();let mut 丈=K::new();let mut 亜=0;for&世 in 亃{if!儛(&mut 丌,丂,世,乍,&mut 丈,&mut 亜){return None;}}(亜<=乍).then_some(伽{丈,与:亜,})}fn 兄(l:&J,丂:F,不:F)->C{let
丰=l.map().valid;let 丐=l.丐(丂)&丰;let 倂=|mut 乁:G|loop{let 丄=乁|(乁.丏()&丐);if 丄==乁{return 乁;}乁=丄;};let mut 乁=倂(l.七(丂));let mut 义=0;while!乁.test(不){let new=乁.丏().丕(乁)&丰;if
new.is_empty(){return C::MAX/20;}乁=倂(乁|new);义+=1;}义}fn 侌(l:&J,不:F,乍:C)->P<伽>{if 乍<0||l.丢().is_some()||不>=M||l.map().乣.test(不){return None;}let 丂=l.东();if
l.丐(丂).test(不)&&l.七(丂).test(不){return Some(伽{丈:K::new(),与:0,});}if 10*兄(l,丂,不)>乍{return None;}let mut 专=俵(l,丂,&[不],乍);let 乸=们::new(l,丂,false);let mut 乓:K<F>
=(l.乓()&l.丐(1-丂)).iter().filter(|&乫|乫!=不&&乸.与[乫]!=亱).collect();乓.sort_by_key(|&乫|(l.map().丘(乫,不),乫));乓.truncate(傉);for 乫 in 乓{if let Some(丝)=俵(l,丂,&[乫,不],乍){if
专.as_ref().is_none_or(|亝|丝.与<亝.与){专=Some(丝);}}}专}const 傇:F=20000;struct 伾{丈:K<L>,与:C,丑:C,万:C,亂:C,丟:C,丱:C,}struct 佶{丑:C,万:C,亂:C,}fn 候(l:&J,丁:F)->佶{let 七=l.七(丁);let
万=l.万(丁)&七;let 丬=万.iter().filter_map(|q|l.亟(q)).map(|丆|10*C::from(丆)).sum();佶{丑:C::from(七.count()),万:丬,亂:C::from(((l.丹()|l.乓())&七).count()),}}struct 俥{丂:F,伍:G,乌:佶,丟:C,伀:F,乍:C,伄:F,path:K<L>,乎:K<伾>,index:HashMap<伐,F,BuildHasherDefault<DefaultHasher>>,}type
伐=[丮;3];fn 兎(mut 井:伐,q:F)->伐{井[q/64]|=1<<(q%64);井}impl 俥{fn 倉(&mut self,丌:&J,乹:P<F>,享:伐,与:C,)->O{let 一=self;let 丂=一.丂;let 亙=match 乹{None=>丌.仉(丂),Some(q)=>G::q(q).丏(),}&一.伍;let
亙=亙.丕(丌.丐(丂));let mut 丨=[G::U;3];for 丆 in 1..=丌.丽().亖(){丨[F::from(丆)-1]=丌.乊(丂,丆);}for q in 亙.iter(){let 事=F::from(丌.丽().亖());let Some(丠)=丨[..事].iter().position(|乑|乑.test(q))else{continue;};let
丆=丠 as I+1;let 介=10*C::from(丆);if 与+介>一.乍{continue;}if 一.伄>=傇{return false;}一.伄+=1;let mut 丄=丌.clone();let 丒=L::丼{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丄.乄(丒){continue;}一.path.push(丒);let
伈=兎(享,q);一.儙(&丄,伈,与+介);let 僗=一.path.len()<一.伀&&丄.丢().is_none();let 僭=!僗||一.倉(&丄,Some(q),伈,与+介);一.path.pop();if!僭{return false;}}true}fn 儙(&mut self,乞:&J,伈:伐,与:C){let
一=self;let 伪=1-一.丂;let 伩=候(乞,伪);let 丑=一.乌.丑-伩.丑;let 万=一.乌.万-伩.万;if 丑<=0&&万<=0{return;}let 亂=一.乌.亂-伩.亂;let 丟=乞.丟(一.丂)-一.丟;let 兑=乞.丢()==Some(乲::互(一.丂));let
丱=if 兑{C::MAX/2}else{3*丑+万+10*亂+2*丟-与};if let Some(&乙)=一.index.get(&伈){if 与>=一.乎[乙].与{return;}}let 串=伾{丈:一.path.clone(),与,丑,万,亂,丟,丱,};match 一.index.get(&伈){Some(&乙)=>一.乎[乙]=串,None=>{一.index.insert(伈,一.乎.len());一.乎.push(串);}}}}fn
倇(l:&J,伀:F,乍:C,乒:F)->K<伾>{乡(l,伀,乍,乒).0}fn 乡(l:&J,伀:F,乍:C,乒:F)->(K<伾>,F){if l.丢().is_some()||伀==0||乒==0{return(K::new(),0);}let 丂=l.东();let 伪=1-丂;let mut
侵=俥{丂,伍:l.丐(伪)|l.丐(伪).丏(),乌:候(l,伪),丟:l.丟(丂),伀,乍,伄:0,path:K::new(),乎:K::new(),index:HashMap::default(),};侵.倉(l,None,[0;3],0);let 伄=侵.伄;let mut 乎=侵.乎;乎.sort_by(|a,b|b.丱.cmp(&a.丱).then(a.与.cmp(&b.与)));乎.truncate(乒);(乎,伄)}const
佄:C=1000000;const 傈:C=200;fn 丶(l:&J,丁:F)->C{let 七=l.七(丁);let 万:C=(l.万(丁)&七).iter().filter_map(|q|l.亟(q)).map(|丆|10*C::from(丆)).sum();l.下(丁)+3*l.丟(丁)+4*C::from(七.count())+万+10*C::from((l.乓()&七).count())}fn
倡(l:&J,丁:F,乻:G)->C{僫(l,丁,侓(l,丁,乻,T[1-丁]),)}fn 僫(l:&J,丁:F,与:C)->C{if 与==亱{return C::MIN;}l.下(丁)+l.丟(丁)-与}fn 倳(亨:C)->C{亨.clamp(-傈,0)}fn 僡(l:&J,丁:F,乻:[G;2],)->(C,[C;2]){let
么=1-丁;match l.丢(){Some(乲::互(免))=>{return(if 免==丁{佄}else{-佄},[0,0])}Some(乲::俍)=>return(0,[0,0]),None=>{}}let mut 丱=丶(l,丁)-丶(l,么);if 丁==l.东(){let mut 丄=l.clone();丄.件();if
丄.丢().is_none()&&佋(&丄,么,true,T[丁])<=丄.下(么){丱-=佄/2;}}let 仿=[倡(l,丁,乻[0]),倡(l,么,乻[1]),];(丱+倳(仿[0])-倳(仿[1]),仿)}
