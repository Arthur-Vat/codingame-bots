// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伸=u16;type Q=u32;type 丮=u64;type 伷=i32;type C=i64;type 亻=f64;type O=bool;type 乖=String;type 亂=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
健:O=true;fn 僑(l:&J,q:F,伲:C,丩:G,东:C)->O{let 丂=l.丛();let 主=1-丂;let 倌=健&&东<=l.下(主)+2*l.丞(主).max(0);let 伈=G::q(q)|G::q(q).丏();if!倌&&(伈&丩).is_empty(){return
false;}let(x,y)=乓(q);let mut 乜=l.clone();if!乜.乃(L::乼{x,y}){return false;}if 伱(&乜,丂)<伲{return true;}倌&&佉(&乜,主,false,U[丂])>东}const 傢:O=false;const 傁:I=2;const
保:I=1;const 俆:I=2;const 佀:F=0;const 佰:F=2;const 傈:O=false;const 傊:O=false;const 傉:O=false;const 傇:O=false;const 偬:F=4;const 俄:F=5;#[derive(Clone)]enum
丫{从(L),事(K<L>),}impl 丫{fn 丈(&self)->&[L]{match self{丫::从(丑)=>std::slice::from_ref(丑),丫::事(丈)=>丈,}}}fn 乓(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 倆(l:&J,丄:&mut
Vec<丫>){丄.clear();if l.両().is_some(){return;}let 丂=l.丛();let 亅=l.丼().亓();let mut 丨=[G::T;4];for 丅 in 1..=亅{丨[F::from(丅)]=l.乇(丂,丅);}for 三 in l.乷().iter(){let
Some((上,丅))=l.人(三)else{continue;};let 乑=G::q(三).丏()&丨[F::from(丅)];for 丣 in 乑.iter(){let(x,y)=乓(丣);丄.push(丫::从(L::乳{上,x,y}));}}let 下=l.下(丂);let 丆=l.丆(丂);let
亀=l.仆(丂);let 乭=match 傁{0=>亀,1=>{let 伔=l.map().valid.业(丆).丏()&丆;亀.业(丆)|伔}_=>亀.业(丆),};for 丅 in 1..=亅{let 不=10*C::from(丅);if 下<不{break;}let mut 丐=乭&丨[F::from(丅)];for
僚 in 1..丅{丐=丐.业(丨[F::from(僚)]);}for q in 丐.iter(){let(x,y)=乓(q);丄.push(丫::从(L::丹{丅,x,y}));}}if 下>=争{let 並=l.丒(丂)&亀&l.仓();if l.丼().佄(){for q in(並&l.map().丷).iter(){let(x,y)=乓(q);丄.push(丫::从(L::亩{x,y}));}}if
l.丼().仪(){let 伔=l.map().valid.业(l.丒(丂)).丏();let 丐=(並&伔).业(l.map().丷);let 丩=丩(l,1-丂);let 伲=C::from((丩&l.丆(丂)).count());let 东=佉(l,1-丂,false,U[丂]);let 丠=if
傢{let(亥,佐)=么(l,丂);亥.业(佐)}else{G::T};for q in 丐.iter().filter(|&q|{let 伈=G::q(q)|G::q(q).丏();!(伈&丠).is_empty()||僑(l,q,伲,丩,东)}){let(x,y)=乓(q);丄.push(丫::从(L::乼{x,y}));}}}let
僐=match 保{1=>争,2=>10,_=>C::MAX,};if 下>=僐{let 丩=丩(l,1-丂);let 五=l.map().valid.业(l.丒(丂)|l.丒(1-丂));for 三 in(l.乷()&丩).iter(){let Some((上,丅))=l.人(三)else{continue;};let
乑=G::q(三).丏()&丨[F::from(丅)]&五;let(他,仗)=乓(三);let 侕=if 保==1{if l.map().丷.test(三){continue;}L::乼{x:他,y:仗}}else{L::丹{丅:1,x:他,y:仗,}};for 丣 in 乑.iter(){let(x,y)=乓(丣);丄.push(丫::事(vec![L::乳{上,x,y},侕]));}}}if
俆!=0&&下>=争&&l.丼().仪(){僈(l,&丨,丄);}if 佀!=0&&下>=20{僺(l,丨[1],下,丄);}if 傇{傾(l,丨[1],下,丄);}for 为 in 俻(l,俄,下,偬){if 为.丈.len()>1{丄.push(丫::事(为.丈));}}if let Some(东)=侄(l,U[1-丂],下){if
东.丈.len()>1{丄.push(丫::事(东.丈));}}}fn 僈(l:&J,丨:&[G;4],丄:&mut Vec<丫>){let 丂=l.丛();let 主=1-丂;let 丩=丩(l,主);let 两=l.丒(丂);let 世=l.丒(主);let 伔=世.丏();let 丷=l.map().丷;for
三 in(l.乷()&丩&l.丆(丂)).iter(){if 丷.test(三){continue;}let Some((上,丅))=l.人(三)else{continue;};let 傪=l.map().丗(三,U[主]);let 乑=G::q(三).丏()&丨[F::from(丅)];let(他,仗)=乓(三);for
丣 in 乑.iter(){let 佼=match 俆{1=>两.test(丣)&&l.map().丗(丣,U[主])>傪,2=>两.test(丣)&&伔.test(丣),_=>两.test(丣)||(世.test(丣)&&丣!=U[主]),};if!佼{continue;}let(x,y)=乓(丣);丄.push(丫::事(vec![L::乳{上,x,y},L::乼{x:他,y:仗},]));}}}fn
僺(l:&J,依:G,下:C,丄:&mut Vec<丫>){let 丂=l.丛();let 严=l.map();let 五=严.valid.业(l.丒(丂)|l.丒(1-丂));let 丠=五&依;let 乭:K<F> =(l.丆(丂).丏()&丠).iter().collect();if 乭.is_empty(){return;}let
侥=|与:F|{乭.iter().copied().min_by_key(|&q|(严.丗(q,与),q))};let mut 侅:K<(F,F)> =K::new();if 傈{for 丕 in 僻(l,&乭){if let Some(丽)=侥(丕){侅.push((丽,丕));}}}else{let
与=U[1-丂];let mut 侲=乭.clone();侲.sort_by_key(|&q|(严.丗(q,与),q));侲.truncate(佰);侅.extend(侲.into_iter().map(|丽|(丽,与)));}let 任=F::try_from(下/10).unwrap_or(0).min(佀);for(丽,与)in
侅{let mut 东=vec![丽];let mut 亨=G::q(丽);while 东.len()<任{let 乶=东[东.len()-1];if 乶==与{break;}let 七=(G::q(乶).丏()&丠).业(亨).iter().min_by_key(|&q|(严.丗(q,与),q));let
Some(七)=七 else{break;};东.push(七);亨.set(七);let 丈=东.iter().map(|&q|{let(x,y)=乓(q);L::丹{丅:1,x,y}}).collect();丄.push(丫::事(丈));}if 傉{僗(l,&东,丄);}}}fn 僗(l:&J,东:&[F],丄:&mut
Vec<丫>){let 丂=l.丛();let 丽=东[0];let 乸=l.乷()&l.万(丂)&G::q(丽).丏();for 三 in 乸.iter(){let Some((上,丅))=l.人(三)else{continue;};if!l.乇(丂,丅).test(丽){continue;}let(x,y)=乓(丽);let
mut 丈=vec![L::乳{上,x,y}];for&q in&东[1..]{let(x,y)=乓(q);丈.push(L::丹{丅:1,x,y});丄.push(丫::事(丈.clone()));}}}fn 僻(l:&J,乭:&[F])->K<F>{let 丂=l.丛();let 严=l.map();let(乄,中)=住(l);let
两:K<F> =l.万(丂).iter().collect();let 世:K<F> =l.万(1-丂).iter().collect();let 低=|丕:F|{乭.iter().map(|&q|严.丗(q,丕)).min().unwrap_or(I::MAX)};let 傸=|丕:F|{let 丩=低(丕);两.iter().any(|&亹|{let
d=严.丗(亹,丕);d<=丩&&世.iter().all(|&主|d<严.丗(主,丕))})};let 侠=|丕:F|{傊&&(l.丒(1-丂).test(丕)||世.iter().any(|&主|严.丗(主,丕)<低(丕)))};let 丠=|丐:G|->K<F>{let mut 乿:K<F> =丐.业(l.丒(丂)).iter().filter(|&丕|低(丕)!=I::MAX&&!傸(丕)&&!侠(丕)).collect();乿.sort_by_key(|&丕|(低(丕),丕));乿};let
mut 乿=丠(中);if 乿.is_empty(){乿=丠(乄);}乿.truncate(佰);乿}fn 傺(l:&J)->K<F>{let 丂=l.丛();let 严=l.map();let(乄,中)=住(l);let 丆=l.丆(丂);let 两=l.丒(丂);let 丩=丩(l,1-丂);let
世:K<F> =l.万(1-丂).iter().collect();let 倄=|q:F|{丆.iter().map(|a|严.丗(a,q)).min().unwrap_or(I::MAX)};let mut 丐:K<F> =(乄|中).iter().filter(|&q|{if 两.test(q){丆.test(q)&&丩.test(q)}else{!l.丒(1-丂).test(q)&&世.iter().any(|&e|严.丗(e,q)<倄(q))}}).collect();丐.sort_by_key(|&q|(I::from(!中.test(q)),倄(q),q));丐.truncate(
2*佰);丐}fn 傾(l:&J,依:G,下:C,丄:&mut Vec<丫>){let 丂=l.丛();let 严=l.map();let 五=严.valid.业(l.丒(丂)|l.丒(1-丂));let 丠=五&依;let 丆=l.丆(丂);let 並=丆&l.仓();let 僙=if 佀==0{3}else{佀};for
q in 傺(l){if!l.丒(丂).test(q){let 丽=(丆.丏()&丠).iter().min_by_key(|&c|(严.丗(c,q),c));if let Some(丽)=丽{let mut 东=vec![丽];let mut 亨=G::q(丽);while 东[东.len()-1]!=q&&东.len()<僙{let
乶=东[东.len()-1];let 七=(G::q(乶).丏()&丠).业(亨).iter().min_by_key(|&c|(严.丗(c,q),c));let Some(七)=七 else{break;};东.push(七);亨.set(七);}let 不=10*C::try_from(东.len()).unwrap_or(C::MAX);if
东.len()>=2&&东[东.len()-1]==q&&不<=下{let 丈=东.iter().map(|&c|{let(x,y)=乓(c);L::丹{丅:1,x,y}}).collect();丄.push(丫::事(丈));}}}if 下>=争&&l.丼().仪(){let 丷=(G::q(q)|G::q(q).丏())&並;for
乧 in 丷.业(严.丷).iter(){let(x,y)=乓(乧);丄.push(丫::从(L::乼{x,y}));}}}}const 偹:I=2;const 傖:C=5;fn 乸(l:&J,丁:F)->G{侤(l,丁,偹)}fn 侤(l:&J,丁:F,三:I)->G{let mut 乸=G::T;if
三>3{return 乸;}for q in(l.万(丁)&l.丆(丁)).iter(){if l.亜(q).is_some_and(|丅|丅>=三){乸|=G::q(q);}}乸}fn 丩(l:&J,临:F)->G{let 乏=l.下(临)+l.丞(临);僳(l,临,乸(l,临),乏)}fn 亣(l:&J,丁:F)->G{丩(l,1-丁)&l.丆(丁)}fn
伱(l:&J,丁:F)->C{C::from(亣(l,丁).count())}const 俒:C=4;fn 僞(丐:C)->C{丐/俒}fn 丶(l:&J,丁:F,亣:G)->C{let 丆=l.丆(丁);let 丶=(l.丶()&丆).业(亣);let 丐=C::from(丆.count());let
侀=if l.乨()<=偕{丐/偔+偓}else{僞(丐)};侀*C::from(丶.count())}const 偕:Q=15;const 偓:C=6;const 偔:C=俒;const 傏:C=2;const 佱:I=1;const 傑:I=3;const 傐:I=6;fn 僖(仃:I,亀:I,伝:I,d:I)->I{match
仃{0=>4,1 if d<=亀=>4,1=>0,_ if d<=亀=>4,_ if d>=伝=>0,_=>(4*Q::from(伝-d)/Q::from(伝-亀))as I,}}fn 偅(l:&J,丁:F)->C{let 丆=l.丆(丁);let 主=if 佱==0{G::T}else{l.丆(1-丁)};let[a,b,c]=丆.伵();let[e,f,g]=主.伵();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丁 as 丮).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丟=(hash as F)&(俦-1);傣.with(|侂|{let mut 侂=侂.borrow_mut();let 仉=&mut 侂[丟];if 仉.0==丆&&仉.1==主&&仉.2==丁 as I+1{return 仉.3;}let 丬=if 佱==0{儦(丆,U[丁])}else{let d=丳(l,主);let
mut 乍=[0u8;M];for(w,&乑)in 乍.iter_mut().zip(d.iter()){*w=僖(佱,傑,傐,乑);}偆(丆,U[丁],&乍)};*仉=(丆,主,丁 as I+1,丬);丬})}const 俦:F=1<<12;thread_local!{static 傣:std::cell::RefCell<K<(G,G,I,C)>>
=std::cell::RefCell::new(vec![(G::T,G::T,0,0);俦]);}fn 儦(丆:G,了:F)->C{偆(丆,了,&[4;M])}fn 偆(丆:G,了:F,乍:&[I;M])->C{struct 伉<'a>{乍:&'a[I;M],丆:G,乯:[I;M],乙:[I;M],size:[I;M],介:I,亴:C,任:C,}fn
仡(d:&mut 伉<'_>,v:F)->I{d.介+=1;let 乯=d.介;d.乯[v]=乯;let mut 乙=乯;let mut 侮=1u8;let mut 为=0u8;for w in(G::q(v).丏()&d.丆).iter(){if d.乯[w]==0{let 倸=仡(d,w);侮+=倸;乙=乙.min(d.乙[w]);if
d.乙[w]>=乯{为+=倸;}}else{乙=乙.min(d.乯[w]);}}d.乙[v]=乙;d.size[v]=侮;if 为>0&&d.乯[v]>1{let w=C::from(d.乍[v]);d.亴+=w;d.任=d.任.max(w*C::from(为));}侮}if!丆.test(了){return
0;}let mut d=伉{乍,丆,乯:[0;M],乙:[0;M],size:[0;M],介:0,亴:0,任:0,};仡(&mut d,了);(d.亴+d.任)/4}fn 僽(l:&J,丁:F)->C{傏*(偅(l,1-丁)-偅(l,丁))}fn 俼(l:&J,丁:F)->C{if l.下(丁)+l.丞(丁)>=0{return
0;}(l.万(丁)&l.丆(丁)).iter().filter_map(|q|l.亜(q)).map(|丅|10*C::from(丅)).sum()}const 偫:C=15;const 傞:[C;2]=[3,1];const 傜:[C;2]=[4,20];fn 儐(l:&J,丁:F)->[C;2]{let
mut 丘=[0;2];for q in(l.万(丁)&l.丆(丁)).iter(){match l.亜(q){Some(2)=>丘[0]+=1,Some(3)=>丘[1]+=1,_=>{}}}丘}fn 倷(l:&J,丁:F)->C{let[倾,儚]=儐(l,丁);let 修=|丅:F|(3-傞[丅])*傜[丅];-偫*倾+修(0)*倾+修(1)*儚}const
偤:C=0;const 偧:C=2;const 偦:C=100;fn 倰(亦:C)->C{if 亦<0{return 0;}偤+偧*亦.min(偦)/2}const 俑:I=2;const 俍:C=2;const 俌:C=4;const 偨:I=3;fn 亲(l:&J,丁:F)->C{if 俍==0{return
0;}let 乁=l.map().valid;let mut 万=G::T;for q in(l.万(丁)&l.丆(丁)).iter(){if l.亜(q).is_some_and(|丅|丅<=偨){万|=G::q(q);}}let mut 代=乁.业(l.丒(丁));let mut 什=G::T;let
mut 乑=0;for 丗 in 1..=俌{代=(代|代.丏())&乁;let 仁=(万&代).业(什);乑+=(丗-1)*C::from(仁.count());什|=仁;}乑+=俌*C::from(万.业(什).count());俍*乑}const 俥:C=8;fn 么(l:&J,丁:F)->(G,G){let
丆=l.丆(丁);let 亥=丆&l.丒(1-丁).丏();let 乒=l.乒()&丆;(亥,亥&(乒|乒.丏()))}fn 偃(l:&J,丁:F)->C{if 俥==0{return 0;}俥*C::from(么(l,丁).1 .count())}const 佬:C=0;const 俉:I=1;const
俊:C=4;const 佭:C=0;const 俈:C=0;const 偛:Q=14;const 做:C=2;const 俤:C=0;const 傡:Q=14;fn 仢(l:&J,丁:F)->C{if 俤==0||l.乨()>傡{return 0;}let 两=丳(l,l.丆(丁));let 世=丳(l,l.丆(1-丁));let
mut 仜=0;for c in l.map().valid.iter(){match 两[c].cmp(&世[c]){std::cmp::Ordering::Less=>仜+=1,std::cmp::Ordering::Greater=>仜-=1,std::cmp::Ordering::Equal=>{}}}俤*仜}struct
俋{valid:G,乄:G,中:G,}thread_local!{static 停:std::cell::RefCell<P<俋>> =const{std::cell::RefCell::new(None)};}fn 住(l:&J)->(G,G){停.with(|q|{let mut 乂=q.borrow_mut();let
乁=l.map().valid;if 乂.as_ref().is_none_or(|z|z.valid!=乁){let 企=企(l);let mut 乄=G::T;for c in 乁.iter(){if 企[c]<=做{乄.set(c);}}*乂=Some(俋{valid:乁,乄,中:仚(l),});}乂.as_ref().map_or((G::T,G::T),|z|(z.乄,z.中))})}fn
佐(l:&J,丁:F)->G{let 丆=l.丆(丁);let 乒=l.乒()&丆;(乒|乒.丏())&丆}fn 丕(l:&J,丁:F)->C{if(佬==0&&俊==0&&佭==0&&俈==0)||l.乨()>偛{return 0;}let(乄,中)=住(l);let mut 亚=0;if 佬!=0{let
两=丳(l,l.丆(丁));let 世=丳(l,l.丆(1-丁));let mut 仜=0;for c in 乄.iter(){let(a,b)=(两[c],世[c]);if a!=I::MAX&&a.saturating_add(俉)<=b{仜+=1;}else if b!=I::MAX&&b.saturating_add(俉)<=a{仜-=1;}}亚+=佬*仜;}let
丘=|乎:G|C::from(乎.count());let(丵,世)=(l.丆(丁),l.丆(1-丁));亚+=俊*(丘(丵&乄)-丘(世&乄));if 佭!=0{亚+=佭*(丘(佐(l,丁)&乄)-丘(佐(l,1-丁)&乄));}亚+=俈*(丘(丵&中)-丘(世&中));亚}fn 僄(l:&J,丁:F)->C{let
乸=[侤(l,丁,俑),侤(l,1-丁,俑),];let(丰,仸)=僅(l,丁,乸);let 丰=丰+倰(仸[0])-倰(仸[1]);if l.両().is_some(){return 丰;}let 两=亣(l,丁);let 世=亣(l,1-丁);let 伲=C::from(世.count())-C::from(两.count());丰-2*俼(l,丁)+2*俼(l,1-丁)+僽(l,丁)+倷(l,丁)-倷(l,1-丁)-亲(l,丁)+亲(l,1-丁)+傖*伲/4+丶(l,丁,两)-丶(l,1-丁,世)+偃(l,丁)-偃(l,1-丁)+丕(l,丁)+仢(l,丁)}const
偵:F=8;const 傄:F=3;const 偭:F=32;const 傅:丮=300;const 偑:亻=0.75;const 偡:P<Q> =Some(7);const 偟:O=false;const 偞:C=10;const 偠:Q=6;struct 享{亸:P<亂>,付:P<亂>,伢:丮,}#[derive(Default)]struct
佶{丈:K<L>,侊:F,亱:丮,丝:F,义:F,俶:F,交:F,亁:O,倴:O,仇:F,}const 傠:O=true;#[derive(Clone)]struct 亄{丈:K<L>,侍:C,}struct 以{丈:K<L>,end:J,first:P<丮>,丧:P<K<亄>>,}type 伊=HashMap<丮,C,BuildHasherDefault<DefaultHasher>>;struct
佴{丂:F,limit:享,亱:丮,伟:伊,仠:P<K<丫>>,侴:O,乘:P<令>,}const 佲:C=0;const 俁:O=true;const 偌:O=false;const 俀:P<I> =None;fn 傯(l:&J,丝:&[亄],丄:&mut Vec<丫>){if!俁&&俀.is_none(){return;}let
伭=丝.iter().map(|乹|乹.丈.as_slice());傰(l,伭,(俁,偌,俀),丄,);}fn 傰<'a>(l:&J,伭:impl Iterator<Item=&'a[L]>,(乒,丏,丅):(O,O,P<I>),丄:&mut Vec<丫>,){let 丂=l.丛();let 丵=l.丆(丂);let
mut 伛=G::T;for 乹 in 伭{for 丑 in 乹{if let L::丹{x,y,..}|L::乳{x,y,..}=*丑{let q=F::from(y)*Y+F::from(x);if 丵.test(q){伛|=G::q(q);}}}}if 伛.is_empty(){return;}let
並=l.仓()&丵;let 下=l.下(丂);let 倥=|丑:L,丄:&mut Vec<丫>|{if!丄.iter().any(|a|a.丈()==[丑]){丄.push(丫::从(丑));}};if 乒&&下>=争&&l.丼().仪(){let mut 丐=伛;if 丏{丐|=伛.丏();}for
q in(丐&並).业(l.map().丷).iter(){let(x,y)=乓(q);倥(L::乼{x,y},丄);}}if let Some(丅)=丅.filter(|&丅|丅<=l.丼().亓()){if 下>=10*C::from(丅){for q in(伛&並).iter(){let(x,y)=乓(q);倥(L::丹{丅,x,y},丄);}}}}fn
佾(l:&mut J,丈:&[L])->O{丈.iter().all(|&丑|l.乃(丑))}fn 互(丈:&[L])->丮{let mut hash:丮=0xcbf29ce484222325;for 丑 in 丈{let(佔,a,b,c)=match*丑{L::乳{上,x,y}=>(1,丮::from(上),x,y),L::丹{丅,x,y}=>(2,丮::from(丅),x,y),L::亩{x,y}=>(3,0,x,y),L::乼{x,y}=>(4,0,x,y),};let
伇=佔|a<<8|丮::from(b)<<40|丮::from(c)<<48;hash=(hash^伇).wrapping_mul(0x0100000001b3);}hash}fn 俩(买:&J,乹:&[L])->J{let mut 七=买.clone();七.仯();for&丑 in 乹{七.乃(丑);}七.仯();七}fn
佖(l:&J)->P<K<L>>{let 丂=l.丛();let 与=U[1-丂];let 伆=|串:&J|串.両()==Some(乮::亏(丂));for q in l.乷().iter(){if l.map().丗(q,与)!=1{continue;}let Some((上,_))=l.人(q)else{continue;};let(x,y)=乓(与);let
丑=L::乳{上,x,y};let mut 串=l.clone();if 串.乃(丑)&&伆(&串){return Some(vec![丑]);}}let 东=侄(l,与,l.下(丂))?;let mut 串=l.clone();(佾(&mut 串,&东.丈)&&伆(&串)).then_some(东.丈)}fn
倶(l:&J,丁:F,个:C)->O{let 不=例(l,丁,l.万(丁),U[1-丁]);不!=亮&&l.下(丁)+个*l.丞(丁)>=不}fn 儏(l:&J)->O{let 丂=l.丛();if 佲==0||l.両().is_some(){return false;}let mut 七=l.clone();七.仯();let
伱=七.両().is_none()&&佉(&七,1-丂,true,U[丂])<=七.下(1-丂);!伱&&倶(l,丂,佲)}impl 佴{fn 俰(&self,买:&J)->O{self.侴&&买.両()!=Some(乮::亏(self.丂))&&!倶(买,self.丂,佲)}fn 亙(&self)->O{self.亱>=self.limit.伢||self.limit.亸.is_some_and(|t|亂::now()>=t)}fn
丰(&mut self,买:&J,丝:&[亄],侁:C)->C{self.亱+=1;self.侪(买,丝,侁)}fn 侪(&self,买:&J,丝:&[亄],侁:C)->C{if 买.両()==Some(乮::亏(self.丂)){return 佂;}if 丝.is_empty(){return self.value(买);}let
mut 佤=C::MAX;for 乹 in 丝{佤=佤.min(self.value(&俩(买,&乹.丈)));if 佤<=侁{break;}}佤}fn value(&self,l:&J)->C{let 伓=僄(l,self.丂);match&self.乘{Some(乘)if l.両().is_none()=>伓+乘.儕(l,self.丂),_=>伓,}}fn
俨(&self,乛:&mut Vec<丫>){if let Some(乘)=&self.乘{for 佛 in&乘.仇{乛.push(丫::事(佛.clone()));}}}#[allow(clippy::too_many_arguments)]fn 僪(&mut self,乾:&J,仮:C,乛:&[丫],乞:&[(C,F)],乣:&mut[O],丝:&[亄],仰:&mut
伊,供:O,)->(P<(F,J,C)>,O){let mut 专=None;let mut 但=仮;for&(佘,丟)in 乞{if 乣[丟]{continue;}if 供&&佘!=C::MAX&&佘<=但-仮{return(专,true);}if self.亙(){return(专,false);}乣[丟]=true;let
丈=乛[丟].丈();let mut 七=乾.clone();if!佾(&mut 七,丈)||self.俰(&七){仰.insert(互(丈),C::MIN/4);continue;}let 丰=self.丰(&七,丝,但);仰.insert(互(丈),丰-仮);if 丰>但{但=丰;专=Some((丟,七,丰));}}(专,false)}fn
侉(&mut self,了:&J,丝:&[亄],伒:&[丮],傳:O)->以{let 一=self;let mut 乵=None;let mut 乾=了.clone();let mut 丈=K::new();let mut 仮=一.丰(&乾,丝,C::MIN);let mut 仰=伊::default();let
mut 乛=K::new();let mut 乞=K::new();while 乾.両().is_none()&&!一.亙(){if 丈.is_empty()&&傳{match&一.仠{Some(仠)=>乛.clone_from(仠),None=>{倆(&乾,&mut 乛);if let Some(乘)=一.乘.as_ref().filter(|g|g.倝){乛.push(丫::事(乘.丙.clone()));}一.俨(&mut
乛);一.仠=Some(乛.clone());}}}else{倆(&乾,&mut 乛);一.俨(&mut 乛);}傯(&乾,丝,&mut 乛);乞.clear();for(丟,亐)in 乛.iter().enumerate(){let k=互(亐.丈());let 佘=仰.get(&k).or(一.伟.get(&k)).copied();乞.push((佘.unwrap_or(C::MAX),丟));}乞.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乣=vec![false;乛.len()];if 丈.is_empty(){for(丟,亐)in 乛.iter().enumerate(){乣[丟]=伒.contains(&互(亐.丈()));}}let mut 专=None;for
供 in[true,false]{let(之,儅)=一.僪(&乾,仮,&乛,&乞,&mut 乣,丝,&mut 仰,供,);专=之;if 专.is_some()||!儅{break;}}let Some((丟,七,丰))=专 else{break;};if 丈.is_empty(){乵=Some(互(乛[丟].丈()));}丈.extend_from_slice(乛[丟].丈());乾=七;仮=丰;}for(k,乌)in
仰{let 倉=一.伟.entry(k).or_insert(乌);*倉=(*倉).max(乌);}以{丈,end:乾,first:乵,丧:None,}}fn 伭(&self,买:&J)->K<K<L>>{let mut 七=买.clone();七.仯();let mut 丄=K::new();if
七.両().is_some(){return 丄;}let 井=七.丛();let 下=七.下(井);if let Some(东)=侄(&七,U[self.丂],下){丄.push(东.丈);}for 为 in 俻(&七,俄,下,傄){丄.push(为.丈);}let mut 倈=K::new();像(&七,&mut
倈);丄.push(倈);let mut 世=佴{丂:井,limit:享{伢:傅,..self.limit},亱:0,伟:伊::default(),仠:None,侴:false,乘:None,};丄.push(世.侉(&七,&[],&[],true).丈);丄.retain(|丈|!丈.is_empty());丄}fn
俪(&self,买:&J)->K<亄>{self.伭(买).into_iter().map(|丈|{let 侍=self.value(&俩(买,&丈));亄{丈,侍}}).collect()}fn 交(&mut self,了:&J,亐:&丫,丝:&[亄])->P<以>{let mut 乜=了.clone();if!佾(&mut
乜,亐.丈())||self.俰(&乜){return None;}self.亱+=1;let 乱=self.侉(&乜,丝,&[],false);let mut 丈=亐.丈().to_vec();丈.extend_from_slice(&乱.丈);Some(以{丈,end:乱.end,first:Some(互(亐.丈())),丧:None,})}fn
倁(&self,丙:&以,丝:&[亄])->C{let 丰=self.侪(&丙.end,丝,C::MIN);match&丙.丧{Some(丧)if!丧.is_empty()=>丰.min(self.侪(&丙.end,丧,C::MIN)),_=>丰,}}}fn 僵(之:&[亄],丝:&mut Vec<亄>)->O{let
乊:K<_> =丝.iter().map(|r|r.丈.clone()).collect();for 乹 in 之{if 丝.iter().any(|r|r.丈==乹.丈){continue;}丝.push(乹.clone());}丝.sort_by_key(|乹|乹.侍);丝.truncate(偵);丝.iter().map(|r|&r.丈).ne(&乊)}fn
儙(l:&J,乐:享,亁:&mut 亡)->佶{债(l,乐,亁,None)}fn 债(l:&J,乐:享,亁:&mut 亡,乘:P<令>,)->佶{let 丂=l.丛();let mut 举=佶::default();if l.両().is_some(){return 举;}if let Some(丈)=佖(l){举.丈=丈;return
举;}let 乗=|僿:亻|match(乐.亸,乐.付){(Some(亸),Some(付))=>Some(亸+(付-亸).mul_f64(僿)),_=>None,};if 偡.is_some_and(|且|l.乨()>=且){亁.end();}if 亁.僮(l){let 丈=亁.且(l,乗(0.4));let
傲=丈.iter().any(|丑|{matches!(丑,L::丹{..}|L::亩{..}|L::乼{..})});let 亲=偟&&!傲&&l.乨()>=偠&&l.下(丂)>=偞;if!亲{举.亁=true;举.丈=丈;return 举;}亁.end();}let mut 也=佴{丂,limit:享{亸:乗(偑),..乐},亱:0,伟:伊::default(),仠:None,侴:儏(l),乘,};let
mut 丝=K::new();let mut 义:K<以> =K::new();let mut 伒=K::new();let mut 佌=false;let mut 交:K<丫> =K::new();let mut 丽=也.乘.as_ref().filter(|g|g.start).map(|g|丫::事(g.丙.clone()));while
举.侊<偭&&(义.is_empty()||!也.亙()){let 儊=match 丽.take(){Some(亐)=>也.交(l,&亐,&丝),None=>None,};let 丙=if let Some(丙)=儊{丙}else if 佌{let Some(亐)=交.pop()else{break;};match
也.交(l,&亐,&丝){Some(丙)=>丙,None=>continue,}}else{也.侉(l,&丝,&伒,true)};举.侊+=1;let 倬=义.iter().any(|p|p.丈==丙.丈);let 乵=丙.first;let 倳=也.亙();let mut 丙=丙;let mut 倖=false;if!倳&&!倬{let
之=也.俪(&丙.end);倖=僵(&之,&mut 丝);丙.丧=Some(之);}if!倬{义.push(丙);}if 倳{break;}if!倖&&!佌{match 乵{Some(乵)=>伒.push(乵),None=>{佌=true;交=也.仠.clone().unwrap_or_default();交.retain(|a|!伒.contains(&互(a.丈())));let
乌=|a:&丫|{也.伟.get(&互(a.丈())).copied().unwrap_or(C::MIN)};交.sort_by_key(乌);}}}}举.交=F::from(佌);举.亱=也.亱;举.丝=丝.len();举.义=义.len();let 仂=||乐.付.is_some_and(|t|亂::now()>=t);let
mut 伂:K<P<C>> =vec![None;义.len()];for 丟 in(0..义.len()).rev(){if 丟+1<义.len()&&仂(){break;}伂[丟]=Some(也.倁(&义[丟],&丝));}let 仿=|伂:&[P<C>]|{let mut 专:P<(C,F)>
=None;for(丟,丰)in 伂.iter().enumerate().rev(){if let Some(丰)=*丰{if 专.is_none_or(|(书,_)|丰>书){专=Some((丰,丟));}}}专};let mut 专=仿(&伂);也.limit.亸=乐.付;while let Some((_,丟))=专{if!傠||义[丟].丧.is_some()||仂(){break;}let
之=也.俪(&义[丟].end);义[丟].丧=Some(之);举.俶+=1;伂[丟]=Some(也.倁(&义[丟],&丝));专=仿(&伂);}if let Some((_,丟))=专{举.倴=也.乘.as_ref().is_some_and(|g|!g.丙.is_empty()&&义[丟].丈.starts_with(&g.丙));举.仇=也.乘.as_ref().map_or(0,|g|{g.仇.iter().filter(|p|p.iter().all(|c|义[丟].丈.contains(c))).count()});let
mut 丌=l.clone();for&丑 in&义[丟].丈{if 丌.乃(丑){举.丈.push(丑);}}}举}fn 僭(乐:&dyn Fn(亂,Q)->享,mut 亁:亡,侚:impl BufRead,佚:impl Write,)->Result<(),乖>{let mut 产=伍::default();个(侚,佚,|仳,亇,且,丽|match
J::read(仳,亇,亟::佧,且){Ok(l)=>{let 件=乘(&l,&mut 产);let 举=match 件{Some(乘)=>{债(&l,乐(丽,且),&mut 亁,Some(乘))}None=>儙(&l,乐(丽,且),&mut 亁),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",且,举.侊,举.亱,举.丝,举.俶,举.交,I::from(举.倴),举.仇,丽.elapsed());储(&举.丈)}Err(仔)=>{eprintln!("plan: cannot read the input: {}"
,仔);"WAIT".to_string()}},)}const 偼:P<C> =None;const 傟:O=false;#[derive(Clone,Copy,PartialEq)]enum 伌{佪,偏,}#[derive(Clone,Copy)]struct 亭{q:F,kind:伌,仙:O,value:C,}fn
乥(l:&J,丁:F)->K<亭>{僡(l,丁,傟)}fn 僡(l:&J,丁:F,儒:O)->K<亭>{let(乄,中)=住(l);let(丵,世)=(l.丆(丁),l.丆(1-丁));let(伙,会)=(丳(l,丵),丳(l,世));let 伓=伥(l,&伙,&伙,&会);let mut 丄=K::new();for
q in(乄|中).iter(){let 侎=丳(l,G::q(q));let 仙=丵.test(q);let 丬=if 儒{伥(l,&伙,&侎,&会)-伥(l,&会,&侎,&伙)}else if 仙{let 假=丳(l,僋(丵,丁,q));伓-伥(l,&假,&假,&会)}else{伥(l,&伙,&侎,&会)-伓};丄.push(亭{q,kind:if
中.test(q){伌::佪}else{伌::偏},仙,value:丬,});}丄.sort_by_key(|o|(-o.value,o.kind!=伌::佪,o.q));丄}fn 伥(l:&J,a:&[I;M],g:&[I;M],b:&[I;M])->C{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 什(乥:&[亭])->K<亭>{乥.iter().filter(|o|o.value>=偼.unwrap_or(1)).copied().collect()}fn 僋(丆:G,丁:F,q:F)->G{let
乱=丆.业(G::q(q));let mut 乩=G::q(U[丁])&乱;loop{let 七=乩|(乩.丏()&乱);if 七==乩{return 乩;}乩=七;}}const 俟:C=10;const 偺:C=100;#[derive(Clone,Copy)]enum 伾{伐(Q),丹(F),}#[derive(Clone,Copy)]struct
乻{个:Q,侘:伾,}#[derive(Clone,Copy)]struct 伎{q:F,ours:P<乻>,余:P<乻>,theirs:P<乻>,亦:C,}fn 僲(l:&J,丁:F,乥:&[亭])->K<伎>{let mut 丄=K::new();for 丱 in 乥.iter().filter(|o|!o.仙){let
q=丱.q;let(僧,余)=俫(l,丁,q);let 两=专(僧,余);let(儘,儗)=俫(l,1-丁,q);let 世=专(儘,儗);let 个=|a:P<乻>|a.map_or(偺,|a|C::from(a.个));丄.push(伎{q,ours:两,余,theirs:世,亦:个(世)-个(两),});}丄}fn
专(亹:P<乻>,伴:P<乻>)->P<乻>{match(亹,伴){(Some(u),Some(t))=>Some(if t.个<u.个{t}else{u}),(u,t)=>u.or(t),}}fn 俫(l:&J,临:F,q:F)->(P<乻>,P<乻>){let 严=l.map();let mut
亹:P<乻> =None;for 三 in l.万(临).iter(){let d=严.丗(三,q);let Some(上)=l.予(三)else{continue;};if d!=I::MAX&&亹.is_none_or(|u|Q::from(d)<u.个){亹=Some(乻{个:Q::from(d),侘:伾::伐(上),});}}let
下=l.下(临)+if 临==l.丛(){0}else{l.丞(临)};let 佣=match(下,l.丞(临)){(下,_)if 下>=俟=>Some(0),(_,丞)if 丞<=0=>None,(下,丞)=>Some((俟-下+丞-1)/丞),};let mut 伴:P<乻> =None;if let
Some(佣)=佣{let 乭=l.丆(临).丏()&严.valid&l.乇(临,1);for 三 in 乭.iter(){let d=严.丗(三,q);if d!=I::MAX&&伴.is_none_or(|t|1+佣 as Q+Q::from(d)<t.个){伴=Some(乻{个:1+佣 as Q+Q::from(d),侘:伾::丹(三),});}}}(亹,伴)}const
俠:C=10;#[derive(Default)]struct 伍{倛:BTreeMap<Q,F>,}#[derive(Default)]struct 亾{仕:K<(Q,F)>,些:K<(F,F)>,back:K<Q>,}fn 仫(l:&J,丁:F,乥:&[亭],仞:&[伎],丝:&mut 伍,)->亾{俬(l,丁,乥,仞,丝,l.万(丁))}fn
俬(l:&J,丁:F,乥:&[亭],仞:&[伎],丝:&mut 伍,侏:G,)->亾{let 严=l.map();let 万:BTreeMap<Q,F> =(l.万(丁)&侏).iter().filter_map(|c|l.予(c).map(|上|(上,c))).collect();let mut 丠:K<(亭,伎)>
=K::new();for o in 什(乥).into_iter().filter(|o|!o.仙){if let Some(r)=仞.iter().find(|r|r.q==o.q){if r.ours.is_some()&&r.亦>=0{丠.push((o,*r));}}}丠.sort_by_key(|(o,r)|{(-o.value,o.kind!=伌::佪,r.ours.map_or(Q::MAX,|a|a.个),o.q,)});let
丿=|亹:Q,丕:F|Q::from(严.丗(万[&亹],丕));let 倎=|云:&BTreeMap<F,Q>,伬:&BTreeSet<Q>,上:Q|{伬.contains(&上)||云.values().any(|&u|u==上)};let mut 云:BTreeMap<F,Q> =BTreeMap::new();for(&上,&丕)in&丝.倛{if
万.contains_key(&上)&&丠.iter().any(|(o,_)|o.q==丕)&&!云.contains_key(&丕){云.insert(丕,上);}}let mut 伬:BTreeSet<Q> =BTreeSet::new();for(o,_)in&丠{let Some(&倐)=云.get(&o.q)else{continue;};let
侥=万.keys().copied().filter(|&上|!倎(&云,&伬,上)).map(|上|(丿(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();if let Some((d,上))=侥{if d<丿(倐,o.q){伬.insert(倐);云.insert(o.q,上);}}}let
mut 些=K::new();for(o,r)in&丠{if 云.contains_key(&o.q){continue;}let 並=万.keys().copied().filter(|&上|!倎(&云,&伬,上)).map(|上|(丿(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();let
偀=並.map(|(d,_)|C::from(d));let 佡=r.余.and_then(|t|match t.侘{伾::丹(q)if l.下(丁)-俠*些.len()as C>=俠&&!些.iter().any(|&(c,_)|c==q)=>{Some((q,C::from(t.个)))}_=>None,});let
倽=佡.map(|(_,t)|t);let 偁=match(偀,倽){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 傷=if 偁{倽}else{偀};let Some(个)=傷 else{continue;};if r.theirs.map_or(100,|a|C::from(a.个))<个{continue;}match(偁,佡,並){(true,Some((q,_)),_)=>些.push((q,o.q)),(false,_,Some((_,上)))=>{云.insert(o.q,上);}_=>{}}}let
mut 丄=亾{些,..亾::default()};for(o,_)in&丠{if let Some(&上)=云.get(&o.q){丄.仕.push((上,o.q));}}丄.back=l.万(丁).iter().filter_map(|c|l.予(c)).filter(|上|!丄.仕.iter().any(|&(u,_)|u==*上)).collect();丄.back.sort_unstable();丝.倛=丄.仕.iter().copied().collect();丄}const
信:C=10;fn 俴(l:&J,丁:F,上:Q)->P<F>{l.万(丁).iter().find(|&c|l.予(c)==Some(上))}fn 倲(l:&J,丁:F,三:F,丕:F)->K<F>{let 严=l.map();let 仁=严.丗(三,丕);if 仁==I::MAX||仁==0{return
K::new();}let mut 丄:K<F> =(G::q(三).丏()&严.valid).iter().filter(|&n|严.丗(n,丕)<仁).collect();丄.sort_by_key(|&n|(l.丒(丁).test(n),n));丄}fn 亢(丌:&mut J,丄:&mut Vec<L>,丑:L)->O{let
倞=丌.乃(丑);if 倞{丄.push(丑);}倞}#[derive(Clone,Copy)]struct 仧{侬:O,guard:I,倀:O,倫:O,}fn 仇(l:&J,产:&亾,乲:仧)->K<K<L>>{let mut 丄=K::new();for&(上,丕)in&产.仕{let 僥=亾{仕:vec![(上,丕)],..亾::default()};let
佛=倵(l,&僥,仧{侬:false,..乲},);if!佛.is_empty(){丄.push(佛);}}丄}fn 丢(丌:&J,上:Q,三:F,丣:F,亣:O,乲:仧,)->P<K<L>>{let 丁=丌.丛();let mut 串=丌.clone();let mut 丈=K::new();let(x,y)=乓(丣);if!亢(&mut
串,&mut 丈,L::乳{上,x,y}){return None;}if 亣&&乲.guard!=0{let(他,仗)=乓(三);let 侕=if 乲.guard==1{L::乼{x:他,y:仗}}else{L::丹{丅:1,x:他,y:仗,}};if!亢(&mut 串,&mut 丈,侕){return
None;}}if 乲.倫&&伱(&串,丁)>伱(丌,丁){return None;}Some(丈)}fn 倵(l:&J,产:&亾,乲:仧)->K<L>{let 丁=l.丛();let 严=l.map();let mut 丌=l.clone();let mut 丄=K::new();let 丩=丩(l,1-丁);let
mut 价:K<(Q,F)> =K::new();for&(上,丕)in&产.仕{let Some(三)=俴(&丌,丁,上)else{continue;};if!丌.乷().test(三){continue;}let 亣=丩.test(三);for 丣 in 倲(&丌,丁,三,丕){if let Some(丈)=丢(&丌,上,三,丣,亣,乲){for
丑 in&丈{丌.乃(*丑);}if 亣&&乲.guard==0{价.push((上,三));}丄.extend(丈);break;}}}for&(上,丕)in&产.仕{if 丌.下(丁)<信{break;}let Some(乗)=俴(&丌,丁,上)else{continue;};let 催=价.iter().find(|&&(u,_)|u==上).map(|&(_,c)|c);let
丐=match 催{Some(q)if 乲.倀=>vec![q],_=>倲(&丌,丁,乗,丕),};for 丣 in 丐{let(x,y)=乓(丣);if 亢(&mut 丌,&mut 丄,L::丹{丅:1,x,y}){break;}}}if 乲.侬{for&(_,丕)in&产.些{if 丌.下(丁)<信{break;}let
乭=丌.丆(丁).丏()&严.valid;let mut 丐:K<F> =乭.iter().filter(|&c|严.丗(c,丕)!=I::MAX).collect();丐.sort_by_key(|&c|(严.丗(c,丕),c));for 丣 in 丐{let(x,y)=乓(丣);if 亢(&mut
丌,&mut 丄,L::丹{丅:1,x,y}){break;}}}}丄}const 俢:I=0;const 備:I=1;const 俔:C=0;const 偽:O=false;const 偝:I=0;const 偗:O=false;const 傌:O=false;const 偋:O=false;const
偙:Q=7;const 傛:Q=14;const 傔:O=俢!=0||俔!=0;struct 令{丙:K<L>,倝:O,start:O,仇:K<K<L>>,乥:K<(F,C,C)>,伀:C,}impl 令{fn 儕(&self,l:&J,丁:F)->C{if self.伀==0{return 0;}let(丵,世)=(l.丆(丁),l.丆(1-丁));let
mut 亚=0;for&(q,丬,五)in&self.乥{亚+=if 丵.test(q){丬}else if 世.test(q){-丬}else{五};}self.伀*亚/4}}fn 乘(l:&J,丝:&mut 伍)->P<令>{let 伦=l.乨();if!傔||!(偙..=傛).contains(&伦)||l.両().is_some(){return
None;}僒(l,丝,傎)}#[derive(Clone,Copy)]struct 佁{仃:I,乲:仧,使:O,伀:C,侩:O,}const 傎:佁=佁{仃:俢,乲:仧{侬:備==2,guard:偝,倀:偗,倫:傌,},使:偋,伀:俔,侩:偽,};fn 僒(l:&J,丝:&mut 伍,乀:佁)->P<令>{let
佁{仃,乲,使,伀,侩,}=乀;let 丁=l.丛();let 乥=乥(l,丁);let 仞=僲(l,丁,&乥);let 产=if 使{let 侏=l.万(丁).业(丩(l,1-丁));俬(l,丁,&乥,&仞,丝,侏)}else{仫(l,丁,&乥,&仞,丝)};let 丙=if 仃!=0{倵(l,&产,乲)}else{K::new()};let
什=什(&乥);let 儖=什.iter().map(|o|{let 亦=仞.iter().find(|r|r.q==o.q).map_or(0,|r|r.亦.clamp(-1,1));let 五=if 侩{o.value*亦}else{0};(o.q,o.value,五)}).collect();let
僩=if 仃&4!=0{仇(l,&产,乲)}else{K::new()};Some(令{仇:僩,倝:!丙.is_empty()&&仃&1!=0,start:!丙.is_empty()&&仃&2!=0,丙,乥:儖,伀,})}const 偍:亻=45.0;const 偘:亻=800.0;const 傒:亻=0.6;fn
乐(丽:亂,且:Q)->享{if let Some(伢)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return 享{亸:None,付:None,伢,};}let 你=儛().min(100.0);let
僠=if 且==1{偘}else{偍};let 乏=Duration::from_secs_f64(僠/1000.0*你);享{亸:Some(丽+乏.mul_f64(傒)),付:Some(丽+乏),伢:丮::MAX,}}const 偿:F=9;const 偻:亼=亼{丐:3,丞:1,下:1,为:2,亴:2,伳:2,丩:6,仢:1,中:0,仫:false,乞:傀,};const
傃:Q=0;const 傀:亍=亍::佩(伽::俏(7));const 俘:I=I::MAX;const 俖:C=2;const 俗:C=18;const 俕:C=0;const 偾:F=32;const 偉:O=false;const 偊:Settings1=Settings1{侰:3,侯:1,侇:4,仂:2,侐:2,佋:2,侷:true,reserve:true,侑:true,乢:亽::乴,fill:true,乕:Z{佟:俘,仭:俖,佒:俗,佃:俕,..Z::仏},};fn
亁()->亡{if 偉{return 亡::new(于::伹(偊),K::new(),Z::仏);}僦(偾)}fn 僦(亝:F)->亡{let 亘=于::伏{个:偿,亝,丱:偻,佝:傃,};let 丯=傍.iter().map(|&乕|Z{佟:俘,仭:俖,佒:俗,佃:俕,..乕}).collect();亡::new(亘,丯,Z::仏)}fn
main(){let 儋=io::stdin();let 儌=io::stdout();if let Err(仔)=僭(&乐,亁(),儋.lock(),儌.lock()){eprintln!("plan: {}",仔);std::process::exit(1);}}const 傗:&str="CG_TIME_SCALE";fn
儛()->亻{僨(std::env::var(傗).ok().as_deref())}fn 僨(丬:P<&str>)->亻{丬.and_then(|亇|亇.trim().parse::<亻>().ok()).filter(|你|你.is_finite()&&*你>0.0).unwrap_or(1.0)}const
_:()=assert!(偖==16);const 偖:F=16;const 偐:亻=0.4;const 偶:I=4;#[derive(Clone,Copy)]struct 仐{中:G,俵:G,伣:G,}impl 仐{fn of(l:&J)->仐{let(a,伖)=倡(l,U[0]);let(b,佊)=倡(l,U[1]);let
亚=a[U[1]];let 僘=伖[U[1]];let mut 乂=仐{中:G::T,俵:G::T,伣:G::T,};for q in l.map().valid.iter(){if 伖[q]==I::MAX||佊[q]==I::MAX{continue;}if 伖[q].abs_diff(佊[q])<=1{乂.俵.set(q);}if
伖[q].abs_diff(佊[q])<=偶{乂.伣.set(q);}let 僢=伸::from(伖[q])+伸::from(佊[q])==伸::from(僘);if 亚>0.0&&僢&&q!=U[0]&&q!=U[1]&&a[q]*b[q]/亚>=偐{乂.中.set(q);}}乂}fn 仚(&self)->G{self.中&self.伣}}fn
儎(l:&J,儈:G)->[I;M]{let 乁=l.map().valid;let mut 丄=[I::MAX;M];let mut 乩=儈&乁;let mut 件=乩;let mut 丢=0u8;while!件.is_empty(){for q in 件.iter(){丄[q]=丢;}let 七=(件.丏()&乁).业(乩);乩|=七;件=七;丢=丢.saturating_add(1);}丄}fn
倡(l:&J,三:F)->([亻;M],[I;M]){let 丿=儎(l,G::q(三));let mut 乞:K<F> =l.map().valid.iter().filter(|&c|丿[c]!=I::MAX).collect();乞.sort_by_key(|&c|丿[c]);let mut 丘=[0.0;M];丘[三]=1.0;for
q in 乞{if q==三{continue;}let 九=G::q(q).丏()&l.map().valid;丘[q]=九.iter().filter(|&n|丿[n]!=I::MAX&&丿[n]+1==丿[q]).map(|n|丘[n]).sum();}(丘,丿)}const Y:F=12;const
M:F=Y*Y;const 亃:F=4*Y;const 亠:丮=(1<<亃)-1;const 伺:丮=1|1<<12|1<<24|1<<36;const 佨:丮=伺<<11;const 俛:丮=(1<<Y)-1;#[derive(Clone,Copy,PartialEq)]struct G([丮;3]);impl
G{const T:G=G([0;3]);const 偈:G=G([亠;3]);#[inline]fn q(q:F)->G{let mut 乎=G::T;乎.set(q);乎}#[inline]fn test(self,q:F)->O{(self.0[q/亃]>>(q%亃))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/亃]|=1<<(q%亃);}#[inline]fn clear(&mut self,q:F){self.0[q/亃]&=!(1<<(q%亃));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伵(self)->[丮;3]{self.0}#[inline]fn count(self)->Q{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 业(self,乆:G)->G{G([self.0[0]&!乆.0[0],self.0[1]&!乆.0[1],self.0[2]&!乆.0[2],])}#[inline]fn
iter(self)->佦{佦{侟:self.0,乗:0,}}#[inline]fn 億(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&俛)<<(亃-Y)),(b>>Y)|((c&俛)<<(亃-Y)),c>>Y,])}#[inline]fn 儁(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亠,((b<<Y)&亠)|(a>>(亃-Y)),((c<<Y)&亠)|(b>>(亃-Y)),])}#[inline]fn
儂(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佨&亠,(b>>1)&!佨&亠,(c>>1)&!佨&亠,])}#[inline]fn 儃(self)->G{let[a,b,c]=self.0;G([(a<<1)&!伺&亠,(b<<1)&!伺&亠,(c<<1)&!伺&亠,])}#[inline]fn
丏(self)->G{let 一=self;一.億()|一.儁()|一.儂()|一.儃()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,乆:G)->G{G([self.0[0]&乆.0[0],self.0[1]&乆.0[1],self.0[2]&乆.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,乆:G)->G{G([self.0[0]|乆.0[0],self.0[1]|乆.0[1],self.0[2]|乆.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,乆:G){*self=*self|乆;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::偈.业(self)}}struct 佦{侟:[丮;3],乗:F,}impl Iterator for 佦{type Item=F;#[inline]fn
next(&mut self)->P<F>{let 一=self;while 一.乗<3{let 佗=一.侟[一.乗];if 佗!=0{一.侟[一.乗]=佗&(佗-1);return Some(一.乗*亃+佗.trailing_zeros()as F);}一.乗+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乳{上:Q,x:I,y:I},丹{丅:I,x:I,y:I},亩{x:I,y:I},乼{x:I,y:I},}impl L{fn 儞(&self)->乖{match*self{L::乳{上,x,y}=>format!("MOVE {} {x} {y}",上),L::丹{丅,x,y}=>format!("TRAIN {} {x} {y}",丅),L::亩{x,y}=>format!("BUILD MINE {x} {y}"),L::乼{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
储(丈:&[L])->乖{if 丈.is_empty(){return "WAIT".to_string();}let mut line=乖::with_capacity(16*丈.len());for(丟,丑)in 丈.iter().enumerate(){if 丟>0{line.push(';');}line.push_str(&丑.儞());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亟{俧,儭,傤,#[default]佧,}impl 亟{fn 亓(self)->I{match self{亟::俧=>1,_=>3,}}fn 佄(self)->O{matches!(self,亟::傤|亟::佧)}fn 仪(self)->O{self==亟::佧}}const 二:I=I::MAX;struct
京{乡:G,valid:G,丷:G,丗:Box<[I]>,丏:[[[I;4];M];2],}thread_local!{static 偪:RefCell<P<Arc<京>>> =const{RefCell::new(None)};}impl 京{fn 儀(乡:G,丷:G)->Arc<京>{偪.with(|乶|{let
mut 乶=乶.borrow_mut();if let Some(严)=乶.as_ref(){if 严.乡==乡&&严.丷==丷{return Arc::clone(严);}}let 严=Arc::new(京::build(乡,丷));*乶=Some(Arc::clone(&严));严})}fn build(乡:G,丷:G)->京{let
乁=!乡;const 傓:[[(伷,伷);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[二;4];M];2];for(丁,乑)in 傓.iter().enumerate(){for q in
乁.iter(){let(x,y)=((q%Y)as 伷,(q/Y)as 伷);let mut 丘=0;for&(僁,僂)in 乑{let(候,倚)=(x+僁,y+僂);if(0..Y as 伷).contains(&候)&&(0..Y as 伷).contains(&倚){let 七=倚 as F*Y+候
as F;if 乁.test(七){丏[丁][q][丘]=七 as I;丘+=1;}}}}}let mut 丗=vec![二;M*M].into_boxed_slice();let mut 侨=[0u8;M];for 三 in 乁.iter(){let 乺=&mut 丗[三*M..(三+1)*M];乺[三]=0;侨[0]=三
as I;let(mut 侖,mut 侶)=(0,1);while 侖<侶{let q=侨[侖]as F;侖+=1;for&七 in&丏[0][q]{if 七!=二&&乺[七 as F]==二{乺[七 as F]=乺[q]+1;侨[侶]=七;侶+=1;}}}}京{乡,valid:乁,丷,丗,丏,}}#[inline]fn
丗(&self,三:F,丣:F)->I{self.丗[三*M+丣]}}const 側:Q=100;const 争:C=15;const 傝:[C;3]=[1,4,20];const U:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 乮{亏(F),俅,}enum
亊{佳,俓,俙,}#[derive(Clone)]struct J{map:Arc<京>,丼:亟,丧:[G;2],丆:[G;2],丵:G,乧:G,万:[G;2],丅:[G;3],仅:G,予:[Q;M],下:[C;2],丞:[C;2],丛:F,乨:Q,仼:Q,両:P<乮>,}fn 伞(丧:G,仲:F)->G{if!丧.test(仲){return
G::T;}let mut 伫=G::q(仲);loop{let 七=(伫|伫.丏())&丧;if 七==伫{return 伫;}伫=七;}}impl J{fn 丛(&self)->F{self.丛}fn 乨(&self)->Q{self.乨}fn 両(&self)->P<乮>{self.両}fn 丼(&self)->亟{self.丼}fn
仺(&self,丁:F)->C{let 一=self;let 万=一.万[丁];一.下[丁]+10*C::from((万&一.丅[0]).count())+20*C::from((万&一.丅[1]).count())+30*C::from((万&一.丅[2]).count())}fn 下(&self,丁:F)->C{self.下[丁]}fn
丞(&self,丁:F)->C{self.丞[丁]}fn 侹(&self,丁:F)->F{self.万[丁].count()as F}fn map(&self)->&京{&self.map}fn 丒(&self,丁:F)->G{self.丧[丁]}fn 丆(&self,丁:F)->G{self.丆[丁]}fn
万(&self,丁:F)->G{self.万[丁]}fn 人(&self,q:F)->P<(Q,I)>{let 一=self;(一.万[0]|一.万[1]).test(q).then(||(一.予[q],一.侞(q)))}fn 乷(&self)->G{self.万[self.丛]&self.仅}fn
丶(&self)->G{self.丵}fn 乒(&self)->G{self.乧}fn 亜(&self,q:F)->P<I>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.侞(q))}fn 予(&self,q:F)->P<Q>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.予[q])}fn
儉(&mut self,丁:F){let 一=self;一.丆=[伞(一.丧[0],U[0]),伞(一.丧[1],U[1])];let 傻=一.万[丁].业(一.丆[丁]);一.侫(傻);一.俷();一.下[丁]+=一.丞[丁];if 一.下[丁]<0{一.下[丁]=0;一.侫(一.万[丁]);}一.仅|=一.万[丁];}fn
俷(&mut self){let 一=self;for 丁 in 0..2{let 丆=一.丆[丁];let 万=丆&一.万[丁];let mut 丞=C::from(丆.count())+4*C::from((丆&一.丵).count());for(丅,儣)in 傝.iter().enumerate(){丞-=儣*C::from((万&一.丅[丅]).count());}一.丞[丁]=丞;}}fn
仯(&mut self){let 一=self;if 一.両.is_some(){return;}if 一.丛==1{if 一.乨>=側{一.両=Some(match 一.仺(0).cmp(&一.仺(1)){std::cmp::Ordering::Greater=>乮::亏(0),std::cmp::Ordering::Less=>乮::亏(1),std::cmp::Ordering::Equal=>乮::俅,});return;}一.乨+=1;}一.丛=1-一.丛;一.儉(一.丛);}fn
作(&mut self,丈:&[L]){let 一=self;if 一.両.is_some(){return;}let 侓=一.仼;for&丑 in 丈{if 一.僀(丑,侓){continue;}一.乃(丑);if 一.両.is_some(){return;}}一.仯();}fn 僀(&self,丑:L,侓:Q)->O{let
一=self;match 丑{L::乳{上,x,y}=>{一.们(x,y).is_none()||上>=侓||一.倂(一.丛,上).is_none()}L::丹{丅,x,y}=>{一.们(x,y).is_none()||丅==0||丅>一.丼.亓()}L::亩{x,y}|L::乼{x,y}=>{一.们(x,y).is_none()}}}#[inline]fn
们(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<Y&&y<Y&&!self.map.乡.test(y*Y+x)).then_some(y*Y+x)}fn 倂(&self,丁:F,上:Q)->P<F>{self.万[丁].iter().find(|&q|self.予[q]==上)}#[inline]fn
侞(&self,q:F)->I{if self.丅[0].test(q){1}else if self.丅[1].test(q){2}else{3}}fn 傮(&self,井:F)->G{let 乒=self.乧&self.丧[井];let 丆=self.丆[井];乒|(丆&(丆&乒).丏())}fn
乇(&self,丁:F,丅:I)->G{let 一=self;debug_assert!((1..=3).contains(&丅),"unit level {} is not 1 to 3",丅);let 井=1-丁;let 万=一.万[0]|一.万[1];let 僃=一.仓();let 儡=match
丅{1 if 一.丼==亟::俧=>一.丅[1]|一.丅[2],1=>万,2=>一.丅[1]|一.丅[2],_=>G::T,};let 乇=僃|一.丧[井].业(儡);if 丅<3{乇.业(一.傮(井))}else{乇}}fn 仆(&self,丁:F)->G{self.丆[丁]|self.丆[丁].丏()}fn
仓(&self)->G{let 一=self;let 万=一.万[0]|一.万[1];let 僓=G::q(U[0])|G::q(U[1]);一.map.valid.业(万|一.丵|一.乧|僓)}fn 乃(&mut self,丑:L)->O{let 一=self;if 一.両.is_some(){return
false;}let 丁=一.丛;let 仑=match 丑{L::丹{丅,x,y}=>一.佡(丁,丅,x,y),L::乳{上,x,y}=>一.僟(丁,上,x,y),L::亩{x,y}=>一.build(丁,true,x,y),L::乼{x,y}=>一.build(丁,false,x,y),};let
Some(仑)=仑 else{return false;};match 仑{亊::佳=>{}亊::俓=>一.丆[丁]=伞(一.丧[丁],U[丁]),亊::俙=>{一.丆=[伞(一.丧[0],U[0]),伞(一.丧[1],U[1])];}}一.俷();if!一.丧[0].test(U[0]){一.両=Some(乮::亏(1));}else
if!一.丧[1].test(U[1]){一.両=Some(乮::亏(0));}true}fn 侫(&mut self,丐:G){let 一=self;一.万[0]=一.万[0].业(丐);一.万[1]=一.万[1].业(丐);for 丅 in&mut 一.丅{*丅=丅.业(丐);}一.仅=一.仅.业(丐);}fn
倹(&mut self,丁:F,q:F)->亊{let 一=self;一.侫(G::q(q));一.丵.clear(q);一.乧.clear(q);let 井=1-丁;if 一.丧[井].test(q){一.丧[井].clear(q);一.丧[丁].set(q);亊::俙}else if 一.丧[丁].test(q){亊::佳}else{一.丧[丁].set(q);亊::俓}}fn
佡(&mut self,丁:F,丅:I,x:I,y:I)->P<亊>{let 一=self;if 丅==0||丅>一.丼.亓(){return None;}let q=一.们(x,y)?;let 不=10*C::from(丅);if 一.下[丁]<不||!一.仆(丁).test(q)||!一.乇(丁,丅).test(q){return
None;}let 仑=一.倹(丁,q);一.倢(丁,丅,q,一.仼);一.仼=一.仼.saturating_add(1);一.下[丁]-=不;Some(仑)}fn 倢(&mut self,丁:F,丅:I,q:F,上:Q){self.万[丁].set(q);self.丅[F::from(丅)-1].set(q);self.予[q]=上;}fn
僟(&mut self,丁:F,上:Q,x:I,y:I)->P<亊>{let 一=self;let 与=一.们(x,y)?;let 三=一.倂(丁,上)?;if!一.仅.test(三){return None;}let 丅=一.侞(三);let 丨=一.乇(丁,丅);let 九=一.map.丏[丁][三];if
九.contains(&(与 as I))&&!丨.test(与){return None;}let mut 专=一.map.丗(三,与);let mut 优=None;for&七 in 九.iter().take_while(|&&七|七!=二){let 七=F::from(七);let 丗=一.map.丗(七,与);if
丗<专&&丨.test(七){专=丗;优=Some(七);}}let 优=优?;let 仑=一.倹(丁,优);一.万[丁].clear(三);一.丅[F::from(丅)-1].clear(三);一.仅.clear(三);一.倢(丁,丅,优,上);Some(仑)}fn build(&mut self,丁:F,丵:O,x:I,y:I)->P<亊>{let
一=self;let 佼=if 丵{一.丼.佄()}else{一.丼.仪()};let q=一.们(x,y)?;let 僣=一.map.丷.test(q);if!佼||一.下[丁]<争||僣!=丵||!一.丧[丁].test(q)||!一.仆(丁).test(q)||!一.仓().test(q){return
None;}if 丵{一.丵.set(q);}else{一.乧.set(q);}一.下[丁]-=争;Some(亊::佳)}}const 偯:C=1000000000;const 佮:C=0;const 偷:C=1;const 傘:C=2;impl J{fn read(仳:&str,且:&str,丼:亟,乨:Q,)->Result<J,乖>{let
mut 丷=K::new();let mut 久=仳.lines();let 丘=侌(&mut 久,"mine spot count")?;for _ in 0..丘{let[x,y]=亶(份(&mut 久,"a mine spot")?,"a mine spot")?;丷.push((佈(x)?,佈(y)?));}俾(久,"the initial input")?;let
mut 久=且.lines();let mut 侗=[0i64;4];for(丬,乚)in 侗.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[侦]=亶(份(&mut 久,乚)?,乚)?;if 侦.abs()>偯{return
Err(format!("{}: {} is out of range",乚,侦));}*丬=侦;}let mut 乡=G::T;let mut 丐=[G::T;2];let mut 丆=[G::T;2];for y in 0..Y{let 乺=份(&mut 久,"a map row")?.trim();if
乺.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,乺));}for(x,伯)in 乺.bytes().enumerate(){let q=y*Y+x;match 伯{b'#'=>乡.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乪=F::from(伯==b'X'||伯==b'x');丐[乪].set(q);if 伯.is_ascii_uppercase(){丆[乪].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伯 as char))}}}}let
傱=侌(&mut 久,"building count")?;let mut 乽=K::new();for _ in 0..傱{let 亶=亶(份(&mut 久,"a building")?,"a building")?;乽.push(亶);}let 侹=侌(&mut 久,"unit count")?;let
mut 万=K::new();for _ in 0..侹{let 亶=亶(份(&mut 久,"a unit")?,"a unit")?;万.push(亶);}俾(久,"the turn's input")?;let 借=乽.iter().find(|俱|俱[0]==0&&俱[1]==佮).ok_or("the bot's own HQ is not among the buildings")?;let
個=侃(借[2],借[3],乡)?;let 丁=U.iter().position(|&仲|仲==個).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",個))?;let mut 倮=G::T;for(x,y)in 丷{if
乡.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}倮.set(y*Y+x);}let 佺=|乪:F|if 乪==0{丁}else{1-丁};let mut l=J{map:京::儀(乡,倮),丼,丧:[G::T;2],丆:[G::T;2],丵:G::T,乧:G::T,万:[G::T;2],丅:[G::T;3],仅:G::T,予:[0;M],下:[0;2],丞:[0;2],丛:丁,乨,仼:1,両:None,};for
乪 in 0..2{let 亗=佺(乪);l.丧[亗]=丐[乪];l.丆[亗]=丆[乪];l.下[亗]=侗[2*乪];l.丞[亗]=侗[2*乪+1];}let mut 倜=G::T;for[亗,佔,x,y]in 乽{let 乪=倨(亗)?;let q=侃(x,y,乡)?;if!丐[乪].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 倜.test(q){return Err(format!("two buildings at ({x}, {y})"));}倜.set(q);match 佔{佮
if q==U[佺(乪)]=>{}佮=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}偷=>l.丵.set(q),傘=>l.乧.set(q),_=>return Err(format!("unknown building type {}",佔)),}}let
mut 乫=G::T;let mut 何=0u32;for[亗,上,丅,x,y]in 万{let 乪=倨(亗)?;let q=侃(x,y,乡)?;if!丐[乪].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乫.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丅){return Err(format!("unit level {} is not 1, 2 or 3",丅));}let 上=Q::try_from(上).map_err(|_|format!("unit id {} is out of range",上))?;if
乫.iter().any(|乆|l.予[乆]==上){return Err(format!("two units have id {}",上));}乫.set(q);l.万[佺(乪)].set(q);l.丅[丅 as F-1].set(q);l.予[q]=上;何=何.max(上);}if!乫.is_empty(){l.仼=何.checked_add(1).ok_or_else(||format!("unit id {} is out of range",何))?;}l.仅=l.万[丁];Ok(l)}}fn
份<'a>(久:&mut Lines<'a>,乚:&str)->Result<&'a str,乖>{久.next().ok_or_else(||format!("the text ends before {}",乚))}fn 俾(mut 久:Lines<'_>,乚:&str)->Result<(),乖>{match
久.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乚)),None=>Ok(()),}}fn 亶<const N:F>(line:&str,乚:&str)->Result<[C;N],乖>{let
mut 伵=line.split_whitespace();let mut 偂=[0i64;N];for 丬 in&mut 偂{let 伇=伵.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乚))?;*丬=伇.parse().map_err(|_|format!("{}: {:?} is not an integer",乚,伇))?;}if
伵.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乚));}Ok(偂)}fn 侌(久:&mut Lines<'_>,乚:&str)->Result<F,乖>{let[丘]=亶(份(久,乚)?,乚)?;F::try_from(丘).ok().filter(|&丘|丘<=M).ok_or_else(||format!("{}: {} is out of range",乚,丘))}fn
佈(丬:C)->Result<F,乖>{F::try_from(丬).ok().filter(|&丬|丬<Y).ok_or_else(||format!("coordinate {} is off the map",丬))}fn 侃(x:C,y:C,乡:G)->Result<F,乖>{let q=佈(y)?*Y+佈(x)?;if
乡.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 倨(亗:C)->Result<F,乖>{match 亗{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亗)),}}#[derive(Clone,Copy,PartialEq)]enum
亽{乴,便,}#[derive(Clone,Copy)]struct Settings1{侰:I,侯:I,侇:C,仂:C,侐:F,佋:I,侷:O,reserve:O,侑:O,乢:亽,fill:O,乕:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{侰:3,侯:1,侇:4,仂:2,侐:2,佋:2,侷:true,reserve:true,侑:true,乢:亽::乴,fill:true,乕:Z::仏,};}const
佹:C=1000;const 佸:C=10;fn 侼(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侧(l:&mut J,丑:L,丄:&mut Vec<L>)->O{let 亢=l.乃(丑);if 亢{丄.push(丑);}亢}fn 侳(l:&mut J,三:F,丣:F,丄:&mut
Vec<L>)->O{let Some((上,_))=l.人(三)else{return false;};let(x,y)=侼(丣);侧(l,L::乳{上,x,y},丄)}fn 儔(l:&J,乂:&仐,乀:&Settings1,乐:F,)->K<F>{let 丂=l.丛();let 两=丳(l,l.丒(丂));let
世=丳(l,l.丒(1-丂));let 五=l.map().valid.业(l.丒(0)|l.丒(1));let 倦=|c:F|两[c]!=I::MAX&&世[c]!=I::MAX;let mut 中:K<F> =乂.仚().业(l.丒(丂)).iter().filter(|&c|倦(c)).collect();中.sort_by_key(|&c|(两[c],c));let
mut 么:K<F> =五.iter().filter(|&c|倦(c)&&两[c].abs_diff(世[c])<=乀.侯).collect();么.sort_by_key(|&c|(两[c].abs_diff(世[c]),两[c],c));let mut 丄:K<F> =K::new();for
q in 中.into_iter().chain(么){if 丄.len()>=乐{break;}let 伝=丄.iter().all(|&t|l.map().丗(t,q)>=乀.侰);let 傶=乂.仚().test(q);if!丄.contains(&q)&&(傶||伝){丄.push(q);}}丄}fn
僔(不:&[K<C>])->K<F>{let n=不.len();let 倏=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 偄=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亳=0usize;let mut 伤=vec![倏;n+1];let mut 侺=vec![false;n+1];loop{侺[亳]=true;let 倍=p[亳];let mut 众=倏;let mut 传=0usize;for j in 1..=n{if!侺[j]{let
乾=不[倍-1][j-1]-u[倍]-v[j];if 乾<伤[j]{伤[j]=乾;偄[j]=亳;}if 伤[j]<众{众=伤[j];传=j;}}}for j in 0..=n{if 侺[j]{u[p[j]]+=众;v[j]-=众;}else{伤[j]-=众;}}亳=传;if p[亳]==0{break;}}loop{let
传=偄[亳];p[亳]=p[传];亳=传;if 亳==0{break;}}}let mut 丄=vec![0usize;n];for j in 1..=n{if p[j]>0{丄[p[j]-1]=j-1;}}丄}#[derive(Clone,Copy)]enum 仍{伐(F),佷,}fn 乭(l:&J)->G{let
丂=l.丛();(l.仆(丂)&l.乇(丂,1)).业(l.丒(丂))}fn 僫(l:&J,乕:&Z)->F{佢(l,乕,乔::乴).iter().filter(|c|matches!(c,L::丹{..})).count()}fn 丠(q:F,並:G)->Q{(G::q(q).丏()&並).count()}fn
倅(l:&J,乌:G,与:F,侜:I)->K<[C;M]>{let 乁=l.map().valid;let mut 专=[C::MIN;M];专[与]=0;let mut 乤=vec![专];for _ in 0..侜{let mut 七=专;for q in 乁.iter(){for n in(G::q(q).丏()&乁).iter(){if
专[n]>C::MIN{let 丬=专[n]+C::from(乌.test(n));if 丬>七[q]{七[q]=丬;}}}}专=七;乤.push(专);}乤}fn 倠(l:&J,乤:&[[C;M]],乌:G,三:F,k:F,)->G{let 乁=l.map().valid;let mut 丄=G::T;let
mut q=三;let mut k=k.min(乤.len()-1);while k>0&&乤[k][q]>0{let Some(七)=(G::q(q).丏()&乁).iter().filter(|&n|乤[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(乌.test(n))+乤[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乌.test(七){丄.set(七);}q=七;k-=1;}丄}fn 且(l:&J,乀:&Settings1)->K<L>{let 丂=l.丛();let 乂=仐::of(l);let mut 丌=l.clone();let mut 丄=K::new();let 世=丳(l,l.丒(1-丂));let
万:K<F> =l.乷().iter().collect();let 倇=(l.下(丂)/佸).max(0)as F;let 些=match 乀.乢{亽::乴=>僫(l,&乀.乕).min(倇),亽::便=>倇,};let mut 休:K<仍> =万.iter().map(|&c|仍::伐(c)).collect();休.extend(std::iter::repeat_n(仍::佷,些));let
乿=儔(l,&乂,乀,休.len()+乀.侐);let 儠=乭(l);let 中=乂.仚();let n=休.len().max(乿.len());let mut 不=vec![vec![0i64;n];n];for(乺,&与)in 乿.iter().enumerate(){for(伕,仉)in 不[乺].iter_mut().enumerate(){let
Some(&傧)=休.get(伕)else{*仉=佹;continue;};let 佅=match 傧{仍::伐(q)=>{let d=l.map().丗(q,与);if d==I::MAX{None}else{Some(C::from(d)-1)}}仍::佷=>儠.iter().map(|c|l.map().丗(c,与)).filter(|&d|d!=I::MAX).min().map(C::from),};*仉=match
佅{None=>佹,Some(佅)=>{let 主=C::from(世[与])-1;let 仂=乀.仂*(佅-主).max(0);let 侀=if 中.test(与){乀.侇}else{0};(佅+仂-侀).max(0)}};}}let 傩=僔(&不);let mut 侔:K<(F,F)> =K::new();let
mut 侸:K<(C,F)> =K::new();for(乺,&与)in 乿.iter().enumerate(){let 伕=傩[乺];if 不[乺][伕]>=佹{continue;}match 休.get(伕){Some(仍::伐(q))=>侔.push((*q,与)),Some(仍::佷)=>侸.push((不[乺][伕],与)),None=>{}}}let
mut 並=l.map().valid.业(l.丒(0)|l.丒(1));let mut 倗:K<F> =K::new();let mut 佞=G::T;let 亖=|q:F|{let 丅=l.亜(q).unwrap_or(1);let 估=並|l.丒(1-丂);(G::q(q).丏()&估&l.乇(丂,丅)).count()};侔.sort_by_key(|&(q,与)|{let
僉=if 乀.侑{亖(q)}else{0};(僉,l.map().丗(q,与),q)});for&(三,与)in&侔{let Some(丅)=丌.亜(三)else{continue;};let 丨=丌.乇(丂,丅);let d=丌.map().丗(三,与);let 乌=(並|丌.丒(1-丂)).业(佞);let
侜=d.saturating_add(乀.佋);let 乤=倅(&丌,乌,与,侜.saturating_sub(1));let 仩=乤[乤.len()-1];let 七=(G::q(三).丏()&丨).iter().filter(|&c|仩[c]>C::MIN).max_by_key(|&c|{let
傹=d.saturating_sub(丌.map().丗(c,与));let 估=並.test(c)||丌.丒(1-丂).test(c);(估&&乀.侷,C::from(乌.test(c))+仩[c],傹,丠(c,並),std::cmp::Reverse(c),)}).filter(|&c|{並.test(c)||丌.丒(1-丂).test(c)||丌.map().丗(c,与)<d});if
let Some(丣)=七{if 侳(&mut 丌,三,丣,&mut 丄){並.clear(丣);倗.push(三);if 乀.reserve{佞|=倠(&丌,&乤,乌,丣,乤.len()-1);}}}}let 两=丳(l,l.丒(丂));let mut 临=G::T;for c in 並.iter(){if
两[c]<=世[c]{临.set(c);}}for&三 in&万{if 倗.contains(&三)||!丌.乷().test(三){continue;}let Some(丅)=丌.亜(三)else{continue;};let 丨=丌.乇(丂,丅);let 估=(G::q(三).丏()&並&丨).iter().max_by_key(|&c|(丠(c,並),世[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丣)=估{if 侳(&mut 丌,三,丣,&mut 丄){並.clear(丣);}continue;}let 丕=[临&並,並].into_iter().find_map(|丐|{丐.iter().min_by_key(|&c|(丌.map().丗(三,c),c))});if let
Some(丕)=丕{let d=丌.map().丗(三,丕);let 七=(G::q(三).丏()&丨).iter().filter(|&c|丌.map().丗(c,丕)<d).min_by_key(|&c|c);if let Some(丣)=七{侳(&mut 丌,三,丣,&mut 丄);}}}侸.sort();let
mut 価=0usize;for&(_,与)in&侸{if 丌.下(丂)<佸{break;}let 丐=乭(&丌);let Some(亀)=丐.iter().map(|c|丌.map().丗(c,与)).min()else{break;};let 乌=(並|丌.丒(1-丂)).业(佞);let 乤=倅(&丌,乌,与,亀.saturating_add(乀.佋));let
仩=乤[乤.len()-1];let q=丐.iter().filter(|&c|丌.map().丗(c,与)==亀).max_by_key(|&c|(仩[c],丠(c,並),std::cmp::Reverse(c)));let Some(q)=q else{break;};let(x,y)=侼(q);if
侧(&mut 丌,L::丹{丅:1,x,y},&mut 丄){並.clear(q);価+=1;if 乀.reserve{佞|=倠(&丌,&乤,乌,q,乤.len()-1);}}}if 乀.fill&&乀.乢==亽::乴{while 価<些&&丌.下(丂)>=佸{let q=(乭(&丌)&並).iter().max_by_key(|&c|{(丠(c,並),std::cmp::Reverse(世[c]),std::cmp::Reverse(c),)});let
Some(q)=q else{break;};let(x,y)=侼(q);if!侧(&mut 丌,L::丹{丅:1,x,y},&mut 丄){break;}並.clear(q);価+=1;}}丄}const 偣:C=15;const 偸:C=6;const 俣:C=10;const 傋:I=3;#[derive(Clone,Copy)]struct
Z{open:C,佇:C,俺:I,侙:C,佒:C,倓:C,佟:I,俹:C,俸:C,佃:C,仱:C,伄:C,习:C,仴:C,仌:C,仭:C,}impl Z{const 仏:Z=Z{open:3,佇:2,俺:12,侙:4,佒:偣,倓:偸,佟:傋,俹:2,俸:2,佃:0,仱:0,伄:0,习:0,仴:0,仌:0,仭:0,};const
fn 儫(self,仱:C,伄:C,习:C)->Z{Z{仱,伄,习,..self}}const fn 優(self,仴:C,仌:C)->Z{Z{仴,仌,..self}}}fn 伶(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 丳(l:&J,三:G)->[I;M]{let mut
丿=[I::MAX;M];let 乁=l.map().valid;let mut 么=三&乁;let mut 乫=么;let mut d=0u8;while!么.is_empty(){for q in 么.iter(){丿[q]=d;}let 七=(么.丏()&乁).业(乫);乫|=七;么=七;d=d.saturating_add(1);}丿}fn
働(l:&J)->I{let 丿=丳(l,l.丒(0));l.丒(1).iter().map(|q|丿[q]).min().unwrap_or(I::MAX)}struct 亿<'a>{乕:&'a Z,丂:F,theirs:[I;M],伅:[I;M],free:G,临:G,ours:G,丷:G,万:G,}impl
亿<'_>{fn new<'a>(l:&J,乕:&'a Z)->亿<'a>{let 丂=l.丛();let 丵=丳(l,l.丒(丂));let 世=丳(l,l.丒(1-丂));let 伅=丳(l,G::q(U[1-丂]));let 並=l.map().valid.业(l.丒(0)|l.丒(1));let
mut 临=G::T;for q in 並.iter(){if 丵[q]<=世[q]{临.set(q);}}亿{乕,丂,theirs:世,伅,free:並,临,ours:l.丒(丂),丷:l.map().丷,万:l.万(丂),}}fn value(&self,q:F)->C{let 一=self;let
p=一.乕;let 丠=C::from((G::q(q).丏()&一.free).count());let 俳=p.俺;let 佇=C::from(俳)-C::from(一.theirs[q].min(俳));let 仲=if p.侙>0{C::from(一.伅[q].min(30))/p.侙}else{0};let
丧=C::from((G::q(q).丏()&一.ours).count());let line=if 丧<=1{p.伄}else{0};let 习=if 一.丷.test(q){p.习}else{0};let 儑=if 丠==0{p.仴}else{0};let 仌=p.仌*(22-C::from(一.伅[q].min(22)));let
仭=p.仭*C::from((G::q(q).丏()&一.万).count());p.open*丠+p.佇*佇-仲+p.仱*丧-line+习-儑+仌-仭}fn 专(&self,丐:G)->P<F>{丐.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
侈(&mut self,q:F){let 一=self;一.free.clear(q);一.临.clear(q);一.ours.set(q);一.万.set(q);}}fn 伩(l:&mut J,丑:L,丄:&mut Vec<L>)->O{let 亢=l.乃(丑);if 亢{丄.push(丑);}亢}fn
倱(l:&mut J,三:F,丣:F,丄:&mut Vec<L>)->O{let Some((上,_))=l.人(三)else{return false;};let(x,y)=伶(丣);伩(l,L::乳{上,x,y},丄)}fn 儢(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-俣}fn
價(l:&J,乕:&Z)->K<L>{佢(l,乕,乔::乴)}#[derive(Clone,Copy,PartialEq)]enum 乔{乴,亪{丶:I,万:I},}fn 佢(l:&J,乕:&Z,乢:乔)->K<L>{let mut 丌=l.clone();let mut 且=亿::new(l,乕);let
mut 丄=K::new();亵(&mut 丌,&mut 且,&mut 丄);match 乢{乔::乴=>僱(&mut 丌,&mut 且,&mut 丄),乔::亪{丶,万}=>{僇(&mut 丌,&mut 且,丶,万,&mut 丄)}}丄}fn 亵(l:&mut J,且:&mut 亿<'_>,丄:&mut
Vec<L>){let 丂=且.丂;let mut 亲:K<F> =K::new();loop{let 丨=l.乇(丂,1);let mut 仿:P<(Q,F)> =None;for 三 in l.乷().iter(){if 亲.contains(&三){continue;}let 亖=(G::q(三).丏()&且.free&丨).count();if
亖==0{亲.push(三);}else if 仿.is_none_or(|(n,_)|亖<n){仿=Some((亖,三));}}let Some((_,三))=仿 else{break;};且.万.clear(三);match 且.专(G::q(三).丏()&且.free&丨){Some(丣)if
倱(l,三,丣,丄)=>且.侈(丣),_=>{且.万.set(三);亲.push(三);}}}for 三 in 亲{if!l.乷().test(三){continue;}let 与=[且.临,且.free].into_iter().find_map(|丐|{丐.iter().min_by_key(|&q|(l.map().丗(三,q),q))});if
let Some(与)=与{倱(l,三,与,丄);}}}fn 僱(l:&mut J,且:&mut 亿<'_>,丄:&mut Vec<L>){let 丂=且.丂;let p=且.乕;let h=(p.佒-C::from(l.乨())).max(p.倓);let 儜=(C::from(働(l))-p.俹).max(0)/p.俸.max(1)+p.佃;loop{let
下=l.下(丂);let 万=l.侹(丂)as C;let 僸=C::from(且.临.count());let q=且.专(l.仆(丂)&且.free&l.乇(丂,1));let 倿=儢(h,(僸/(万+1)).min(儜));let 俲=q.is_some()&&下>=俣&&倿>0;let 倔=4*h-争;let
习=倕(l,且);let 傴=习.is_some()&&下>=争&&倔>0;if 傴&&(!俲||倔>=倿){let(x,y)=伶(习.unwrap_or_default());if 伩(l,L::亩{x,y},丄){continue;}break;}let Some(q)=q.filter(|_|俲)else{break;};let(x,y)=伶(q);if!伩(l,L::丹{丅:1,x,y},丄){break;}且.侈(q);}}fn
倕(l:&J,且:&亿<'_>)->P<F>{let 丂=且.丂;(l.map().丷&l.丆(丂)&l.仓()).iter().filter(|&习|且.theirs[习]>且.乕.佟).min_by_key(|&习|(l.map().丗(U[丂],习),习))}fn 僇(l:&mut J,且:&mut
亿<'_>,丶:I,万:I,丄:&mut Vec<L>,){let 丂=且.丂;for _ in 0..丶{let Some(习)=倕(l,且)else{break;};let(x,y)=伶(习);if!伩(l,L::亩{x,y},丄){break;}}for _ in 0..万{let Some(q)=且.专(l.仆(丂)&且.free&l.乇(丂,1))else{break;};let(x,y)=伶(q);if!伩(l,L::丹{丅:1,x,y},丄){break;}且.侈(q);}}fn
乩(l:&J)->O{if l.両().is_some(){return false;}let 丂=l.丛();let 下=l.下(丂);let 丩=仦::new(l,丂,true);l.丒(1-丂).iter().any(|q|丩.不[q]<=下)}#[derive(Clone,Copy)]struct
丸{乍:I,乢:乔,}const 侽:I=I::MAX;const 偎:[乔;8]=[乔::乴,乔::亪{丶:0,万:0},乔::亪{丶:0,万:1},乔::亪{丶:0,万:2},乔::亪{丶:0,万:侽,},乔::亪{丶:1,万:0},乔::亪{丶:1,万:1},乔::亪{丶:1,万:侽,},];struct
佫<'a>{丯:&'a[Z],仄:Z,rest:丸,亰:P<亂>,乀:Settings2,}impl 佫<'_>{fn 乕(&self,丢:丸)->&Z{&self.丯[F::from(丢.乍)%self.丯.len()]}fn 丈(&self,l:&J,丢:丸)->K<L>{佢(l,self.乕(丢),丢.乢)}}fn
傥(l:&J,丈:&[L],仄:&Z)->(J,O){let mut 七=l.clone();七.作(丈);if 七.両().is_some()||乩(&七){return(七,true);}let 乹=價(&七,仄);七.作(&乹);let 亷=七.両().is_some()||乩(&七)||七.乨()>促;(七,亷)}fn
僰(l:&J,丙:&[丸],仒:&佫<'_>)->K<丮>{let 丂=l.丛();let mut 伡=vec![伮(l,丂)];let mut 乬=l.clone();for 且 in 0..促 as F{let 丢=丙.get(且).copied().unwrap_or(仒.rest);let 丈=仒.丈(&乬,丢);let(七,亷)=傥(&乬,&丈,&仒.仄);if
亷{break;}乬=七;伡.push(伮(&乬,丂));}伡}fn 伮(l:&J,丂:F)->丮{let mut hash:丮=0xcbf29ce484222325;let mut 侢=|伇:丮|hash=(hash^伇).wrapping_mul(0x0100000001b3);for(儓,乎)in[l.丒(丂),l.万(丂),l.丶()].into_iter().enumerate(){for
q in 乎.iter(){侢(((儓 as 丮)<<8)|q as 丮);}侢(0xff);}侢(l.下(丂)as 丮);hash}enum 于{伏{个:F,亝:F,丱:亼,佝:Q,},伹(Settings1),}struct 亡{亘:于,丯:K<Z>,仄:Z,丙:K<丸>,位:K<丮>,仝:F,义:Q,亷:O,}impl
亡{fn new(亘:于,丯:K<Z>,仄:Z)->亡{亡{亘,丯,仄,丙:K::new(),位:K::new(),仝:0,义:0,亷:false,}}fn 僮(&mut self,l:&J)->O{let 一=self;let 儆=matches!(一.亘,于::伏{..}|于::伹(_));let
僯=!儆&&一.义>0&&一.仝>=一.位.len();if!一.亷&&(僯||乩(l)){一.亷=true;}!一.亷}fn end(&mut self){self.亷=true;}fn 且(&mut self,l:&J,亰:P<亂>)->K<L>{let 一=self;let 仒=佫{丯:&一.丯,仄:一.仄,rest:丸{乍:0,乢:乔::乴,},亰,乀:Settings2::DEFAULT,};if
let 于::伹(乀)=一.亘{一.义+=1;一.仝+=1;return 且(l,&乀);}let 丂=l.丛();let 僤=一.位.get(一.仝)==Some(&伮(l,丂));let 僆=matches!(一.亘,于::伏{佝,..}if 佝>0&&l.乨()>=佝);if!僤||僆{let
乵=一.义==0;一.丙=match 一.亘{于::伏{个,亝,丱,..}=>{let 丱=&丱;let 乱=傆;let mut 乑=丙(l,&一.丯,乱,个,亝,亰,丱,);乑.resize(个,乱);乑}于::伹(_)=>K::new(),};一.位=match 一.亘{于::伏{..}=>儇(l,&一.丯,&一.丙),_=>僰(l,&一.丙,&仒),};一.仝=0;一.义+=1;}let
丢=一.丙.get(一.仝).copied().unwrap_or(仒.rest);一.仝+=1;仒.丈(l,丢)}}fn 儇(l:&J,丯:&[Z],丙:&[丸])->K<丮>{let 丂=l.丛();let mut 伡=K::with_capacity(丙.len());let mut 乬=l.clone();for&丢
in 丙{if 乬.両().is_some(){break;}伡.push(伮(&乬,丂));乬=佻(&乬,&丈(&乬,丯,丢));}伡}const 促:Q=20;const 傚:C=3;#[derive(Clone,Copy)]struct Settings2{个:C,}impl Settings2{const
DEFAULT:Settings2=Settings2{个:傚};}fn 乑(l:&J,三:G)->[I;M]{let 乁=l.map().valid;let mut 丿=[I::MAX;M];let mut 么=三&乁;let mut 乫=么;let mut d=0u8;while!么.is_empty(){for
q in 么.iter(){丿[q]=d;}let 七=(么.丏()&乁).业(乫);乫|=七;么=七;d=d.saturating_add(1);}丿}fn 儥(l:&J,丁:F)->C{let 两=乑(l,l.丆(丁));let 世=乑(l,l.丆(1-丁));l.map().valid.iter().map(|q|match
两[q].cmp(&世[q]){_ if 两[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 傍:[Z;27]=丯();const
傆:丸=丸{乍:16,乢:乔::乴,};const fn 丯()->[Z;27]{let 仱=[0,2,4];let 伄=[0,3,6];let 仌=[0,1,2];let mut 丄=[Z::仏;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
仴=if g>0||t>0{4}else{0};丄[i]=Z::仏.儫(仱[g],伄[t],0).優(仴,仌[w]);i+=1;}丄}#[derive(Default)]struct 併{丐:C,丞:C,下:C,亴:C,为:C,伳:C,丩:C,仢:C,}#[derive(Clone,Copy)]struct
亼{丐:C,丞:C,下:C,为:C,亴:C,伳:C,丩:C,仢:C,中:C,仫:O,乞:亍,}#[derive(Clone,Copy,PartialEq)]enum 亍{俎,佩(伽),亞(亞),係,}fn 儝(l:&J,丁:F)->[I;M]{let 两=丳(l,G::q(U[丁]));let 世=丳(l,G::q(U[1-丁]));let
mut 亥=G::T;let mut 临=G::T;for c in l.map().valid.iter(){if 两[c]==I::MAX||世[c]==I::MAX{continue;}if 两[c]==世[c]{亥.set(c);}if 两[c]<=世[c]{临.set(c);}}let d=丳(l,亥);let
mut 丄=[I::MAX;M];for c in 临.iter(){丄[c]=d[c];}丄}fn 傭(l:&J,丁:F,亥:&[I;M])->C{let d=丳(l,l.丆(丁));(0..M).filter(|&c|亥[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&亥[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 亞{偒{书:C},俜{书:C,代:C},傂,}impl
亞{fn 侻(self,a:C,b:C,d:C)->C{let 仟=(a-b).abs()/2;match self{亞::偒{书}=>(书-仟-(a+b-d)/2).max(0),亞::俜{书,代}if a<=代=>(书-仟).max(0),亞::俜{..}=>0,亞::傂=>a.min(d/2),}}}fn
僊(l:&J,丁:F,侒:亞)->[C;M]{let 两=丳(l,G::q(U[丁]));let 世=丳(l,G::q(U[1-丁]));let d=C::from(两[U[1-丁]]);let mut 丄=[0;M];for c in 0..M{if 两[c]!=I::MAX&&世[c]!=I::MAX{丄[c]=侒.侻(C::from(两[c]),C::from(世[c]),d);}}丄}#[derive(Clone,Copy,PartialEq)]enum
伽{俏(C),}impl 伽{fn 侻(self,仟:C)->C{match self{伽::俏(书)=>(书-仟).max(0),}}}type 伿=[C;4];fn 僼(l:&J,丁:F)->[C;M]{struct 伉{丆:G,乯:[伸;M],乙:[伸;M],size:[C;M],为:[C;M],介:伸,}fn
仡(d:&mut 伉,v:F){d.介+=1;d.乯[v]=d.介;d.乙[v]=d.介;d.size[v]=1;for w in(G::q(v).丏()&d.丆).iter(){if d.乯[w]==0{仡(d,w);d.size[v]+=d.size[w];d.乙[v]=d.乙[v].min(d.乙[w]);if
d.乙[w]>=d.乯[v]{d.为[v]+=d.size[w];}}else{d.乙[v]=d.乙[v].min(d.乯[w]);}}}let mut d=伉{丆:l.丆(丁),乯:[0;M],乙:[0;M],size:[0;M],为:[0;M],介:0,};let 了=U[丁];if d.丆.test(了){仡(&mut
d,了);}d.为[了]=0;d.为}fn 僾(l:&J,丁:F)->併{let 丆=l.丆(丁);let 为=僼(l,丁);let 伅=丳(l,G::q(U[1-丁]));let mut 丄=併{丐:C::from(丆.count()),丞:l.丞(丁),下:l.下(丁),丩:C::MAX,仢:儥(l,丁)/2,..併::default()};for
q in 丆.iter(){if 为[q]>0{丄.亴+=1;丄.为=丄.为.max(为[q]);}if q!=U[丁]&&(G::q(q).丏()&丆).count()<=1{丄.伳+=1;}丄.丩=丄.丩.min(C::from(伅[q]));}丄}fn 儤(l:&J,丁:F,丱:&亼,伣:G)->C{let
仙=C::from((l.丒(丁)&伣).count());let s=僾(l,丁);let o=丱;o.丐*s.丐+o.丞*s.丞+o.下*s.下-o.为*s.为-o.亴*s.亴-o.伳*s.伳-o.丩*s.丩.min(30)+o.仢*s.仢+o.中*仙}fn 仚(l:&J)->G{仐::of(l).仚()}fn
僝(l:&J)->G{let 两=丳(l,G::q(U[0]));let 世=丳(l,G::q(U[1]));let mut 乄=G::T;for c in l.map().valid.iter(){if 两[c]!=I::MAX&&世[c]!=I::MAX&&两[c].abs_diff(世[c])<=2{乄.set(c);}}乄}fn
企(l:&J)->[C;M]{let a=丳(l,G::q(U[0]));let b=丳(l,G::q(U[1]));let mut 丄=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丄[c]=C::from(a[c].abs_diff(b[c]))/2;}}丄}#[derive(Clone,Copy)]struct
仨{中:G,乄:G,乍:[C;M],企:[C;M],么:G,front:[I;M],}impl 仨{fn of(l:&J,丱:&亼)->仨{let 們=matches!(丱.乞,亍::俎);let mut 乍=[0;M];if let 亍::佩(傼)=丱.乞{for(w,仟)in 乍.iter_mut().zip(企(l)){*w=if
仟==C::MAX{0}else{傼.侻(仟)};}}if let 亍::亞(侒)=丱.乞{乍=僊(l,l.丛(),侒);}let(僷,么)=match 丱.乞{_=>([C::MAX;M],G::T),};仨{乍,企:僷,么,front:if 丱.乞==亍::係{儝(l,l.丛())}else{[I::MAX;M]},中:if
丱.中!=0||們{仐::of(l).仚()}else{G::T},乄:if 們{僝(l)}else{G::T},}}}fn 仵(l:&J,丁:F,丱:&亼,乂:仨)->伿{let 侵=儤(l,丁,丱,乂.中);match 丱.乞{亍::佩(_)|亍::亞(_)=>{let 丆=l.丆(丁);let
儧:C=丆.iter().map(|c|乂.乍[c]).sum();[C::from(丆.count()),儧,侵,0]}亍::係=>[C::from(l.丆(丁).count()),傭(l,丁,&乂.front),侵,0,],亍::俎=>{let 丒=l.丒(丁);let 中=C::from((丒&乂.中).count());let
乄=C::from((丒&乂.乄).count());match 丱.乞{_=>[C::from(l.丆(丁).count()),中,乄,侵],}}}}fn 佻(l:&J,丈:&[L])->J{let mut 七=l.clone();七.作(丈);if 七.両().is_none(){七.作(&[]);}七}const
侾:I=I::MAX;fn 丈(l:&J,丯:&[Z],丢:丸)->K<L>{if 丢.乍==侾{let 乀=Settings1{乕:丯.first().copied().unwrap_or(Z::仏),乢:if 丢.乢==乔::乴{亽::乴}else{亽::便},..Settings1::DEFAULT};return
且(l,&乀);}佢(l,&丯[F::from(丢.乍)%丯.len()],丢.乢,)}fn 倣(l:&J,丯:&[Z],丙:&[丸],乱:丸,个:F,)->J{let mut 乬=l.clone();for 且 in 0..个{if 乬.両().is_some(){break;}let 丢=丙.get(且).copied().unwrap_or(乱);乬=佻(&乬,&丈(&乬,丯,丢));}乬}fn
儍(l:&J,丯:&[Z],仫:O)->K<(丸,K<L>)>{let mut 丄:K<(丸,K<L>)> =K::new();if 仫{for 乢 in[乔::乴,乔::亪{丶:0,万:I::MAX,},]{let 丢=丸{乍:侾,乢,};let c=丈(l,丯,丢);if!丄.iter().any(|(_,乆)|*乆==c){丄.push((丢,c));}}}for
乍 in 0..丯.len().min(F::from(I::MAX)){for&乢 in&偎{let 丢=丸{乍:乍 as I,乢,};let c=丈(l,丯,丢);if!丄.iter().any(|(_,乆)|*乆==c){丄.push((丢,c));}}}丄}fn 傫(l:&J,丯:&[Z],乱:丸,个:F,亝:F,亰:P<亂>,丱:&亼,)->K<丸>{let
丂=l.丛();let 伨=||亰.is_some_and(|t|亂::now()>=t);let 乂=仨::of(l,丱);let 伪=|丙:&[丸]|仵(&倣(l,丯,丙,乱,个),丂,丱,乂);let mut 专:(伿,K<丸>)=(伪(&[]),K::new());let mut 么:K<(J,K<丸>)>
=vec![(l.clone(),K::new())];for 傿 in 0..个{let mut 亯:K<(伿,J,K<丸>)> =K::new();let mut 乫:HashMap<丮,F> =HashMap::new();for(倘,丙)in&么{if 伨(){break;}for(丢,c)in
儍(倘,丯,丱.仫){let 七=佻(倘,&c);let mut 來=丙.clone();來.push(丢);let v=仵(&七,丂,丱,乂);let 仵=伮(&七,丂);match 乫.get(&仵){Some(&乗)if 亯[乗].0>=v=>{}Some(&乗)=>亯[乗]=(v,七,來),None=>{乫.insert(仵,亯.len());亯.push((v,七,來));}}}}if
亯.is_empty(){break;}亯.sort_by(|a,b|b.0 .cmp(&a.0));亯.truncate(亝);let 乶=傿+1==个||伨();for(i,(_,_,丙))in 亯.iter().take(if 乶{亝}else{1}).enumerate(){if i>0&&伨(){break;}let
v=伪(丙);if v>专.0{专=(v,丙.clone());}}if 乶{break;}么=亯.into_iter().map(|(_,s,p)|(s,p)).collect();}专.1}fn 丙(l:&J,丯:&[Z],乱:丸,个:F,僜:F,亰:P<亂>,丱:&亼,)->K<丸>{let 丂=l.丛();let
乂=仨::of(l,丱);let 伪=|丙:&[丸]|仵(&倣(l,丯,丙,乱,个),丂,丱,乂);let mut 专:(伿,K<丸>)=(伪(&[]),K::new());let mut 亝=2;loop{let 之=傫(l,丯,乱,个,亝,亰,丱);let v=伪(&之);if v>专.0{专=(v,之);}let
伨=亰.is_some_and(|t|亂::now()>=t);if 伨||(亰.is_none()&&亝>=僜){break;}亝*=2;}专.1}const 偢:F=4+12;fn 个(侚:impl BufRead,mut 佚:impl Write,mut 佽:impl FnMut(&str,&str,Q,亂)->乖,)->Result<(),乖>{let
mut 久=俐{倧:侚};let Some(乵)=久.line()?else{return Ok(());};let mut 仳=format!("{}\n",乵);久.take(丘(&乵,"mine spot count")?,&mut 仳)?;let mut 且=0;loop{let Some(乵)=久.line()?else{return
Ok(());};let 丽=亂::now();且+=1;let mut 亇=format!("{}\n",乵);久.take(偢-1,&mut 亇)?;for 乚 in["building count","unit count"]{let line=久.倩()?;let n=丘(&line,乚)?;亇.push_str(&line);亇.push('\n');久.take(n,&mut
亇)?;}let 佽=佽(&仳,&亇,且,丽);writeln!(佚,"{}",佽).and_then(|()|佚.flush()).map_err(|仔|format!("turn {}: cannot write the answer: {}",且,仔))?;}}struct 俐<R>{倧:R,}impl<R:BufRead>俐<R>{fn
line(&mut self)->Result<P<乖>,乖>{let mut line=乖::new();match self.倧.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(仔)=>Err(format!("cannot read input: {}",仔)),}}fn
倩(&mut self)->Result<乖,乖>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,亇:&mut String)->Result<(),乖>{for
_ in 0..n{亇.push_str(&self.倩()?);亇.push('\n');}Ok(())}}fn 丘(line:&str,乚:&str)->Result<F,乖>{match line.trim().parse::<F>(){Ok(n)if n<=偮=>Ok(n),_=>Err(format!("bad {} {line:?}",乚)),}}const
偮:F=144;const 傕:C=3;const 偩:C=2;const 偳:F=16;fn 僬(l:&J,丄:&mut Vec<L>){let 丁=l.丛();let mut 亘=俚{s:l.clone(),丁,主:1-丁,与:U[1-丁],佑:U[丁],且:l.乨(),丄,};亘.run();}struct
俚<'a>{s:J,丁:F,主:F,与:F,佑:F,且:Q,丄:&'a mut Vec<L>,}fn 佥(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侣(上:Q,q:F)->L{let(x,y)=佥(q);L::乳{上,x,y}}fn 值(丅:I,q:F)->L{let(x,y)=佥(q);L::丹{丅,x,y}}impl
俚<'_>{fn run(&mut self){let 一=self;if 一.佖(){return;}一.亵();if 一.佖(){return;}一.乽();一.些();}fn 丿(&self,三:F,丣:F)->C{C::from(self.s.map().丗(三,丣))}fn 乃(&mut self,丑:L)->O{let
乣=self.s.乃(丑);if 乣{self.丄.push(丑);}乣}fn 伆(&self,串:&J)->O{串.両()==Some(乮::亏(self.丁))}fn 佖(&mut self)->O{let 一=self;if 一.s.両().is_some(){return false;}for
q in 一.s.乷().iter(){if 一.丿(q,一.与)!=1{continue;}let Some((上,_))=一.s.人(q)else{continue;};let 丑=侣(上,一.与);let mut 串=一.s.clone();if 串.乃(丑)&&一.伆(&串){return 一.乃(丑);}}let
Some(东)=一.債()else{return false;};let mut 串=一.s.clone();if!东.iter().all(|&丑|串.乃(丑))||!一.伆(&串){return false;}for 丑 in 东{一.乃(丑);}true}fn 債(&self)->P<K<L>>{let
一=self;const 俇:C=C::MAX;let 亅=一.s.丼().亓();let 丨:K<G> =(1..=亅).map(|丅|一.s.乇(一.丁,丅)).collect();let 伜=|q:F|{丨.iter().position(|乎|乎.test(q)).map(|丟|(丟 as I+1,10*(丟
as C+1)))};let 丆=一.s.丆(一.丁);let mut 不=[俇;M];let mut 乊=[F::MAX;M];let mut 乣=[false;M];for q in 丆.iter(){不[q]=0;}loop{let mut 七:P<F> =None;for q in 0..M{if!乣[q]&&不[q]<俇&&七.is_none_or(|n|不[q]<不[n]){七=Some(q);}}let
Some(q)=七 else{break;};乣[q]=true;if q==一.与{break;}for&九 in&一.s.map().丏[0][q]{if 九==二{break;}let 九=F::from(九);if 乣[九]||丆.test(九){continue;}let Some((_,仈))=伜(九)else{continue;};if
不[q]+仈<不[九]{不[九]=不[q]+仈;乊[九]=q;}}}if 不[一.与]>一.s.下(一.丁){return None;}let mut 东=K::new();let mut q=一.与;while!丆.test(q){let(丅,_)=伜(q)?;东.push(值(丅,q));q=乊[q];}东.reverse();Some(东)}fn
乌(&self,乜:&J)->C{let(丂,今)=(self.丁,self.主);let 乊=&self.s;let 丘=|乎:G|C::from(乎.count());let 丧=丘(乜.丒(丂))-丘(乊.丒(丂));let 亨=丘(乊.丒(今))-丘(乜.丒(今));let 侠=丘(乊.丆(今))-丘(乜.丆(今));let
体=乊.仺(今)-乜.仺(今);let 丶=丘(乊.丶()&乊.丒(今))-丘(乜.丶()&乜.丒(今));12*丧+20*亨+8*(侠-亨).max(0)+5*体+25*丶}fn 倯(&self,q:F)->C{let 一=self;let 习=一.s.map().丷.test(q);if 习&&一.s.丼().佄()&&!一.s.丒(一.丁).test(q){20}else{0}}fn
俿(&self,乐:C)->O{let 一=self;一.s.万(一.主).iter().any(|q|一.丿(q,一.佑)<=乐)}fn 亵(&mut self){let 一=self;let mut 万:K<(C,F)> =一.s.乷().iter().map(|q|(一.丿(q,一.与),q)).collect();万.sort_unstable();for(仁,q)in
万{if 一.s.両().is_some(){return;}let Some((上,丅))=一.s.人(q)else{continue;};let 丨=一.s.乇(一.丁,丅);let 九=一.s.map().丏[一.丁][q];let mut 专:P<(C,F)> =None;for&七 in 九.iter().take_while(|&&七|七!=二){let
七=F::from(七);if!丨.test(七){continue;}let mut 串=一.s.clone();if!串.乃(侣(上,七)){continue;}let 丬=if 一.伆(&串){100000}else{一.乌(&串)+10*(仁-一.丿(七,一.与))+一.倯(七)};if 专.is_none_or(|(书,_)|丬>书){专=Some((丬,七));}}if
let Some((丬,七))=专{if 丬>0{一.乃(侣(上,七));}}}}fn 乽(&mut self){let 一=self;if 一.s.両().is_some(){return;}if 一.俿(傕){一.償();}else{一.丶();}}fn 償(&mut self){let 一=self;if!一.s.丼().仪()||一.s.下(一.丁)<争{return;}let
九=一.s.map().丏[一.丁][一.佑];let 丐:K<F> =九.iter().take_while(|&&q|q!=二).map(|&q|F::from(q)).collect();let 丵=一.s.丒(一.丁);if 丐.iter().any(|&q|丵.test(q)&&一.s.乒().test(q)){return;}for
q in 丐{let(x,y)=佥(q);if 一.乃(L::乼{x,y}){return;}}}fn 丶(&mut self){let 一=self;if!一.s.丼().佄(){return;}while 一.s.下(一.丁)>=争{let 並=一.s.map().丷&一.s.丒(一.丁)&一.s.丆(一.丁)&一.s.仓();let
Some(q)=並.iter().min_by_key(|&q|(一.丿(q,一.佑),q))else{return;};let(x,y)=佥(q);if!一.乃(L::亩{x,y}){return;}}}fn 些(&mut self){let 一=self;let 倪=if 一.且<=3{0}else{5};for
_ in 0..偳{if 一.s.両().is_some(){return;}let 下=一.s.下(一.丁);if 下<10{return;}let 僕=一.俿(偩);let 亀=一.s.仆(一.丁);let mut 专:P<(C,L)> =None;let mut 侱:P<(C,L)> =None;let
mut 俯=G::T;for 丅 in 1..=一.s.丼().亓(){let 丨=一.s.乇(一.丁,丅);let 亖=(亀&丨).业(俯);俯=丨;let 仈=10*C::from(丅);if 仈>下{continue;}for q in 亖.iter(){let 丑=值(丅,q);let mut
串=一.s.clone();if!串.乃(丑){continue;}let 价=串.下(一.丁);let 丞=串.丞(一.丁);let 体=一.s.仺(一.主)-串.仺(一.主);let 傦=if 体>0&&僕{价+丞>=0}else{丞>=0&&(价>=倪||体>0)};if!傦{continue;}let
乌=一.乌(&串);let 丬=乌+一.倯(q)-6*一.丿(q,一.与)-10*(C::from(丅)-1);if 乌>0{if 专.is_none_or(|(书,_)|丬>书){专=Some((丬,丑));}}else if 丞>=3&&价>=倪+10&&丅==1{if 侱.is_none_or(|(书,_)|丬>书){侱=Some((丬,丑));}}}}let
Some((_,丑))=专.or(侱)else{return;};if!一.乃(丑){return;}}}}fn 像(l:&J,丄:&mut Vec<L>){丄.clear();僬(l,丄);}const 亮:C=C::MAX;const 偲:F=8;struct 仦{不:[C;M],乊:[I;M],}fn
佉(l:&J,丁:F,亵:O,与:F)->C{例(l,丁,仛(l,丁,亵),与)}fn 例(l:&J,丁:F,仛:G,与:F)->C{let mut 不=亮;倃(l,丁,仛,|丢,new|{if new.test(与){不=10*丢 as C;}不!=亮});不}fn 僳(l:&J,丁:F,仛:G,乏:C)->G{let
mut 乩=G::T;倃(l,丁,仛,|丢,new|{if 10*丢 as C>乏{return true;}乩|=new;false});乩.业(l.丆(丁))}fn 仛(l:&J,丁:F,亵:O)->G{if 亵&&丁==l.丛(){l.乷()}else{G::T}}fn 倃(l:&J,丁:F,仛:G,mut
仡:impl FnMut(F,G)->O,){let 亅=l.丼().亓();let mut 伜=[G::T;3];let mut 乫=G::T;for 丅 in 1..=亅{let 乎=l.乇(丁,丅);伜[F::from(丅)-1]=乎.业(乫);乫|=乎;}let mut 丠=[l.丆(丁),G::T,G::T,G::T];for
q in(仛&l.万(丁)).iter(){if let Some(丅)=l.亜(q).filter(|&丅|丅<=亅){丠[0]|=G::q(q).丏()&l.乇(丁,丅);}}let mut 乣=G::T;for 丢 in 0..{let new=丠[丢%4].业(乣);丠[丢%4]=G::T;if
仡(丢,new){return;}乣|=new;let 七=new.丏().业(乣);for(仩,丐)in 伜.iter().enumerate(){丠[(丢+仩+1)%4]|=七&*丐;}if 丠.iter().all(|乎|乎.业(乣).is_empty()){break;}}}impl 仦{fn
new(l:&J,丁:F,亵:O)->仦{let 亅=F::from(l.丼().亓());let mut 丨=[G::T;3];for(丟,乎)in 丨.iter_mut().enumerate().take(亅){*乎=l.乇(丁,丟 as I+1);}let 丆=l.丆(丁);let 丏=&l.map().丏[0];let
mut 不=[亮;M];let mut 乊=[二;M];let mut 丠=[0u8;M];let mut 丘=0;let mut 倭=G::T;for q in 丆.iter(){不[q]=0;丠[丘]=q as I;丘+=1;}if 亵&&丁==l.丛(){for 三 in l.乷().iter(){let
Some(丅)=l.亜(三)else{continue;};let 乑=G::q(三).丏()&丨[F::from(丅)-1];for q in 乑.iter(){if 不[q]!=0{不[q]=0;丠[丘]=q as I;丘+=1;}}}}while 丘>0{let mut 专=0;for 丟 in
1..丘{let(a,b)=(F::from(丠[丟]),F::from(丠[专]));if(不[a],a)<(不[b],b){专=丟;}}let q=F::from(丠[专]);丘-=1;丠[专]=丠[丘];倭.set(q);for&七 in&丏[q]{if 七==二{break;}let 七=F::from(七);if
倭.test(七)||丆.test(七){continue;}let Some(丅)=丨[..亅].iter().position(|乎|乎.test(七))else{continue;};let 倻=不[q]+10*(丅 as C+1);if 倻<不[七]{if 不[七]==亮{丠[丘]=七 as
I;丘+=1;}不[七]=倻;乊[七]=q as I;}}}仦{不,乊}}fn path(&self,与:F)->K<F>{let mut path=K::new();if 与>=M||self.不[与]==亮{return path;}let mut q=与;while self.乊[q]!=二{path.push(q);q=F::from(self.乊[q]);}path.reverse();path}}struct
伻{丈:K<L>,不:C,}fn 僛(l:&J,丁:F,q:F)->P<I>{(1..=l.丼().亓()).find(|&丅|l.乇(丁,丅).test(q))}fn 僶(丌:&mut J,丂:F,丕:F,乏:C,丈:&mut Vec<L>,亙:&mut i64,)->O{loop{if 丌.両()==Some(乮::亏(丂)){return
true;}if 丌.丒(丂).test(丕)&&丌.丆(丂).test(丕){return true;}let 丩=仦::new(丌,丂,false);let 不=丩.不[丕];if 不==亮||*亙+不>乏{return false;}let Some(&q)=丩.path(丕).first()else{return
false;};let Some(丅)=僛(丌,丂,q)else{return false;};let 丑=L::丹{丅,x:(q%Y)as I,y:(q/Y)as I,};if!丌.乃(丑){return false;}*亙+=10*C::from(丅);丈.push(丑);}}fn 俭(l:&J,丂:F,乿:&[F],乏:C)->P<伻>{let
mut 丌=l.clone();let mut 丈=K::new();let mut 亙=0;for&丕 in 乿{if!僶(&mut 丌,丂,丕,乏,&mut 丈,&mut 亙){return None;}}(亙<=乏).then_some(伻{丈,不:亙,})}fn 侄(l:&J,与:F,乏:C)->P<伻>{if
乏<0||l.両().is_some()||与>=M||l.map().乡.test(与){return None;}let 丂=l.丛();if l.丒(丂).test(与)&&l.丆(丂).test(与){return Some(伻{丈:K::new(),不:0,});}let mut 专=俭(l,丂,&[与],乏);let
乵=仦::new(l,丂,false);let mut 乒:K<F> =(l.乒()&l.丒(1-丂)).iter().filter(|&乧|乧!=与&&乵.不[乧]!=亮).collect();乒.sort_by_key(|&乧|(l.map().丗(乧,与),乧));乒.truncate(偲);for
乧 in 乒{if let Some(东)=俭(l,丂,&[乧,与],乏){if 专.as_ref().is_none_or(|书|东.不<书.不){专=Some(东);}}}专}const 偰:F=20000;struct 似{丈:K<L>,不:C,丐:C,万:C,乽:C,丞:C,丰:C,}struct
佯{丐:C,万:C,乽:C,}fn 倊(l:&J,丁:F)->佯{let 丆=l.丆(丁);let 万=l.万(丁)&丆;let 丬=万.iter().filter_map(|q|l.亜(q)).map(|丅|10*C::from(丅)).sum();佯{丐:C::from(丆.count()),万:丬,乽:C::from(((l.丶()|l.乒())&丆).count()),}}struct
俞{丂:F,伈:G,乊:佯,丞:C,仹:F,乏:C,仾:F,path:K<L>,之:K<似>,index:HashMap<伋,F,BuildHasherDefault<DefaultHasher>>,}type 伋=[丮;3];fn 儩(mut 互:伋,q:F)->伋{互[q/64]|=1<<(q%64);互}impl
俞{fn 俽(&mut self,丌:&J,乶:P<F>,亨:伋,不:C,)->O{let 一=self;let 丂=一.丂;let 亖=match 乶{None=>丌.仆(丂),Some(q)=>G::q(q).丏(),}&一.伈;let 亖=亖.业(丌.丒(丂));let mut 丨=[G::T;3];for
丅 in 1..=丌.丼().亓(){丨[F::from(丅)-1]=丌.乇(丂,丅);}for q in 亖.iter(){let 亅=F::from(丌.丼().亓());let Some(丟)=丨[..亅].iter().position(|乎|乎.test(q))else{continue;};let
丅=丟 as I+1;let 仈=10*C::from(丅);if 不+仈>一.乏{continue;}if 一.仾>=偰{return false;}一.仾+=1;let mut 七=丌.clone();let 丑=L::丹{丅,x:(q%Y)as I,y:(q/Y)as I,};if!七.乃(丑){continue;}一.path.push(丑);let
伃=儩(亨,q);一.僴(&七,伃,不+仈);let 傽=一.path.len()<一.仹&&七.両().is_none();let 僎=!傽||一.俽(&七,Some(q),伃,不+仈);一.path.pop();if!僎{return false;}}true}fn 僴(&mut self,乜:&J,伃:伋,不:C){let
一=self;let 伧=1-一.丂;let 伦=倊(乜,伧);let 丐=一.乊.丐-伦.丐;let 万=一.乊.万-伦.万;if 丐<=0&&万<=0{return;}let 乽=一.乊.乽-伦.乽;let 丞=乜.丞(一.丂)-一.丞;let 儬=乜.両()==Some(乮::亏(一.丂));let
丰=if 儬{C::MAX/2}else{3*丐+万+10*乽+2*丞-不};if let Some(&乗)=一.index.get(&伃){if 不>=一.之[乗].不{return;}}let 为=似{丈:一.path.clone(),不,丐,万,乽,丞,丰,};match 一.index.get(&伃){Some(&乗)=>一.之[乗]=为,None=>{一.index.insert(伃,一.之.len());一.之.push(为);}}}}fn
俻(l:&J,仹:F,乏:C,乐:F)->K<似>{也(l,仹,乏,乐).0}fn 也(l:&J,仹:F,乏:C,乐:F)->(K<似>,F){if l.両().is_some()||仹==0||乐==0{return(K::new(),0);}let 丂=l.丛();let 伧=1-丂;let mut
侭=俞{丂,伈:l.丒(伧)|l.丒(伧).丏(),乊:倊(l,伧),丞:l.丞(丂),仹,乏,仾:0,path:K::new(),之:K::new(),index:HashMap::default(),};侭.俽(l,None,[0;3],0);let 仾=侭.仾;let mut 之=侭.之;之.sort_by(|a,b|b.丰.cmp(&a.丰).then(a.不.cmp(&b.不)));之.truncate(乐);(之,仾)}const
佂:C=1000000;const 偱:C=200;fn 临(l:&J,丁:F)->C{let 丆=l.丆(丁);let 万:C=(l.万(丁)&丆).iter().filter_map(|q|l.亜(q)).map(|丅|10*C::from(丅)).sum();l.下(丁)+3*l.丞(丁)+4*C::from(丆.count())+万+10*C::from((l.乒()&丆).count())}fn
倒(l:&J,丁:F,乸:G)->C{僌(l,丁,例(l,丁,乸,U[1-丁]),)}fn 僌(l:&J,丁:F,不:C)->C{if 不==亮{return C::MIN;}l.下(丁)+l.丞(丁)-不}fn 値(亦:C)->C{亦.clamp(-偱,0)}fn 僅(l:&J,丁:F,乸:[G;2],)->(C,[C;2]){let
乆=1-丁;match l.両(){Some(乮::亏(儨))=>{return(if 儨==丁{佂}else{-佂},[0,0])}Some(乮::俅)=>return(0,[0,0]),None=>{}}let mut 丰=临(l,丁)-临(l,乆);if 丁==l.丛(){let mut 七=l.clone();七.仯();if
七.両().is_none()&&佉(&七,乆,true,U[丁])<=七.下(乆){丰-=佂/2;}}let 仸=[倒(l,丁,乸[0]),倒(l,乆,乸[1]),];(丰+値(仸[0])-値(仸[1]),仸)}
