// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伸=u16;type Q=u32;type 丮=u64;type 伷=i32;type C=i64;type 人=f64;type O=bool;type 乖=String;type 亂=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
偩:O=true;fn 僙(l:&J,q:F,伲:C,丩:G,东:C)->O{let 丂=l.丛();let 主=1-丂;let 倍=偩&&东<=l.下(主)+2*l.丞(主).max(0);let 伉=G::q(q)|G::q(q).丏();if!倍&&(伉&丩).is_empty(){return
false;}let(x,y)=乓(q);let mut 乜=l.clone();if!乜.乃(L::乻{x,y}){return false;}if 伱(&乜,丂)<伲{return true;}倍&&佈(&乜,主,false,U[丂])>东}const 傧:O=false;const 傅:I=2;const
俟:I=1;const 俈:I=2;const 伿:F=0;const 佲:F=2;const 傌:O=false;const 傎:O=false;const 傍:O=false;const 傋:O=false;const 偰:F=4;const 俅:F=5;#[derive(Clone)]enum
丫{从(L),事(K<L>),}impl 丫{fn 丈(&self)->&[L]{match self{丫::从(丑)=>std::slice::from_ref(丑),丫::事(丈)=>丈,}}}fn 乓(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 倇(l:&J,七:&mut
Vec<丫>){七.clear();if l.両().is_some(){return;}let 丂=l.丛();let 亅=l.丼().互();let mut 丨=[G::T;4];for 丆 in 1..=亅{丨[F::from(丆)]=l.么(丂,丆);}for 三 in l.乶().iter(){let
Some((上,丆))=l.亹(三)else{continue;};let 乑=G::q(三).丏()&丨[F::from(丆)];for 丣 in 乑.iter(){let(x,y)=乓(丣);七.push(丫::从(L::乲{上,x,y}));}}let 下=l.下(丂);let 丅=l.丅(丂);let
亀=l.仅(丂);let 乬=match 傅{0=>亀,1=>{let 伕=l.map().valid.业(丅).丏()&丅;亀.业(丅)|伕}_=>亀.业(丅),};for 丆 in 1..=亅{let 不=10*C::from(丆);if 下<不{break;}let mut 丐=乬&丨[F::from(丆)];for
僢 in 1..丆{丐=丐.业(丨[F::from(僢)]);}for q in 丐.iter(){let(x,y)=乓(q);七.push(丫::从(L::丹{丆,x,y}));}}if 下>=争{let 並=l.丒(丂)&亀&l.仓();if l.丼().佃(){for q in(並&l.map().丷).iter(){let(x,y)=乓(q);七.push(丫::从(L::亨{x,y}));}}if
l.丼().仫(){let 伕=l.map().valid.业(l.丒(丂)).丏();let 丐=(並&伕).业(l.map().丷);let 丩=丩(l,1-丂);let 伲=C::from((丩&l.丅(丂)).count());let 东=佈(l,1-丂,false,U[丂]);let 丠=if
傧{let(交,住)=久(l,丂);交.业(住)}else{G::T};for q in 丐.iter().filter(|&q|{let 伉=G::q(q)|G::q(q).丏();!(伉&丠).is_empty()||僙(l,q,伲,丩,东)}){let(x,y)=乓(q);七.push(丫::从(L::乻{x,y}));}}}let
僘=match 俟{1=>争,2=>10,_=>C::MAX,};if 下>=僘{let 丩=丩(l,1-丂);let 亓=l.map().valid.业(l.丒(丂)|l.丒(1-丂));for 三 in(l.乶()&丩).iter(){let Some((上,丆))=l.亹(三)else{continue;};let
乑=G::q(三).丏()&丨[F::from(丆)]&亓;let(仗,付)=乓(三);let 侗=if 俟==1{if l.map().丷.test(三){continue;}L::乻{x:仗,y:付}}else{L::丹{丆:1,x:仗,y:付,}};for 丣 in 乑.iter(){let(x,y)=乓(丣);七.push(丫::事(vec![L::乲{上,x,y},侗]));}}}if
俈!=0&&下>=争&&l.丼().仫(){働(l,&丨,七);}if 伿!=0&&下>=20{儂(l,丨[1],下,七);}if 傋{僃(l,丨[1],下,七);}for 为 in 俼(l,俅,下,偰){if 为.丈.len()>1{七.push(丫::事(为.丈));}}if let Some(东)=來(l,U[1-丂],下){if
东.丈.len()>1{七.push(丫::事(东.丈));}}}fn 働(l:&J,丨:&[G;4],七:&mut Vec<丫>){let 丂=l.丛();let 主=1-丂;let 丩=丩(l,主);let 两=l.丒(丂);let 世=l.丒(主);let 伕=世.丏();let 丷=l.map().丷;for
三 in(l.乶()&丩&l.丅(丂)).iter(){if 丷.test(三){continue;}let Some((上,丆))=l.亹(三)else{continue;};let 傯=l.map().丗(三,U[主]);let 乑=G::q(三).丏()&丨[F::from(丆)];let(仗,付)=乓(三);for
丣 in 乑.iter(){let 佾=match 俈{1=>两.test(丣)&&l.map().丗(丣,U[主])>傯,2=>两.test(丣)&&伕.test(丣),_=>两.test(丣)||(世.test(丣)&&丣!=U[主]),};if!佾{continue;}let(x,y)=乓(丣);七.push(丫::事(vec![L::乲{上,x,y},L::乻{x:仗,y:付},]));}}}fn
儂(l:&J,侟:G,下:C,七:&mut Vec<丫>){let 丂=l.丛();let 严=l.map();let 亓=严.valid.业(l.丒(丂)|l.丒(1-丂));let 丠=亓&侟;let 乬:K<F> =(l.丅(丂).丏()&丠).iter().collect();if 乬.is_empty(){return;}let
侧=|与:F|{乬.iter().copied().min_by_key(|&q|(严.丗(q,与),q))};let mut 侇:K<(F,F)> =K::new();if 傌{for 丕 in 儃(l,&乬){if let Some(丽)=侧(丕){侇.push((丽,丕));}}}else{let
与=U[1-丂];let mut 侴=乬.clone();侴.sort_by_key(|&q|(严.丗(q,与),q));侴.truncate(佲);侇.extend(侴.into_iter().map(|丽|(丽,与)));}let 份=F::try_from(下/10).unwrap_or(0).min(伿);for(丽,与)in
侇{let mut 东=vec![丽];let mut 产=G::q(丽);while 东.len()<份{let 乵=东[东.len()-1];if 乵==与{break;}let 丄=(G::q(乵).丏()&丠).业(产).iter().min_by_key(|&q|(严.丗(q,与),q));let
Some(丄)=丄 else{break;};东.push(丄);产.set(丄);let 丈=东.iter().map(|&q|{let(x,y)=乓(q);L::丹{丆:1,x,y}}).collect();七.push(丫::事(丈));}if 傍{僟(l,&东,七);}}}fn 僟(l:&J,东:&[F],七:&mut
Vec<丫>){let 丂=l.丛();let 丽=东[0];let 乷=l.乶()&l.万(丂)&G::q(丽).丏();for 三 in 乷.iter(){let Some((上,丆))=l.亹(三)else{continue;};if!l.么(丂,丆).test(丽){continue;}let(x,y)=乓(丽);let
mut 丈=vec![L::乲{上,x,y}];for&q in&东[1..]{let(x,y)=乓(q);丈.push(L::丹{丆:1,x,y});七.push(丫::事(丈.clone()));}}}fn 儃(l:&J,乬:&[F])->K<F>{let 丂=l.丛();let 严=l.map();let(乄,中)=低(l);let
两:K<F> =l.万(丂).iter().collect();let 世:K<F> =l.万(1-丂).iter().collect();let 位=|丕:F|{乬.iter().map(|&q|严.丗(q,丕)).min().unwrap_or(I::MAX)};let 傽=|丕:F|{let 丩=位(丕);两.iter().any(|&亸|{let
d=严.丗(亸,丕);d<=丩&&世.iter().all(|&主|d<严.丗(主,丕))})};let 侢=|丕:F|{傎&&(l.丒(1-丂).test(丕)||世.iter().any(|&主|严.丗(主,丕)<位(丕)))};let 丠=|丐:G|->K<F>{let mut 乿:K<F> =丐.业(l.丒(丂)).iter().filter(|&丕|位(丕)!=I::MAX&&!傽(丕)&&!侢(丕)).collect();乿.sort_by_key(|&丕|(位(丕),丕));乿};let
mut 乿=丠(中);if 乿.is_empty(){乿=丠(乄);}乿.truncate(佲);乿}fn 傿(l:&J)->K<F>{let 丂=l.丛();let 严=l.map();let(乄,中)=低(l);let 丅=l.丅(丂);let 两=l.丒(丂);let 丩=丩(l,1-丂);let
世:K<F> =l.万(1-丂).iter().collect();let 倅=|q:F|{丅.iter().map(|a|严.丗(a,q)).min().unwrap_or(I::MAX)};let mut 丐:K<F> =(乄|中).iter().filter(|&q|{if 两.test(q){丅.test(q)&&丩.test(q)}else{!l.丒(1-丂).test(q)&&世.iter().any(|&e|严.丗(e,q)<倅(q))}}).collect();丐.sort_by_key(|&q|(I::from(!中.test(q)),倅(q),q));丐.truncate(
2*佲);丐}fn 僃(l:&J,侟:G,下:C,七:&mut Vec<丫>){let 丂=l.丛();let 严=l.map();let 亓=严.valid.业(l.丒(丂)|l.丒(1-丂));let 丠=亓&侟;let 丅=l.丅(丂);let 並=丅&l.仓();let 僡=if 伿==0{3}else{伿};for
q in 傿(l){if!l.丒(丂).test(q){let 丽=(丅.丏()&丠).iter().min_by_key(|&c|(严.丗(c,q),c));if let Some(丽)=丽{let mut 东=vec![丽];let mut 产=G::q(丽);while 东[东.len()-1]!=q&&东.len()<僡{let
乵=东[东.len()-1];let 丄=(G::q(乵).丏()&丠).业(产).iter().min_by_key(|&c|(严.丗(c,q),c));let Some(丄)=丄 else{break;};东.push(丄);产.set(丄);}let 不=10*C::try_from(东.len()).unwrap_or(C::MAX);if
东.len()>=2&&东[东.len()-1]==q&&不<=下{let 丈=东.iter().map(|&c|{let(x,y)=乓(c);L::丹{丆:1,x,y}}).collect();七.push(丫::事(丈));}}}if 下>=争&&l.丼().仫(){let 丷=(G::q(q)|G::q(q).丏())&並;for
乧 in 丷.业(严.丷).iter(){let(x,y)=乓(乧);七.push(丫::从(L::乻{x,y}));}}}}const 偽:I=2;const 傛:C=5;fn 乷(l:&J,丁:F)->G{侦(l,丁,偽)}fn 侦(l:&J,丁:F,三:I)->G{let mut 乷=G::T;if
三>3{return 乷;}for q in(l.万(丁)&l.丅(丁)).iter(){if l.亜(q).is_some_and(|丆|丆>=三){乷|=G::q(q);}}乷}fn 丩(l:&J,临:F)->G{let 乏=l.下(临)+l.丞(临);僻(l,临,乷(l,临),乏)}fn 亢(l:&J,丁:F)->G{丩(l,1-丁)&l.丅(丁)}fn
伱(l:&J,丁:F)->C{C::from(亢(l,丁).count())}const 俕:C=4;fn 僦(丐:C)->C{丐/俕}fn 丶(l:&J,丁:F,亢:G)->C{let 丅=l.丅(丁);let 丶=(l.丶()&丅).业(亢);let 丐=C::from(丅.count());let
侂=if l.乤()<=偖{丐/偕+偔}else{僦(丐)};侂*C::from(丶.count())}const 偖:Q=15;const 偔:C=6;const 偕:C=俕;const 傓:C=2;const 佳:I=1;const 傕:I=3;const 傔:I=6;fn 僞(仂:I,亀:I,仕:I,d:I)->I{match
仂{0=>4,1 if d<=亀=>4,1=>0,_ if d<=亀=>4,_ if d>=仕=>0,_=>(4*Q::from(仕-d)/Q::from(仕-亀))as I,}}fn 假(l:&J,丁:F)->C{let 丅=l.丅(丁);let 主=if 佳==0{G::T}else{l.丅(1-丁)};let[a,b,c]=丅.伵();let[e,f,g]=主.伵();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丁 as 丮).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丟=(hash as F)&(俨-1);储.with(|侄|{let mut 侄=侄.borrow_mut();let 仈=&mut 侄[丟];if 仈.0==丅&&仈.1==主&&仈.2==丁 as I+1{return 仈.3;}let 丬=if 佳==0{儮(丅,U[丁])}else{let d=丱(l,主);let
mut 乍=[0u8;M];for(w,&乑)in 乍.iter_mut().zip(d.iter()){*w=僞(佳,傕,傔,乑);}偈(丅,U[丁],&乍)};*仈=(丅,主,丁 as I+1,丬);丬})}const 俨:F=1<<12;thread_local!{static 储:std::cell::RefCell<K<(G,G,I,C)>>
=std::cell::RefCell::new(vec![(G::T,G::T,0,0);俨]);}fn 儮(丅:G,了:F)->C{偈(丅,了,&[4;M])}fn 偈(丅:G,了:F,乍:&[I;M])->C{struct 伊<'a>{乍:&'a[I;M],丅:G,乮:[I;M],乙:[I;M],size:[I;M],今:I,亳:C,份:C,}fn
代(d:&mut 伊<'_>,v:F)->I{d.今+=1;let 乮=d.今;d.乮[v]=乮;let mut 乙=乮;let mut 侰=1u8;let mut 为=0u8;for w in(G::q(v).丏()&d.丅).iter(){if d.乮[w]==0{let 倹=代(d,w);侰+=倹;乙=乙.min(d.乙[w]);if
d.乙[w]>=乮{为+=倹;}}else{乙=乙.min(d.乮[w]);}}d.乙[v]=乙;d.size[v]=侰;if 为>0&&d.乮[v]>1{let w=C::from(d.乍[v]);d.亳+=w;d.份=d.份.max(w*C::from(为));}侰}if!丅.test(了){return
0;}let mut d=伊{乍,丅,乮:[0;M],乙:[0;M],size:[0;M],今:0,亳:0,份:0,};代(&mut d,了);(d.亳+d.份)/4}fn 儅(l:&J,丁:F)->C{傓*(假(l,1-丁)-假(l,丁))}fn 俽(l:&J,丁:F)->C{if l.下(丁)+l.丞(丁)>=0{return
0;}(l.万(丁)&l.丅(丁)).iter().filter_map(|q|l.亜(q)).map(|丆|10*C::from(丆)).sum()}const 偯:C=15;const 傣:[C;2]=[3,1];const 傡:[C;2]=[4,20];fn 儘(l:&J,丁:F)->[C;2]{let
mut 丘=[0;2];for q in(l.万(丁)&l.丅(丁)).iter(){match l.亜(q){Some(2)=>丘[0]+=1,Some(3)=>丘[1]+=1,_=>{}}}丘}fn 倸(l:&J,丁:F)->C{let[倿,儢]=儘(l,丁);let 俰=|丆:F|(3-傣[丆])*傡[丆];-偯*倿+俰(0)*倿+俰(1)*儢}const
偨:C=0;const 偫:C=2;const 偪:C=100;fn 倱(亥:C)->C{if 亥<0{return 0;}偨+偫*亥.min(偪)/2}const 俔:I=2;const 俑:C=2;const 俐:C=4;const 偬:I=3;fn 亱(l:&J,丁:F)->C{if 俑==0{return
0;}let 乂=l.map().valid;let mut 万=G::T;for q in(l.万(丁)&l.丅(丁)).iter(){if l.亜(q).is_some_and(|丆|丆<=偬){万|=G::q(q);}}let mut 佤=乂.业(l.丒(丁));let mut 亿=G::T;let
mut 乑=0;for 丗 in 1..=俐{佤=(佤|佤.丏())&乂;let 什=(万&佤).业(亿);乑+=(丗-1)*C::from(什.count());亿|=什;}乑+=俐*C::from(万.业(亿).count());俑*乑}const 俧:C=8;fn 久(l:&J,丁:F)->(G,G){let
丅=l.丅(丁);let 交=丅&l.丒(1-丁).丏();let 乒=l.乒()&丅;(交,交&(乒|乒.丏()))}fn 偄(l:&J,丁:F)->C{if 俧==0{return 0;}俧*C::from(久(l,丁).1 .count())}const 佮:C=0;const 俍:I=1;const
俎:C=4;const 佯:C=0;const 俌:C=0;const 偟:Q=14;const 偞:C=2;const 俦:C=0;const 傦:Q=14;fn 令(l:&J,丁:F)->C{if 俦==0||l.乤()>傦{return 0;}let 两=丱(l,l.丅(丁));let 世=丱(l,l.丅(1-丁));let
mut 仝=0;for c in l.map().valid.iter(){match 两[c].cmp(&世[c]){std::cmp::Ordering::Less=>仝+=1,std::cmp::Ordering::Greater=>仝-=1,std::cmp::Ordering::Equal=>{}}}俦*仝}struct
俏{valid:G,乄:G,中:G,}thread_local!{static 偠:std::cell::RefCell<P<俏>> =const{std::cell::RefCell::new(None)};}fn 低(l:&J)->(G,G){偠.with(|q|{let mut 丿=q.borrow_mut();let
乂=l.map().valid;if 丿.as_ref().is_none_or(|z|z.valid!=乂){let 仡=仡(l);let mut 乄=G::T;for c in 乂.iter(){if 仡[c]<=偞{乄.set(c);}}*丿=Some(俏{valid:乂,乄,中:仛(l),});}丿.as_ref().map_or((G::T,G::T),|z|(z.乄,z.中))})}fn
住(l:&J,丁:F)->G{let 丅=l.丅(丁);let 乒=l.乒()&丅;(乒|乒.丏())&丅}fn 丕(l:&J,丁:F)->C{if(佮==0&&俎==0&&佯==0&&俌==0)||l.乤()>偟{return 0;}let(乄,中)=低(l);let mut 亚=0;if 佮!=0{let
两=丱(l,l.丅(丁));let 世=丱(l,l.丅(1-丁));let mut 仝=0;for c in 乄.iter(){let(a,b)=(两[c],世[c]);if a!=I::MAX&&a.saturating_add(俍)<=b{仝+=1;}else if b!=I::MAX&&b.saturating_add(俍)<=a{仝-=1;}}亚+=佮*仝;}let
丘=|乎:G|C::from(乎.count());let(丵,世)=(l.丅(丁),l.丅(1-丁));亚+=俎*(丘(丵&乄)-丘(世&乄));if 佯!=0{亚+=佯*(丘(住(l,丁)&乄)-丘(住(l,1-丁)&乄));}亚+=俌*(丘(丵&中)-丘(世&中));亚}const 俋:C=4;const
俊:C=0;const 偛:C=4;const 停:Q=12;fn 僑()->[G;2]{static 做:std::sync::OnceLock<[G;2]> =std::sync::OnceLock::new();*做.get_or_init(僒)}fn 僒()->[G;2]{let mut 七=[G::T;2];for
c in 0..M{let(x,y)=((c%Y)as C,(c/Y)as C);if x-y>=俋{七[0].set(c);}else if y-x>=俋{七[1].set(c);}}七}fn 偆(l:&J,丁:F)->C{let[a,b]=僑();let 丅=l.丅(丁);C::from((丅&a).count().min((丅&b).count())).min(偛)}fn
僐(l:&J,丁:F)->C{if 俊==0||l.乤()>停{return 0;}俊*(偆(l,丁)-偆(l,1-丁))}fn 僉(l:&J,丁:F)->C{let 乷=[侦(l,丁,俔),侦(l,1-丁,俔),];let(丰,仺)=僊(l,丁,乷);let 丰=丰+倱(仺[0])-倱(仺[1]);if
l.両().is_some(){return 丰;}let 两=亢(l,丁);let 世=亢(l,1-丁);let 伲=C::from(世.count())-C::from(两.count());丰-2*俽(l,丁)+2*俽(l,1-丁)+儅(l,丁)+倸(l,丁)-倸(l,1-丁)-亱(l,丁)+亱(l,1-丁)+傛*伲/4+丶(l,丁,两)-丶(l,1-丁,世)+偄(l,丁)-偄(l,1-丁)+丕(l,丁)+令(l,丁)+僐(l,丁)}const
偹:F=8;const 傈:F=3;const 偱:F=32;const 傉:丮=300;const 偓:人=0.75;const 健:P<Q> =Some(7);const 偣:O=false;const 偢:C=10;const 偤:Q=6;struct 亪{亷:P<亂>,仙:P<亂>,伢:丮,}#[derive(Default)]struct
佸{丈:K<L>,侌:F,亰:丮,丝:F,义:F,俷:F,亣:F,亁:O,倵:O,仆:F,}const 傥:O=true;#[derive(Clone)]struct 亄{丈:K<L>,侏:C,}struct 仦{丈:K<L>,end:J,first:P<丮>,丧:P<K<亄>>,}type 伋=HashMap<丮,C,BuildHasherDefault<DefaultHasher>>;struct
佶{丂:F,limit:亪,亰:丮,伟:伋,仢:P<K<丫>>,侶:O,乘:P<以>,}const 佴:C=0;const 係:O=true;const 偎:O=false;const 俁:P<I> =None;fn 傴(l:&J,丝:&[亄],七:&mut Vec<丫>){if!係&&俁.is_none(){return;}let
伭=丝.iter().map(|乸|乸.丈.as_slice());債(l,伭,(係,偎,俁),七,);}fn 債<'a>(l:&J,伭:impl Iterator<Item=&'a[L]>,(乒,丏,丆):(O,O,P<I>),七:&mut Vec<丫>,){let 丂=l.丛();let 丵=l.丅(丂);let
mut 伜=G::T;for 乸 in 伭{for 丑 in 乸{if let L::丹{x,y,..}|L::乲{x,y,..}=*丑{let q=F::from(y)*Y+F::from(x);if 丵.test(q){伜|=G::q(q);}}}}if 伜.is_empty(){return;}let
並=l.仓()&丵;let 下=l.下(丂);let 倦=|丑:L,七:&mut Vec<丫>|{if!七.iter().any(|a|a.丈()==[丑]){七.push(丫::从(丑));}};if 乒&&下>=争&&l.丼().仫(){let mut 丐=伜;if 丏{丐|=伜.丏();}for
q in(丐&並).业(l.map().丷).iter(){let(x,y)=乓(q);倦(L::乻{x,y},七);}}if let Some(丆)=丆.filter(|&丆|丆<=l.丼().互()){if 下>=10*C::from(丆){for q in(伜&並).iter(){let(x,y)=乓(q);倦(L::丹{丆,x,y},七);}}}}fn
侀(l:&mut J,丈:&[L])->O{丈.iter().all(|&丑|l.乃(丑))}fn 云(丈:&[L])->丮{let mut hash:丮=0xcbf29ce484222325;for 丑 in 丈{let(体,a,b,c)=match*丑{L::乲{上,x,y}=>(1,丮::from(上),x,y),L::丹{丆,x,y}=>(2,丮::from(丆),x,y),L::亨{x,y}=>(3,0,x,y),L::乻{x,y}=>(4,0,x,y),};let
伈=体|a<<8|丮::from(b)<<40|丮::from(c)<<48;hash=(hash^伈).wrapping_mul(0x0100000001b3);}hash}fn 俫(乯:&J,乸:&[L])->J{let mut 丄=乯.clone();丄.仱();for&丑 in 乸{丄.乃(丑);}丄.仱();丄}fn
何(l:&J)->P<K<L>>{let 丂=l.丛();let 与=U[1-丂];let 伇=|丳:&J|丳.両()==Some(乭::于(丂));for q in l.乶().iter(){if l.map().丗(q,与)!=1{continue;}let Some((上,_))=l.亹(q)else{continue;};let(x,y)=乓(与);let
丑=L::乲{上,x,y};let mut 丳=l.clone();if 丳.乃(丑)&&伇(&丳){return Some(vec![丑]);}}let 东=來(l,与,l.下(丂))?;let mut 丳=l.clone();(侀(&mut 丳,&东.丈)&&伇(&丳)).then_some(东.丈)}fn
倷(l:&J,丁:F,个:C)->O{let 不=侍(l,丁,l.万(丁),U[1-丁]);不!=亭&&l.下(丁)+个*l.丞(丁)>=不}fn 儗(l:&J)->O{let 丂=l.丛();if 佴==0||l.両().is_some(){return false;}let mut 丄=l.clone();丄.仱();let
伱=丄.両().is_none()&&佈(&丄,1-丂,true,U[丂])<=丄.下(1-丂);!伱&&倷(l,丂,佴)}impl 佶{fn 俲(&self,乯:&J)->O{self.侶&&乯.両()!=Some(乭::于(self.丂))&&!倷(乯,self.丂,佴)}fn 亘(&self)->O{self.亰>=self.limit.伢||self.limit.亷.is_some_and(|t|亂::now()>=t)}fn
丰(&mut self,乯:&J,丝:&[亄],侃:C)->C{self.亰+=1;self.侬(乯,丝,侃)}fn 侬(&self,乯:&J,丝:&[亄],侃:C)->C{if 乯.両()==Some(乭::于(self.丂)){return 佁;}if 丝.is_empty(){return self.value(乯);}let
mut 佥=C::MAX;for 乸 in 丝{佥=佥.min(self.value(&俫(乯,&乸.丈)));if 佥<=侃{break;}}佥}fn value(&self,l:&J)->C{let 伔=僉(l,self.丂);match&self.乘{Some(乘)if l.両().is_none()=>伔+乘.儝(l,self.丂),_=>伔,}}fn
俪(&self,乛:&mut Vec<丫>){if let Some(乘)=&self.乘{for 佚 in&乘.仆{乛.push(丫::事(佚.clone()));}}}#[allow(clippy::too_many_arguments)]fn 僲(&mut self,乾:&J,仰:C,乛:&[丫],乞:&[(C,F)],乣:&mut[O],丝:&[亄],仲:&mut
伋,依:O,)->(P<(F,J,C)>,O){let mut 专=None;let mut 佅=仰;for&(佗,丟)in 乞{if 乣[丟]{continue;}if 依&&佗!=C::MAX&&佗<=佅-仰{return(专,true);}if self.亘(){return(专,false);}乣[丟]=true;let
丈=乛[丟].丈();let mut 丄=乾.clone();if!侀(&mut 丄,丈)||self.俲(&丄){仲.insert(云(丈),C::MIN/4);continue;}let 丰=self.丰(&丄,丝,佅);仲.insert(云(丈),丰-仰);if 丰>佅{佅=丰;专=Some((丟,丄,丰));}}(专,false)}fn
例(&mut self,了:&J,丝:&[亄],伓:&[丮],傸:O)->仦{let 一=self;let mut 乴=None;let mut 乾=了.clone();let mut 丈=K::new();let mut 仰=一.丰(&乾,丝,C::MIN);let mut 仲=伋::default();let
mut 乛=K::new();let mut 乞=K::new();while 乾.両().is_none()&&!一.亘(){if 丈.is_empty()&&傸{match&一.仢{Some(仢)=>乛.clone_from(仢),None=>{倇(&乾,&mut 乛);if let Some(乘)=一.乘.as_ref().filter(|g|g.倞){乛.push(丫::事(乘.丙.clone()));}一.俪(&mut
乛);一.仢=Some(乛.clone());}}}else{倇(&乾,&mut 乛);一.俪(&mut 乛);}傴(&乾,丝,&mut 乛);乞.clear();for(丟,亏)in 乛.iter().enumerate(){let k=云(亏.丈());let 佗=仲.get(&k).or(一.伟.get(&k)).copied();乞.push((佗.unwrap_or(C::MAX),丟));}乞.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乣=vec![false;乛.len()];if 丈.is_empty(){for(丟,亏)in 乛.iter().enumerate(){乣[丟]=伓.contains(&云(亏.丈()));}}let mut 专=None;for
依 in[true,false]{let(之,儍)=一.僲(&乾,仰,&乛,&乞,&mut 乣,丝,&mut 仲,依,);专=之;if 专.is_some()||!儍{break;}}let Some((丟,丄,丰))=专 else{break;};if 丈.is_empty(){乴=Some(云(乛[丟].丈()));}丈.extend_from_slice(乛[丟].丈());乾=丄;仰=丰;}for(k,乌)in
仲{let 倊=一.伟.entry(k).or_insert(乌);*倊=(*倊).max(乌);}仦{丈,end:乾,first:乴,丧:None,}}fn 伭(&self,乯:&J)->K<K<L>>{let mut 丄=乯.clone();丄.仱();let mut 七=K::new();if
丄.両().is_some(){return 七;}let 五=丄.丛();let 下=丄.下(五);if let Some(东)=來(&丄,U[self.丂],下){七.push(东.丈);}for 为 in 俼(&丄,俅,下,傈){七.push(为.丈);}let mut 倉=K::new();僗(&丄,&mut
倉);七.push(倉);let mut 世=佶{丂:五,limit:亪{伢:傉,..self.limit},亰:0,伟:伋::default(),仢:None,侶:false,乘:None,};七.push(世.例(&丄,&[],&[],true).丈);七.retain(|丈|!丈.is_empty());七}fn
俬(&self,乯:&J)->K<亄>{self.伭(乯).into_iter().map(|丈|{let 侏=self.value(&俫(乯,&丈));亄{丈,侏}}).collect()}fn 亣(&mut self,了:&J,亏:&丫,丝:&[亄])->P<仦>{let mut 乜=了.clone();if!侀(&mut
乜,亏.丈())||self.俲(&乜){return None;}self.亰+=1;let 买=self.例(&乜,丝,&[],false);let mut 丈=亏.丈().to_vec();丈.extend_from_slice(&买.丈);Some(仦{丈,end:买.end,first:Some(云(亏.丈())),丧:None,})}fn
倂(&self,丙:&仦,丝:&[亄])->C{let 丰=self.侬(&丙.end,丝,C::MIN);match&丙.丧{Some(丧)if!丧.is_empty()=>丰.min(self.侬(&丙.end,丧,C::MIN)),_=>丰,}}}fn 僽(之:&[亄],丝:&mut Vec<亄>)->O{let
乊:K<_> =丝.iter().map(|r|r.丈.clone()).collect();for 乸 in 之{if 丝.iter().any(|r|r.丈==乸.丈){continue;}丝.push(乸.clone());}丝.sort_by_key(|乸|乸.侏);丝.truncate(偹);丝.iter().map(|r|&r.丈).ne(&乊)}fn
儡(l:&J,乐:亪,亁:&mut 亠)->佸{倻(l,乐,亁,None)}fn 倻(l:&J,乐:亪,亁:&mut 亠,乘:P<以>,)->佸{let 丂=l.丛();let mut 举=佸::default();if l.両().is_some(){return 举;}if let Some(丈)=何(l){举.丈=丈;return
举;}let 乗=|儇:人|match(乐.亷,乐.仙){(Some(亷),Some(仙))=>Some(亷+(仙-亷).mul_f64(儇)),_=>None,};if 健.is_some_and(|且|l.乤()>=且){亁.end();}if 亁.僶(l){let 丈=亁.且(l,乗(0.4));let
傷=丈.iter().any(|丑|{matches!(丑,L::丹{..}|L::亨{..}|L::乻{..})});let 亱=偣&&!傷&&l.乤()>=偤&&l.下(丂)>=偢;if!亱{举.亁=true;举.丈=丈;return 举;}亁.end();}let mut 也=佶{丂,limit:亪{亷:乗(偓),..乐},亰:0,伟:伋::default(),仢:None,侶:儗(l),乘,};let
mut 丝=K::new();let mut 义:K<仦> =K::new();let mut 伓=K::new();let mut 佋=false;let mut 亣:K<丫> =K::new();let mut 丽=也.乘.as_ref().filter(|g|g.start).map(|g|丫::事(g.丙.clone()));while
举.侌<偱&&(义.is_empty()||!也.亘()){let 儒=match 丽.take(){Some(亏)=>也.亣(l,&亏,&丝),None=>None,};let 丙=if let Some(丙)=儒{丙}else if 佋{let Some(亏)=亣.pop()else{break;};match
也.亣(l,&亏,&丝){Some(丙)=>丙,None=>continue,}}else{也.例(l,&丝,&伓,true)};举.侌+=1;let 倭=义.iter().any(|p|p.丈==丙.丈);let 乴=丙.first;let 倴=也.亘();let mut 丙=丙;let mut 倗=false;if!倴&&!倭{let
之=也.俬(&丙.end);倗=僽(&之,&mut 丝);丙.丧=Some(之);}if!倭{义.push(丙);}if 倴{break;}if!倗&&!佋{match 乴{Some(乴)=>伓.push(乴),None=>{佋=true;亣=也.仢.clone().unwrap_or_default();亣.retain(|a|!伓.contains(&云(a.丈())));let
乌=|a:&丫|{也.伟.get(&云(a.丈())).copied().unwrap_or(C::MIN)};亣.sort_by_key(乌);}}}}举.亣=F::from(佋);举.亰=也.亰;举.丝=丝.len();举.义=义.len();let 仁=||乐.仙.is_some_and(|t|亂::now()>=t);let
mut 伃:K<P<C>> =vec![None;义.len()];for 丟 in(0..义.len()).rev(){if 丟+1<义.len()&&仁(){break;}伃[丟]=Some(也.倂(&义[丟],&丝));}let 企=|伃:&[P<C>]|{let mut 专:P<(C,F)>
=None;for(丟,丰)in 伃.iter().enumerate().rev(){if let Some(丰)=*丰{if 专.is_none_or(|(亙,_)|丰>亙){专=Some((丰,丟));}}}专};let mut 专=企(&伃);也.limit.亷=乐.仙;while let Some((_,丟))=专{if!傥||义[丟].丧.is_some()||仁(){break;}let
之=也.俬(&义[丟].end);义[丟].丧=Some(之);举.俷+=1;伃[丟]=Some(也.倂(&义[丟],&丝));专=企(&伃);}if let Some((_,丟))=专{举.倵=也.乘.as_ref().is_some_and(|g|!g.丙.is_empty()&&义[丟].丈.starts_with(&g.丙));举.仆=也.乘.as_ref().map_or(0,|g|{g.仆.iter().filter(|p|p.iter().all(|c|义[丟].丈.contains(c))).count()});let
mut 丌=l.clone();for&丑 in&义[丟].丈{if 丌.乃(丑){举.丈.push(丑);}}}举}fn 僵(乐:&dyn Fn(亂,Q)->亪,mut 亁:亠,侜:impl BufRead,余:impl Write,)->Result<(),乖>{let mut 亦=伎::default();个(侜,余,|仵,亇,且,丽|match
J::read(仵,亇,亞::佨,且){Ok(l)=>{let 仸=乘(&l,&mut 亦);let 举=match 仸{Some(乘)=>{倻(&l,乐(丽,且),&mut 亁,Some(乘))}None=>儡(&l,乐(丽,且),&mut 亁),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",且,举.侌,举.亰,举.丝,举.俷,举.亣,I::from(举.倵),举.仆,丽.elapsed());傭(&举.丈)}Err(仔)=>{eprintln!("plan: cannot read the input: {}"
,仔);"WAIT".to_string()}},)}const 傀:P<C> =None;const 傤:O=false;#[derive(Clone,Copy,PartialEq)]enum 伍{佪,偑,}#[derive(Clone,Copy)]struct 京{q:F,kind:伍,仚:O,value:C,}fn
书(l:&J,丁:F)->K<京>{僩(l,丁,傤)}fn 僩(l:&J,丁:F,儚:O)->K<京>{let(乄,中)=低(l);let(丵,世)=(l.丅(丁),l.丅(1-丁));let(会,伛)=(丱(l,丵),丱(l,世));let 伔=伥(l,&会,&会,&伛);let mut 七=K::new();for
q in(乄|中).iter(){let 侐=丱(l,G::q(q));let 仚=丵.test(q);let 丬=if 儚{伥(l,&会,&侐,&伛)-伥(l,&伛,&侐,&会)}else if 仚{let 偉=丱(l,僓(丵,丁,q));伔-伥(l,&偉,&偉,&伛)}else{伥(l,&会,&侐,&伛)-伔};七.push(京{q,kind:if
中.test(q){伍::佪}else{伍::偑},仚,value:丬,});}七.sort_by_key(|o|(-o.value,o.kind!=伍::佪,o.q));七}fn 伥(l:&J,a:&[I;M],g:&[I;M],b:&[I;M])->C{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 亿(书:&[京])->K<京>{书.iter().filter(|o|o.value>=傀.unwrap_or(1)).copied().collect()}fn 僓(丅:G,丁:F,q:F)->G{let
买=丅.业(G::q(q));let mut 乨=G::q(U[丁])&买;loop{let 丄=乨|(乨.丏()&买);if 丄==乨{return 乨;}乨=丄;}}const 信:C=10;const 偾:C=100;#[derive(Clone,Copy)]enum 伽{休(Q),丹(F),}#[derive(Clone,Copy)]struct
乺{个:Q,侚:伽,}#[derive(Clone,Copy)]struct 伏{q:F,ours:P<乺>,佘:P<乺>,theirs:P<乺>,亥:C,}fn 僺(l:&J,丁:F,书:&[京])->K<伏>{let mut 七=K::new();for 串 in 书.iter().filter(|o|!o.仚){let
q=串.q;let(僯,佘)=俭(l,丁,q);let 两=专(僯,佘);let(儠,償)=俭(l,1-丁,q);let 世=专(儠,償);let 个=|a:P<乺>|a.map_or(偾,|a|C::from(a.个));七.push(伏{q,ours:两,佘,theirs:世,亥:个(世)-个(两),});}七}fn
专(亸:P<乺>,伴:P<乺>)->P<乺>{match(亸,伴){(Some(u),Some(t))=>Some(if t.个<u.个{t}else{u}),(u,t)=>u.or(t),}}fn 俭(l:&J,临:F,q:F)->(P<乺>,P<乺>){let 严=l.map();let mut
亸:P<乺> =None;for 三 in l.万(临).iter(){let d=严.丗(三,q);let Some(上)=l.予(三)else{continue;};if d!=I::MAX&&亸.is_none_or(|u|Q::from(d)<u.个){亸=Some(乺{个:Q::from(d),侚:伽::休(上),});}}let
下=l.下(临)+if 临==l.丛(){0}else{l.丞(临)};let 佣=match(下,l.丞(临)){(下,_)if 下>=信=>Some(0),(_,丞)if 丞<=0=>None,(下,丞)=>Some((信-下+丞-1)/丞),};let mut 伴:P<乺> =None;if let
Some(佣)=佣{let 乬=l.丅(临).丏()&严.valid&l.么(临,1);for 三 in 乬.iter(){let d=严.丗(三,q);if d!=I::MAX&&伴.is_none_or(|t|1+佣 as Q+Q::from(d)<t.个){伴=Some(乺{个:1+佣 as Q+Q::from(d),侚:伽::丹(三),});}}}(亸,伴)}const
俢:C=10;#[derive(Default)]struct 伎{倜:BTreeMap<Q,F>,}#[derive(Default)]struct 亽{他:K<(Q,F)>,些:K<(F,F)>,back:K<Q>,}fn 们(l:&J,丁:F,书:&[京],仟:&[伏],丝:&mut 伎,)->亽{修(l,丁,书,仟,丝,l.万(丁))}fn
修(l:&J,丁:F,书:&[京],仟:&[伏],丝:&mut 伎,侑:G,)->亽{let 严=l.map();let 万:BTreeMap<Q,F> =(l.万(丁)&侑).iter().filter_map(|c|l.予(c).map(|上|(上,c))).collect();let mut 丠:K<(京,伏)>
=K::new();for o in 亿(书).into_iter().filter(|o|!o.仚){if let Some(r)=仟.iter().find(|r|r.q==o.q){if r.ours.is_some()&&r.亥>=0{丠.push((o,*r));}}}丠.sort_by_key(|(o,r)|{(-o.value,o.kind!=伍::佪,r.ours.map_or(Q::MAX,|a|a.个),o.q,)});let
乀=|亸:Q,丕:F|Q::from(严.丗(万[&亸],丕));let 倏=|亐:&BTreeMap<F,Q>,伬:&BTreeSet<Q>,上:Q|{伬.contains(&上)||亐.values().any(|&u|u==上)};let mut 亐:BTreeMap<F,Q> =BTreeMap::new();for(&上,&丕)in&丝.倜{if
万.contains_key(&上)&&丠.iter().any(|(o,_)|o.q==丕)&&!亐.contains_key(&丕){亐.insert(丕,上);}}let mut 伬:BTreeSet<Q> =BTreeSet::new();for(o,_)in&丠{let Some(&們)=亐.get(&o.q)else{continue;};let
侧=万.keys().copied().filter(|&上|!倏(&亐,&伬,上)).map(|上|(乀(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();if let Some((d,上))=侧{if d<乀(們,o.q){伬.insert(們);亐.insert(o.q,上);}}}let
mut 些=K::new();for(o,r)in&丠{if 亐.contains_key(&o.q){continue;}let 並=万.keys().copied().filter(|&上|!倏(&亐,&伬,上)).map(|上|(乀(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();let
偁=並.map(|(d,_)|C::from(d));let 佡=r.佘.and_then(|t|match t.侚{伽::丹(q)if l.下(丁)-俢*些.len()as C>=俢&&!些.iter().any(|&(c,_)|c==q)=>{Some((q,C::from(t.个)))}_=>None,});let
倾=佡.map(|(_,t)|t);let 偂=match(偁,倾){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 傼=if 偂{倾}else{偁};let Some(个)=傼 else{continue;};if r.theirs.map_or(100,|a|C::from(a.个))<个{continue;}match(偂,佡,並){(true,Some((q,_)),_)=>些.push((q,o.q)),(false,_,Some((_,上)))=>{亐.insert(o.q,上);}_=>{}}}let
mut 七=亽{些,..亽::default()};for(o,_)in&丠{if let Some(&上)=亐.get(&o.q){七.他.push((上,o.q));}}七.back=l.万(丁).iter().filter_map(|c|l.予(c)).filter(|上|!七.他.iter().any(|&(u,_)|u==*上)).collect();七.back.sort_unstable();丝.倜=七.他.iter().copied().collect();七}const
俣:C=10;fn 俵(l:&J,丁:F,上:Q)->P<F>{l.万(丁).iter().find(|&c|l.予(c)==Some(上))}fn 倳(l:&J,丁:F,三:F,丕:F)->K<F>{let 严=l.map();let 什=严.丗(三,丕);if 什==I::MAX||什==0{return
K::new();}let mut 七:K<F> =(G::q(三).丏()&严.valid).iter().filter(|&n|严.丗(n,丕)<什).collect();七.sort_by_key(|&n|(l.丒(丁).test(n),n));七}fn 亡(丌:&mut J,七:&mut Vec<L>,丑:L)->O{let
借=丌.乃(丑);if 借{七.push(丑);}借}#[derive(Clone,Copy)]struct 仨{侮:O,guard:I,倁:O,倬:O,}fn 仆(l:&J,亦:&亽,乱:仨)->K<K<L>>{let mut 七=K::new();for&(上,丕)in&亦.他{let 僭=亽{他:vec![(上,丕)],..亽::default()};let
佚=倶(l,&僭,仨{侮:false,..乱},);if!佚.is_empty(){七.push(佚);}}七}fn 丢(丌:&J,上:Q,三:F,丣:F,亢:O,乱:仨,)->P<K<L>>{let 丁=丌.丛();let mut 丳=丌.clone();let mut 丈=K::new();let(x,y)=乓(丣);if!亡(&mut
丳,&mut 丈,L::乲{上,x,y}){return None;}if 亢&&乱.guard!=0{let(仗,付)=乓(三);let 侗=if 乱.guard==1{L::乻{x:仗,y:付}}else{L::丹{丆:1,x:仗,y:付,}};if!亡(&mut 丳,&mut 丈,侗){return
None;}}if 乱.倬&&伱(&丳,丁)>伱(丌,丁){return None;}Some(丈)}fn 倶(l:&J,亦:&亽,乱:仨)->K<L>{let 丁=l.丛();let 严=l.map();let mut 丌=l.clone();let mut 七=K::new();let 丩=丩(l,1-丁);let
mut 仹:K<(Q,F)> =K::new();for&(上,丕)in&亦.他{let Some(三)=俵(&丌,丁,上)else{continue;};if!丌.乶().test(三){continue;}let 亢=丩.test(三);for 丣 in 倳(&丌,丁,三,丕){if let Some(丈)=丢(&丌,上,三,丣,亢,乱){for
丑 in&丈{丌.乃(*丑);}if 亢&&乱.guard==0{仹.push((上,三));}七.extend(丈);break;}}}for&(上,丕)in&亦.他{if 丌.下(丁)<俣{break;}let Some(乗)=俵(&丌,丁,上)else{continue;};let 傱=仹.iter().find(|&&(u,_)|u==上).map(|&(_,c)|c);let
丐=match 傱{Some(q)if 乱.倁=>vec![q],_=>倳(&丌,丁,乗,丕),};for 丣 in 丐{let(x,y)=乓(丣);if 亡(&mut 丌,&mut 七,L::丹{丆:1,x,y}){break;}}}if 乱.侮{for&(_,丕)in&亦.些{if 丌.下(丁)<俣{break;}let
乬=丌.丅(丁).丏()&严.valid;let mut 丐:K<F> =乬.iter().filter(|&c|严.丗(c,丕)!=I::MAX).collect();丐.sort_by_key(|&c|(严.丗(c,丕),c));for 丣 in 丐{let(x,y)=乓(丣);if 亡(&mut
丌,&mut 七,L::丹{丆:1,x,y}){break;}}}}七}const 俤:I=0;const 傞:I=1;const 俗:C=0;const 傁:O=false;const 偡:I=0;const 偘:O=false;const 傐:O=false;const 偍:O=false;const
偝:Q=7;const 傠:Q=14;const 傘:O=俤!=0||俗!=0;struct 以{丙:K<L>,倞:O,start:O,仆:K<K<L>>,书:K<(F,C,C)>,伂:C,}impl 以{fn 儝(&self,l:&J,丁:F)->C{if self.伂==0{return 0;}let(丵,世)=(l.丅(丁),l.丅(1-丁));let
mut 亚=0;for&(q,丬,亓)in&self.书{亚+=if 丵.test(q){丬}else if 世.test(q){-丬}else{亓};}self.伂*亚/4}}fn 乘(l:&J,丝:&mut 伎)->P<以>{let 伦=l.乤();if!傘||!(偝..=傠).contains(&伦)||l.両().is_some(){return
None;}僚(l,丝,傒)}#[derive(Clone,Copy)]struct 佀{仂:I,乱:仨,侁:O,伂:C,侫:O,}const 傒:佀=佀{仂:俤,乱:仨{侮:傞==2,guard:偡,倁:偘,倬:傐,},侁:偍,伂:俗,侫:傁,};fn 僚(l:&J,丝:&mut 伎,乁:佀)->P<以>{let
佀{仂,乱,侁,伂,侫,}=乁;let 丁=l.丛();let 书=书(l,丁);let 仟=僺(l,丁,&书);let 亦=if 侁{let 侑=l.万(丁).业(丩(l,1-丁));修(l,丁,&书,&仟,丝,侑)}else{们(l,丁,&书,&仟,丝)};let 丙=if 仂!=0{倶(l,&亦,乱)}else{K::new()};let
亿=亿(&书);let 儞=亿.iter().map(|o|{let 亥=仟.iter().find(|r|r.q==o.q).map_or(0,|r|r.亥.clamp(-1,1));let 亓=if 侫{o.value*亥}else{0};(o.q,o.value,亓)}).collect();let
僱=if 仂&4!=0{仆(l,&亦,乱)}else{K::new()};Some(以{仆:僱,倞:!丙.is_empty()&&仂&1!=0,start:!丙.is_empty()&&仂&2!=0,丙,书:儞,伂,})}const 偏:人=45.0;const 偙:人=800.0;const 傖:人=0.6;fn
乐(丽:亂,且:Q)->亪{if let Some(伢)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return 亪{亷:None,仙:None,伢,};}let 佟=儣().min(100.0);let
僨=if 且==1{偙}else{偏};let 乏=Duration::from_secs_f64(僨/1000.0*佟);亪{亷:Some(丽+乏.mul_f64(傖)),仙:Some(丽+乏),伢:丮::MAX,}}const 傃:F=9;const 偿:亻=亻{丐:3,丞:1,下:1,为:2,亳:2,伳:2,丩:6,令:1,中:0,们:false,乞:傄,};const
傇:Q=0;const 傄:乼=乼::佭{仭:8};const 俛:I=I::MAX;const 俙:C=2;const 俚:C=18;const 俘:C=0;const 傂:F=32;const 偋:O=false;const 偌:Settings1=Settings1{侲:3,侱:1,侉:4,仁:2,侒:2,佊:2,侸:true,reserve:true,侓:true,乢:亼::乳,fill:true,乕:Z{佞:俛,仯:俙,佑:俚,佂:俘,..Z::仏},};fn
亁()->亠{if 偋{return 亠::new(亍::伹(偌),K::new(),Z::仏);}僮(傂)}fn 僮(亝:F)->亠{let 亗=亍::伐{个:傃,亝,串:偿,作:傇,};let 丯=傑.iter().map(|&乕|Z{佞:俛,仯:俙,佑:俚,佂:俘,..乕}).collect();亠::new(亗,丯,Z::仏)}fn
main(){let 儓=io::stdin();let 儔=io::stdout();if let Err(仔)=僵(&乐,亁(),儓.lock(),儔.lock()){eprintln!("plan: {}",仔);std::process::exit(1);}}const 傜:&str="CG_TIME_SCALE";fn
儣()->人{僰(std::env::var(傜).ok().as_deref())}fn 僰(丬:P<&str>)->人{丬.and_then(|亇|亇.trim().parse::<人>().ok()).filter(|佟|佟.is_finite()&&*佟>0.0).unwrap_or(1.0)}const
_:()=assert!(偗==16);const 偗:F=16;const 偒:人=0.4;const 偺:I=4;#[derive(Clone,Copy)]struct 仐{中:G,俶:G,伣:G,}impl 仐{fn of(l:&J)->仐{let(a,众)=倢(l,U[0]);let(b,佉)=倢(l,U[1]);let
亚=a[U[1]];let 僠=众[U[1]];let mut 丿=仐{中:G::T,俶:G::T,伣:G::T,};for q in l.map().valid.iter(){if 众[q]==I::MAX||佉[q]==I::MAX{continue;}if 众[q].abs_diff(佉[q])<=1{丿.俶.set(q);}if
众[q].abs_diff(佉[q])<=偺{丿.伣.set(q);}let 僪=伸::from(众[q])+伸::from(佉[q])==伸::from(僠);if 亚>0.0&&僪&&q!=U[0]&&q!=U[1]&&a[q]*b[q]/亚>=偒{丿.中.set(q);}}丿}fn 仛(&self)->G{self.中&self.伣}}fn
儖(l:&J,儐:G)->[I;M]{let 乂=l.map().valid;let mut 七=[I::MAX;M];let mut 乨=儐&乂;let mut 仸=乨;let mut 丢=0u8;while!仸.is_empty(){for q in 仸.iter(){七[q]=丢;}let 丄=(仸.丏()&乂).业(乨);乨|=丄;仸=丄;丢=丢.saturating_add(1);}七}fn
倢(l:&J,三:F)->([人;M],[I;M]){let 乀=儖(l,G::q(三));let mut 乞:K<F> =l.map().valid.iter().filter(|&c|乀[c]!=I::MAX).collect();乞.sort_by_key(|&c|乀[c]);let mut 丘=[0.0;M];丘[三]=1.0;for
q in 乞{if q==三{continue;}let 九=G::q(q).丏()&l.map().valid;丘[q]=九.iter().filter(|&n|乀[n]!=I::MAX&&乀[n]+1==乀[q]).map(|n|丘[n]).sum();}(丘,乀)}const Y:F=12;const
M:F=Y*Y;const 亃:F=4*Y;const 亟:丮=(1<<亃)-1;const 伺:丮=1|1<<12|1<<24|1<<36;const 佩:丮=伺<<11;const 俞:丮=(1<<Y)-1;#[derive(Clone,Copy,PartialEq)]struct G([丮;3]);impl
G{const T:G=G([0;3]);const 偊:G=G([亟;3]);#[inline]fn q(q:F)->G{let mut 乎=G::T;乎.set(q);乎}#[inline]fn test(self,q:F)->O{(self.0[q/亃]>>(q%亃))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/亃]|=1<<(q%亃);}#[inline]fn clear(&mut self,q:F){self.0[q/亃]&=!(1<<(q%亃));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伵(self)->[丮;3]{self.0}#[inline]fn count(self)->Q{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 业(self,乇:G)->G{G([self.0[0]&!乇.0[0],self.0[1]&!乇.0[1],self.0[2]&!乇.0[2],])}#[inline]fn
iter(self)->佧{佧{価:self.0,乗:0,}}#[inline]fn 儌(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&俞)<<(亃-Y)),(b>>Y)|((c&俞)<<(亃-Y)),c>>Y,])}#[inline]fn 儉(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亟,((b<<Y)&亟)|(a>>(亃-Y)),((c<<Y)&亟)|(b>>(亃-Y)),])}#[inline]fn
儊(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佩&亟,(b>>1)&!佩&亟,(c>>1)&!佩&亟,])}#[inline]fn 儋(self)->G{let[a,b,c]=self.0;G([(a<<1)&!伺&亟,(b<<1)&!伺&亟,(c<<1)&!伺&亟,])}#[inline]fn
丏(self)->G{let 一=self;一.儌()|一.儉()|一.儊()|一.儋()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,乇:G)->G{G([self.0[0]&乇.0[0],self.0[1]&乇.0[1],self.0[2]&乇.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,乇:G)->G{G([self.0[0]|乇.0[0],self.0[1]|乇.0[1],self.0[2]|乇.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,乇:G){*self=*self|乇;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::偊.业(self)}}struct 佧{価:[丮;3],乗:F,}impl Iterator for 佧{type Item=F;#[inline]fn
next(&mut self)->P<F>{let 一=self;while 一.乗<3{let 佖=一.価[一.乗];if 佖!=0{一.価[一.乗]=佖&(佖-1);return Some(一.乗*亃+佖.trailing_zeros()as F);}一.乗+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乲{上:Q,x:I,y:I},丹{丆:I,x:I,y:I},亨{x:I,y:I},乻{x:I,y:I},}impl L{fn 儦(&self)->乖{match*self{L::乲{上,x,y}=>format!("MOVE {} {x} {y}",上),L::丹{丆,x,y}=>format!("TRAIN {} {x} {y}",丆),L::亨{x,y}=>format!("BUILD MINE {x} {y}"),L::乻{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
傭(丈:&[L])->乖{if 丈.is_empty(){return "WAIT".to_string();}let mut line=乖::with_capacity(16*丈.len());for(丟,丑)in 丈.iter().enumerate(){if 丟>0{line.push(';');}line.push_str(&丑.儦());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亞{俩,儵,傩,#[default]佨,}impl 亞{fn 互(self)->I{match self{亞::俩=>1,_=>3,}}fn 佃(self)->O{matches!(self,亞::傩|亞::佨)}fn 仫(self)->O{self==亞::佨}}const 二:I=I::MAX;struct
享{乡:G,valid:G,丷:G,丗:Box<[I]>,丏:[[[I;4];M];2],}thread_local!{static 偮:RefCell<P<Arc<享>>> =const{RefCell::new(None)};}impl 享{fn 儈(乡:G,丷:G)->Arc<享>{偮.with(|乵|{let
mut 乵=乵.borrow_mut();if let Some(严)=乵.as_ref(){if 严.乡==乡&&严.丷==丷{return Arc::clone(严);}}let 严=Arc::new(享::build(乡,丷));*乵=Some(Arc::clone(&严));严})}fn build(乡:G,丷:G)->享{let
乂=!乡;const 傗:[[(伷,伷);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[二;4];M];2];for(丁,乑)in 傗.iter().enumerate(){for q in
乂.iter(){let(x,y)=((q%Y)as 伷,(q/Y)as 伷);let mut 丘=0;for&(僆,僇)in 乑{let(倚,倛)=(x+僆,y+僇);if(0..Y as 伷).contains(&倚)&&(0..Y as 伷).contains(&倛){let 丄=倛 as F*Y+倚
as F;if 乂.test(丄){丏[丁][q][丘]=丄 as I;丘+=1;}}}}}let mut 丗=vec![二;M*M].into_boxed_slice();let mut 侪=[0u8;M];for 三 in 乂.iter(){let 乹=&mut 丗[三*M..(三+1)*M];乹[三]=0;侪[0]=三
as I;let(mut 侘,mut 侷)=(0,1);while 侘<侷{let q=侪[侘]as F;侘+=1;for&丄 in&丏[0][q]{if 丄!=二&&乹[丄 as F]==二{乹[丄 as F]=乹[q]+1;侪[侷]=丄;侷+=1;}}}}享{乡,valid:乂,丷,丗,丏,}}#[inline]fn
丗(&self,三:F,丣:F)->I{self.丗[三*M+丣]}}const 偸:Q=100;const 争:C=15;const 傢:[C;3]=[1,4,20];const U:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 乭{于(F),俇,}enum
亊{併,俖,俜,}#[derive(Clone)]struct J{map:Arc<享>,丼:亞,丧:[G;2],丅:[G;2],丵:G,乧:G,万:[G;2],丆:[G;3],仄:G,予:[Q;M],下:[C;2],丞:[C;2],丛:F,乤:Q,仾:Q,両:P<乭>,}fn 伞(丧:G,仴:F)->G{if!丧.test(仴){return
G::T;}let mut 伫=G::q(仴);loop{let 丄=(伫|伫.丏())&丧;if 丄==伫{return 伫;}伫=丄;}}impl J{fn 丛(&self)->F{self.丛}fn 乤(&self)->Q{self.乤}fn 両(&self)->P<乭>{self.両}fn 丼(&self)->亞{self.丼}fn
仼(&self,丁:F)->C{let 一=self;let 万=一.万[丁];一.下[丁]+10*C::from((万&一.丆[0]).count())+20*C::from((万&一.丆[1]).count())+30*C::from((万&一.丆[2]).count())}fn 下(&self,丁:F)->C{self.下[丁]}fn
丞(&self,丁:F)->C{self.丞[丁]}fn 侺(&self,丁:F)->F{self.万[丁].count()as F}fn map(&self)->&享{&self.map}fn 丒(&self,丁:F)->G{self.丧[丁]}fn 丅(&self,丁:F)->G{self.丅[丁]}fn
万(&self,丁:F)->G{self.万[丁]}fn 亹(&self,q:F)->P<(Q,I)>{let 一=self;(一.万[0]|一.万[1]).test(q).then(||(一.予[q],一.侠(q)))}fn 乶(&self)->G{self.万[self.丛]&self.仄}fn
丶(&self)->G{self.丵}fn 乒(&self)->G{self.乧}fn 亜(&self,q:F)->P<I>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.侠(q))}fn 予(&self,q:F)->P<Q>{(q<M&&(self.万[0]|self.万[1]).test(q)).then(||self.予[q])}fn
儑(&mut self,丁:F){let 一=self;一.丅=[伞(一.丧[0],U[0]),伞(一.丧[1],U[1])];let 僀=一.万[丁].业(一.丅[丁]);一.侭(僀);一.俸();一.下[丁]+=一.丞[丁];if 一.下[丁]<0{一.下[丁]=0;一.侭(一.万[丁]);}一.仄|=一.万[丁];}fn
俸(&mut self){let 一=self;for 丁 in 0..2{let 丅=一.丅[丁];let 万=丅&一.万[丁];let mut 丞=C::from(丅.count())+4*C::from((丅&一.丵).count());for(丆,儫)in 傢.iter().enumerate(){丞-=儫*C::from((万&一.丆[丆]).count());}一.丞[丁]=丞;}}fn
仱(&mut self){let 一=self;if 一.両.is_some(){return;}if 一.丛==1{if 一.乤>=偸{一.両=Some(match 一.仼(0).cmp(&一.仼(1)){std::cmp::Ordering::Greater=>乭::于(0),std::cmp::Ordering::Less=>乭::于(1),std::cmp::Ordering::Equal=>乭::俇,});return;}一.乤+=1;}一.丛=1-一.丛;一.儑(一.丛);}fn
佛(&mut self,丈:&[L]){let 一=self;if 一.両.is_some(){return;}let 侕=一.仾;for&丑 in 丈{if 一.僅(丑,侕){continue;}一.乃(丑);if 一.両.is_some(){return;}}一.仱();}fn 僅(&self,丑:L,侕:Q)->O{let
一=self;match 丑{L::乲{上,x,y}=>{一.仮(x,y).is_none()||上>=侕||一.倃(一.丛,上).is_none()}L::丹{丆,x,y}=>{一.仮(x,y).is_none()||丆==0||丆>一.丼.互()}L::亨{x,y}|L::乻{x,y}=>{一.仮(x,y).is_none()}}}#[inline]fn
仮(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<Y&&y<Y&&!self.map.乡.test(y*Y+x)).then_some(y*Y+x)}fn 倃(&self,丁:F,上:Q)->P<F>{self.万[丁].iter().find(|&q|self.予[q]==上)}#[inline]fn
侠(&self,q:F)->I{if self.丆[0].test(q){1}else if self.丆[1].test(q){2}else{3}}fn 傳(&self,五:F)->G{let 乒=self.乧&self.丧[五];let 丅=self.丅[五];乒|(丅&(丅&乒).丏())}fn
么(&self,丁:F,丆:I)->G{let 一=self;debug_assert!((1..=3).contains(&丆),"unit level {} is not 1 to 3",丆);let 五=1-丁;let 万=一.万[0]|一.万[1];let 僈=一.仓();let 儩=match
丆{1 if 一.丼==亞::俩=>一.丆[1]|一.丆[2],1=>万,2=>一.丆[1]|一.丆[2],_=>G::T,};let 么=僈|一.丧[五].业(儩);if 丆<3{么.业(一.傳(五))}else{么}}fn 仅(&self,丁:F)->G{self.丅[丁]|self.丅[丁].丏()}fn
仓(&self)->G{let 一=self;let 万=一.万[0]|一.万[1];let 僛=G::q(U[0])|G::q(U[1]);一.map.valid.业(万|一.丵|一.乧|僛)}fn 乃(&mut self,丑:L)->O{let 一=self;if 一.両.is_some(){return
false;}let 丁=一.丛;let 仑=match 丑{L::丹{丆,x,y}=>一.佡(丁,丆,x,y),L::乲{上,x,y}=>一.僧(丁,上,x,y),L::亨{x,y}=>一.build(丁,true,x,y),L::乻{x,y}=>一.build(丁,false,x,y),};let
Some(仑)=仑 else{return false;};match 仑{亊::併=>{}亊::俖=>一.丅[丁]=伞(一.丧[丁],U[丁]),亊::俜=>{一.丅=[伞(一.丧[0],U[0]),伞(一.丧[1],U[1])];}}一.俸();if!一.丧[0].test(U[0]){一.両=Some(乭::于(1));}else
if!一.丧[1].test(U[1]){一.両=Some(乭::于(0));}true}fn 侭(&mut self,丐:G){let 一=self;一.万[0]=一.万[0].业(丐);一.万[1]=一.万[1].业(丐);for 丆 in&mut 一.丆{*丆=丆.业(丐);}一.仄=一.仄.业(丐);}fn
债(&mut self,丁:F,q:F)->亊{let 一=self;一.侭(G::q(q));一.丵.clear(q);一.乧.clear(q);let 五=1-丁;if 一.丧[五].test(q){一.丧[五].clear(q);一.丧[丁].set(q);亊::俜}else if 一.丧[丁].test(q){亊::併}else{一.丧[丁].set(q);亊::俖}}fn
佡(&mut self,丁:F,丆:I,x:I,y:I)->P<亊>{let 一=self;if 丆==0||丆>一.丼.互(){return None;}let q=一.仮(x,y)?;let 不=10*C::from(丆);if 一.下[丁]<不||!一.仅(丁).test(q)||!一.么(丁,丆).test(q){return
None;}let 仑=一.债(丁,q);一.倣(丁,丆,q,一.仾);一.仾=一.仾.saturating_add(1);一.下[丁]-=不;Some(仑)}fn 倣(&mut self,丁:F,丆:I,q:F,上:Q){self.万[丁].set(q);self.丆[F::from(丆)-1].set(q);self.予[q]=上;}fn
僧(&mut self,丁:F,上:Q,x:I,y:I)->P<亊>{let 一=self;let 与=一.仮(x,y)?;let 三=一.倃(丁,上)?;if!一.仄.test(三){return None;}let 丆=一.侠(三);let 丨=一.么(丁,丆);let 九=一.map.丏[丁][三];if
九.contains(&(与 as I))&&!丨.test(与){return None;}let mut 专=一.map.丗(三,与);let mut 伙=None;for&丄 in 九.iter().take_while(|&&丄|丄!=二){let 丄=F::from(丄);let 丗=一.map.丗(丄,与);if
丗<专&&丨.test(丄){专=丗;伙=Some(丄);}}let 伙=伙?;let 仑=一.债(丁,伙);一.万[丁].clear(三);一.丆[F::from(丆)-1].clear(三);一.仄.clear(三);一.倣(丁,丆,伙,上);Some(仑)}fn build(&mut self,丁:F,丵:O,x:I,y:I)->P<亊>{let
一=self;let 佾=if 丵{一.丼.佃()}else{一.丼.仫()};let q=一.仮(x,y)?;let 僫=一.map.丷.test(q);if!佾||一.下[丁]<争||僫!=丵||!一.丧[丁].test(q)||!一.仅(丁).test(q)||!一.仓().test(q){return
None;}if 丵{一.丵.set(q);}else{一.乧.set(q);}一.下[丁]-=争;Some(亊::併)}}const 偳:C=1000000000;const 佰:C=0;const 偻:C=1;const 傝:C=2;impl J{fn read(仵:&str,且:&str,丼:亞,乤:Q,)->Result<J,乖>{let
mut 丷=K::new();let mut 乆=仵.lines();let 丘=侎(&mut 乆,"mine spot count")?;for _ in 0..丘{let[x,y]=亵(仿(&mut 乆,"a mine spot")?,"a mine spot")?;丷.push((佇(x)?,佇(y)?));}俿(乆,"the initial input")?;let
mut 乆=且.lines();let mut 侙=[0i64;4];for(丬,乚)in 侙.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[侨]=亵(仿(&mut 乆,乚)?,乚)?;if 侨.abs()>偳{return
Err(format!("{}: {} is out of range",乚,侨));}*丬=侨;}let mut 乡=G::T;let mut 丐=[G::T;2];let mut 丅=[G::T;2];for y in 0..Y{let 乹=仿(&mut 乆,"a map row")?.trim();if
乹.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,乹));}for(x,伯)in 乹.bytes().enumerate(){let q=y*Y+x;match 伯{b'#'=>乡.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乩=F::from(伯==b'X'||伯==b'x');丐[乩].set(q);if 伯.is_ascii_uppercase(){丅[乩].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伯 as char))}}}}let
傶=侎(&mut 乆,"building count")?;let mut 乽=K::new();for _ in 0..傶{let 亵=亵(仿(&mut 乆,"a building")?,"a building")?;乽.push(亵);}let 侺=侎(&mut 乆,"unit count")?;let
mut 万=K::new();for _ in 0..侺{let 亵=亵(仿(&mut 乆,"a unit")?,"a unit")?;万.push(亵);}俿(乆,"the turn's input")?;let 倠=乽.iter().find(|俳|俳[0]==0&&俳[1]==佰).ok_or("the bot's own HQ is not among the buildings")?;let
倌=侅(倠[2],倠[3],乡)?;let 丁=U.iter().position(|&仴|仴==倌).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",倌))?;let mut 倯=G::T;for(x,y)in 丷{if
乡.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}倯.set(y*Y+x);}let 佼=|乩:F|if 乩==0{丁}else{1-丁};let mut l=J{map:享::儈(乡,倯),丼,丧:[G::T;2],丅:[G::T;2],丵:G::T,乧:G::T,万:[G::T;2],丆:[G::T;3],仄:G::T,予:[0;M],下:[0;2],丞:[0;2],丛:丁,乤,仾:1,両:None,};for
乩 in 0..2{let 亖=佼(乩);l.丧[亖]=丐[乩];l.丅[亖]=丅[乩];l.下[亖]=侙[2*乩];l.丞[亖]=侙[2*乩+1];}let mut 倝=G::T;for[亖,体,x,y]in 乽{let 乩=倩(亖)?;let q=侅(x,y,乡)?;if!丐[乩].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 倝.test(q){return Err(format!("two buildings at ({x}, {y})"));}倝.set(q);match 体{佰
if q==U[佼(乩)]=>{}佰=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}偻=>l.丵.set(q),傝=>l.乧.set(q),_=>return Err(format!("unknown building type {}",体)),}}let
mut 乪=G::T;let mut 佔=0u32;for[亖,上,丆,x,y]in 万{let 乩=倩(亖)?;let q=侅(x,y,乡)?;if!丐[乩].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乪.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丆){return Err(format!("unit level {} is not 1, 2 or 3",丆));}let 上=Q::try_from(上).map_err(|_|format!("unit id {} is out of range",上))?;if
乪.iter().any(|乇|l.予[乇]==上){return Err(format!("two units have id {}",上));}乪.set(q);l.万[佼(乩)].set(q);l.丆[丆 as F-1].set(q);l.予[q]=上;佔=佔.max(上);}if!乪.is_empty(){l.仾=佔.checked_add(1).ok_or_else(||format!("unit id {} is out of range",佔))?;}l.仄=l.万[丁];Ok(l)}}fn
仿<'a>(乆:&mut Lines<'a>,乚:&str)->Result<&'a str,乖>{乆.next().ok_or_else(||format!("the text ends before {}",乚))}fn 俿(mut 乆:Lines<'_>,乚:&str)->Result<(),乖>{match
乆.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乚)),None=>Ok(()),}}fn 亵<const N:F>(line:&str,乚:&str)->Result<[C;N],乖>{let
mut 伵=line.split_whitespace();let mut 偃=[0i64;N];for 丬 in&mut 偃{let 伈=伵.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乚))?;*丬=伈.parse().map_err(|_|format!("{}: {:?} is not an integer",乚,伈))?;}if
伵.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乚));}Ok(偃)}fn 侎(乆:&mut Lines<'_>,乚:&str)->Result<F,乖>{let[丘]=亵(仿(乆,乚)?,乚)?;F::try_from(丘).ok().filter(|&丘|丘<=M).ok_or_else(||format!("{}: {} is out of range",乚,丘))}fn
佇(丬:C)->Result<F,乖>{F::try_from(丬).ok().filter(|&丬|丬<Y).ok_or_else(||format!("coordinate {} is off the map",丬))}fn 侅(x:C,y:C,乡:G)->Result<F,乖>{let q=佇(y)?*Y+佇(x)?;if
乡.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 倩(亖:C)->Result<F,乖>{match 亖{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亖)),}}#[derive(Clone,Copy,PartialEq)]enum
亼{乳,俀,}#[derive(Clone,Copy)]struct Settings1{侲:I,侱:I,侉:C,仁:C,侒:F,佊:I,侸:O,reserve:O,侓:O,乢:亼,fill:O,乕:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{侲:3,侱:1,侉:4,仁:2,侒:2,佊:2,侸:true,reserve:true,侓:true,乢:亼::乳,fill:true,乕:Z::仏,};}const
佻:C=1000;const 佺:C=10;fn 侽(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侩(l:&mut J,丑:L,七:&mut Vec<L>)->O{let 亡=l.乃(丑);if 亡{七.push(丑);}亡}fn 侵(l:&mut J,三:F,丣:F,七:&mut
Vec<L>)->O{let Some((上,_))=l.亹(三)else{return false;};let(x,y)=侽(丣);侩(l,L::乲{上,x,y},七)}fn 儜(l:&J,丿:&仐,乁:&Settings1,乐:F,)->K<F>{let 丂=l.丛();let 两=丱(l,l.丒(丂));let
世=丱(l,l.丒(1-丂));let 亓=l.map().valid.业(l.丒(0)|l.丒(1));let 倧=|c:F|两[c]!=I::MAX&&世[c]!=I::MAX;let mut 中:K<F> =丿.仛().业(l.丒(丂)).iter().filter(|&c|倧(c)).collect();中.sort_by_key(|&c|(两[c],c));let
mut 久:K<F> =亓.iter().filter(|&c|倧(c)&&两[c].abs_diff(世[c])<=乁.侱).collect();久.sort_by_key(|&c|(两[c].abs_diff(世[c]),两[c],c));let mut 七:K<F> =K::new();for
q in 中.into_iter().chain(久){if 七.len()>=乐{break;}let 仕=七.iter().all(|&t|l.map().丗(t,q)>=乁.侲);let 傻=丿.仛().test(q);if!七.contains(&q)&&(傻||仕){七.push(q);}}七}fn
僜(不:&[K<C>])->K<F>{let n=不.len();let 倐=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 偅=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亲=0usize;let mut 伤=vec![倐;n+1];let mut 侻=vec![false;n+1];loop{侻[亲]=true;let 倎=p[亲];let mut 优=倐;let mut 传=0usize;for j in 1..=n{if!侻[j]{let
乾=不[倎-1][j-1]-u[倎]-v[j];if 乾<伤[j]{伤[j]=乾;偅[j]=亲;}if 伤[j]<优{优=伤[j];传=j;}}}for j in 0..=n{if 侻[j]{u[p[j]]+=优;v[j]-=优;}else{伤[j]-=优;}}亲=传;if p[亲]==0{break;}}loop{let
传=偅[亲];p[亲]=p[传];亲=传;if 亲==0{break;}}}let mut 七=vec![0usize;n];for j in 1..=n{if p[j]>0{七[p[j]-1]=j-1;}}七}#[derive(Clone,Copy)]enum 仌{休(F),佹,}fn 乬(l:&J)->G{let
丂=l.丛();(l.仅(丂)&l.么(丂,1)).业(l.丒(丂))}fn 僳(l:&J,乕:&Z)->F{佢(l,乕,乔::乳).iter().filter(|c|matches!(c,L::丹{..})).count()}fn 丠(q:F,並:G)->Q{(G::q(q).丏()&並).count()}fn
倆(l:&J,乌:G,与:F,侞:I)->K<[C;M]>{let 乂=l.map().valid;let mut 专=[C::MIN;M];专[与]=0;let mut 乥=vec![专];for _ in 0..侞{let mut 丄=专;for q in 乂.iter(){for n in(G::q(q).丏()&乂).iter(){if
专[n]>C::MIN{let 丬=专[n]+C::from(乌.test(n));if 丬>丄[q]{丄[q]=丬;}}}}专=丄;乥.push(专);}乥}fn 倡(l:&J,乥:&[[C;M]],乌:G,三:F,k:F,)->G{let 乂=l.map().valid;let mut 七=G::T;let
mut q=三;let mut k=k.min(乥.len()-1);while k>0&&乥[k][q]>0{let Some(丄)=(G::q(q).丏()&乂).iter().filter(|&n|乥[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(乌.test(n))+乥[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乌.test(丄){七.set(丄);}q=丄;k-=1;}七}fn 且(l:&J,乁:&Settings1)->K<L>{let 丂=l.丛();let 丿=仐::of(l);let mut 丌=l.clone();let mut 七=K::new();let 世=丱(l,l.丒(1-丂));let
万:K<F> =l.乶().iter().collect();let 倈=(l.下(丂)/佺).max(0)as F;let 些=match 乁.乢{亼::乳=>僳(l,&乁.乕).min(倈),亼::俀=>倈,};let mut 伒:K<仌> =万.iter().map(|&c|仌::休(c)).collect();伒.extend(std::iter::repeat_n(仌::佹,些));let
乿=儜(l,&丿,乁,伒.len()+乁.侒);let 儨=乬(l);let 中=丿.仛();let n=伒.len().max(乿.len());let mut 不=vec![vec![0i64;n];n];for(乹,&与)in 乿.iter().enumerate(){for(伖,仈)in 不[乹].iter_mut().enumerate(){let
Some(&催)=伒.get(伖)else{*仈=佻;continue;};let 佄=match 催{仌::休(q)=>{let d=l.map().丗(q,与);if d==I::MAX{None}else{Some(C::from(d)-1)}}仌::佹=>儨.iter().map(|c|l.map().丗(c,与)).filter(|&d|d!=I::MAX).min().map(C::from),};*仈=match
佄{None=>佻,Some(佄)=>{let 主=C::from(世[与])-1;let 仁=乁.仁*(佄-主).max(0);let 侂=if 中.test(与){乁.侉}else{0};(佄+仁-侂).max(0)}};}}let 傮=僜(&不);let mut 侖:K<(F,F)> =K::new();let
mut 侹:K<(C,F)> =K::new();for(乹,&与)in 乿.iter().enumerate(){let 伖=傮[乹];if 不[乹][伖]>=佻{continue;}match 伒.get(伖){Some(仌::休(q))=>侖.push((*q,与)),Some(仌::佹)=>侹.push((不[乹][伖],与)),None=>{}}}let
mut 並=l.map().valid.业(l.丒(0)|l.丒(1));let mut 倘:K<F> =K::new();let mut 佝=G::T;let 井=|q:F|{let 丆=l.亜(q).unwrap_or(1);let 估=並|l.丒(1-丂);(G::q(q).丏()&估&l.么(丂,丆)).count()};侖.sort_by_key(|&(q,与)|{let
僎=if 乁.侓{井(q)}else{0};(僎,l.map().丗(q,与),q)});for&(三,与)in&侖{let Some(丆)=丌.亜(三)else{continue;};let 丨=丌.么(丂,丆);let d=丌.map().丗(三,与);let 乌=(並|丌.丒(1-丂)).业(佝);let
侞=d.saturating_add(乁.佊);let 乥=倆(&丌,乌,与,侞.saturating_sub(1));let 仪=乥[乥.len()-1];let 丄=(G::q(三).丏()&丨).iter().filter(|&c|仪[c]>C::MIN).max_by_key(|&c|{let
傾=d.saturating_sub(丌.map().丗(c,与));let 估=並.test(c)||丌.丒(1-丂).test(c);(估&&乁.侸,C::from(乌.test(c))+仪[c],傾,丠(c,並),std::cmp::Reverse(c),)}).filter(|&c|{並.test(c)||丌.丒(1-丂).test(c)||丌.map().丗(c,与)<d});if
let Some(丣)=丄{if 侵(&mut 丌,三,丣,&mut 七){並.clear(丣);倘.push(三);if 乁.reserve{佝|=倡(&丌,&乥,乌,丣,乥.len()-1);}}}}let 两=丱(l,l.丒(丂));let mut 临=G::T;for c in 並.iter(){if
两[c]<=世[c]{临.set(c);}}for&三 in&万{if 倘.contains(&三)||!丌.乶().test(三){continue;}let Some(丆)=丌.亜(三)else{continue;};let 丨=丌.么(丂,丆);let 估=(G::q(三).丏()&並&丨).iter().max_by_key(|&c|(丠(c,並),世[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丣)=估{if 侵(&mut 丌,三,丣,&mut 七){並.clear(丣);}continue;}let 丕=[临&並,並].into_iter().find_map(|丐|{丐.iter().min_by_key(|&c|(丌.map().丗(三,c),c))});if let
Some(丕)=丕{let d=丌.map().丗(三,丕);let 丄=(G::q(三).丏()&丨).iter().filter(|&c|丌.map().丗(c,丕)<d).min_by_key(|&c|c);if let Some(丣)=丄{侵(&mut 丌,三,丣,&mut 七);}}}侹.sort();let
mut 侣=0usize;for&(_,与)in&侹{if 丌.下(丂)<佺{break;}let 丐=乬(&丌);let Some(亀)=丐.iter().map(|c|丌.map().丗(c,与)).min()else{break;};let 乌=(並|丌.丒(1-丂)).业(佝);let 乥=倆(&丌,乌,与,亀.saturating_add(乁.佊));let
仪=乥[乥.len()-1];let q=丐.iter().filter(|&c|丌.map().丗(c,与)==亀).max_by_key(|&c|(仪[c],丠(c,並),std::cmp::Reverse(c)));let Some(q)=q else{break;};let(x,y)=侽(q);if
侩(&mut 丌,L::丹{丆:1,x,y},&mut 七){並.clear(q);侣+=1;if 乁.reserve{佝|=倡(&丌,&乥,乌,q,乥.len()-1);}}}if 乁.fill&&乁.乢==亼::乳{while 侣<些&&丌.下(丂)>=佺{let q=(乬(&丌)&並).iter().max_by_key(|&c|{(丠(c,並),std::cmp::Reverse(世[c]),std::cmp::Reverse(c),)});let
Some(q)=q else{break;};let(x,y)=侽(q);if!侩(&mut 丌,L::丹{丆:1,x,y},&mut 七){break;}並.clear(q);侣+=1;}}七}const 偧:C=15;const 偼:C=6;const 俥:C=10;const 傏:I=3;#[derive(Clone,Copy)]struct
Z{open:C,但:C,俻:I,供:C,佑:C,倔:C,佞:I,俺:C,俹:C,佂:C,仳:C,伅:C,习:C,件:C,介:C,仯:C,}impl Z{const 仏:Z=Z{open:3,但:2,俻:12,供:4,佑:偧,倔:偼,佞:傏,俺:2,俹:2,佂:0,仳:0,伅:0,习:0,件:0,介:0,仯:0,};const
fn 儳(self,仳:C,伅:C,习:C)->Z{Z{仳,伅,习,..self}}const fn 儲(self,件:C,介:C)->Z{Z{件,介,..self}}}fn 伶(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 丱(l:&J,三:G)->[I;M]{let mut
乀=[I::MAX;M];let 乂=l.map().valid;let mut 久=三&乂;let mut 乪=久;let mut d=0u8;while!久.is_empty(){for q in 久.iter(){乀[q]=d;}let 丄=(久.丏()&乂).业(乪);乪|=丄;久=丄;d=d.saturating_add(1);}乀}fn
僕(l:&J)->I{let 乀=丱(l,l.丒(0));l.丒(1).iter().map(|q|乀[q]).min().unwrap_or(I::MAX)}struct 亾<'a>{乕:&'a Z,丂:F,theirs:[I;M],伆:[I;M],free:G,临:G,ours:G,丷:G,万:G,}impl
亾<'_>{fn new<'a>(l:&J,乕:&'a Z)->亾<'a>{let 丂=l.丛();let 丵=丱(l,l.丒(丂));let 世=丱(l,l.丒(1-丂));let 伆=丱(l,G::q(U[1-丂]));let 並=l.map().valid.业(l.丒(0)|l.丒(1));let
mut 临=G::T;for q in 並.iter(){if 丵[q]<=世[q]{临.set(q);}}亾{乕,丂,theirs:世,伆,free:並,临,ours:l.丒(丂),丷:l.map().丷,万:l.万(丂),}}fn value(&self,q:F)->C{let 一=self;let
p=一.乕;let 丠=C::from((G::q(q).丏()&一.free).count());let 仭=p.俻;let 但=C::from(仭)-C::from(一.theirs[q].min(仭));let 仴=if p.供>0{C::from(一.伆[q].min(30))/p.供}else{0};let
丧=C::from((G::q(q).丏()&一.ours).count());let line=if 丧<=1{p.伅}else{0};let 习=if 一.丷.test(q){p.习}else{0};let 儙=if 丠==0{p.件}else{0};let 介=p.介*(22-C::from(一.伆[q].min(22)));let
仯=p.仯*C::from((G::q(q).丏()&一.万).count());p.open*丠+p.但*但-仴+p.仳*丧-line+习-儙+介-仯}fn 专(&self,丐:G)->P<F>{丐.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
侊(&mut self,q:F){let 一=self;一.free.clear(q);一.临.clear(q);一.ours.set(q);一.万.set(q);}}fn 伩(l:&mut J,丑:L,七:&mut Vec<L>)->O{let 亡=l.乃(丑);if 亡{七.push(丑);}亡}fn
倲(l:&mut J,三:F,丣:F,七:&mut Vec<L>)->O{let Some((上,_))=l.亹(三)else{return false;};let(x,y)=伶(丣);伩(l,L::乲{上,x,y},七)}fn 優(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-俥}fn
儁(l:&J,乕:&Z)->K<L>{佢(l,乕,乔::乳)}#[derive(Clone,Copy,PartialEq)]enum 乔{乳,亩{丶:I,万:I},}fn 佢(l:&J,乕:&Z,乢:乔)->K<L>{let mut 丌=l.clone();let mut 且=亾::new(l,乕);let
mut 七=K::new();亴(&mut 丌,&mut 且,&mut 七);match 乢{乔::乳=>價(&mut 丌,&mut 且,&mut 七),乔::亩{丶,万}=>{僌(&mut 丌,&mut 且,丶,万,&mut 七)}}七}fn 亴(l:&mut J,且:&mut 亾<'_>,七:&mut
Vec<L>){let 丂=且.丂;let mut 亱:K<F> =K::new();loop{let 丨=l.么(丂,1);let mut 企:P<(Q,F)> =None;for 三 in l.乶().iter(){if 亱.contains(&三){continue;}let 井=(G::q(三).丏()&且.free&丨).count();if
井==0{亱.push(三);}else if 企.is_none_or(|(n,_)|井<n){企=Some((井,三));}}let Some((_,三))=企 else{break;};且.万.clear(三);match 且.专(G::q(三).丏()&且.free&丨){Some(丣)if
倲(l,三,丣,七)=>且.侊(丣),_=>{且.万.set(三);亱.push(三);}}}for 三 in 亱{if!l.乶().test(三){continue;}let 与=[且.临,且.free].into_iter().find_map(|丐|{丐.iter().min_by_key(|&q|(l.map().丗(三,q),q))});if
let Some(与)=与{倲(l,三,与,七);}}}fn 價(l:&mut J,且:&mut 亾<'_>,七:&mut Vec<L>){let 丂=且.丂;let p=且.乕;let h=(p.佑-C::from(l.乤())).max(p.倔);let 儤=(C::from(僕(l))-p.俺).max(0)/p.俹.max(1)+p.佂;loop{let
下=l.下(丂);let 万=l.侺(丂)as C;let 儀=C::from(且.临.count());let q=且.专(l.仅(丂)&且.free&l.么(丂,1));let 偀=優(h,(儀/(万+1)).min(儤));let 俴=q.is_some()&&下>=俥&&偀>0;let 倕=4*h-争;let
习=倖(l,且);let 傹=习.is_some()&&下>=争&&倕>0;if 傹&&(!俴||倕>=偀){let(x,y)=伶(习.unwrap_or_default());if 伩(l,L::亨{x,y},七){continue;}break;}let Some(q)=q.filter(|_|俴)else{break;};let(x,y)=伶(q);if!伩(l,L::丹{丆:1,x,y},七){break;}且.侊(q);}}fn
倖(l:&J,且:&亾<'_>)->P<F>{let 丂=且.丂;(l.map().丷&l.丅(丂)&l.仓()).iter().filter(|&习|且.theirs[习]>且.乕.佞).min_by_key(|&习|(l.map().丗(U[丂],习),习))}fn 僌(l:&mut J,且:&mut
亾<'_>,丶:I,万:I,七:&mut Vec<L>,){let 丂=且.丂;for _ in 0..丶{let Some(习)=倖(l,且)else{break;};let(x,y)=伶(习);if!伩(l,L::亨{x,y},七){break;}}for _ in 0..万{let Some(q)=且.专(l.仅(丂)&且.free&l.么(丂,1))else{break;};let(x,y)=伶(q);if!伩(l,L::丹{丆:1,x,y},七){break;}且.侊(q);}}fn
乨(l:&J)->O{if l.両().is_some(){return false;}let 丂=l.丛();let 下=l.下(丂);let 丩=仧::new(l,丂,true);l.丒(1-丂).iter().any(|q|丩.不[q]<=下)}#[derive(Clone,Copy)]struct
丸{乍:I,乢:乔,}const 侾:I=I::MAX;const 偐:[乔;8]=[乔::乳,乔::亩{丶:0,万:0},乔::亩{丶:0,万:1},乔::亩{丶:0,万:2},乔::亩{丶:0,万:侾,},乔::亩{丶:1,万:0},乔::亩{丶:1,万:1},乔::亩{丶:1,万:侾,},];struct
佫<'a>{丯:&'a[Z],仃:Z,rest:丸,亯:P<亂>,乁:Settings2,}impl 佫<'_>{fn 乕(&self,丢:丸)->&Z{&self.丯[F::from(丢.乍)%self.丯.len()]}fn 丈(&self,l:&J,丢:丸)->K<L>{佢(l,self.乕(丢),丢.乢)}}fn
傪(l:&J,丈:&[L],仃:&Z)->(J,O){let mut 丄=l.clone();丄.佛(丈);if 丄.両().is_some()||乨(&丄){return(丄,true);}let 乸=儁(&丄,仃);丄.佛(&乸);let 亶=丄.両().is_some()||乨(&丄)||丄.乤()>俄;(丄,亶)}fn
僸(l:&J,丙:&[丸],仒:&佫<'_>)->K<丮>{let 丂=l.丛();let mut 伡=vec![伮(l,丂)];let mut 乫=l.clone();for 且 in 0..俄 as F{let 丢=丙.get(且).copied().unwrap_or(仒.rest);let 丈=仒.丈(&乫,丢);let(丄,亶)=傪(&乫,&丈,&仒.仃);if
亶{break;}乫=丄;伡.push(伮(&乫,丂));}伡}fn 伮(l:&J,丂:F)->丮{let mut hash:丮=0xcbf29ce484222325;let mut 侤=|伈:丮|hash=(hash^伈).wrapping_mul(0x0100000001b3);for(儛,乎)in[l.丒(丂),l.万(丂),l.丶()].into_iter().enumerate(){for
q in 乎.iter(){侤(((儛 as 丮)<<8)|q as 丮);}侤(0xff);}侤(l.下(丂)as 丮);hash}enum 亍{伐{个:F,亝:F,串:亻,作:Q,},伹(Settings1),}struct 亠{亗:亍,丯:K<Z>,仃:Z,丙:K<丸>,佌:K<丮>,仞:F,义:Q,亶:O,}impl
亠{fn new(亗:亍,丯:K<Z>,仃:Z)->亠{亠{亗,丯,仃,丙:K::new(),佌:K::new(),仞:0,义:0,亶:false,}}fn 僶(&mut self,l:&J)->O{let 一=self;let 儎=matches!(一.亗,亍::伐{..}|亍::伹(_));let
僷=!儎&&一.义>0&&一.仞>=一.佌.len();if!一.亶&&(僷||乨(l)){一.亶=true;}!一.亶}fn end(&mut self){self.亶=true;}fn 且(&mut self,l:&J,亯:P<亂>)->K<L>{let 一=self;let 仒=佫{丯:&一.丯,仃:一.仃,rest:丸{乍:0,乢:乔::乳,},亯,乁:Settings2::DEFAULT,};if
let 亍::伹(乁)=一.亗{一.义+=1;一.仞+=1;return 且(l,&乁);}let 丂=l.丛();let 僬=一.佌.get(一.仞)==Some(&伮(l,丂));let 僋=matches!(一.亗,亍::伐{作,..}if 作>0&&l.乤()>=作);if!僬||僋{let
乴=一.义==0;一.丙=match 一.亗{亍::伐{个,亝,串,..}=>{let 串=&串;let 买=傊;let mut 乑=丙(l,&一.丯,买,个,亝,亯,串,);乑.resize(个,买);乑}亍::伹(_)=>K::new(),};一.佌=match 一.亗{亍::伐{..}=>儏(l,&一.丯,&一.丙),_=>僸(l,&一.丙,&仒),};一.仞=0;一.义+=1;}let
丢=一.丙.get(一.仞).copied().unwrap_or(仒.rest);一.仞+=1;仒.丈(l,丢)}}fn 儏(l:&J,丯:&[Z],丙:&[丸])->K<丮>{let 丂=l.丛();let mut 伡=K::with_capacity(丙.len());let mut 乫=l.clone();for&丢
in 丙{if 乫.両().is_some(){break;}伡.push(伮(&乫,丂));乫=佽(&乫,&丈(&乫,丯,丢));}伡}const 俄:Q=20;const 傟:C=3;#[derive(Clone,Copy)]struct Settings2{个:C,}impl Settings2{const
DEFAULT:Settings2=Settings2{个:傟};}fn 乑(l:&J,三:G)->[I;M]{let 乂=l.map().valid;let mut 乀=[I::MAX;M];let mut 久=三&乂;let mut 乪=久;let mut d=0u8;while!久.is_empty(){for
q in 久.iter(){乀[q]=d;}let 丄=(久.丏()&乂).业(乪);乪|=丄;久=丄;d=d.saturating_add(1);}乀}fn 儭(l:&J,丁:F)->C{let 两=乑(l,l.丅(丁));let 世=乑(l,l.丅(1-丁));l.map().valid.iter().map(|q|match
两[q].cmp(&世[q]){_ if 两[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 傑:[Z;27]=丯();const
傊:丸=丸{乍:16,乢:乔::乳,};const fn 丯()->[Z;27]{let 仳=[0,2,4];let 伅=[0,3,6];let 介=[0,1,2];let mut 七=[Z::仏;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
件=if g>0||t>0{4}else{0};七[i]=Z::仏.儳(仳[g],伅[t],0).儲(件,介[w]);i+=1;}七}#[derive(Default)]struct 佷{丐:C,丞:C,下:C,亳:C,为:C,伳:C,丩:C,令:C,}#[derive(Clone,Copy)]struct
亻{丐:C,丞:C,下:C,为:C,亳:C,伳:C,丩:C,令:C,中:C,们:O,乞:乼,}#[derive(Clone,Copy,PartialEq)]enum 乼{俒,俆(佬),佭{仭:C},仍(仍),促,}fn 儥(l:&J,丁:F)->[I;M]{let 两=丱(l,G::q(U[丁]));let
世=丱(l,G::q(U[1-丁]));let mut 交=G::T;let mut 临=G::T;for c in l.map().valid.iter(){if 两[c]==I::MAX||世[c]==I::MAX{continue;}if 两[c]==世[c]{交.set(c);}if 两[c]<=世[c]{临.set(c);}}let
d=丱(l,交);let mut 七=[I::MAX;M];for c in 临.iter(){七[c]=d[c];}七}fn 傲(l:&J,丁:F,交:&[I;M])->C{let d=丱(l,l.丅(丁));(0..M).filter(|&c|交[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&交[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 仍{傆,}impl
仍{fn 侼(self,a:C,b:C,d:C)->C{let 仠=(a-b).abs()/2;match self{仍::傆=>a.min(d/2),}}}fn 像(l:&J,丁:F,侔:仍)->[C;M]{let 两=丱(l,G::q(U[丁]));let 世=丱(l,G::q(U[1-丁]));let
d=C::from(两[U[1-丁]]);let mut 七=[0;M];for c in 0..M{if 两[c]!=I::MAX&&世[c]!=I::MAX{七[c]=侔.侼(C::from(两[c]),C::from(世[c]),d);}}七}#[derive(Clone,Copy,PartialEq)]enum
佬{備(C),}impl 佬{fn 侼(self,仠:C)->C{match self{佬::備(亙)=>(亙-仠).max(0).pow(2),}}}type 伾=[C;4];fn 億(l:&J,丁:F)->[C;M]{struct 伊{丅:G,乮:[伸;M],乙:[伸;M],size:[C;M],为:[C;M],今:伸,}fn
代(d:&mut 伊,v:F){d.今+=1;d.乮[v]=d.今;d.乙[v]=d.今;d.size[v]=1;for w in(G::q(v).丏()&d.丅).iter(){if d.乮[w]==0{代(d,w);d.size[v]+=d.size[w];d.乙[v]=d.乙[v].min(d.乙[w]);if
d.乙[w]>=d.乮[v]{d.为[v]+=d.size[w];}}else{d.乙[v]=d.乙[v].min(d.乮[w]);}}}let mut d=伊{丅:l.丅(丁),乮:[0;M],乙:[0;M],size:[0;M],为:[0;M],今:0,};let 了=U[丁];if d.丅.test(了){代(&mut
d,了);}d.为[了]=0;d.为}fn 儆(l:&J,丁:F)->佷{let 丅=l.丅(丁);let 为=億(l,丁);let 伆=丱(l,G::q(U[1-丁]));let mut 七=佷{丐:C::from(丅.count()),丞:l.丞(丁),下:l.下(丁),丩:C::MAX,令:儭(l,丁)/2,..佷::default()};for
q in 丅.iter(){if 为[q]>0{七.亳+=1;七.为=七.为.max(为[q]);}if q!=U[丁]&&(G::q(q).丏()&丅).count()<=1{七.伳+=1;}七.丩=七.丩.min(C::from(伆[q]));}七}fn 儬(l:&J,丁:F,串:&亻,伣:G)->C{let
仚=C::from((l.丒(丁)&伣).count());let s=儆(l,丁);let o=串;o.丐*s.丐+o.丞*s.丞+o.下*s.下-o.为*s.为-o.亳*s.亳-o.伳*s.伳-o.丩*s.丩.min(30)+o.令*s.令+o.中*仚}fn 仛(l:&J)->G{仐::of(l).仛()}fn
僥(l:&J)->G{let 两=丱(l,G::q(U[0]));let 世=丱(l,G::q(U[1]));let mut 乄=G::T;for c in l.map().valid.iter(){if 两[c]!=I::MAX&&世[c]!=I::MAX&&两[c].abs_diff(世[c])<=2{乄.set(c);}}乄}fn
仡(l:&J)->[C;M]{let a=丱(l,G::q(U[0]));let b=丱(l,G::q(U[1]));let mut 七=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{七[c]=C::from(a[c].abs_diff(b[c]))/2;}}七}#[derive(Clone,Copy)]struct
仩{中:G,乄:G,乍:[C;M],仡:[C;M],久:G,front:[I;M],}impl 仩{fn of(l:&J,串:&亻)->仩{let 倒=matches!(串.乞,乼::俒);let mut 乍=[0;M];if let 乼::俆(僁)=串.乞{for(w,仠)in 乍.iter_mut().zip(仡(l)){*w=if
仠==C::MAX{0}else{僁.侼(仠)};}}if let 乼::仍(侔)=串.乞{乍=像(l,l.丛(),侔);}let(僿,久)=match 串.乞{乼::佭{..}=>{let r=仡(l);let mut f=G::T;for(c,&仠)in r.iter().enumerate(){if
仠==0{f.set(c);}}(r,f)}_=>([C::MAX;M],G::T),};仩{乍,仡:僿,久,front:if 串.乞==乼::促{儥(l,l.丛())}else{[I::MAX;M]},中:if 串.中!=0||倒{仐::of(l).仛()}else{G::T},乄:if 倒{僥(l)}else{G::T},}}}fn
价(l:&J,丁:F,串:&亻,丿:仩)->伾{let 你=儬(l,丁,串,丿.中);match 串.乞{乼::俆(_)|乼::仍(_)=>{let 丅=l.丅(丁);let 儯:C=丅.iter().map(|c|丿.乍[c]).sum();[C::from(丅.count()),儯,你,0]}乼::佭{仭}=>{let
丅=l.丅(丁);let d=丱(l,丅);let 仕:C=丿.久.iter().map(|c|C::from(d[c]).min(仭)).sum();[C::from(丅.count()),-仕,你,0]}乼::促=>[C::from(l.丅(丁).count()),傲(l,丁,&丿.front),你,0,],乼::俒=>{let
丒=l.丒(丁);let 中=C::from((丒&丿.中).count());let 乄=C::from((丒&丿.乄).count());match 串.乞{_=>[C::from(l.丅(丁).count()),中,乄,你],}}}}fn 佽(l:&J,丈:&[L])->J{let mut 丄=l.clone();丄.佛(丈);if
丄.両().is_none(){丄.佛(&[]);}丄}const 便:I=I::MAX;fn 丈(l:&J,丯:&[Z],丢:丸)->K<L>{if 丢.乍==便{let 乁=Settings1{乕:丯.first().copied().unwrap_or(Z::仏),乢:if 丢.乢==乔::乳{亼::乳}else{亼::俀},..Settings1::DEFAULT};return
且(l,&乁);}佢(l,&丯[F::from(丢.乍)%丯.len()],丢.乢,)}fn 値(l:&J,丯:&[Z],丙:&[丸],买:丸,个:F,)->J{let mut 乫=l.clone();for 且 in 0..个{if 乫.両().is_some(){break;}let 丢=丙.get(且).copied().unwrap_or(买);乫=佽(&乫,&丈(&乫,丯,丢));}乫}fn
儕(l:&J,丯:&[Z],们:O)->K<(丸,K<L>)>{let mut 七:K<(丸,K<L>)> =K::new();if 们{for 乢 in[乔::乳,乔::亩{丶:0,万:I::MAX,},]{let 丢=丸{乍:便,乢,};let c=丈(l,丯,丢);if!七.iter().any(|(_,乇)|*乇==c){七.push((丢,c));}}}for
乍 in 0..丯.len().min(F::from(I::MAX)){for&乢 in&偐{let 丢=丸{乍:乍 as I,乢,};let c=丈(l,丯,丢);if!七.iter().any(|(_,乇)|*乇==c){七.push((丢,c));}}}七}fn 傰(l:&J,丯:&[Z],买:丸,个:F,亝:F,亯:P<亂>,串:&亻,)->K<丸>{let
丂=l.丛();let 伨=||亯.is_some_and(|t|亂::now()>=t);let 丿=仩::of(l,串);let 伪=|丙:&[丸]|价(&値(l,丯,丙,买,个),丂,串,丿);let mut 专:(伾,K<丸>)=(伪(&[]),K::new());let mut 久:K<(J,K<丸>)>
=vec![(l.clone(),K::new())];for 僄 in 0..个{let mut 亮:K<(伾,J,K<丸>)> =K::new();let mut 乪:HashMap<丮,F> =HashMap::new();for(候,丙)in&久{if 伨(){break;}for(丢,c)in
儕(候,丯,串.们){let 丄=佽(候,&c);let mut 侈=丙.clone();侈.push(丢);let v=价(&丄,丂,串,丿);let 价=伮(&丄,丂);match 乪.get(&价){Some(&乗)if 亮[乗].0>=v=>{}Some(&乗)=>亮[乗]=(v,丄,侈),None=>{乪.insert(价,亮.len());亮.push((v,丄,侈));}}}}if
亮.is_empty(){break;}亮.sort_by(|a,b|b.0 .cmp(&a.0));亮.truncate(亝);let 乵=僄+1==个||伨();for(i,(_,_,丙))in 亮.iter().take(if 乵{亝}else{1}).enumerate(){if i>0&&伨(){break;}let
v=伪(丙);if v>专.0{专=(v,丙.clone());}}if 乵{break;}久=亮.into_iter().map(|(_,s,p)|(s,p)).collect();}专.1}fn 丙(l:&J,丯:&[Z],买:丸,个:F,僤:F,亯:P<亂>,串:&亻,)->K<丸>{let 丂=l.丛();let
丿=仩::of(l,串);let 伪=|丙:&[丸]|价(&値(l,丯,丙,买,个),丂,串,丿);let mut 专:(伾,K<丸>)=(伪(&[]),K::new());let mut 亝=2;loop{let 之=傰(l,丯,买,个,亝,亯,串);let v=伪(&之);if v>专.0{专=(v,之);}let
伨=亯.is_some_and(|t|亂::now()>=t);if 伨||(亯.is_none()&&亝>=僤){break;}亝*=2;}专.1}const 偦:F=4+12;fn 个(侜:impl BufRead,mut 余:impl Write,mut 使:impl FnMut(&str,&str,Q,亂)->乖,)->Result<(),乖>{let
mut 乆=俓{倨:侜};let Some(乴)=乆.line()?else{return Ok(());};let mut 仵=format!("{}\n",乴);乆.take(丘(&乴,"mine spot count")?,&mut 仵)?;let mut 且=0;loop{let Some(乴)=乆.line()?else{return
Ok(());};let 丽=亂::now();且+=1;let mut 亇=format!("{}\n",乴);乆.take(偦-1,&mut 亇)?;for 乚 in["building count","unit count"]{let line=乆.倪()?;let n=丘(&line,乚)?;亇.push_str(&line);亇.push('\n');乆.take(n,&mut
亇)?;}let 使=使(&仵,&亇,且,丽);writeln!(余,"{}",使).and_then(|()|余.flush()).map_err(|仔|format!("turn {}: cannot write the answer: {}",且,仔))?;}}struct 俓<R>{倨:R,}impl<R:BufRead>俓<R>{fn
line(&mut self)->Result<P<乖>,乖>{let mut line=乖::new();match self.倨.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(仔)=>Err(format!("cannot read input: {}",仔)),}}fn
倪(&mut self)->Result<乖,乖>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,亇:&mut String)->Result<(),乖>{for
_ in 0..n{亇.push_str(&self.倪()?);亇.push('\n');}Ok(())}}fn 丘(line:&str,乚:&str)->Result<F,乖>{match line.trim().parse::<F>(){Ok(n)if n<=偲=>Ok(n),_=>Err(format!("bad {} {line:?}",乚)),}}const
偲:F=144;const 傚:C=3;const 偭:C=2;const 偷:F=16;fn 僴(l:&J,七:&mut Vec<L>){let 丁=l.丛();let mut 亗=保{s:l.clone(),丁,主:1-丁,与:U[1-丁],佐:U[丁],且:l.乤(),七,};亗.run();}struct
保<'a>{s:J,丁:F,主:F,与:F,佐:F,且:Q,七:&'a mut Vec<L>,}fn 佦(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侥(上:Q,q:F)->L{let(x,y)=佦(q);L::乲{上,x,y}}fn 倽(丆:I,q:F)->L{let(x,y)=佦(q);L::丹{丆,x,y}}impl
保<'_>{fn run(&mut self){let 一=self;if 一.何(){return;}一.亴();if 一.何(){return;}一.乽();一.些();}fn 乀(&self,三:F,丣:F)->C{C::from(self.s.map().丗(三,丣))}fn 乃(&mut self,丑:L)->O{let
乣=self.s.乃(丑);if 乣{self.七.push(丑);}乣}fn 伇(&self,丳:&J)->O{丳.両()==Some(乭::于(self.丁))}fn 何(&mut self)->O{let 一=self;if 一.s.両().is_some(){return false;}for
q in 一.s.乶().iter(){if 一.乀(q,一.与)!=1{continue;}let Some((上,_))=一.s.亹(q)else{continue;};let 丑=侥(上,一.与);let mut 丳=一.s.clone();if 丳.乃(丑)&&一.伇(&丳){return 一.乃(丑);}}let
Some(东)=一.傺()else{return false;};let mut 丳=一.s.clone();if!东.iter().all(|&丑|丳.乃(丑))||!一.伇(&丳){return false;}for 丑 in 东{一.乃(丑);}true}fn 傺(&self)->P<K<L>>{let
一=self;const 俉:C=C::MAX;let 亅=一.s.丼().互();let 丨:K<G> =(1..=亅).map(|丆|一.s.么(一.丁,丆)).collect();let 伝=|q:F|{丨.iter().position(|乎|乎.test(q)).map(|丟|(丟 as I+1,10*(丟
as C+1)))};let 丅=一.s.丅(一.丁);let mut 不=[俉;M];let mut 乊=[F::MAX;M];let mut 乣=[false;M];for q in 丅.iter(){不[q]=0;}loop{let mut 丄:P<F> =None;for q in 0..M{if!乣[q]&&不[q]<俉&&丄.is_none_or(|n|不[q]<不[n]){丄=Some(q);}}let
Some(q)=丄 else{break;};乣[q]=true;if q==一.与{break;}for&九 in&一.s.map().丏[0][q]{if 九==二{break;}let 九=F::from(九);if 乣[九]||丅.test(九){continue;}let Some((_,仇))=伝(九)else{continue;};if
不[q]+仇<不[九]{不[九]=不[q]+仇;乊[九]=q;}}}if 不[一.与]>一.s.下(一.丁){return None;}let mut 东=K::new();let mut q=一.与;while!丅.test(q){let(丆,_)=伝(q)?;东.push(倽(丆,q));q=乊[q];}东.reverse();Some(东)}fn
乌(&self,乜:&J)->C{let(丂,仉)=(self.丁,self.主);let 乊=&self.s;let 丘=|乎:G|C::from(乎.count());let 丧=丘(乜.丒(丂))-丘(乊.丒(丂));let 产=丘(乊.丒(仉))-丘(乜.丒(仉));let 侢=丘(乊.丅(仉))-丘(乜.丅(仉));let
佒=乊.仼(仉)-乜.仼(仉);let 丶=丘(乊.丶()&乊.丒(仉))-丘(乜.丶()&乜.丒(仉));12*丧+20*产+8*(侢-产).max(0)+5*佒+25*丶}fn 倰(&self,q:F)->C{let 一=self;let 习=一.s.map().丷.test(q);if 习&&一.s.丼().佃()&&!一.s.丒(一.丁).test(q){20}else{0}}fn
倀(&self,乐:C)->O{let 一=self;一.s.万(一.主).iter().any(|q|一.乀(q,一.佐)<=乐)}fn 亴(&mut self){let 一=self;let mut 万:K<(C,F)> =一.s.乶().iter().map(|q|(一.乀(q,一.与),q)).collect();万.sort_unstable();for(什,q)in
万{if 一.s.両().is_some(){return;}let Some((上,丆))=一.s.亹(q)else{continue;};let 丨=一.s.么(一.丁,丆);let 九=一.s.map().丏[一.丁][q];let mut 专:P<(C,F)> =None;for&丄 in 九.iter().take_while(|&&丄|丄!=二){let
丄=F::from(丄);if!丨.test(丄){continue;}let mut 丳=一.s.clone();if!丳.乃(侥(上,丄)){continue;}let 丬=if 一.伇(&丳){100000}else{一.乌(&丳)+10*(什-一.乀(丄,一.与))+一.倰(丄)};if 专.is_none_or(|(亙,_)|丬>亙){专=Some((丬,丄));}}if
let Some((丬,丄))=专{if 丬>0{一.乃(侥(上,丄));}}}}fn 乽(&mut self){let 一=self;if 一.s.両().is_some(){return;}if 一.倀(傚){一.儧();}else{一.丶();}}fn 儧(&mut self){let 一=self;if!一.s.丼().仫()||一.s.下(一.丁)<争{return;}let
九=一.s.map().丏[一.丁][一.佐];let 丐:K<F> =九.iter().take_while(|&&q|q!=二).map(|&q|F::from(q)).collect();let 丵=一.s.丒(一.丁);if 丐.iter().any(|&q|丵.test(q)&&一.s.乒().test(q)){return;}for
q in 丐{let(x,y)=佦(q);if 一.乃(L::乻{x,y}){return;}}}fn 丶(&mut self){let 一=self;if!一.s.丼().佃(){return;}while 一.s.下(一.丁)>=争{let 並=一.s.map().丷&一.s.丒(一.丁)&一.s.丅(一.丁)&一.s.仓();let
Some(q)=並.iter().min_by_key(|&q|(一.乀(q,一.佐),q))else{return;};let(x,y)=佦(q);if!一.乃(L::亨{x,y}){return;}}}fn 些(&mut self){let 一=self;let 倫=if 一.且<=3{0}else{5};for
_ in 0..偷{if 一.s.両().is_some(){return;}let 下=一.s.下(一.丁);if 下<10{return;}let 僝=一.倀(偭);let 亀=一.s.仅(一.丁);let mut 专:P<(C,L)> =None;let mut 侳:P<(C,L)> =None;let
mut 俱=G::T;for 丆 in 1..=一.s.丼().互(){let 丨=一.s.么(一.丁,丆);let 井=(亀&丨).业(俱);俱=丨;let 仇=10*C::from(丆);if 仇>下{continue;}for q in 井.iter(){let 丑=倽(丆,q);let mut
丳=一.s.clone();if!丳.乃(丑){continue;}let 仹=丳.下(一.丁);let 丞=丳.丞(一.丁);let 佒=一.s.仼(一.主)-丳.仼(一.主);let 傫=if 佒>0&&僝{仹+丞>=0}else{丞>=0&&(仹>=倫||佒>0)};if!傫{continue;}let
乌=一.乌(&丳);let 丬=乌+一.倰(q)-6*一.乀(q,一.与)-10*(C::from(丆)-1);if 乌>0{if 专.is_none_or(|(亙,_)|丬>亙){专=Some((丬,丑));}}else if 丞>=3&&仹>=倫+10&&丆==1{if 侳.is_none_or(|(亙,_)|丬>亙){侳=Some((丬,丑));}}}}let
Some((_,丑))=专.or(侳)else{return;};if!一.乃(丑){return;}}}}fn 僗(l:&J,七:&mut Vec<L>){七.clear();僴(l,七);}const 亭:C=C::MAX;const 偶:F=8;struct 仧{不:[C;M],乊:[I;M],}fn
佈(l:&J,丁:F,亴:O,与:F)->C{侍(l,丁,仜(l,丁,亴),与)}fn 侍(l:&J,丁:F,仜:G,与:F)->C{let mut 不=亭;倄(l,丁,仜,|丢,new|{if new.test(与){不=10*丢 as C;}不!=亭});不}fn 僻(l:&J,丁:F,仜:G,乏:C)->G{let
mut 乨=G::T;倄(l,丁,仜,|丢,new|{if 10*丢 as C>乏{return true;}乨|=new;false});乨.业(l.丅(丁))}fn 仜(l:&J,丁:F,亴:O)->G{if 亴&&丁==l.丛(){l.乶()}else{G::T}}fn 倄(l:&J,丁:F,仜:G,mut
代:impl FnMut(F,G)->O,){let 亅=l.丼().互();let mut 伝=[G::T;3];let mut 乪=G::T;for 丆 in 1..=亅{let 乎=l.么(丁,丆);伝[F::from(丆)-1]=乎.业(乪);乪|=乎;}let mut 丠=[l.丅(丁),G::T,G::T,G::T];for
q in(仜&l.万(丁)).iter(){if let Some(丆)=l.亜(q).filter(|&丆|丆<=亅){丠[0]|=G::q(q).丏()&l.么(丁,丆);}}let mut 乣=G::T;for 丢 in 0..{let new=丠[丢%4].业(乣);丠[丢%4]=G::T;if
代(丢,new){return;}乣|=new;let 丄=new.丏().业(乣);for(仪,丐)in 伝.iter().enumerate(){丠[(丢+仪+1)%4]|=丄&*丐;}if 丠.iter().all(|乎|乎.业(乣).is_empty()){break;}}}impl 仧{fn
new(l:&J,丁:F,亴:O)->仧{let 亅=F::from(l.丼().互());let mut 丨=[G::T;3];for(丟,乎)in 丨.iter_mut().enumerate().take(亅){*乎=l.么(丁,丟 as I+1);}let 丅=l.丅(丁);let 丏=&l.map().丏[0];let
mut 不=[亭;M];let mut 乊=[二;M];let mut 丠=[0u8;M];let mut 丘=0;let mut 倮=G::T;for q in 丅.iter(){不[q]=0;丠[丘]=q as I;丘+=1;}if 亴&&丁==l.丛(){for 三 in l.乶().iter(){let
Some(丆)=l.亜(三)else{continue;};let 乑=G::q(三).丏()&丨[F::from(丆)-1];for q in 乑.iter(){if 不[q]!=0{不[q]=0;丠[丘]=q as I;丘+=1;}}}}while 丘>0{let mut 专=0;for 丟 in
1..丘{let(a,b)=(F::from(丠[丟]),F::from(丠[专]));if(不[a],a)<(不[b],b){专=丟;}}let q=F::from(丠[专]);丘-=1;丠[专]=丠[丘];倮.set(q);for&丄 in&丏[q]{if 丄==二{break;}let 丄=F::from(丄);if
倮.test(丄)||丅.test(丄){continue;}let Some(丆)=丨[..亅].iter().position(|乎|乎.test(丄))else{continue;};let 值=不[q]+10*(丆 as C+1);if 值<不[丄]{if 不[丄]==亭{丠[丘]=丄 as
I;丘+=1;}不[丄]=值;乊[丄]=q as I;}}}仧{不,乊}}fn path(&self,与:F)->K<F>{let mut path=K::new();if 与>=M||self.不[与]==亭{return path;}let mut q=与;while self.乊[q]!=二{path.push(q);q=F::from(self.乊[q]);}path.reverse();path}}struct
伻{丈:K<L>,不:C,}fn 僣(l:&J,丁:F,q:F)->P<I>{(1..=l.丼().互()).find(|&丆|l.么(丁,丆).test(q))}fn 僾(丌:&mut J,丂:F,丕:F,乏:C,丈:&mut Vec<L>,亘:&mut i64,)->O{loop{if 丌.両()==Some(乭::于(丂)){return
true;}if 丌.丒(丂).test(丕)&&丌.丅(丂).test(丕){return true;}let 丩=仧::new(丌,丂,false);let 不=丩.不[丕];if 不==亭||*亘+不>乏{return false;}let Some(&q)=丩.path(丕).first()else{return
false;};let Some(丆)=僣(丌,丂,q)else{return false;};let 丑=L::丹{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丌.乃(丑){return false;}*亘+=10*C::from(丆);丈.push(丑);}}fn 俯(l:&J,丂:F,乿:&[F],乏:C)->P<伻>{let
mut 丌=l.clone();let mut 丈=K::new();let mut 亘=0;for&丕 in 乿{if!僾(&mut 丌,丂,丕,乏,&mut 丈,&mut 亘){return None;}}(亘<=乏).then_some(伻{丈,不:亘,})}fn 來(l:&J,与:F,乏:C)->P<伻>{if
乏<0||l.両().is_some()||与>=M||l.map().乡.test(与){return None;}let 丂=l.丛();if l.丒(丂).test(与)&&l.丅(丂).test(与){return Some(伻{丈:K::new(),不:0,});}let mut 专=俯(l,丂,&[与],乏);let
乴=仧::new(l,丂,false);let mut 乒:K<F> =(l.乒()&l.丒(1-丂)).iter().filter(|&乧|乧!=与&&乴.不[乧]!=亭).collect();乒.sort_by_key(|&乧|(l.map().丗(乧,与),乧));乒.truncate(偶);for
乧 in 乒{if let Some(东)=俯(l,丂,&[乧,与],乏){if 专.as_ref().is_none_or(|亙|东.不<亙.不){专=Some(东);}}}专}const 側:F=20000;struct 似{丈:K<L>,不:C,丐:C,万:C,乽:C,丞:C,丰:C,}struct
佱{丐:C,万:C,乽:C,}fn 個(l:&J,丁:F)->佱{let 丅=l.丅(丁);let 万=l.万(丁)&丅;let 丬=万.iter().filter_map(|q|l.亜(q)).map(|丆|10*C::from(丆)).sum();佱{丐:C::from(丅.count()),万:丬,乽:C::from(((l.丶()|l.乒())&丅).count()),}}struct
俠{丂:F,伉:G,乊:佱,丞:C,任:F,乏:C,伀:F,path:K<L>,之:K<似>,index:HashMap<伌,F,BuildHasherDefault<DefaultHasher>>,}type 伌=[丮;3];fn 儱(mut 云:伌,q:F)->伌{云[q/64]|=1<<(q%64);云}impl
俠{fn 俾(&mut self,丌:&J,乵:P<F>,产:伌,不:C,)->O{let 一=self;let 丂=一.丂;let 井=match 乵{None=>丌.仅(丂),Some(q)=>G::q(q).丏(),}&一.伉;let 井=井.业(丌.丒(丂));let mut 丨=[G::T;3];for
丆 in 1..=丌.丼().互(){丨[F::from(丆)-1]=丌.么(丂,丆);}for q in 井.iter(){let 亅=F::from(丌.丼().互());let Some(丟)=丨[..亅].iter().position(|乎|乎.test(q))else{continue;};let
丆=丟 as I+1;let 仇=10*C::from(丆);if 不+仇>一.乏{continue;}if 一.伀>=側{return false;}一.伀+=1;let mut 丄=丌.clone();let 丑=L::丹{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丄.乃(丑){continue;}一.path.push(丑);let
伄=儱(产,q);一.僼(&丄,伄,不+仇);let 僂=一.path.len()<一.任&&丄.両().is_none();let 僖=!僂||一.俾(&丄,Some(q),伄,不+仇);一.path.pop();if!僖{return false;}}true}fn 僼(&mut self,乜:&J,伄:伌,不:C){let
一=self;let 伧=1-一.丂;let 伦=個(乜,伧);let 丐=一.乊.丐-伦.丐;let 万=一.乊.万-伦.万;if 丐<=0&&万<=0{return;}let 乽=一.乊.乽-伦.乽;let 丞=乜.丞(一.丂)-一.丞;let 儴=乜.両()==Some(乭::于(一.丂));let
丰=if 儴{C::MAX/2}else{3*丐+万+10*乽+2*丞-不};if let Some(&乗)=一.index.get(&伄){if 不>=一.之[乗].不{return;}}let 为=似{丈:一.path.clone(),不,丐,万,乽,丞,丰,};match 一.index.get(&伄){Some(&乗)=>一.之[乗]=为,None=>{一.index.insert(伄,一.之.len());一.之.push(为);}}}}fn
俼(l:&J,任:F,乏:C,乐:F)->K<似>{也(l,任,乏,乐).0}fn 也(l:&J,任:F,乏:C,乐:F)->(K<似>,F){if l.両().is_some()||任==0||乐==0{return(K::new(),0);}let 丂=l.丛();let 伧=1-丂;let mut
侯=俠{丂,伉:l.丒(伧)|l.丒(伧).丏(),乊:個(l,伧),丞:l.丞(丂),任,乏,伀:0,path:K::new(),之:K::new(),index:HashMap::default(),};侯.俾(l,None,[0;3],0);let 伀=侯.伀;let mut 之=侯.之;之.sort_by(|a,b|b.丰.cmp(&a.丰).then(a.不.cmp(&b.不)));之.truncate(乐);(之,伀)}const
佁:C=1000000;const 偵:C=200;fn 临(l:&J,丁:F)->C{let 丅=l.丅(丁);let 万:C=(l.万(丁)&丅).iter().filter_map(|q|l.亜(q)).map(|丆|10*C::from(丆)).sum();l.下(丁)+3*l.丞(丁)+4*C::from(丅.count())+万+10*C::from((l.乒()&丅).count())}fn
倓(l:&J,丁:F,乷:G)->C{僔(l,丁,侍(l,丁,乷,U[1-丁]),)}fn 僔(l:&J,丁:F,不:C)->C{if 不==亭{return C::MIN;}l.下(丁)+l.丞(丁)-不}fn 倥(亥:C)->C{亥.clamp(-偵,0)}fn 僊(l:&J,丁:F,乷:[G;2],)->(C,[C;2]){let
乇=1-丁;match l.両(){Some(乭::于(儰))=>{return(if 儰==丁{佁}else{-佁},[0,0])}Some(乭::俇)=>return(0,[0,0]),None=>{}}let mut 丰=临(l,丁)-临(l,乇);if 丁==l.丛(){let mut 丄=l.clone();丄.仱();if
丄.両().is_none()&&佈(&丄,乇,true,U[丁])<=丄.下(乇){丰-=佁/2;}}let 仺=[倓(l,丁,乷[0]),倓(l,乇,乷[1]),];(丰+倥(仺[0])-倥(仺[1]),仺)}
