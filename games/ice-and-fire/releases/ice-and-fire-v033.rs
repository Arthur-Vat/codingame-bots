// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伸=u16;type Q=u32;type 丮=u64;type 伷=i32;type C=i64;type 亽=f64;type O=bool;type 乖=String;type 亂=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
偫:O=true;fn 僙(l:&J,q:F,伲:C,丨:G,东:C,书:G,)->O{let 丂=l.丛();let 丱=1-丂;let 倎=偫&&东<=l.下(丱)+2*l.丞(丱).max(0);let 們=书.test(q);let 亩=G::q(q)|G::q(q).丏();if!倎&&(!們||(亩&丨).is_empty()){return
false;}let(x,y)=乓(q);let mut 乜=l.clone();if!乜.乃(L::乽{x,y}){return false;}if 們&&伱(&乜,丂)<伲{return true;}倎&&佉(&乜,丱,false,U[丂])>东}const 储:O=false;const 傇:I=2;const
俞:I=1;const 俋:I=3;const 偛:Q=10;const 俟:I=2;fn 倛(l:&J,之:I)->G{let 丹=l.map().valid;let 丱=1-l.丛();let mut 亩=l.丆(丱)|l.万(丱);let mut 亶=之;while 亶!=0{亩|=亩.丏()&丹;亶-=1;}亩}fn
僘(l:&J)->G{if 俋==0{l.map().valid}else{倛(l,俋)}}const 俆:I=2;const 佀:F=0;const 佰:F=2;const 傎:O=false;const 傐:O=false;const 傏:O=false;const 傍:O=false;const
偲:F=4;const 俄:F=5;#[derive(Clone)]enum 丫{仐(L),事(K<L>),}impl 丫{fn 丈(&self)->&[L]{match self{丫::仐(丑)=>std::slice::from_ref(丑),丫::事(丈)=>丈,}}}fn 乓(q:F)->(I,I){((q%Y)as
I,(q/Y)as I)}fn 倈(l:&J,丄:&mut Vec<丫>){丄.clear();if l.両().is_some(){return;}let 丂=l.丛();let 亅=l.丽().亓();let mut 丩=[G::T;4];for 丅 in 1..=亅{丩[F::from(丅)]=l.么(丂,丅);}for
三 in l.乸().iter(){let Some((上,丅))=l.亼(三)else{continue;};let 之=G::q(三).丏()&丩[F::from(丅)];for 丣 in 之.iter(){let(x,y)=乓(丣);丄.push(丫::仐(L::乴{上,x,y}));}}let
下=l.下(丂);let 丆=l.丆(丂);let 书=l.仈(丂);let 乮=match 傇{0=>书,1=>{let 伔=l.map().valid.业(丆).丏()&丆;书.业(丆)|伔}_=>书.业(丆),};for 丅 in 1..=亅{let 不=10*C::from(丅);if 下<不{break;}let
mut 丐=乮&丩[F::from(丅)];for 僢 in 1..丅{丐=丐.业(丩[F::from(僢)]);}for q in 丐.iter(){let(x,y)=乓(q);丄.push(丫::仐(L::主{丅,x,y}));}}if 下>=争{let 並=l.丒(丂)&书&l.仕();if l.丽().佄(){for
q in(並&l.map().丸).iter(){let(x,y)=乓(q);丄.push(丫::仐(L::亪{x,y}));}}if l.丽().们(){let 伔=l.map().valid.业(l.丒(丂)).丏();let 丐=(並&伔).业(l.map().丸);let 丨=丨(l,1-丂);let
伲=C::from((丨&l.丆(丂)).count());let 东=佉(l,1-丂,false,U[丂]);let 书=僘(l);let 丠=if 储{let(亥,佐)=义(l,丂);亥.业(佐)}else{G::T};for q in 丐.iter().filter(|&q|{let 亩=G::q(q)|G::q(q).丏();!(亩&丠).is_empty()||僙(l,q,伲,丨,东,书)}){let(x,y)=乓(q);丄.push(丫::仐(L::乽{x,y}));}}}let
僗=match 俞{1=>争,2=>10,_=>C::MAX,};if 下>=僗{let mut 丨=丨(l,1-丂);if 俟!=0&&l.乤()<=偛{丨|=倛(l,俟);}let 五=l.map().valid.业(l.丒(丂)|l.丒(1-丂));for 三 in(l.乸()&丨).iter(){let
Some((上,丅))=l.亼(三)else{continue;};let 之=G::q(三).丏()&丩[F::from(丅)]&五;let(付,仙)=乓(三);let 侕=if 俞==1{if l.map().丸.test(三){continue;}L::乽{x:付,y:仙}}else{L::主{丅:1,x:付,y:仙,}};for
丣 in 之.iter(){let(x,y)=乓(丣);丄.push(丫::事(vec![L::乴{上,x,y},侕]));}}}if 俆!=0&&下>=争&&l.丽().们(){像(l,&丩,丄);}if 佀!=0&&下>=20{儂(l,丩[1],下,丄);}if 傍{僅(l,丩[1],下,丄);}for
丼 in 俽(l,俄,下,偲){if 丼.丈.len()>1{丄.push(丫::事(丼.丈));}}if let Some(东)=侄(l,U[1-丂],下){if 东.丈.len()>1{丄.push(丫::事(东.丈));}}}fn 像(l:&J,丩:&[G;4],丄:&mut Vec<丫>){let
丂=l.丛();let 丱=1-丂;let 丨=丨(l,丱);let 两=l.丒(丂);let 世=l.丒(丱);let 伔=世.丏();let 丸=l.map().丸;for 三 in(l.乸()&丨&l.丆(丂)).iter(){if 丸.test(三){continue;}let Some((上,丅))=l.亼(三)else{continue;};let
傰=l.map().丗(三,U[丱]);let 之=G::q(三).丏()&丩[F::from(丅)];let(付,仙)=乓(三);for 丣 in 之.iter(){let 佼=match 俆{1=>两.test(丣)&&l.map().丗(丣,U[丱])>傰,2=>两.test(丣)&&伔.test(丣),_=>两.test(丣)||(世.test(丣)&&丣!=U[丱]),};if!佼{continue;}let(x,y)=乓(丣);丄.push(丫::事(vec![L::乴{上,x,y},L::乽{x:付,y:仙},]));}}}fn
儂(l:&J,依:G,下:C,丄:&mut Vec<丫>){let 丂=l.丛();let 严=l.map();let 五=严.valid.业(l.丒(丂)|l.丒(1-丂));let 丠=五&依;let 乮:K<F> =(l.丆(丂).丏()&丠).iter().collect();if 乮.is_empty(){return;}let
侥=|与:F|{乮.iter().copied().min_by_key(|&q|(严.丗(q,与),q))};let mut 侅:K<(F,F)> =K::new();if 傎{for 丕 in 儃(l,&乮){if let Some(举)=侥(丕){侅.push((举,丕));}}}else{let
与=U[1-丂];let mut 侲=乮.clone();侲.sort_by_key(|&q|(严.丗(q,与),q));侲.truncate(佰);侅.extend(侲.into_iter().map(|举|(举,与)));}let 仼=F::try_from(下/10).unwrap_or(0).min(佀);for(举,与)in
侅{let mut 东=vec![举];let mut 亨=G::q(举);while 东.len()<仼{let 乷=东[东.len()-1];if 乷==与{break;}let 七=(G::q(乷).丏()&丠).业(亨).iter().min_by_key(|&q|(严.丗(q,与),q));let
Some(七)=七 else{break;};东.push(七);亨.set(七);let 丈=东.iter().map(|&q|{let(x,y)=乓(q);L::主{丅:1,x,y}}).collect();丄.push(丫::事(丈));}if 傏{僟(l,&东,丄);}}}fn 僟(l:&J,东:&[F],丄:&mut
Vec<丫>){let 丂=l.丛();let 举=东[0];let 乹=l.乸()&l.万(丂)&G::q(举).丏();for 三 in 乹.iter(){let Some((上,丅))=l.亼(三)else{continue;};if!l.么(丂,丅).test(举){continue;}let(x,y)=乓(举);let
mut 丈=vec![L::乴{上,x,y}];for&q in&东[1..]{let(x,y)=乓(q);丈.push(L::主{丅:1,x,y});丄.push(丫::事(丈.clone()));}}}fn 儃(l:&J,乮:&[F])->K<F>{let 丂=l.丛();let 严=l.map();let(乄,中)=住(l);let
两:K<F> =l.万(丂).iter().collect();let 世:K<F> =l.万(1-丂).iter().collect();let 低=|丕:F|{乮.iter().map(|&q|严.丗(q,丕)).min().unwrap_or(I::MAX)};let 傾=|丕:F|{let 丨=低(丕);两.iter().any(|&亻|{let
d=严.丗(亻,丕);d<=丨&&世.iter().all(|&丱|d<严.丗(丱,丕))})};let 侠=|丕:F|{傐&&(l.丒(1-丂).test(丕)||世.iter().any(|&丱|严.丗(丱,丕)<低(丕)))};let 丠=|丐:G|->K<F>{let mut 亀:K<F> =丐.业(l.丒(丂)).iter().filter(|&丕|低(丕)!=I::MAX&&!傾(丕)&&!侠(丕)).collect();亀.sort_by_key(|&丕|(低(丕),丕));亀};let
mut 亀=丠(中);if 亀.is_empty(){亀=丠(乄);}亀.truncate(佰);亀}fn 僀(l:&J)->K<F>{let 丂=l.丛();let 严=l.map();let(乄,中)=住(l);let 丆=l.丆(丂);let 两=l.丒(丂);let 丨=丨(l,1-丂);let
世:K<F> =l.万(1-丂).iter().collect();let 倆=|q:F|{丆.iter().map(|a|严.丗(a,q)).min().unwrap_or(I::MAX)};let mut 丐:K<F> =(乄|中).iter().filter(|&q|{if 两.test(q){丆.test(q)&&丨.test(q)}else{!l.丒(1-丂).test(q)&&世.iter().any(|&e|严.丗(e,q)<倆(q))}}).collect();丐.sort_by_key(|&q|(I::from(!中.test(q)),倆(q),q));丐.truncate(
2*佰);丐}fn 僅(l:&J,依:G,下:C,丄:&mut Vec<丫>){let 丂=l.丛();let 严=l.map();let 五=严.valid.业(l.丒(丂)|l.丒(1-丂));let 丠=五&依;let 丆=l.丆(丂);let 並=丆&l.仕();let 僡=if 佀==0{3}else{佀};for
q in 僀(l){if!l.丒(丂).test(q){let 举=(丆.丏()&丠).iter().min_by_key(|&c|(严.丗(c,q),c));if let Some(举)=举{let mut 东=vec![举];let mut 亨=G::q(举);while 东[东.len()-1]!=q&&东.len()<僡{let
乷=东[东.len()-1];let 七=(G::q(乷).丏()&丠).业(亨).iter().min_by_key(|&c|(严.丗(c,q),c));let Some(七)=七 else{break;};东.push(七);亨.set(七);}let 不=10*C::try_from(东.len()).unwrap_or(C::MAX);if
东.len()>=2&&东[东.len()-1]==q&&不<=下{let 丈=东.iter().map(|&c|{let(x,y)=乓(c);L::主{丅:1,x,y}}).collect();丄.push(丫::事(丈));}}}if 下>=争&&l.丽().们(){let 丸=(G::q(q)|G::q(q).丏())&並;for
乩 in 丸.业(严.丸).iter(){let(x,y)=乓(乩);丄.push(丫::仐(L::乽{x,y}));}}}}const 偿:I=2;const 傜:C=5;fn 乹(l:&J,丁:F)->G{侤(l,丁,偿)}fn 侤(l:&J,丁:F,三:I)->G{let mut 乹=G::T;if
三>3{return 乹;}for q in(l.万(丁)&l.丆(丁)).iter(){if l.亜(q).is_some_and(|丅|丅>=三){乹|=G::q(q);}}乹}fn 丨(l:&J,丵:F)->G{let 乑=l.下(丵)+l.丞(丵);僻(l,丵,乹(l,丵),乑)}fn 亣(l:&J,丁:F)->G{丨(l,1-丁)&l.丆(丁)}fn
伱(l:&J,丁:F)->C{C::from(亣(l,丁).count())}const 俓:C=4;fn 僦(丐:C)->C{丐/俓}fn 丷(l:&J,丁:F,亣:G)->C{let 丆=l.丆(丁);let 丷=(l.丷()&丆).业(亣);let 丐=C::from(丆.count());let
侀=if l.乤()<=做{丐/偙+偘}else{僦(丐)};侀*C::from(丷.count())}const 做:Q=15;const 偘:C=6;const 偙:C=俓;const 傕:C=2;const 佱:I=1;const 傗:I=3;const 傖:I=6;fn 僞(仅:I,书:I,伝:I,d:I)->I{match
仅{0=>4,1 if d<=书=>4,1=>0,_ if d<=书=>4,_ if d>=伝=>0,_=>(4*Q::from(伝-d)/Q::from(伝-书))as I,}}fn 偉(l:&J,丁:F)->C{let 丆=l.丆(丁);let 丱=if 佱==0{G::T}else{l.丆(1-丁)};let[a,b,c]=丆.伵();let[e,f,g]=丱.伵();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丁 as 丮).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丟=(hash as F)&(俨-1);傩.with(|侂|{let mut 侂=侂.borrow_mut();let 介=&mut 侂[丟];if 介.0==丆&&介.1==丱&&介.2==丁 as I+1{return 介.3;}let 丬=if 佱==0{儮(丆,U[丁])}else{let d=临(l,丱);let
mut 乏=[0u8;M];for(w,&之)in 乏.iter_mut().zip(d.iter()){*w=僞(佱,傗,傖,之);}偊(丆,U[丁],&乏)};*介=(丆,丱,丁 as I+1,丬);丬})}const 俨:F=1<<12;thread_local!{static 傩:std::cell::RefCell<K<(G,G,I,C)>>
=std::cell::RefCell::new(vec![(G::T,G::T,0,0);俨]);}fn 儮(丆:G,了:F)->C{偊(丆,了,&[4;M])}fn 偊(丆:G,了:F,乏:&[I;M])->C{struct 伉<'a>{乏:&'a[I;M],丆:G,买:[I;M],乙:[I;M],size:[I;M],仍:I,亵:C,仼:C,}fn
代(d:&mut 伉<'_>,v:F)->I{d.仍+=1;let 买=d.仍;d.买[v]=买;let mut 乙=买;let mut 侮=1u8;let mut 丼=0u8;for w in(G::q(v).丏()&d.丆).iter(){if d.买[w]==0{let 值=代(d,w);侮+=值;乙=乙.min(d.乙[w]);if
d.乙[w]>=买{丼+=值;}}else{乙=乙.min(d.买[w]);}}d.乙[v]=乙;d.size[v]=侮;if 丼>0&&d.买[v]>1{let w=C::from(d.乏[v]);d.亵+=w;d.仼=d.仼.max(w*C::from(丼));}侮}if!丆.test(了){return
0;}let mut d=伉{乏,丆,买:[0;M],乙:[0;M],size:[0;M],仍:0,亵:0,仼:0,};代(&mut d,了);(d.亵+d.仼)/4}fn 儅(l:&J,丁:F)->C{傕*(偉(l,1-丁)-偉(l,丁))}fn 俾(l:&J,丁:F)->C{if l.下(丁)+l.丞(丁)>=0{return
0;}(l.万(丁)&l.丆(丁)).iter().filter_map(|q|l.亜(q)).map(|丅|10*C::from(丅)).sum()}const 偱:C=15;const 傤:[C;2]=[3,1];const 傢:[C;2]=[4,20];fn 儘(l:&J,丁:F)->[C;2]{let
mut 丘=[0;2];for q in(l.万(丁)&l.丆(丁)).iter(){match l.亜(q){Some(2)=>丘[0]+=1,Some(3)=>丘[1]+=1,_=>{}}}丘}fn 倻(l:&J,丁:F)->C{let[偂,儢]=儘(l,丁);let 俰=|丅:F|(3-傤[丅])*傢[丅];-偱*偂+俰(0)*偂+俰(1)*儢}const
偪:C=0;const 偭:C=2;const 偬:C=100;fn 倴(亦:C)->C{if 亦<0{return 0;}偪+偭*亦.min(偬)/2}const 俒:I=2;const 俎:C=2;const 俍:C=4;const 偮:I=3;fn 亳(l:&J,丁:F)->C{if 俎==0{return
0;}let 丹=l.map().valid;let mut 万=G::T;for q in(l.万(丁)&l.丆(丁)).iter(){if l.亜(q).is_some_and(|丅|丅<=偮){万|=G::q(q);}}let mut 以=丹.业(l.丒(丁));let mut 仂=G::T;let
mut 之=0;for 丗 in 1..=俍{以=(以|以.丏())&丹;let 仃=(万&以).业(仂);之+=(丗-1)*C::from(仃.count());仂|=仃;}之+=俍*C::from(万.业(仂).count());俎*之}const 俧:C=8;fn 义(l:&J,丁:F)->(G,G){let
丆=l.丆(丁);let 亥=丆&l.丒(1-丁).丏();let 乇=l.乇()&丆;(亥,亥&(乇|乇.丏()))}fn 假(l:&J,丁:F)->C{if 俧==0{return 0;}俧*C::from(义(l,丁).1 .count())}const 佬:C=0;const 俉:I=1;const
俊:C=4;const 佭:C=0;const 俈:C=0;const 偡:Q=14;const 偠:C=2;const 俦:C=0;const 傧:Q=14;fn 令(l:&J,丁:F)->C{if 俦==0||l.乤()>傧{return 0;}let 两=临(l,l.丆(丁));let 世=临(l,l.丆(1-丁));let
mut 仞=0;for c in l.map().valid.iter(){match 两[c].cmp(&世[c]){std::cmp::Ordering::Less=>仞+=1,std::cmp::Ordering::Greater=>仞-=1,std::cmp::Ordering::Equal=>{}}}俦*仞}struct
俌{valid:G,乄:G,中:G,}thread_local!{static 偢:std::cell::RefCell<P<俌>> =const{std::cell::RefCell::new(None)};}fn 住(l:&J)->(G,G){偢.with(|q|{let mut 乂=q.borrow_mut();let
丹=l.map().valid;if 乂.as_ref().is_none_or(|z|z.valid!=丹){let 伂=伂(l);let mut 乄=G::T;for c in 丹.iter(){if 伂[c]<=偠{乄.set(c);}}*乂=Some(俌{valid:丹,乄,中:仜(l),});}乂.as_ref().map_or((G::T,G::T),|z|(z.乄,z.中))})}fn
佐(l:&J,丁:F)->G{let 丆=l.丆(丁);let 乇=l.乇()&丆;(乇|乇.丏())&丆}fn 丕(l:&J,丁:F)->C{if(佬==0&&俊==0&&佭==0&&俈==0)||l.乤()>偡{return 0;}let(乄,中)=住(l);let mut 亚=0;if 佬!=0{let
两=临(l,l.丆(丁));let 世=临(l,l.丆(1-丁));let mut 仞=0;for c in 乄.iter(){let(a,b)=(两[c],世[c]);if a!=I::MAX&&a.saturating_add(俉)<=b{仞+=1;}else if b!=I::MAX&&b.saturating_add(俉)<=a{仞-=1;}}亚+=佬*仞;}let
丘=|乐:G|C::from(乐.count());let(丶,世)=(l.丆(丁),l.丆(1-丁));亚+=俊*(丘(丶&乄)-丘(世&乄));if 佭!=0{亚+=佭*(丘(佐(l,丁)&乄)-丘(佐(l,1-丁)&乄));}亚+=俈*(丘(丶&中)-丘(世&中));亚}fn 僋(l:&J,丁:F)->C{let
乹=[侤(l,丁,俒),侤(l,1-丁,俒),];let(丰,仹)=僌(l,丁,乹);let 丰=丰+倴(仹[0])-倴(仹[1]);if l.両().is_some(){return 丰;}let 两=亣(l,丁);let 世=亣(l,1-丁);let 伲=C::from(世.count())-C::from(两.count());丰-2*俾(l,丁)+2*俾(l,1-丁)+儅(l,丁)+倻(l,丁)-倻(l,1-丁)-亳(l,丁)+亳(l,1-丁)+傜*伲/4+丷(l,丁,两)-丷(l,1-丁,世)+假(l,丁)-假(l,1-丁)+丕(l,丁)+令(l,丁)}const
偻:F=8;const 傊:F=3;const 偳:F=32;const 傋:丮=300;const 偕:亽=0.75;const 偧:P<Q> =Some(7);const 健:O=false;const 偤:C=10;const 偦:Q=6;struct 京{人:P<亂>,仚:P<亂>,伢:丮,}#[derive(Default)]struct
佶{丈:K<L>,侊:F,亲:丮,丝:F,乊:F,俸:F,交:F,亁:O,倸:O,仉:F,}const 傦:O=true;#[derive(Clone)]struct 亄{丈:K<L>,侍:C,}struct 仧{丈:K<L>,end:J,first:P<丮>,丧:P<K<亄>>,}type 伊=HashMap<丮,C,BuildHasherDefault<DefaultHasher>>;struct
佴{丂:F,limit:京,亲:丮,伟:伊,仢:P<K<丫>>,侴:O,乘:P<仦>,}const 佲:C=0;const 俁:O=true;const 偐:O=false;const 俀:P<I> =None;fn 債(l:&J,丝:&[亄],丄:&mut Vec<丫>){if!俁&&俀.is_none(){return;}let
伭=丝.iter().map(|乺|乺.丈.as_slice());傶(l,伭,(俁,偐,俀),丄,);}fn 傶<'a>(l:&J,伭:impl Iterator<Item=&'a[L]>,(乇,丏,丅):(O,O,P<I>),丄:&mut Vec<丫>,){let 丂=l.丛();let 丶=l.丆(丂);let
mut 伛=G::T;for 乺 in 伭{for 丑 in 乺{if let L::主{x,y,..}|L::乴{x,y,..}=*丑{let q=F::from(y)*Y+F::from(x);if 丶.test(q){伛|=G::q(q);}}}}if 伛.is_empty(){return;}let
並=l.仕()&丶;let 下=l.下(丂);let 倩=|丑:L,丄:&mut Vec<丫>|{if!丄.iter().any(|a|a.丈()==[丑]){丄.push(丫::仐(丑));}};if 乇&&下>=争&&l.丽().们(){let mut 丐=伛;if 丏{丐|=伛.丏();}for
q in(丐&並).业(l.map().丸).iter(){let(x,y)=乓(q);倩(L::乽{x,y},丄);}}if let Some(丅)=丅.filter(|&丅|丅<=l.丽().亓()){if 下>=10*C::from(丅){for q in(伛&並).iter(){let(x,y)=乓(q);倩(L::主{丅,x,y},丄);}}}}fn
佾(l:&mut J,丈:&[L])->O{丈.iter().all(|&丑|l.乃(丑))}fn 互(丈:&[L])->丮{let mut hash:丮=0xcbf29ce484222325;for 丑 in 丈{let(佔,a,b,c)=match*丑{L::乴{上,x,y}=>(1,丮::from(上),x,y),L::主{丅,x,y}=>(2,丮::from(丅),x,y),L::亪{x,y}=>(3,0,x,y),L::乽{x,y}=>(4,0,x,y),};let
伈=佔|a<<8|丮::from(b)<<40|丮::from(c)<<48;hash=(hash^伈).wrapping_mul(0x0100000001b3);}hash}fn 俫(乱:&J,乺:&[L])->J{let mut 七=乱.clone();七.仱();for&丑 in 乺{七.乃(丑);}七.仱();七}fn
佖(l:&J)->P<K<L>>{let 丂=l.丛();let 与=U[1-丂];let 伇=|丳:&J|丳.両()==Some(乯::亏(丂));for q in l.乸().iter(){if l.map().丗(q,与)!=1{continue;}let Some((上,_))=l.亼(q)else{continue;};let(x,y)=乓(与);let
丑=L::乴{上,x,y};let mut 丳=l.clone();if 丳.乃(丑)&&伇(&丳){return Some(vec![丑]);}}let 东=侄(l,与,l.下(丂))?;let mut 丳=l.clone();(佾(&mut 丳,&东.丈)&&伇(&丳)).then_some(东.丈)}fn
债(l:&J,丁:F,个:C)->O{let 不=例(l,丁,l.万(丁),U[1-丁]);不!=亯&&l.下(丁)+个*l.丞(丁)>=不}fn 儗(l:&J)->O{let 丂=l.丛();if 佲==0||l.両().is_some(){return false;}let mut 七=l.clone();七.仱();let
伱=七.両().is_none()&&佉(&七,1-丂,true,U[丂])<=七.下(1-丂);!伱&&债(l,丂,佲)}impl 佴{fn 俲(&self,乱:&J)->O{self.侴&&乱.両()!=Some(乯::亏(self.丂))&&!债(乱,self.丂,佲)}fn 亙(&self)->O{self.亲>=self.limit.伢||self.limit.人.is_some_and(|t|亂::now()>=t)}fn
丰(&mut self,乱:&J,丝:&[亄],侁:C)->C{self.亲+=1;self.侪(乱,丝,侁)}fn 侪(&self,乱:&J,丝:&[亄],侁:C)->C{if 乱.両()==Some(乯::亏(self.丂)){return 佂;}if 丝.is_empty(){return self.value(乱);}let
mut 佤=C::MAX;for 乺 in 丝{佤=佤.min(self.value(&俫(乱,&乺.丈)));if 佤<=侁{break;}}佤}fn value(&self,l:&J)->C{let 伓=僋(l,self.丂);match&self.乘{Some(乘)if l.両().is_none()=>伓+乘.儝(l,self.丂),_=>伓,}}fn
俪(&self,乛:&mut Vec<丫>){if let Some(乘)=&self.乘{for 佛 in&乘.仉{乛.push(丫::事(佛.clone()));}}}#[allow(clippy::too_many_arguments)]fn 僲(&mut self,乿:&J,仰:C,乛:&[丫],乞:&[(C,F)],乣:&mut[O],丝:&[亄],仲:&mut
伊,供:O,)->(P<(F,J,C)>,O){let mut 专=None;let mut 但=仰;for&(佘,丟)in 乞{if 乣[丟]{continue;}if 供&&佘!=C::MAX&&佘<=但-仰{return(专,true);}if self.亙(){return(专,false);}乣[丟]=true;let
丈=乛[丟].丈();let mut 七=乿.clone();if!佾(&mut 七,丈)||self.俲(&七){仲.insert(互(丈),C::MIN/4);continue;}let 丰=self.丰(&七,丝,但);仲.insert(互(丈),丰-仰);if 丰>但{但=丰;专=Some((丟,七,丰));}}(专,false)}fn
侉(&mut self,了:&J,丝:&[亄],伒:&[丮],傹:O)->仧{let 一=self;let mut 乶=None;let mut 乿=了.clone();let mut 丈=K::new();let mut 仰=一.丰(&乿,丝,C::MIN);let mut 仲=伊::default();let
mut 乛=K::new();let mut 乞=K::new();while 乿.両().is_none()&&!一.亙(){if 丈.is_empty()&&傹{match&一.仢{Some(仢)=>乛.clone_from(仢),None=>{倈(&乿,&mut 乛);if let Some(乘)=一.乘.as_ref().filter(|g|g.倡){乛.push(丫::事(乘.丙.clone()));}一.俪(&mut
乛);一.仢=Some(乛.clone());}}}else{倈(&乿,&mut 乛);一.俪(&mut 乛);}債(&乿,丝,&mut 乛);乞.clear();for(丟,亐)in 乛.iter().enumerate(){let k=互(亐.丈());let 佘=仲.get(&k).or(一.伟.get(&k)).copied();乞.push((佘.unwrap_or(C::MAX),丟));}乞.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乣=vec![false;乛.len()];if 丈.is_empty(){for(丟,亐)in 乛.iter().enumerate(){乣[丟]=伒.contains(&互(亐.丈()));}}let mut 专=None;for
供 in[true,false]{let(乍,儍)=一.僲(&乿,仰,&乛,&乞,&mut 乣,丝,&mut 仲,供,);专=乍;if 专.is_some()||!儍{break;}}let Some((丟,七,丰))=专 else{break;};if 丈.is_empty(){乶=Some(互(乛[丟].丈()));}丈.extend_from_slice(乛[丟].丈());乿=七;仰=丰;}for(k,乎)in
仲{let 個=一.伟.entry(k).or_insert(乎);*個=(*個).max(乎);}仧{丈,end:乿,first:乶,丧:None,}}fn 伭(&self,乱:&J)->K<K<L>>{let mut 七=乱.clone();七.仱();let mut 丄=K::new();if
七.両().is_some(){return 丄;}let 井=七.丛();let 下=七.下(井);if let Some(东)=侄(&七,U[self.丂],下){丄.push(东.丈);}for 丼 in 俽(&七,俄,下,傊){丄.push(丼.丈);}let mut 倊=K::new();僖(&七,&mut
倊);丄.push(倊);let mut 世=佴{丂:井,limit:京{伢:傋,..self.limit},亲:0,伟:伊::default(),仢:None,侴:false,乘:None,};丄.push(世.侉(&七,&[],&[],true).丈);丄.retain(|丈|!丈.is_empty());丄}fn
俬(&self,乱:&J)->K<亄>{self.伭(乱).into_iter().map(|丈|{let 侍=self.value(&俫(乱,&丈));亄{丈,侍}}).collect()}fn 交(&mut self,了:&J,亐:&丫,丝:&[亄])->P<仧>{let mut 乜=了.clone();if!佾(&mut
乜,亐.丈())||self.俲(&乜){return None;}self.亲+=1;let 乲=self.侉(&乜,丝,&[],false);let mut 丈=亐.丈().to_vec();丈.extend_from_slice(&乲.丈);Some(仧{丈,end:乲.end,first:Some(互(亐.丈())),丧:None,})}fn
倃(&self,丙:&仧,丝:&[亄])->C{let 丰=self.侪(&丙.end,丝,C::MIN);match&丙.丧{Some(丧)if!丧.is_empty()=>丰.min(self.侪(&丙.end,丧,C::MIN)),_=>丰,}}}fn 僽(乍:&[亄],丝:&mut Vec<亄>)->O{let
乌:K<_> =丝.iter().map(|r|r.丈.clone()).collect();for 乺 in 乍{if 丝.iter().any(|r|r.丈==乺.丈){continue;}丝.push(乺.clone());}丝.sort_by_key(|乺|乺.侍);丝.truncate(偻);丝.iter().map(|r|&r.丈).ne(&乌)}fn
儡(l:&J,乒:京,亁:&mut 亡)->佶{倾(l,乒,亁,None)}fn 倾(l:&J,乒:京,亁:&mut 亡,乘:P<仦>,)->佶{let 丂=l.丛();let mut 丿=佶::default();if l.両().is_some(){return 丿;}if let Some(丈)=佖(l){丿.丈=丈;return
丿;}let 乗=|儇:亽|match(乒.人,乒.仚){(Some(人),Some(仚))=>Some(人+(仚-人).mul_f64(儇)),_=>None,};if 偧.is_some_and(|且|l.乤()>=且){亁.end();}if 亁.僶(l){let 丈=亁.且(l,乗(0.4));let
傸=丈.iter().any(|丑|{matches!(丑,L::主{..}|L::亪{..}|L::乽{..})});let 亳=健&&!傸&&l.乤()>=偦&&l.下(丂)>=偤;if!亳{丿.亁=true;丿.丈=丈;return 丿;}亁.end();}let mut 也=佴{丂,limit:京{人:乗(偕),..乒},亲:0,伟:伊::default(),仢:None,侴:儗(l),乘,};let
mut 丝=K::new();let mut 乊:K<仧> =K::new();let mut 伒=K::new();let mut 佌=false;let mut 交:K<丫> =K::new();let mut 举=也.乘.as_ref().filter(|g|g.start).map(|g|丫::事(g.丙.clone()));while
丿.侊<偳&&(乊.is_empty()||!也.亙()){let 儒=match 举.take(){Some(亐)=>也.交(l,&亐,&丝),None=>None,};let 丙=if let Some(丙)=儒{丙}else if 佌{let Some(亐)=交.pop()else{break;};match
也.交(l,&亐,&丝){Some(丙)=>丙,None=>continue,}}else{也.侉(l,&丝,&伒,true)};丿.侊+=1;let 倰=乊.iter().any(|p|p.丈==丙.丈);let 乶=丙.first;let 倷=也.亙();let mut 丙=丙;let mut 候=false;if!倷&&!倰{let
乍=也.俬(&丙.end);候=僽(&乍,&mut 丝);丙.丧=Some(乍);}if!倰{乊.push(丙);}if 倷{break;}if!候&&!佌{match 乶{Some(乶)=>伒.push(乶),None=>{佌=true;交=也.仢.clone().unwrap_or_default();交.retain(|a|!伒.contains(&互(a.丈())));let
乎=|a:&丫|{也.伟.get(&互(a.丈())).copied().unwrap_or(C::MIN)};交.sort_by_key(乎);}}}}丿.交=F::from(佌);丿.亲=也.亲;丿.丝=丝.len();丿.乊=乊.len();let 仄=||乒.仚.is_some_and(|t|亂::now()>=t);let
mut 伃:K<P<C>> =vec![None;乊.len()];for 丟 in(0..乊.len()).rev(){if 丟+1<乊.len()&&仄(){break;}伃[丟]=Some(也.倃(&乊[丟],&丝));}let 伀=|伃:&[P<C>]|{let mut 专:P<(C,F)>
=None;for(丟,丰)in 伃.iter().enumerate().rev(){if let Some(丰)=*丰{if 专.is_none_or(|(乨,_)|丰>乨){专=Some((丰,丟));}}}专};let mut 专=伀(&伃);也.limit.人=乒.仚;while let Some((_,丟))=专{if!傦||乊[丟].丧.is_some()||仄(){break;}let
乍=也.俬(&乊[丟].end);乊[丟].丧=Some(乍);丿.俸+=1;伃[丟]=Some(也.倃(&乊[丟],&丝));专=伀(&伃);}if let Some((_,丟))=专{丿.倸=也.乘.as_ref().is_some_and(|g|!g.丙.is_empty()&&乊[丟].丈.starts_with(&g.丙));丿.仉=也.乘.as_ref().map_or(0,|g|{g.仉.iter().filter(|p|p.iter().all(|c|乊[丟].丈.contains(c))).count()});let
mut 丌=l.clone();for&丑 in&乊[丟].丈{if 丌.乃(丑){丿.丈.push(丑);}}}丿}fn 僵(乒:&dyn Fn(亂,Q)->京,mut 亁:亡,侚:impl BufRead,佚:impl Write,)->Result<(),乖>{let mut 产=伍::default();个(侚,佚,|仵,亇,且,举|match
J::read(仵,亇,亟::佧,且){Ok(l)=>{let 仸=乘(&l,&mut 产);let 丿=match 仸{Some(乘)=>{倾(&l,乒(举,且),&mut 亁,Some(乘))}None=>儡(&l,乒(举,且),&mut 亁),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",且,丿.侊,丿.亲,丿.丝,丿.俸,丿.交,I::from(丿.倸),丿.仉,举.elapsed());傮(&丿.丈)}Err(他)=>{eprintln!("plan: cannot read the input: {}"
,他);"WAIT".to_string()}},)}const 傂:P<C> =None;const 傥:O=false;#[derive(Clone,Copy,PartialEq)]enum 伌{佪,偓,}#[derive(Clone,Copy)]struct 亮{q:F,kind:伌,仛:O,value:C,}fn
乧(l:&J,丁:F)->K<亮>{僩(l,丁,傥)}fn 僩(l:&J,丁:F,儚:O)->K<亮>{let(乄,中)=住(l);let(丶,世)=(l.丆(丁),l.丆(1-丁));let(伙,会)=(临(l,丶),临(l,世));let 伓=伥(l,&伙,&伙,&会);let mut 丄=K::new();for
q in(乄|中).iter(){let 侎=临(l,G::q(q));let 仛=丶.test(q);let 丬=if 儚{伥(l,&伙,&侎,&会)-伥(l,&会,&侎,&伙)}else if 仛{let 偋=临(l,僒(丶,丁,q));伓-伥(l,&偋,&偋,&会)}else{伥(l,&伙,&侎,&会)-伓};丄.push(亮{q,kind:if
中.test(q){伌::佪}else{伌::偓},仛,value:丬,});}丄.sort_by_key(|o|(-o.value,o.kind!=伌::佪,o.q));丄}fn 伥(l:&J,a:&[I;M],g:&[I;M],b:&[I;M])->C{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 仂(乧:&[亮])->K<亮>{乧.iter().filter(|o|o.value>=傂.unwrap_or(1)).copied().collect()}fn 僒(丆:G,丁:F,q:F)->G{let
乲=丆.业(G::q(q));let mut 乪=G::q(U[丁])&乲;loop{let 七=乪|(乪.丏()&乲);if 七==乪{return 乪;}乪=七;}}const 信:C=10;const 傀:C=100;#[derive(Clone,Copy)]enum 伾{伐(Q),主(F),}#[derive(Clone,Copy)]struct
乼{个:Q,侘:伾,}#[derive(Clone,Copy)]struct 伎{q:F,ours:P<乼>,余:P<乼>,theirs:P<乼>,亦:C,}fn 僺(l:&J,丁:F,乧:&[亮])->K<伎>{let mut 丄=K::new();for 串 in 乧.iter().filter(|o|!o.仛){let
q=串.q;let(僯,余)=俭(l,丁,q);let 两=专(僯,余);let(儠,償)=俭(l,1-丁,q);let 世=专(儠,償);let 个=|a:P<乼>|a.map_or(傀,|a|C::from(a.个));丄.push(伎{q,ours:两,余,theirs:世,亦:个(世)-个(两),});}丄}fn
专(亻:P<乼>,伴:P<乼>)->P<乼>{match(亻,伴){(Some(u),Some(t))=>Some(if t.个<u.个{t}else{u}),(u,t)=>u.or(t),}}fn 俭(l:&J,丵:F,q:F)->(P<乼>,P<乼>){let 严=l.map();let mut
亻:P<乼> =None;for 三 in l.万(丵).iter(){let d=严.丗(三,q);let Some(上)=l.予(三)else{continue;};if d!=I::MAX&&亻.is_none_or(|u|Q::from(d)<u.个){亻=Some(乼{个:Q::from(d),侘:伾::伐(上),});}}let
下=l.下(丵)+if 丵==l.丛(){0}else{l.丞(丵)};let 佣=match(下,l.丞(丵)){(下,_)if 下>=信=>Some(0),(_,丞)if 丞<=0=>None,(下,丞)=>Some((信-下+丞-1)/丞),};let mut 伴:P<乼> =None;if let
Some(佣)=佣{let 乮=l.丆(丵).丏()&严.valid&l.么(丵,1);for 三 in 乮.iter(){let d=严.丗(三,q);if d!=I::MAX&&伴.is_none_or(|t|1+佣 as Q+Q::from(d)<t.个){伴=Some(乼{个:1+佣 as Q+Q::from(d),侘:伾::主(三),});}}}(亻,伴)}const
俢:C=10;#[derive(Default)]struct 伍{借:BTreeMap<Q,F>,}#[derive(Default)]struct 什{仗:K<(Q,F)>,些:K<(F,F)>,back:K<Q>,}fn 仭(l:&J,丁:F,乧:&[亮],仠:&[伎],丝:&mut 伍,)->什{修(l,丁,乧,仠,丝,l.万(丁))}fn
修(l:&J,丁:F,乧:&[亮],仠:&[伎],丝:&mut 伍,侏:G,)->什{let 严=l.map();let 万:BTreeMap<Q,F> =(l.万(丁)&侏).iter().filter_map(|c|l.予(c).map(|上|(上,c))).collect();let mut 丠:K<(亮,伎)>
=K::new();for o in 仂(乧).into_iter().filter(|o|!o.仛){if let Some(r)=仠.iter().find(|r|r.q==o.q){if r.ours.is_some()&&r.亦>=0{丠.push((o,*r));}}}丠.sort_by_key(|(o,r)|{(-o.value,o.kind!=伌::佪,r.ours.map_or(Q::MAX,|a|a.个),o.q,)});let
乀=|亻:Q,丕:F|Q::from(严.丗(万[&亻],丕));let 倐=|云:&BTreeMap<F,Q>,伬:&BTreeSet<Q>,上:Q|{伬.contains(&上)||云.values().any(|&u|u==上)};let mut 云:BTreeMap<F,Q> =BTreeMap::new();for(&上,&丕)in&丝.借{if
万.contains_key(&上)&&丠.iter().any(|(o,_)|o.q==丕)&&!云.contains_key(&丕){云.insert(丕,上);}}let mut 伬:BTreeSet<Q> =BTreeSet::new();for(o,_)in&丠{let Some(&倓)=云.get(&o.q)else{continue;};let
侥=万.keys().copied().filter(|&上|!倐(&云,&伬,上)).map(|上|(乀(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();if let Some((d,上))=侥{if d<乀(倓,o.q){伬.insert(倓);云.insert(o.q,上);}}}let
mut 些=K::new();for(o,r)in&丠{if 云.contains_key(&o.q){continue;}let 並=万.keys().copied().filter(|&上|!倐(&云,&伬,上)).map(|上|(乀(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();let
偄=並.map(|(d,_)|C::from(d));let 佡=r.余.and_then(|t|match t.侘{伾::主(q)if l.下(丁)-俢*些.len()as C>=俢&&!些.iter().any(|&(c,_)|c==q)=>{Some((q,C::from(t.个)))}_=>None,});let
偁=佡.map(|(_,t)|t);let 偅=match(偄,偁){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 傽=if 偅{偁}else{偄};let Some(个)=傽 else{continue;};if r.theirs.map_or(100,|a|C::from(a.个))<个{continue;}match(偅,佡,並){(true,Some((q,_)),_)=>些.push((q,o.q)),(false,_,Some((_,上)))=>{云.insert(o.q,上);}_=>{}}}let
mut 丄=什{些,..什::default()};for(o,_)in&丠{if let Some(&上)=云.get(&o.q){丄.仗.push((上,o.q));}}丄.back=l.万(丁).iter().filter_map(|c|l.予(c)).filter(|上|!丄.仗.iter().any(|&(u,_)|u==*上)).collect();丄.back.sort_unstable();丝.借=丄.仗.iter().copied().collect();丄}const
俣:C=10;fn 俶(l:&J,丁:F,上:Q)->P<F>{l.万(丁).iter().find(|&c|l.予(c)==Some(上))}fn 倶(l:&J,丁:F,三:F,丕:F)->K<F>{let 严=l.map();let 仃=严.丗(三,丕);if 仃==I::MAX||仃==0{return
K::new();}let mut 丄:K<F> =(G::q(三).丏()&严.valid).iter().filter(|&n|严.丗(n,丕)<仃).collect();丄.sort_by_key(|&n|(l.丒(丁).test(n),n));丄}fn 亢(丌:&mut J,丄:&mut Vec<L>,丑:L)->O{let
倢=丌.乃(丑);if 倢{丄.push(丑);}倢}#[derive(Clone,Copy)]struct 仩{侬:O,guard:I,倂:O,倯:O,}fn 仉(l:&J,产:&什,乳:仩)->K<K<L>>{let mut 丄=K::new();for&(上,丕)in&产.仗{let 僭=什{仗:vec![(上,丕)],..什::default()};let
佛=倹(l,&僭,仩{侬:false,..乳},);if!佛.is_empty(){丄.push(佛);}}丄}fn 丢(丌:&J,上:Q,三:F,丣:F,亣:O,乳:仩,)->P<K<L>>{let 丁=丌.丛();let mut 丳=丌.clone();let mut 丈=K::new();let(x,y)=乓(丣);if!亢(&mut
丳,&mut 丈,L::乴{上,x,y}){return None;}if 亣&&乳.guard!=0{let(付,仙)=乓(三);let 侕=if 乳.guard==1{L::乽{x:付,y:仙}}else{L::主{丅:1,x:付,y:仙,}};if!亢(&mut 丳,&mut 丈,侕){return
None;}}if 乳.倯&&伱(&丳,丁)>伱(丌,丁){return None;}Some(丈)}fn 倹(l:&J,产:&什,乳:仩)->K<L>{let 丁=l.丛();let 严=l.map();let mut 丌=l.clone();let mut 丄=K::new();let 丨=丨(l,1-丁);let
mut 亶:K<(Q,F)> =K::new();for&(上,丕)in&产.仗{let Some(三)=俶(&丌,丁,上)else{continue;};if!丌.乸().test(三){continue;}let 亣=丨.test(三);for 丣 in 倶(&丌,丁,三,丕){if let Some(丈)=丢(&丌,上,三,丣,亣,乳){for
丑 in&丈{丌.乃(*丑);}if 亣&&乳.guard==0{亶.push((上,三));}丄.extend(丈);break;}}}for&(上,丕)in&产.仗{if 丌.下(丁)<俣{break;}let Some(乗)=俶(&丌,丁,上)else{continue;};let 傲=亶.iter().find(|&&(u,_)|u==上).map(|&(_,c)|c);let
丐=match 傲{Some(q)if 乳.倂=>vec![q],_=>倶(&丌,丁,乗,丕),};for 丣 in 丐{let(x,y)=乓(丣);if 亢(&mut 丌,&mut 丄,L::主{丅:1,x,y}){break;}}}if 乳.侬{for&(_,丕)in&产.些{if 丌.下(丁)<俣{break;}let
乮=丌.丆(丁).丏()&严.valid;let mut 丐:K<F> =乮.iter().filter(|&c|严.丗(c,丕)!=I::MAX).collect();丐.sort_by_key(|&c|(严.丗(c,丕),c));for 丣 in 丐{let(x,y)=乓(丣);if 亢(&mut
丌,&mut 丄,L::主{丅:1,x,y}){break;}}}}丄}const 俤:I=0;const 傟:I=1;const 俕:C=0;const 傃:O=false;const 偣:I=0;const 偝:O=false;const 傒:O=false;const 偏:O=false;const
偟:Q=7;const 傡:Q=14;const 傚:O=俤!=0||俕!=0;struct 仦{丙:K<L>,倡:O,start:O,仉:K<K<L>>,乧:K<(F,C,C)>,企:C,}impl 仦{fn 儝(&self,l:&J,丁:F)->C{if self.企==0{return 0;}let(丶,世)=(l.丆(丁),l.丆(1-丁));let
mut 亚=0;for&(q,丬,五)in&self.乧{亚+=if 丶.test(q){丬}else if 世.test(q){-丬}else{五};}self.企*亚/4}}fn 乘(l:&J,丝:&mut 伍)->P<仦>{let 伦=l.乤();if!傚||!(偟..=傡).contains(&伦)||l.両().is_some(){return
None;}僚(l,丝,傔)}#[derive(Clone,Copy)]struct 佁{仅:I,乳:仩,使:O,企:C,侩:O,}const 傔:佁=佁{仅:俤,乳:仩{侬:傟==2,guard:偣,倂:偝,倯:傒,},使:偏,企:俕,侩:傃,};fn 僚(l:&J,丝:&mut 伍,乁:佁)->P<仦>{let
佁{仅,乳,使,企,侩,}=乁;let 丁=l.丛();let 乧=乧(l,丁);let 仠=僺(l,丁,&乧);let 产=if 使{let 侏=l.万(丁).业(丨(l,1-丁));修(l,丁,&乧,&仠,丝,侏)}else{仭(l,丁,&乧,&仠,丝)};let 丙=if 仅!=0{倹(l,&产,乳)}else{K::new()};let
仂=仂(&乧);let 儞=仂.iter().map(|o|{let 亦=仠.iter().find(|r|r.q==o.q).map_or(0,|r|r.亦.clamp(-1,1));let 五=if 侩{o.value*亦}else{0};(o.q,o.value,五)}).collect();let
僱=if 仅&4!=0{仉(l,&产,乳)}else{K::new()};Some(仦{仉:僱,倡:!丙.is_empty()&&仅&1!=0,start:!丙.is_empty()&&仅&2!=0,丙,乧:儞,企,})}const 偑:亽=45.0;const 偞:亽=800.0;const 傘:亽=0.6;fn
乒(举:亂,且:Q)->京{if let Some(伢)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return 京{人:None,仚:None,伢,};}let 你=儣().min(100.0);let
僨=if 且==1{偞}else{偑};let 乑=Duration::from_secs_f64(僨/1000.0*你);京{人:Some(举+乑.mul_f64(傘)),仚:Some(举+乑),伢:丮::MAX,}}const 傅:F=9;const 傁:亾=亾{丐:3,丞:1,下:1,丼:2,亵:2,伳:2,丨:6,令:1,中:0,仭:false,乞:傆,};const
傉:Q=0;const 傆:亍=亍::佩(伽::俐(7));const 俙:I=I::MAX;const 俗:C=2;const 俘:C=18;const 俖:C=0;const 傄:F=32;const 偍:O=false;const 偎:Settings1=Settings1{侰:3,侯:1,侇:4,仄:2,侐:2,佋:2,侷:true,reserve:true,侑:true,乢:亿::乵,fill:true,乕:Z{佟:俙,仯:俗,佒:俘,佃:俖,..Z::仑},};fn
亁()->亡{if 偍{return 亡::new(于::伹(偎),K::new(),Z::仑);}僮(傄)}fn 僮(亝:F)->亡{let 亘=于::伏{个:傅,亝,串:傁,佝:傉,};let 丯=傓.iter().map(|&乕|Z{佟:俙,仯:俗,佒:俘,佃:俖,..乕}).collect();亡::new(亘,丯,Z::仑)}fn
main(){let 儓=io::stdin();let 儔=io::stdout();if let Err(他)=僵(&乒,亁(),儓.lock(),儔.lock()){eprintln!("plan: {}",他);std::process::exit(1);}}const 傝:&str="CG_TIME_SCALE";fn
儣()->亽{僰(std::env::var(傝).ok().as_deref())}fn 僰(丬:P<&str>)->亽{丬.and_then(|亇|亇.trim().parse::<亽>().ok()).filter(|你|你.is_finite()&&*你>0.0).unwrap_or(1.0)}const
_:()=assert!(停==16);const 停:F=16;const 偔:亽=0.4;const 偼:I=4;#[derive(Clone,Copy)]struct 仒{中:G,俷:G,伣:G,}impl 仒{fn of(l:&J)->仒{let(a,伖)=倥(l,U[0]);let(b,佊)=倥(l,U[1]);let
亚=a[U[1]];let 僠=伖[U[1]];let mut 乂=仒{中:G::T,俷:G::T,伣:G::T,};for q in l.map().valid.iter(){if 伖[q]==I::MAX||佊[q]==I::MAX{continue;}if 伖[q].abs_diff(佊[q])<=1{乂.俷.set(q);}if
伖[q].abs_diff(佊[q])<=偼{乂.伣.set(q);}let 僪=伸::from(伖[q])+伸::from(佊[q])==伸::from(僠);if 亚>0.0&&僪&&q!=U[0]&&q!=U[1]&&a[q]*b[q]/亚>=偔{乂.中.set(q);}}乂}fn 仜(&self)->G{self.中&self.伣}}fn
儖(l:&J,儐:G)->[I;M]{let 丹=l.map().valid;let mut 丄=[I::MAX;M];let mut 乪=儐&丹;let mut 仸=乪;let mut 丢=0u8;while!仸.is_empty(){for q in 仸.iter(){丄[q]=丢;}let 七=(仸.丏()&丹).业(乪);乪|=七;仸=七;丢=丢.saturating_add(1);}丄}fn
倥(l:&J,三:F)->([亽;M],[I;M]){let 乀=儖(l,G::q(三));let mut 乞:K<F> =l.map().valid.iter().filter(|&c|乀[c]!=I::MAX).collect();乞.sort_by_key(|&c|乀[c]);let mut 丘=[0.0;M];丘[三]=1.0;for
q in 乞{if q==三{continue;}let 九=G::q(q).丏()&l.map().valid;丘[q]=九.iter().filter(|&n|乀[n]!=I::MAX&&乀[n]+1==乀[q]).map(|n|丘[n]).sum();}(丘,乀)}const Y:F=12;const
M:F=Y*Y;const 亃:F=4*Y;const 亠:丮=(1<<亃)-1;const 伺:丮=1|1<<12|1<<24|1<<36;const 佨:丮=伺<<11;const 俜:丮=(1<<Y)-1;#[derive(Clone,Copy,PartialEq)]struct G([丮;3]);impl
G{const T:G=G([0;3]);const 偌:G=G([亠;3]);#[inline]fn q(q:F)->G{let mut 乐=G::T;乐.set(q);乐}#[inline]fn test(self,q:F)->O{(self.0[q/亃]>>(q%亃))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/亃]|=1<<(q%亃);}#[inline]fn clear(&mut self,q:F){self.0[q/亃]&=!(1<<(q%亃));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伵(self)->[丮;3]{self.0}#[inline]fn count(self)->Q{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 业(self,乆:G)->G{G([self.0[0]&!乆.0[0],self.0[1]&!乆.0[1],self.0[2]&!乆.0[2],])}#[inline]fn
iter(self)->佦{佦{侟:self.0,乗:0,}}#[inline]fn 儌(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&俜)<<(亃-Y)),(b>>Y)|((c&俜)<<(亃-Y)),c>>Y,])}#[inline]fn 儉(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亠,((b<<Y)&亠)|(a>>(亃-Y)),((c<<Y)&亠)|(b>>(亃-Y)),])}#[inline]fn
儊(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佨&亠,(b>>1)&!佨&亠,(c>>1)&!佨&亠,])}#[inline]fn 儋(self)->G{let[a,b,c]=self.0;G([(a<<1)&!伺&亠,(b<<1)&!伺&亠,(c<<1)&!伺&亠,])}#[inline]fn
丏(self)->G{let 一=self;一.儌()|一.儉()|一.儊()|一.儋()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,乆:G)->G{G([self.0[0]&乆.0[0],self.0[1]&乆.0[1],self.0[2]&乆.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,乆:G)->G{G([self.0[0]|乆.0[0],self.0[1]|乆.0[1],self.0[2]|乆.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,乆:G){*self=*self|乆;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::偌.业(self)}}struct 佦{侟:[丮;3],乗:F,}impl Iterator for 佦{type Item=F;#[inline]fn
next(&mut self)->P<F>{let 一=self;while 一.乗<3{let 佗=一.侟[一.乗];if 佗!=0{一.侟[一.乗]=佗&(佗-1);return Some(一.乗*亃+佗.trailing_zeros()as F);}一.乗+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乴{上:Q,x:I,y:I},主{丅:I,x:I,y:I},亪{x:I,y:I},乽{x:I,y:I},}impl L{fn 儦(&self)->乖{match*self{L::乴{上,x,y}=>format!("MOVE {} {x} {y}",上),L::主{丅,x,y}=>format!("TRAIN {} {x} {y}",丅),L::亪{x,y}=>format!("BUILD MINE {x} {y}"),L::乽{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
傮(丈:&[L])->乖{if 丈.is_empty(){return "WAIT".to_string();}let mut line=乖::with_capacity(16*丈.len());for(丟,丑)in 丈.iter().enumerate(){if 丟>0{line.push(';');}line.push_str(&丑.儦());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亟{俩,儵,傪,#[default]佧,}impl 亟{fn 亓(self)->I{match self{亟::俩=>1,_=>3,}}fn 佄(self)->O{matches!(self,亟::傪|亟::佧)}fn 们(self)->O{self==亟::佧}}const 二:I=I::MAX;struct
亭{乡:G,valid:G,丸:G,丗:Box<[I]>,丏:[[[I;4];M];2],}thread_local!{static 偰:RefCell<P<Arc<亭>>> =const{RefCell::new(None)};}impl 亭{fn 儈(乡:G,丸:G)->Arc<亭>{偰.with(|乷|{let
mut 乷=乷.borrow_mut();if let Some(严)=乷.as_ref(){if 严.乡==乡&&严.丸==丸{return Arc::clone(严);}}let 严=Arc::new(亭::build(乡,丸));*乷=Some(Arc::clone(&严));严})}fn build(乡:G,丸:G)->亭{let
丹=!乡;const 備:[[(伷,伷);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[二;4];M];2];for(丁,之)in 備.iter().enumerate(){for q in
丹.iter(){let(x,y)=((q%Y)as 伷,(q/Y)as 伷);let mut 丘=0;for&(僈,僉)in 之{let(倝,倞)=(x+僈,y+僉);if(0..Y as 伷).contains(&倝)&&(0..Y as 伷).contains(&倞){let 七=倞 as F*Y+倝
as F;if 丹.test(七){丏[丁][q][丘]=七 as I;丘+=1;}}}}}let mut 丗=vec![二;M*M].into_boxed_slice();let mut 侨=[0u8;M];for 三 in 丹.iter(){let 乻=&mut 丗[三*M..(三+1)*M];乻[三]=0;侨[0]=三
as I;let(mut 侖,mut 侶)=(0,1);while 侖<侶{let q=侨[侖]as F;侖+=1;for&七 in&丏[0][q]{if 七!=二&&乻[七 as F]==二{乻[七 as F]=乻[q]+1;侨[侶]=七;侶+=1;}}}}亭{乡,valid:丹,丸,丗,丏,}}#[inline]fn
丗(&self,三:F,丣:F)->I{self.丗[三*M+丣]}}const 偺:Q=100;const 争:C=15;const 傣:[C;3]=[1,4,20];const U:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 乯{亏(F),俅,}enum
亊{佳,俔,俚,}#[derive(Clone)]struct J{map:Arc<亭>,丽:亟,丧:[G;2],丆:[G;2],丶:G,乩:G,万:[G;2],丅:[G;3],仇:G,予:[Q;M],下:[C;2],丞:[C;2],丛:F,乤:Q,份:Q,両:P<乯>,}fn 伞(丧:G,仴:F)->G{if!丧.test(仴){return
G::T;}let mut 伫=G::q(仴);loop{let 七=(伫|伫.丏())&丧;if 七==伫{return 伫;}伫=七;}}impl J{fn 丛(&self)->F{self.丛}fn 乤(&self)->Q{self.乤}fn 両(&self)->P<乯>{self.両}fn 丽(&self)->亟{self.丽}fn
任(&self,丁:F)->C{let 一=self;let 万=一.万[丁];一.下[丁]+10*C::from((万&一.丅[0]).count())+20*C::from((万&一.丅[1]).count())+30*C::from((万&一.丅[2]).count())}fn 下(&self,丁:F)->C{self.下[丁]}fn
丞(&self,丁:F)->C{self.丞[丁]}fn 侹(&self,丁:F)->F{self.万[丁].count()as F}fn map(&self)->&亭{&self.map}fn 丒(&self,丁:F)->G{self.丧[丁]}fn 丆(&self,丁:F)->G{self.丆[丁]}fn
万(&self,丁:F)->G{self.万[丁]}fn 亼(&self,q:F)->P<(Q,I)>{let 一=self;(一.万[0]|一.万[1]).test(q).then(||(一.予[q],一.侞(q)))}fn 乸(&self)->G{self.万[self.丛]&self.仇}fn
丷(&self)->G{self.丶}fn 乇(&self)->G{self.乩}fn 亜(&self,q:F)->P<I>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.侞(q))}fn 予(&self,q:F)->P<Q>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.予[q])}fn
儑(&mut self,丁:F){let 一=self;一.丆=[伞(一.丧[0],U[0]),伞(一.丧[1],U[1])];let 僂=一.万[丁].业(一.丆[丁]);一.侫(僂);一.俹();一.下[丁]+=一.丞[丁];if 一.下[丁]<0{一.下[丁]=0;一.侫(一.万[丁]);}一.仇|=一.万[丁];}fn
俹(&mut self){let 一=self;for 丁 in 0..2{let 丆=一.丆[丁];let 万=丆&一.万[丁];let mut 丞=C::from(丆.count())+4*C::from((丆&一.丶).count());for(丅,儫)in 傣.iter().enumerate(){丞-=儫*C::from((万&一.丅[丅]).count());}一.丞[丁]=丞;}}fn
仱(&mut self){let 一=self;if 一.両.is_some(){return;}if 一.丛==1{if 一.乤>=偺{一.両=Some(match 一.任(0).cmp(&一.任(1)){std::cmp::Ordering::Greater=>乯::亏(0),std::cmp::Ordering::Less=>乯::亏(1),std::cmp::Ordering::Equal=>乯::俅,});return;}一.乤+=1;}一.丛=1-一.丛;一.儑(一.丛);}fn
作(&mut self,丈:&[L]){let 一=self;if 一.両.is_some(){return;}let 侓=一.份;for&丑 in 丈{if 一.僇(丑,侓){continue;}一.乃(丑);if 一.両.is_some(){return;}}一.仱();}fn 僇(&self,丑:L,侓:Q)->O{let
一=self;match 丑{L::乴{上,x,y}=>{一.仮(x,y).is_none()||上>=侓||一.倄(一.丛,上).is_none()}L::主{丅,x,y}=>{一.仮(x,y).is_none()||丅==0||丅>一.丽.亓()}L::亪{x,y}|L::乽{x,y}=>{一.仮(x,y).is_none()}}}#[inline]fn
仮(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<Y&&y<Y&&!self.map.乡.test(y*Y+x)).then_some(y*Y+x)}fn 倄(&self,丁:F,上:Q)->P<F>{self.万[丁].iter().find(|&q|self.予[q]==上)}#[inline]fn
侞(&self,q:F)->I{if self.丅[0].test(q){1}else if self.丅[1].test(q){2}else{3}}fn 傴(&self,井:F)->G{let 乇=self.乩&self.丧[井];let 丆=self.丆[井];乇|(丆&(丆&乇).丏())}fn
么(&self,丁:F,丅:I)->G{let 一=self;debug_assert!((1..=3).contains(&丅),"unit level {} is not 1 to 3",丅);let 井=1-丁;let 万=一.万[0]|一.万[1];let 僊=一.仕();let 儩=match
丅{1 if 一.丽==亟::俩=>一.丅[1]|一.丅[2],1=>万,2=>一.丅[1]|一.丅[2],_=>G::T,};let 么=僊|一.丧[井].业(儩);if 丅<3{么.业(一.傴(井))}else{么}}fn 仈(&self,丁:F)->G{self.丆[丁]|self.丆[丁].丏()}fn
仕(&self)->G{let 一=self;let 万=一.万[0]|一.万[1];let 僛=G::q(U[0])|G::q(U[1]);一.map.valid.业(万|一.丶|一.乩|僛)}fn 乃(&mut self,丑:L)->O{let 一=self;if 一.両.is_some(){return
false;}let 丁=一.丛;let 仓=match 丑{L::主{丅,x,y}=>一.佡(丁,丅,x,y),L::乴{上,x,y}=>一.僧(丁,上,x,y),L::亪{x,y}=>一.build(丁,true,x,y),L::乽{x,y}=>一.build(丁,false,x,y),};let
Some(仓)=仓 else{return false;};match 仓{亊::佳=>{}亊::俔=>一.丆[丁]=伞(一.丧[丁],U[丁]),亊::俚=>{一.丆=[伞(一.丧[0],U[0]),伞(一.丧[1],U[1])];}}一.俹();if!一.丧[0].test(U[0]){一.両=Some(乯::亏(1));}else
if!一.丧[1].test(U[1]){一.両=Some(乯::亏(0));}true}fn 侫(&mut self,丐:G){let 一=self;一.万[0]=一.万[0].业(丐);一.万[1]=一.万[1].业(丐);for 丅 in&mut 一.丅{*丅=丅.业(丐);}一.仇=一.仇.业(丐);}fn
倽(&mut self,丁:F,q:F)->亊{let 一=self;一.侫(G::q(q));一.丶.clear(q);一.乩.clear(q);let 井=1-丁;if 一.丧[井].test(q){一.丧[井].clear(q);一.丧[丁].set(q);亊::俚}else if 一.丧[丁].test(q){亊::佳}else{一.丧[丁].set(q);亊::俔}}fn
佡(&mut self,丁:F,丅:I,x:I,y:I)->P<亊>{let 一=self;if 丅==0||丅>一.丽.亓(){return None;}let q=一.仮(x,y)?;let 不=10*C::from(丅);if 一.下[丁]<不||!一.仈(丁).test(q)||!一.么(丁,丅).test(q){return
None;}let 仓=一.倽(丁,q);一.倦(丁,丅,q,一.份);一.份=一.份.saturating_add(1);一.下[丁]-=不;Some(仓)}fn 倦(&mut self,丁:F,丅:I,q:F,上:Q){self.万[丁].set(q);self.丅[F::from(丅)-1].set(q);self.予[q]=上;}fn
僧(&mut self,丁:F,上:Q,x:I,y:I)->P<亊>{let 一=self;let 与=一.仮(x,y)?;let 三=一.倄(丁,上)?;if!一.仇.test(三){return None;}let 丅=一.侞(三);let 丩=一.么(丁,丅);let 九=一.map.丏[丁][三];if
九.contains(&(与 as I))&&!丩.test(与){return None;}let mut 专=一.map.丗(三,与);let mut 优=None;for&七 in 九.iter().take_while(|&&七|七!=二){let 七=F::from(七);let 丗=一.map.丗(七,与);if
丗<专&&丩.test(七){专=丗;优=Some(七);}}let 优=优?;let 仓=一.倽(丁,优);一.万[丁].clear(三);一.丅[F::from(丅)-1].clear(三);一.仇.clear(三);一.倦(丁,丅,优,上);Some(仓)}fn build(&mut self,丁:F,丶:O,x:I,y:I)->P<亊>{let
一=self;let 佼=if 丶{一.丽.佄()}else{一.丽.们()};let q=一.仮(x,y)?;let 僫=一.map.丸.test(q);if!佼||一.下[丁]<争||僫!=丶||!一.丧[丁].test(q)||!一.仈(丁).test(q)||!一.仕().test(q){return
None;}if 丶{一.丶.set(q);}else{一.乩.set(q);}一.下[丁]-=争;Some(亊::佳)}}const 偵:C=1000000000;const 佮:C=0;const 偽:C=1;const 傞:C=2;impl J{fn read(仵:&str,且:&str,丽:亟,乤:Q,)->Result<J,乖>{let
mut 丸=K::new();let mut 久=仵.lines();let 丘=侌(&mut 久,"mine spot count")?;for _ in 0..丘{let[x,y]=亸(仾(&mut 久,"a mine spot")?,"a mine spot")?;丸.push((佈(x)?,佈(y)?));}倀(久,"the initial input")?;let
mut 久=且.lines();let mut 侗=[0i64;4];for(丬,乚)in 侗.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[侦]=亸(仾(&mut 久,乚)?,乚)?;if 侦.abs()>偵{return
Err(format!("{}: {} is out of range",乚,侦));}*丬=侦;}let mut 乡=G::T;let mut 丐=[G::T;2];let mut 丆=[G::T;2];for y in 0..Y{let 乻=仾(&mut 久,"a map row")?.trim();if
乻.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,乻));}for(x,伯)in 乻.bytes().enumerate(){let q=y*Y+x;match 伯{b'#'=>乡.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乫=F::from(伯==b'X'||伯==b'x');丐[乫].set(q);if 伯.is_ascii_uppercase(){丆[乫].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伯 as char))}}}}let
傷=侌(&mut 久,"building count")?;let mut 乾=K::new();for _ in 0..傷{let 亸=亸(仾(&mut 久,"a building")?,"a building")?;乾.push(亸);}let 侹=侌(&mut 久,"unit count")?;let
mut 万=K::new();for _ in 0..侹{let 亸=亸(仾(&mut 久,"a unit")?,"a unit")?;万.push(亸);}倀(久,"the turn's input")?;let 倣=乾.iter().find(|俳|俳[0]==0&&俳[1]==佮).ok_or("the bot's own HQ is not among the buildings")?;let
倍=侃(倣[2],倣[3],乡)?;let 丁=U.iter().position(|&仴|仴==倍).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",倍))?;let mut 倲=G::T;for(x,y)in 丸{if
乡.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}倲.set(y*Y+x);}let 佺=|乫:F|if 乫==0{丁}else{1-丁};let mut l=J{map:亭::儈(乡,倲),丽,丧:[G::T;2],丆:[G::T;2],丶:G::T,乩:G::T,万:[G::T;2],丅:[G::T;3],仇:G::T,予:[0;M],下:[0;2],丞:[0;2],丛:丁,乤,份:1,両:None,};for
乫 in 0..2{let 亗=佺(乫);l.丧[亗]=丐[乫];l.丆[亗]=丆[乫];l.下[亗]=侗[2*乫];l.丞[亗]=侗[2*乫+1];}let mut 倠=G::T;for[亗,佔,x,y]in 乾{let 乫=倬(亗)?;let q=侃(x,y,乡)?;if!丐[乫].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 倠.test(q){return Err(format!("two buildings at ({x}, {y})"));}倠.set(q);match 佔{佮
if q==U[佺(乫)]=>{}佮=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}偽=>l.丶.set(q),傞=>l.乩.set(q),_=>return Err(format!("unknown building type {}",佔)),}}let
mut 乬=G::T;let mut 何=0u32;for[亗,上,丅,x,y]in 万{let 乫=倬(亗)?;let q=侃(x,y,乡)?;if!丐[乫].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乬.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丅){return Err(format!("unit level {} is not 1, 2 or 3",丅));}let 上=Q::try_from(上).map_err(|_|format!("unit id {} is out of range",上))?;if
乬.iter().any(|乆|l.予[乆]==上){return Err(format!("two units have id {}",上));}乬.set(q);l.万[佺(乫)].set(q);l.丅[丅 as F-1].set(q);l.予[q]=上;何=何.max(上);}if!乬.is_empty(){l.份=何.checked_add(1).ok_or_else(||format!("unit id {} is out of range",何))?;}l.仇=l.万[丁];Ok(l)}}fn
仾<'a>(久:&mut Lines<'a>,乚:&str)->Result<&'a str,乖>{久.next().ok_or_else(||format!("the text ends before {}",乚))}fn 倀(mut 久:Lines<'_>,乚:&str)->Result<(),乖>{match
久.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乚)),None=>Ok(()),}}fn 亸<const N:F>(line:&str,乚:&str)->Result<[C;N],乖>{let
mut 伵=line.split_whitespace();let mut 偆=[0i64;N];for 丬 in&mut 偆{let 伈=伵.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乚))?;*丬=伈.parse().map_err(|_|format!("{}: {:?} is not an integer",乚,伈))?;}if
伵.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乚));}Ok(偆)}fn 侌(久:&mut Lines<'_>,乚:&str)->Result<F,乖>{let[丘]=亸(仾(久,乚)?,乚)?;F::try_from(丘).ok().filter(|&丘|丘<=M).ok_or_else(||format!("{}: {} is out of range",乚,丘))}fn
佈(丬:C)->Result<F,乖>{F::try_from(丬).ok().filter(|&丬|丬<Y).ok_or_else(||format!("coordinate {} is off the map",丬))}fn 侃(x:C,y:C,乡:G)->Result<F,乖>{let q=佈(y)?*Y+佈(x)?;if
乡.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 倬(亗:C)->Result<F,乖>{match 亗{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亗)),}}#[derive(Clone,Copy,PartialEq)]enum
亿{乵,便,}#[derive(Clone,Copy)]struct Settings1{侰:I,侯:I,侇:C,仄:C,侐:F,佋:I,侷:O,reserve:O,侑:O,乢:亿,fill:O,乕:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{侰:3,侯:1,侇:4,仄:2,侐:2,佋:2,侷:true,reserve:true,侑:true,乢:亿::乵,fill:true,乕:Z::仑,};}const
佹:C=1000;const 佸:C=10;fn 侼(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侧(l:&mut J,丑:L,丄:&mut Vec<L>)->O{let 亢=l.乃(丑);if 亢{丄.push(丑);}亢}fn 侳(l:&mut J,三:F,丣:F,丄:&mut
Vec<L>)->O{let Some((上,_))=l.亼(三)else{return false;};let(x,y)=侼(丣);侧(l,L::乴{上,x,y},丄)}fn 儜(l:&J,乂:&仒,乁:&Settings1,乒:F,)->K<F>{let 丂=l.丛();let 两=临(l,l.丒(丂));let
世=临(l,l.丒(1-丂));let 五=l.map().valid.业(l.丒(0)|l.丒(1));let 倪=|c:F|两[c]!=I::MAX&&世[c]!=I::MAX;let mut 中:K<F> =乂.仜().业(l.丒(丂)).iter().filter(|&c|倪(c)).collect();中.sort_by_key(|&c|(两[c],c));let
mut 义:K<F> =五.iter().filter(|&c|倪(c)&&两[c].abs_diff(世[c])<=乁.侯).collect();义.sort_by_key(|&c|(两[c].abs_diff(世[c]),两[c],c));let mut 丄:K<F> =K::new();for
q in 中.into_iter().chain(义){if 丄.len()>=乒{break;}let 伝=丄.iter().all(|&t|l.map().丗(t,q)>=乁.侰);let 傼=乂.仜().test(q);if!丄.contains(&q)&&(傼||伝){丄.push(q);}}丄}fn
僜(不:&[K<C>])->K<F>{let n=不.len();let 倒=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 偈=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亴=0usize;let mut 伤=vec![倒;n+1];let mut 侺=vec![false;n+1];loop{侺[亴]=true;let 倏=p[亴];let mut 众=倒;let mut 传=0usize;for j in 1..=n{if!侺[j]{let
乿=不[倏-1][j-1]-u[倏]-v[j];if 乿<伤[j]{伤[j]=乿;偈[j]=亴;}if 伤[j]<众{众=伤[j];传=j;}}}for j in 0..=n{if 侺[j]{u[p[j]]+=众;v[j]-=众;}else{伤[j]-=众;}}亴=传;if p[亴]==0{break;}}loop{let
传=偈[亴];p[亴]=p[传];亴=传;if 亴==0{break;}}}let mut 丄=vec![0usize;n];for j in 1..=n{if p[j]>0{丄[p[j]-1]=j-1;}}丄}#[derive(Clone,Copy)]enum 仏{伐(F),佷,}fn 乮(l:&J)->G{let
丂=l.丛();(l.仈(丂)&l.么(丂,1)).业(l.丒(丂))}fn 僳(l:&J,乕:&Z)->F{佢(l,乕,乔::乵).iter().filter(|c|matches!(c,L::主{..})).count()}fn 丠(q:F,並:G)->Q{(G::q(q).丏()&並).count()}fn
倇(l:&J,乎:G,与:F,侜:I)->K<[C;M]>{let 丹=l.map().valid;let mut 专=[C::MIN;M];专[与]=0;let mut 乥=vec![专];for _ in 0..侜{let mut 七=专;for q in 丹.iter(){for n in(G::q(q).丏()&丹).iter(){if
专[n]>C::MIN{let 丬=专[n]+C::from(乎.test(n));if 丬>七[q]{七[q]=丬;}}}}专=七;乥.push(专);}乥}fn 値(l:&J,乥:&[[C;M]],乎:G,三:F,k:F,)->G{let 丹=l.map().valid;let mut 丄=G::T;let
mut q=三;let mut k=k.min(乥.len()-1);while k>0&&乥[k][q]>0{let Some(七)=(G::q(q).丏()&丹).iter().filter(|&n|乥[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(乎.test(n))+乥[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乎.test(七){丄.set(七);}q=七;k-=1;}丄}fn 且(l:&J,乁:&Settings1)->K<L>{let 丂=l.丛();let 乂=仒::of(l);let mut 丌=l.clone();let mut 丄=K::new();let 世=临(l,l.丒(1-丂));let
万:K<F> =l.乸().iter().collect();let 倉=(l.下(丂)/佸).max(0)as F;let 些=match 乁.乢{亿::乵=>僳(l,&乁.乕).min(倉),亿::便=>倉,};let mut 休:K<仏> =万.iter().map(|&c|仏::伐(c)).collect();休.extend(std::iter::repeat_n(仏::佷,些));let
亀=儜(l,&乂,乁,休.len()+乁.侐);let 儨=乮(l);let 中=乂.仜();let n=休.len().max(亀.len());let mut 不=vec![vec![0i64;n];n];for(乻,&与)in 亀.iter().enumerate(){for(伕,介)in 不[乻].iter_mut().enumerate(){let
Some(&傭)=休.get(伕)else{*介=佹;continue;};let 佅=match 傭{仏::伐(q)=>{let d=l.map().丗(q,与);if d==I::MAX{None}else{Some(C::from(d)-1)}}仏::佷=>儨.iter().map(|c|l.map().丗(c,与)).filter(|&d|d!=I::MAX).min().map(C::from),};*介=match
佅{None=>佹,Some(佅)=>{let 丱=C::from(世[与])-1;let 仄=乁.仄*(佅-丱).max(0);let 侀=if 中.test(与){乁.侇}else{0};(佅+仄-侀).max(0)}};}}let 傯=僜(&不);let mut 侔:K<(F,F)> =K::new();let
mut 侸:K<(C,F)> =K::new();for(乻,&与)in 亀.iter().enumerate(){let 伕=傯[乻];if 不[乻][伕]>=佹{continue;}match 休.get(伕){Some(仏::伐(q))=>侔.push((*q,与)),Some(仏::佷)=>侸.push((不[乻][伕],与)),None=>{}}}let
mut 並=l.map().valid.业(l.丒(0)|l.丒(1));let mut 倚:K<F> =K::new();let mut 佞=G::T;let 亖=|q:F|{let 丅=l.亜(q).unwrap_or(1);let 估=並|l.丒(1-丂);(G::q(q).丏()&估&l.么(丂,丅)).count()};侔.sort_by_key(|&(q,与)|{let
僐=if 乁.侑{亖(q)}else{0};(僐,l.map().丗(q,与),q)});for&(三,与)in&侔{let Some(丅)=丌.亜(三)else{continue;};let 丩=丌.么(丂,丅);let d=丌.map().丗(三,与);let 乎=(並|丌.丒(1-丂)).业(佞);let
侜=d.saturating_add(乁.佋);let 乥=倇(&丌,乎,与,侜.saturating_sub(1));let 仫=乥[乥.len()-1];let 七=(G::q(三).丏()&丩).iter().filter(|&c|仫[c]>C::MIN).max_by_key(|&c|{let
傿=d.saturating_sub(丌.map().丗(c,与));let 估=並.test(c)||丌.丒(1-丂).test(c);(估&&乁.侷,C::from(乎.test(c))+仫[c],傿,丠(c,並),std::cmp::Reverse(c),)}).filter(|&c|{並.test(c)||丌.丒(1-丂).test(c)||丌.map().丗(c,与)<d});if
let Some(丣)=七{if 侳(&mut 丌,三,丣,&mut 丄){並.clear(丣);倚.push(三);if 乁.reserve{佞|=値(&丌,&乥,乎,丣,乥.len()-1);}}}}let 两=临(l,l.丒(丂));let mut 丵=G::T;for c in 並.iter(){if
两[c]<=世[c]{丵.set(c);}}for&三 in&万{if 倚.contains(&三)||!丌.乸().test(三){continue;}let Some(丅)=丌.亜(三)else{continue;};let 丩=丌.么(丂,丅);let 估=(G::q(三).丏()&並&丩).iter().max_by_key(|&c|(丠(c,並),世[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丣)=估{if 侳(&mut 丌,三,丣,&mut 丄){並.clear(丣);}continue;}let 丕=[丵&並,並].into_iter().find_map(|丐|{丐.iter().min_by_key(|&c|(丌.map().丗(三,c),c))});if let
Some(丕)=丕{let d=丌.map().丗(三,丕);let 七=(G::q(三).丏()&丩).iter().filter(|&c|丌.map().丗(c,丕)<d).min_by_key(|&c|c);if let Some(丣)=七{侳(&mut 丌,三,丣,&mut 丄);}}}侸.sort();let
mut 価=0usize;for&(_,与)in&侸{if 丌.下(丂)<佸{break;}let 丐=乮(&丌);let Some(书)=丐.iter().map(|c|丌.map().丗(c,与)).min()else{break;};let 乎=(並|丌.丒(1-丂)).业(佞);let 乥=倇(&丌,乎,与,书.saturating_add(乁.佋));let
仫=乥[乥.len()-1];let q=丐.iter().filter(|&c|丌.map().丗(c,与)==书).max_by_key(|&c|(仫[c],丠(c,並),std::cmp::Reverse(c)));let Some(q)=q else{break;};let(x,y)=侼(q);if
侧(&mut 丌,L::主{丅:1,x,y},&mut 丄){並.clear(q);価+=1;if 乁.reserve{佞|=値(&丌,&乥,乎,q,乥.len()-1);}}}if 乁.fill&&乁.乢==亿::乵{while 価<些&&丌.下(丂)>=佸{let q=(乮(&丌)&並).iter().max_by_key(|&c|{(丠(c,並),std::cmp::Reverse(世[c]),std::cmp::Reverse(c),)});let
Some(q)=q else{break;};let(x,y)=侼(q);if!侧(&mut 丌,L::主{丅:1,x,y},&mut 丄){break;}並.clear(q);価+=1;}}丄}const 偩:C=15;const 偾:C=6;const 俥:C=10;const 傑:I=3;#[derive(Clone,Copy)]struct
Z{open:C,佇:C,俼:I,侙:C,佒:C,倖:C,佟:I,俻:C,俺:C,佃:C,仳:C,伅:C,习:C,件:C,从:C,仯:C,}impl Z{const 仑:Z=Z{open:3,佇:2,俼:12,侙:4,佒:偩,倖:偾,佟:傑,俻:2,俺:2,佃:0,仳:0,伅:0,习:0,件:0,从:0,仯:0,};const
fn 儳(self,仳:C,伅:C,习:C)->Z{Z{仳,伅,习,..self}}const fn 儲(self,件:C,从:C)->Z{Z{件,从,..self}}}fn 伶(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 临(l:&J,三:G)->[I;M]{let mut
乀=[I::MAX;M];let 丹=l.map().valid;let mut 义=三&丹;let mut 乬=义;let mut d=0u8;while!义.is_empty(){for q in 义.iter(){乀[q]=d;}let 七=(义.丏()&丹).业(乬);乬|=七;义=七;d=d.saturating_add(1);}乀}fn
僔(l:&J)->I{let 乀=临(l,l.丒(0));l.丒(1).iter().map(|q|乀[q]).min().unwrap_or(I::MAX)}struct 仁<'a>{乕:&'a Z,丂:F,theirs:[I;M],伆:[I;M],free:G,丵:G,ours:G,丸:G,万:G,}impl
仁<'_>{fn new<'a>(l:&J,乕:&'a Z)->仁<'a>{let 丂=l.丛();let 丶=临(l,l.丒(丂));let 世=临(l,l.丒(1-丂));let 伆=临(l,G::q(U[1-丂]));let 並=l.map().valid.业(l.丒(0)|l.丒(1));let
mut 丵=G::T;for q in 並.iter(){if 丶[q]<=世[q]{丵.set(q);}}仁{乕,丂,theirs:世,伆,free:並,丵,ours:l.丒(丂),丸:l.map().丸,万:l.万(丂),}}fn value(&self,q:F)->C{let 一=self;let
p=一.乕;let 丠=C::from((G::q(q).丏()&一.free).count());let 俵=p.俼;let 佇=C::from(俵)-C::from(一.theirs[q].min(俵));let 仴=if p.侙>0{C::from(一.伆[q].min(30))/p.侙}else{0};let
丧=C::from((G::q(q).丏()&一.ours).count());let line=if 丧<=1{p.伅}else{0};let 习=if 一.丸.test(q){p.习}else{0};let 儙=if 丠==0{p.件}else{0};let 从=p.从*(22-C::from(一.伆[q].min(22)));let
仯=p.仯*C::from((G::q(q).丏()&一.万).count());p.open*丠+p.佇*佇-仴+p.仳*丧-line+习-儙+从-仯}fn 专(&self,丐:G)->P<F>{丐.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
侈(&mut self,q:F){let 一=self;一.free.clear(q);一.丵.clear(q);一.ours.set(q);一.万.set(q);}}fn 伩(l:&mut J,丑:L,丄:&mut Vec<L>)->O{let 亢=l.乃(丑);if 亢{丄.push(丑);}亢}fn
倵(l:&mut J,三:F,丣:F,丄:&mut Vec<L>)->O{let Some((上,_))=l.亼(三)else{return false;};let(x,y)=伶(丣);伩(l,L::乴{上,x,y},丄)}fn 優(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-俥}fn
儁(l:&J,乕:&Z)->K<L>{佢(l,乕,乔::乵)}#[derive(Clone,Copy,PartialEq)]enum 乔{乵,享{丷:I,万:I},}fn 佢(l:&J,乕:&Z,乢:乔)->K<L>{let mut 丌=l.clone();let mut 且=仁::new(l,乕);let
mut 丄=K::new();亷(&mut 丌,&mut 且,&mut 丄);match 乢{乔::乵=>價(&mut 丌,&mut 且,&mut 丄),乔::享{丷,万}=>{僎(&mut 丌,&mut 且,丷,万,&mut 丄)}}丄}fn 亷(l:&mut J,且:&mut 仁<'_>,丄:&mut
Vec<L>){let 丂=且.丂;let mut 亳:K<F> =K::new();loop{let 丩=l.么(丂,1);let mut 伀:P<(Q,F)> =None;for 三 in l.乸().iter(){if 亳.contains(&三){continue;}let 亖=(G::q(三).丏()&且.free&丩).count();if
亖==0{亳.push(三);}else if 伀.is_none_or(|(n,_)|亖<n){伀=Some((亖,三));}}let Some((_,三))=伀 else{break;};且.万.clear(三);match 且.专(G::q(三).丏()&且.free&丩){Some(丣)if
倵(l,三,丣,丄)=>且.侈(丣),_=>{且.万.set(三);亳.push(三);}}}for 三 in 亳{if!l.乸().test(三){continue;}let 与=[且.丵,且.free].into_iter().find_map(|丐|{丐.iter().min_by_key(|&q|(l.map().丗(三,q),q))});if
let Some(与)=与{倵(l,三,与,丄);}}}fn 價(l:&mut J,且:&mut 仁<'_>,丄:&mut Vec<L>){let 丂=且.丂;let p=且.乕;let h=(p.佒-C::from(l.乤())).max(p.倖);let 儤=(C::from(僔(l))-p.俻).max(0)/p.俺.max(1)+p.佃;loop{let
下=l.下(丂);let 万=l.侹(丂)as C;let 儀=C::from(且.丵.count());let q=且.专(l.仈(丂)&且.free&l.么(丂,1));let 偃=優(h,(儀/(万+1)).min(儤));let 俴=q.is_some()&&下>=俥&&偃>0;let 倗=4*h-争;let
习=倘(l,且);let 傺=习.is_some()&&下>=争&&倗>0;if 傺&&(!俴||倗>=偃){let(x,y)=伶(习.unwrap_or_default());if 伩(l,L::亪{x,y},丄){continue;}break;}let Some(q)=q.filter(|_|俴)else{break;};let(x,y)=伶(q);if!伩(l,L::主{丅:1,x,y},丄){break;}且.侈(q);}}fn
倘(l:&J,且:&仁<'_>)->P<F>{let 丂=且.丂;(l.map().丸&l.丆(丂)&l.仕()).iter().filter(|&习|且.theirs[习]>且.乕.佟).min_by_key(|&习|(l.map().丗(U[丂],习),习))}fn 僎(l:&mut J,且:&mut
仁<'_>,丷:I,万:I,丄:&mut Vec<L>,){let 丂=且.丂;for _ in 0..丷{let Some(习)=倘(l,且)else{break;};let(x,y)=伶(习);if!伩(l,L::亪{x,y},丄){break;}}for _ in 0..万{let Some(q)=且.专(l.仈(丂)&且.free&l.么(丂,1))else{break;};let(x,y)=伶(q);if!伩(l,L::主{丅:1,x,y},丄){break;}且.侈(q);}}fn
乪(l:&J)->O{if l.両().is_some(){return false;}let 丂=l.丛();let 下=l.下(丂);let 丨=仨::new(l,丂,true);l.丒(1-丂).iter().any(|q|丨.不[q]<=下)}#[derive(Clone,Copy)]struct
为{乏:I,乢:乔,}const 侽:I=I::MAX;const 偒:[乔;8]=[乔::乵,乔::享{丷:0,万:0},乔::享{丷:0,万:1},乔::享{丷:0,万:2},乔::享{丷:0,万:侽,},乔::享{丷:1,万:0},乔::享{丷:1,万:1},乔::享{丷:1,万:侽,},];struct
佫<'a>{丯:&'a[Z],仆:Z,rest:为,亱:P<亂>,乁:Settings2,}impl 佫<'_>{fn 乕(&self,丢:为)->&Z{&self.丯[F::from(丢.乏)%self.丯.len()]}fn 丈(&self,l:&J,丢:为)->K<L>{佢(l,self.乕(丢),丢.乢)}}fn
傫(l:&J,丈:&[L],仆:&Z)->(J,O){let mut 七=l.clone();七.作(丈);if 七.両().is_some()||乪(&七){return(七,true);}let 乺=儁(&七,仆);七.作(&乺);let 亹=七.両().is_some()||乪(&七)||七.乤()>促;(七,亹)}fn
僸(l:&J,丙:&[为],仔:&佫<'_>)->K<丮>{let 丂=l.丛();let mut 伡=vec![伮(l,丂)];let mut 乭=l.clone();for 且 in 0..促 as F{let 丢=丙.get(且).copied().unwrap_or(仔.rest);let 丈=仔.丈(&乭,丢);let(七,亹)=傫(&乭,&丈,&仔.仆);if
亹{break;}乭=七;伡.push(伮(&乭,丂));}伡}fn 伮(l:&J,丂:F)->丮{let mut hash:丮=0xcbf29ce484222325;let mut 侢=|伈:丮|hash=(hash^伈).wrapping_mul(0x0100000001b3);for(儛,乐)in[l.丒(丂),l.万(丂),l.丷()].into_iter().enumerate(){for
q in 乐.iter(){侢(((儛 as 丮)<<8)|q as 丮);}侢(0xff);}侢(l.下(丂)as 丮);hash}enum 于{伏{个:F,亝:F,串:亾,佝:Q,},伹(Settings1),}struct 亡{亘:于,丯:K<Z>,仆:Z,丙:K<为>,位:K<丮>,仟:F,乊:Q,亹:O,}impl
亡{fn new(亘:于,丯:K<Z>,仆:Z)->亡{亡{亘,丯,仆,丙:K::new(),位:K::new(),仟:0,乊:0,亹:false,}}fn 僶(&mut self,l:&J)->O{let 一=self;let 儎=matches!(一.亘,于::伏{..}|于::伹(_));let
僷=!儎&&一.乊>0&&一.仟>=一.位.len();if!一.亹&&(僷||乪(l)){一.亹=true;}!一.亹}fn end(&mut self){self.亹=true;}fn 且(&mut self,l:&J,亱:P<亂>)->K<L>{let 一=self;let 仔=佫{丯:&一.丯,仆:一.仆,rest:为{乏:0,乢:乔::乵,},亱,乁:Settings2::DEFAULT,};if
let 于::伹(乁)=一.亘{一.乊+=1;一.仟+=1;return 且(l,&乁);}let 丂=l.丛();let 僬=一.位.get(一.仟)==Some(&伮(l,丂));let 働=matches!(一.亘,于::伏{佝,..}if 佝>0&&l.乤()>=佝);if!僬||働{let
乶=一.乊==0;一.丙=match 一.亘{于::伏{个,亝,串,..}=>{let 串=&串;let 乲=傌;let mut 之=丙(l,&一.丯,乲,个,亝,亱,串,);之.resize(个,乲);之}于::伹(_)=>K::new(),};一.位=match 一.亘{于::伏{..}=>儏(l,&一.丯,&一.丙),_=>僸(l,&一.丙,&仔),};一.仟=0;一.乊+=1;}let
丢=一.丙.get(一.仟).copied().unwrap_or(仔.rest);一.仟+=1;仔.丈(l,丢)}}fn 儏(l:&J,丯:&[Z],丙:&[为])->K<丮>{let 丂=l.丛();let mut 伡=K::with_capacity(丙.len());let mut 乭=l.clone();for&丢
in 丙{if 乭.両().is_some(){break;}伡.push(伮(&乭,丂));乭=佻(&乭,&丈(&乭,丯,丢));}伡}const 促:Q=20;const 傠:C=3;#[derive(Clone,Copy)]struct Settings2{个:C,}impl Settings2{const
DEFAULT:Settings2=Settings2{个:傠};}fn 之(l:&J,三:G)->[I;M]{let 丹=l.map().valid;let mut 乀=[I::MAX;M];let mut 义=三&丹;let mut 乬=义;let mut d=0u8;while!义.is_empty(){for
q in 义.iter(){乀[q]=d;}let 七=(义.丏()&丹).业(乬);乬|=七;义=七;d=d.saturating_add(1);}乀}fn 儭(l:&J,丁:F)->C{let 两=之(l,l.丆(丁));let 世=之(l,l.丆(1-丁));l.map().valid.iter().map(|q|match
两[q].cmp(&世[q]){_ if 两[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 傓:[Z;27]=丯();const
傌:为=为{乏:16,乢:乔::乵,};const fn 丯()->[Z;27]{let 仳=[0,2,4];let 伅=[0,3,6];let 从=[0,1,2];let mut 丄=[Z::仑;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
件=if g>0||t>0{4}else{0};丄[i]=Z::仑.儳(仳[g],伅[t],0).儲(件,从[w]);i+=1;}丄}#[derive(Default)]struct 併{丐:C,丞:C,下:C,亵:C,丼:C,伳:C,丨:C,令:C,}#[derive(Clone,Copy)]struct
亾{丐:C,丞:C,下:C,丼:C,亵:C,伳:C,丨:C,令:C,中:C,仭:O,乞:亍,}#[derive(Clone,Copy,PartialEq)]enum 亍{俏,佩(伽),亞(亞),係,}fn 儥(l:&J,丁:F)->[I;M]{let 两=临(l,G::q(U[丁]));let 世=临(l,G::q(U[1-丁]));let
mut 亥=G::T;let mut 丵=G::T;for c in l.map().valid.iter(){if 两[c]==I::MAX||世[c]==I::MAX{continue;}if 两[c]==世[c]{亥.set(c);}if 两[c]<=世[c]{丵.set(c);}}let d=临(l,亥);let
mut 丄=[I::MAX;M];for c in 丵.iter(){丄[c]=d[c];}丄}fn 傳(l:&J,丁:F,亥:&[I;M])->C{let d=临(l,l.丆(丁));(0..M).filter(|&c|亥[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&亥[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 亞{偗{乨:C},保{乨:C,以:C},傈,}impl
亞{fn 侻(self,a:C,b:C,d:C)->C{let 仡=(a-b).abs()/2;match self{亞::偗{乨}=>(乨-仡-(a+b-d)/2).max(0),亞::保{乨,以}if a<=以=>(乨-仡).max(0),亞::保{..}=>0,亞::傈=>a.min(d/2),}}}fn
僑(l:&J,丁:F,侒:亞)->[C;M]{let 两=临(l,G::q(U[丁]));let 世=临(l,G::q(U[1-丁]));let d=C::from(两[U[1-丁]]);let mut 丄=[0;M];for c in 0..M{if 两[c]!=I::MAX&&世[c]!=I::MAX{丄[c]=侒.侻(C::from(两[c]),C::from(世[c]),d);}}丄}#[derive(Clone,Copy,PartialEq)]enum
伽{俐(C),}impl 伽{fn 侻(self,仡:C)->C{match self{伽::俐(乨)=>(乨-仡).max(0),}}}type 伿=[C;4];fn 億(l:&J,丁:F)->[C;M]{struct 伉{丆:G,买:[伸;M],乙:[伸;M],size:[C;M],丼:[C;M],仍:伸,}fn
代(d:&mut 伉,v:F){d.仍+=1;d.买[v]=d.仍;d.乙[v]=d.仍;d.size[v]=1;for w in(G::q(v).丏()&d.丆).iter(){if d.买[w]==0{代(d,w);d.size[v]+=d.size[w];d.乙[v]=d.乙[v].min(d.乙[w]);if
d.乙[w]>=d.买[v]{d.丼[v]+=d.size[w];}}else{d.乙[v]=d.乙[v].min(d.买[w]);}}}let mut d=伉{丆:l.丆(丁),买:[0;M],乙:[0;M],size:[0;M],丼:[0;M],仍:0,};let 了=U[丁];if d.丆.test(了){代(&mut
d,了);}d.丼[了]=0;d.丼}fn 儆(l:&J,丁:F)->併{let 丆=l.丆(丁);let 丼=億(l,丁);let 伆=临(l,G::q(U[1-丁]));let mut 丄=併{丐:C::from(丆.count()),丞:l.丞(丁),下:l.下(丁),丨:C::MAX,令:儭(l,丁)/2,..併::default()};for
q in 丆.iter(){if 丼[q]>0{丄.亵+=1;丄.丼=丄.丼.max(丼[q]);}if q!=U[丁]&&(G::q(q).丏()&丆).count()<=1{丄.伳+=1;}丄.丨=丄.丨.min(C::from(伆[q]));}丄}fn 儬(l:&J,丁:F,串:&亾,伣:G)->C{let
仛=C::from((l.丒(丁)&伣).count());let s=儆(l,丁);let o=串;o.丐*s.丐+o.丞*s.丞+o.下*s.下-o.丼*s.丼-o.亵*s.亵-o.伳*s.伳-o.丨*s.丨.min(30)+o.令*s.令+o.中*仛}fn 仜(l:&J)->G{仒::of(l).仜()}fn
僥(l:&J)->G{let 两=临(l,G::q(U[0]));let 世=临(l,G::q(U[1]));let mut 乄=G::T;for c in l.map().valid.iter(){if 两[c]!=I::MAX&&世[c]!=I::MAX&&两[c].abs_diff(世[c])<=2{乄.set(c);}}乄}fn
伂(l:&J)->[C;M]{let a=临(l,G::q(U[0]));let b=临(l,G::q(U[1]));let mut 丄=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丄[c]=C::from(a[c].abs_diff(b[c]))/2;}}丄}#[derive(Clone,Copy)]struct
仪{中:G,乄:G,乏:[C;M],伂:[C;M],义:G,front:[I;M],}impl 仪{fn of(l:&J,串:&亾)->仪{let 倔=matches!(串.乞,亍::俏);let mut 乏=[0;M];if let 亍::佩(僃)=串.乞{for(w,仡)in 乏.iter_mut().zip(伂(l)){*w=if
仡==C::MAX{0}else{僃.侻(仡)};}}if let 亍::亞(侒)=串.乞{乏=僑(l,l.丛(),侒);}let(僿,义)=match 串.乞{_=>([C::MAX;M],G::T),};仪{乏,伂:僿,义,front:if 串.乞==亍::係{儥(l,l.丛())}else{[I::MAX;M]},中:if
串.中!=0||倔{仒::of(l).仜()}else{G::T},乄:if 倔{僥(l)}else{G::T},}}}fn 价(l:&J,丁:F,串:&亾,乂:仪)->伿{let 侵=儬(l,丁,串,乂.中);match 串.乞{亍::佩(_)|亍::亞(_)=>{let 丆=l.丆(丁);let
儯:C=丆.iter().map(|c|乂.乏[c]).sum();[C::from(丆.count()),儯,侵,0]}亍::係=>[C::from(l.丆(丁).count()),傳(l,丁,&乂.front),侵,0,],亍::俏=>{let 丒=l.丒(丁);let 中=C::from((丒&乂.中).count());let
乄=C::from((丒&乂.乄).count());match 串.乞{_=>[C::from(l.丆(丁).count()),中,乄,侵],}}}}fn 佻(l:&J,丈:&[L])->J{let mut 七=l.clone();七.作(丈);if 七.両().is_none(){七.作(&[]);}七}const
侾:I=I::MAX;fn 丈(l:&J,丯:&[Z],丢:为)->K<L>{if 丢.乏==侾{let 乁=Settings1{乕:丯.first().copied().unwrap_or(Z::仑),乢:if 丢.乢==乔::乵{亿::乵}else{亿::便},..Settings1::DEFAULT};return
且(l,&乁);}佢(l,&丯[F::from(丢.乏)%丯.len()],丢.乢,)}fn 倧(l:&J,丯:&[Z],丙:&[为],乲:为,个:F,)->J{let mut 乭=l.clone();for 且 in 0..个{if 乭.両().is_some(){break;}let 丢=丙.get(且).copied().unwrap_or(乲);乭=佻(&乭,&丈(&乭,丯,丢));}乭}fn
儕(l:&J,丯:&[Z],仭:O)->K<(为,K<L>)>{let mut 丄:K<(为,K<L>)> =K::new();if 仭{for 乢 in[乔::乵,乔::享{丷:0,万:I::MAX,},]{let 丢=为{乏:侾,乢,};let c=丈(l,丯,丢);if!丄.iter().any(|(_,乆)|*乆==c){丄.push((丢,c));}}}for
乏 in 0..丯.len().min(F::from(I::MAX)){for&乢 in&偒{let 丢=为{乏:乏 as I,乢,};let c=丈(l,丯,丢);if!丄.iter().any(|(_,乆)|*乆==c){丄.push((丢,c));}}}丄}fn 傱(l:&J,丯:&[Z],乲:为,个:F,亝:F,亱:P<亂>,串:&亾,)->K<为>{let
丂=l.丛();let 伨=||亱.is_some_and(|t|亂::now()>=t);let 乂=仪::of(l,串);let 伪=|丙:&[为]|价(&倧(l,丯,丙,乲,个),丂,串,乂);let mut 专:(伿,K<为>)=(伪(&[]),K::new());let mut 义:K<(J,K<为>)>
=vec![(l.clone(),K::new())];for 僆 in 0..个{let mut 亰:K<(伿,J,K<为>)> =K::new();let mut 乬:HashMap<丮,F> =HashMap::new();for(倜,丙)in&义{if 伨(){break;}for(丢,c)in
儕(倜,丯,串.仭){let 七=佻(倜,&c);let mut 來=丙.clone();來.push(丢);let v=价(&七,丂,串,乂);let 价=伮(&七,丂);match 乬.get(&价){Some(&乗)if 亰[乗].0>=v=>{}Some(&乗)=>亰[乗]=(v,七,來),None=>{乬.insert(价,亰.len());亰.push((v,七,來));}}}}if
亰.is_empty(){break;}亰.sort_by(|a,b|b.0 .cmp(&a.0));亰.truncate(亝);let 乷=僆+1==个||伨();for(i,(_,_,丙))in 亰.iter().take(if 乷{亝}else{1}).enumerate(){if i>0&&伨(){break;}let
v=伪(丙);if v>专.0{专=(v,丙.clone());}}if 乷{break;}义=亰.into_iter().map(|(_,s,p)|(s,p)).collect();}专.1}fn 丙(l:&J,丯:&[Z],乲:为,个:F,僤:F,亱:P<亂>,串:&亾,)->K<为>{let 丂=l.丛();let
乂=仪::of(l,串);let 伪=|丙:&[为]|价(&倧(l,丯,丙,乲,个),丂,串,乂);let mut 专:(伿,K<为>)=(伪(&[]),K::new());let mut 亝=2;loop{let 乍=傱(l,丯,乲,个,亝,亱,串);let v=伪(&乍);if v>专.0{专=(v,乍);}let
伨=亱.is_some_and(|t|亂::now()>=t);if 伨||(亱.is_none()&&亝>=僤){break;}亝*=2;}专.1}const 偨:F=4+12;fn 个(侚:impl BufRead,mut 佚:impl Write,mut 佽:impl FnMut(&str,&str,Q,亂)->乖,)->Result<(),乖>{let
mut 久=俑{倫:侚};let Some(乶)=久.line()?else{return Ok(());};let mut 仵=format!("{}\n",乶);久.take(丘(&乶,"mine spot count")?,&mut 仵)?;let mut 且=0;loop{let Some(乶)=久.line()?else{return
Ok(());};let 举=亂::now();且+=1;let mut 亇=format!("{}\n",乶);久.take(偨-1,&mut 亇)?;for 乚 in["building count","unit count"]{let line=久.倭()?;let n=丘(&line,乚)?;亇.push_str(&line);亇.push('\n');久.take(n,&mut
亇)?;}let 佽=佽(&仵,&亇,且,举);writeln!(佚,"{}",佽).and_then(|()|佚.flush()).map_err(|他|format!("turn {}: cannot write the answer: {}",且,他))?;}}struct 俑<R>{倫:R,}impl<R:BufRead>俑<R>{fn
line(&mut self)->Result<P<乖>,乖>{let mut line=乖::new();match self.倫.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(他)=>Err(format!("cannot read input: {}",他)),}}fn
倭(&mut self)->Result<乖,乖>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,亇:&mut String)->Result<(),乖>{for
_ in 0..n{亇.push_str(&self.倭()?);亇.push('\n');}Ok(())}}fn 丘(line:&str,乚:&str)->Result<F,乖>{match line.trim().parse::<F>(){Ok(n)if n<=側=>Ok(n),_=>Err(format!("bad {} {line:?}",乚)),}}const
側:F=144;const 傛:C=3;const 偯:C=2;const 偹:F=16;fn 僴(l:&J,丄:&mut Vec<L>){let 丁=l.丛();let mut 亘=俛{s:l.clone(),丁,丱:1-丁,与:U[1-丁],佑:U[丁],且:l.乤(),丄,};亘.run();}struct
俛<'a>{s:J,丁:F,丱:F,与:F,佑:F,且:Q,丄:&'a mut Vec<L>,}fn 佥(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侣(上:Q,q:F)->L{let(x,y)=佥(q);L::乴{上,x,y}}fn 偀(丅:I,q:F)->L{let(x,y)=佥(q);L::主{丅,x,y}}impl
俛<'_>{fn run(&mut self){let 一=self;if 一.佖(){return;}一.亷();if 一.佖(){return;}一.乾();一.些();}fn 乀(&self,三:F,丣:F)->C{C::from(self.s.map().丗(三,丣))}fn 乃(&mut self,丑:L)->O{let
乣=self.s.乃(丑);if 乣{self.丄.push(丑);}乣}fn 伇(&self,丳:&J)->O{丳.両()==Some(乯::亏(self.丁))}fn 佖(&mut self)->O{let 一=self;if 一.s.両().is_some(){return false;}for
q in 一.s.乸().iter(){if 一.乀(q,一.与)!=1{continue;}let Some((上,_))=一.s.亼(q)else{continue;};let 丑=侣(上,一.与);let mut 丳=一.s.clone();if 丳.乃(丑)&&一.伇(&丳){return 一.乃(丑);}}let
Some(东)=一.傻()else{return false;};let mut 丳=一.s.clone();if!东.iter().all(|&丑|丳.乃(丑))||!一.伇(&丳){return false;}for 丑 in 东{一.乃(丑);}true}fn 傻(&self)->P<K<L>>{let
一=self;const 俇:C=C::MAX;let 亅=一.s.丽().亓();let 丩:K<G> =(1..=亅).map(|丅|一.s.么(一.丁,丅)).collect();let 伜=|q:F|{丩.iter().position(|乐|乐.test(q)).map(|丟|(丟 as I+1,10*(丟
as C+1)))};let 丆=一.s.丆(一.丁);let mut 不=[俇;M];let mut 乌=[F::MAX;M];let mut 乣=[false;M];for q in 丆.iter(){不[q]=0;}loop{let mut 七:P<F> =None;for q in 0..M{if!乣[q]&&不[q]<俇&&七.is_none_or(|n|不[q]<不[n]){七=Some(q);}}let
Some(q)=七 else{break;};乣[q]=true;if q==一.与{break;}for&九 in&一.s.map().丏[0][q]{if 九==二{break;}let 九=F::from(九);if 乣[九]||丆.test(九){continue;}let Some((_,今))=伜(九)else{continue;};if
不[q]+今<不[九]{不[九]=不[q]+今;乌[九]=q;}}}if 不[一.与]>一.s.下(一.丁){return None;}let mut 东=K::new();let mut q=一.与;while!丆.test(q){let(丅,_)=伜(q)?;东.push(偀(丅,q));q=乌[q];}东.reverse();Some(东)}fn
乎(&self,乜:&J)->C{let(丂,仌)=(self.丁,self.丱);let 乌=&self.s;let 丘=|乐:G|C::from(乐.count());let 丧=丘(乜.丒(丂))-丘(乌.丒(丂));let 亨=丘(乌.丒(仌))-丘(乜.丒(仌));let 侠=丘(乌.丆(仌))-丘(乜.丆(仌));let
体=乌.任(仌)-乜.任(仌);let 丷=丘(乌.丷()&乌.丒(仌))-丘(乜.丷()&乜.丒(仌));12*丧+20*亨+8*(侠-亨).max(0)+5*体+25*丷}fn 倳(&self,q:F)->C{let 一=self;let 习=一.s.map().丸.test(q);if 习&&一.s.丽().佄()&&!一.s.丒(一.丁).test(q){20}else{0}}fn
倁(&self,乒:C)->O{let 一=self;一.s.万(一.丱).iter().any(|q|一.乀(q,一.佑)<=乒)}fn 亷(&mut self){let 一=self;let mut 万:K<(C,F)> =一.s.乸().iter().map(|q|(一.乀(q,一.与),q)).collect();万.sort_unstable();for(仃,q)in
万{if 一.s.両().is_some(){return;}let Some((上,丅))=一.s.亼(q)else{continue;};let 丩=一.s.么(一.丁,丅);let 九=一.s.map().丏[一.丁][q];let mut 专:P<(C,F)> =None;for&七 in 九.iter().take_while(|&&七|七!=二){let
七=F::from(七);if!丩.test(七){continue;}let mut 丳=一.s.clone();if!丳.乃(侣(上,七)){continue;}let 丬=if 一.伇(&丳){100000}else{一.乎(&丳)+10*(仃-一.乀(七,一.与))+一.倳(七)};if 专.is_none_or(|(乨,_)|丬>乨){专=Some((丬,七));}}if
let Some((丬,七))=专{if 丬>0{一.乃(侣(上,七));}}}}fn 乾(&mut self){let 一=self;if 一.s.両().is_some(){return;}if 一.倁(傛){一.儧();}else{一.丷();}}fn 儧(&mut self){let 一=self;if!一.s.丽().们()||一.s.下(一.丁)<争{return;}let
九=一.s.map().丏[一.丁][一.佑];let 丐:K<F> =九.iter().take_while(|&&q|q!=二).map(|&q|F::from(q)).collect();let 丶=一.s.丒(一.丁);if 丐.iter().any(|&q|丶.test(q)&&一.s.乇().test(q)){return;}for
q in 丐{let(x,y)=佥(q);if 一.乃(L::乽{x,y}){return;}}}fn 丷(&mut self){let 一=self;if!一.s.丽().佄(){return;}while 一.s.下(一.丁)>=争{let 並=一.s.map().丸&一.s.丒(一.丁)&一.s.丆(一.丁)&一.s.仕();let
Some(q)=並.iter().min_by_key(|&q|(一.乀(q,一.佑),q))else{return;};let(x,y)=佥(q);if!一.乃(L::亪{x,y}){return;}}}fn 些(&mut self){let 一=self;let 倮=if 一.且<=3{0}else{5};for
_ in 0..偹{if 一.s.両().is_some(){return;}let 下=一.s.下(一.丁);if 下<10{return;}let 僝=一.倁(偯);let 书=一.s.仈(一.丁);let mut 专:P<(C,L)> =None;let mut 侱:P<(C,L)> =None;let
mut 俱=G::T;for 丅 in 1..=一.s.丽().亓(){let 丩=一.s.么(一.丁,丅);let 亖=(书&丩).业(俱);俱=丩;let 今=10*C::from(丅);if 今>下{continue;}for q in 亖.iter(){let 丑=偀(丅,q);let mut
丳=一.s.clone();if!丳.乃(丑){continue;}let 亶=丳.下(一.丁);let 丞=丳.丞(一.丁);let 体=一.s.任(一.丱)-丳.任(一.丱);let 催=if 体>0&&僝{亶+丞>=0}else{丞>=0&&(亶>=倮||体>0)};if!催{continue;}let
乎=一.乎(&丳);let 丬=乎+一.倳(q)-6*一.乀(q,一.与)-10*(C::from(丅)-1);if 乎>0{if 专.is_none_or(|(乨,_)|丬>乨){专=Some((丬,丑));}}else if 丞>=3&&亶>=倮+10&&丅==1{if 侱.is_none_or(|(乨,_)|丬>乨){侱=Some((丬,丑));}}}}let
Some((_,丑))=专.or(侱)else{return;};if!一.乃(丑){return;}}}}fn 僖(l:&J,丄:&mut Vec<L>){丄.clear();僴(l,丄);}const 亯:C=C::MAX;const 偸:F=8;struct 仨{不:[C;M],乌:[I;M],}fn
佉(l:&J,丁:F,亷:O,与:F)->C{例(l,丁,仝(l,丁,亷),与)}fn 例(l:&J,丁:F,仝:G,与:F)->C{let mut 不=亯;倅(l,丁,仝,|丢,new|{if new.test(与){不=10*丢 as C;}不!=亯});不}fn 僻(l:&J,丁:F,仝:G,乑:C)->G{let
mut 乪=G::T;倅(l,丁,仝,|丢,new|{if 10*丢 as C>乑{return true;}乪|=new;false});乪.业(l.丆(丁))}fn 仝(l:&J,丁:F,亷:O)->G{if 亷&&丁==l.丛(){l.乸()}else{G::T}}fn 倅(l:&J,丁:F,仝:G,mut
代:impl FnMut(F,G)->O,){let 亅=l.丽().亓();let mut 伜=[G::T;3];let mut 乬=G::T;for 丅 in 1..=亅{let 乐=l.么(丁,丅);伜[F::from(丅)-1]=乐.业(乬);乬|=乐;}let mut 丠=[l.丆(丁),G::T,G::T,G::T];for
q in(仝&l.万(丁)).iter(){if let Some(丅)=l.亜(q).filter(|&丅|丅<=亅){丠[0]|=G::q(q).丏()&l.么(丁,丅);}}let mut 乣=G::T;for 丢 in 0..{let new=丠[丢%4].业(乣);丠[丢%4]=G::T;if
代(丢,new){return;}乣|=new;let 七=new.丏().业(乣);for(仫,丐)in 伜.iter().enumerate(){丠[(丢+仫+1)%4]|=七&*丐;}if 丠.iter().all(|乐|乐.业(乣).is_empty()){break;}}}impl 仨{fn
new(l:&J,丁:F,亷:O)->仨{let 亅=F::from(l.丽().亓());let mut 丩=[G::T;3];for(丟,乐)in 丩.iter_mut().enumerate().take(亅){*乐=l.么(丁,丟 as I+1);}let 丆=l.丆(丁);let 丏=&l.map().丏[0];let
mut 不=[亯;M];let mut 乌=[二;M];let mut 丠=[0u8;M];let mut 丘=0;let mut 倱=G::T;for q in 丆.iter(){不[q]=0;丠[丘]=q as I;丘+=1;}if 亷&&丁==l.丛(){for 三 in l.乸().iter(){let
Some(丅)=l.亜(三)else{continue;};let 之=G::q(三).丏()&丩[F::from(丅)-1];for q in 之.iter(){if 不[q]!=0{不[q]=0;丠[丘]=q as I;丘+=1;}}}}while 丘>0{let mut 专=0;for 丟 in
1..丘{let(a,b)=(F::from(丠[丟]),F::from(丠[专]));if(不[a],a)<(不[b],b){专=丟;}}let q=F::from(丠[专]);丘-=1;丠[专]=丠[丘];倱.set(q);for&七 in&丏[q]{if 七==二{break;}let 七=F::from(七);if
倱.test(七)||丆.test(七){continue;}let Some(丅)=丩[..亅].iter().position(|乐|乐.test(七))else{continue;};let 倿=不[q]+10*(丅 as C+1);if 倿<不[七]{if 不[七]==亯{丠[丘]=七 as
I;丘+=1;}不[七]=倿;乌[七]=q as I;}}}仨{不,乌}}fn path(&self,与:F)->K<F>{let mut path=K::new();if 与>=M||self.不[与]==亯{return path;}let mut q=与;while self.乌[q]!=二{path.push(q);q=F::from(self.乌[q]);}path.reverse();path}}struct
伻{丈:K<L>,不:C,}fn 僣(l:&J,丁:F,q:F)->P<I>{(1..=l.丽().亓()).find(|&丅|l.么(丁,丅).test(q))}fn 僾(丌:&mut J,丂:F,丕:F,乑:C,丈:&mut Vec<L>,亙:&mut i64,)->O{loop{if 丌.両()==Some(乯::亏(丂)){return
true;}if 丌.丒(丂).test(丕)&&丌.丆(丂).test(丕){return true;}let 丨=仨::new(丌,丂,false);let 不=丨.不[丕];if 不==亯||*亙+不>乑{return false;}let Some(&q)=丨.path(丕).first()else{return
false;};let Some(丅)=僣(丌,丂,q)else{return false;};let 丑=L::主{丅,x:(q%Y)as I,y:(q/Y)as I,};if!丌.乃(丑){return false;}*亙+=10*C::from(丅);丈.push(丑);}}fn 俯(l:&J,丂:F,亀:&[F],乑:C)->P<伻>{let
mut 丌=l.clone();let mut 丈=K::new();let mut 亙=0;for&丕 in 亀{if!僾(&mut 丌,丂,丕,乑,&mut 丈,&mut 亙){return None;}}(亙<=乑).then_some(伻{丈,不:亙,})}fn 侄(l:&J,与:F,乑:C)->P<伻>{if
乑<0||l.両().is_some()||与>=M||l.map().乡.test(与){return None;}let 丂=l.丛();if l.丒(丂).test(与)&&l.丆(丂).test(与){return Some(伻{丈:K::new(),不:0,});}let mut 专=俯(l,丂,&[与],乑);let
乶=仨::new(l,丂,false);let mut 乇:K<F> =(l.乇()&l.丒(1-丂)).iter().filter(|&乩|乩!=与&&乶.不[乩]!=亯).collect();乇.sort_by_key(|&乩|(l.map().丗(乩,与),乩));乇.truncate(偸);for
乩 in 乇{if let Some(东)=俯(l,丂,&[乩,与],乑){if 专.as_ref().is_none_or(|乨|东.不<乨.不){专=Some(东);}}}专}const 偶:F=20000;struct 似{丈:K<L>,不:C,丐:C,万:C,乾:C,丞:C,丰:C,}struct
佯{丐:C,万:C,乾:C,}fn 倌(l:&J,丁:F)->佯{let 丆=l.丆(丁);let 万=l.万(丁)&丆;let 丬=万.iter().filter_map(|q|l.亜(q)).map(|丅|10*C::from(丅)).sum();佯{丐:C::from(丆.count()),万:丬,乾:C::from(((l.丷()|l.乇())&丆).count()),}}struct
俠{丂:F,亩:G,乌:佯,丞:C,仺:F,乑:C,仿:F,path:K<L>,乍:K<似>,index:HashMap<伋,F,BuildHasherDefault<DefaultHasher>>,}type 伋=[丮;3];fn 儱(mut 互:伋,q:F)->伋{互[q/64]|=1<<(q%64);互}impl
俠{fn 俿(&mut self,丌:&J,乷:P<F>,亨:伋,不:C,)->O{let 一=self;let 丂=一.丂;let 亖=match 乷{None=>丌.仈(丂),Some(q)=>G::q(q).丏(),}&一.亩;let 亖=亖.业(丌.丒(丂));let mut 丩=[G::T;3];for
丅 in 1..=丌.丽().亓(){丩[F::from(丅)-1]=丌.么(丂,丅);}for q in 亖.iter(){let 亅=F::from(丌.丽().亓());let Some(丟)=丩[..亅].iter().position(|乐|乐.test(q))else{continue;};let
丅=丟 as I+1;let 今=10*C::from(丅);if 不+今>一.乑{continue;}if 一.仿>=偶{return false;}一.仿+=1;let mut 七=丌.clone();let 丑=L::主{丅,x:(q%Y)as I,y:(q/Y)as I,};if!七.乃(丑){continue;}一.path.push(丑);let
伄=儱(亨,q);一.僼(&七,伄,不+今);let 僄=一.path.len()<一.仺&&七.両().is_none();let 僕=!僄||一.俿(&七,Some(q),伄,不+今);一.path.pop();if!僕{return false;}}true}fn 僼(&mut self,乜:&J,伄:伋,不:C){let
一=self;let 伧=1-一.丂;let 伦=倌(乜,伧);let 丐=一.乌.丐-伦.丐;let 万=一.乌.万-伦.万;if 丐<=0&&万<=0{return;}let 乾=一.乌.乾-伦.乾;let 丞=乜.丞(一.丂)-一.丞;let 儴=乜.両()==Some(乯::亏(一.丂));let
丰=if 儴{C::MAX/2}else{3*丐+万+10*乾+2*丞-不};if let Some(&乗)=一.index.get(&伄){if 不>=一.乍[乗].不{return;}}let 丼=似{丈:一.path.clone(),不,丐,万,乾,丞,丰,};match 一.index.get(&伄){Some(&乗)=>一.乍[乗]=丼,None=>{一.index.insert(伄,一.乍.len());一.乍.push(丼);}}}}fn
俽(l:&J,仺:F,乑:C,乒:F)->K<似>{也(l,仺,乑,乒).0}fn 也(l:&J,仺:F,乑:C,乒:F)->(K<似>,F){if l.両().is_some()||仺==0||乒==0{return(K::new(),0);}let 丂=l.丛();let 伧=1-丂;let mut
侭=俠{丂,亩:l.丒(伧)|l.丒(伧).丏(),乌:倌(l,伧),丞:l.丞(丂),仺,乑,仿:0,path:K::new(),乍:K::new(),index:HashMap::default(),};侭.俿(l,None,[0;3],0);let 仿=侭.仿;let mut 乍=侭.乍;乍.sort_by(|a,b|b.丰.cmp(&a.丰).then(a.不.cmp(&b.不)));乍.truncate(乒);(乍,仿)}const
佂:C=1000000;const 偷:C=200;const 偖:C=2;fn 僁(l:&J,丁:F)->G{let 丆=l.丆(丁);let 乇=l.乇()&丆;(乇|乇.丏())&丆}fn 丵(l:&J,丁:F)->C{let 丆=l.丆(丁);let 万:C=(l.万(丁)&丆).iter().filter_map(|q|l.亜(q)).map(|丅|10*C::from(丅)).sum();l.下(丁)+3*l.丞(丁)+4*C::from(丆.count())+万+偖*C::from(僁(l,丁).count())}fn
倕(l:&J,丁:F,乹:G)->C{僓(l,丁,例(l,丁,乹,U[1-丁]),)}fn 僓(l:&J,丁:F,不:C)->C{if 不==亯{return C::MIN;}l.下(丁)+l.丞(丁)-不}fn 倨(亦:C)->C{亦.clamp(-偷,0)}fn 僌(l:&J,丁:F,乹:[G;2],)->(C,[C;2]){let
乆=1-丁;match l.両(){Some(乯::亏(儰))=>{return(if 儰==丁{佂}else{-佂},[0,0])}Some(乯::俅)=>return(0,[0,0]),None=>{}}let mut 丰=丵(l,丁)-丵(l,乆);if 丁==l.丛(){let mut 七=l.clone();七.仱();if
七.両().is_none()&&佉(&七,乆,true,U[丁])<=七.下(乆){丰-=佂/2;}}let 仹=[倕(l,丁,乹[0]),倕(l,乆,乹[1]),];(丰+倨(仹[0])-倨(仹[1]),仹)}
