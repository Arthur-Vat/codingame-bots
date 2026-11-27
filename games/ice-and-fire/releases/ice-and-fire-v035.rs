// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伺=u16;type Q=u32;type 丮=u64;type 伹=i32;type C=i64;type 亮=f64;type O=bool;type 乛=String;type 乸=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitAndAssign,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
傁:O=true;fn 僺(l:&J,q:F,伴:C,丩:G,丞:C)->O{let 丂=l.丝();let 丸=1-丂;let 借=傁&&丞<=l.下(丸)+2*l.丠(丸).max(0);let 伏=G::q(q)|G::q(q).丏();if!借&&(伏&丩).is_empty(){return
false;}let(x,y)=乘(q);let mut 乡=l.clone();if!乡.义(L::亀{x,y}){return false;}if 伳(&乡,丂)<伴{return true;}借&&佋(&乡,丸,false,T[丂])>丞}const 僂:O=false;const 傝:I=2;const
俨:I=1;const 俒:I=2;const 佁:F=0;const 佻:F=2;const 傦:O=false;const 储:O=false;const 傧:O=false;const 傥:O=false;const 偮:P<I> =Some(3);fn 儎(l:&J,丂:F,丷:I)->G{let
丬=l.map().valid;let mut 乇=l.丐(1-丂)&丬;for _ in 0..丷{乇=(乇|乇.丏())&丬;}乇}const 傈:F=4;const 俏:F=5;#[derive(Clone)]enum 丫{仔(L),争(K<L>),}impl 丫{fn 万(&self)->&[L]{match
self{丫::仔(专)=>std::slice::from_ref(专),丫::争(万)=>万,}}}fn 乘(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 倗(l:&J,丅:&mut Vec<丫>){丅.clear();if l.丢().is_some(){return;}let
丂=l.丝();let 亍=l.丿().亚();let mut 丨=[G::U;4];for 丆 in 1..=亍{丨[F::from(丆)]=l.乍(丂,丆);}for 三 in l.乼().iter(){let Some((上,丆))=l.亾(三)else{continue;};let 丷=G::q(三).丏()&丨[F::from(丆)];for
两 in 丷.iter(){let(x,y)=乘(两);丅.push(丫::仔(L::乹{上,x,y}));}}let 下=l.下(丂);let 七=l.七(丂);let 乇=l.介(丂);let 乴=match 傝{0=>乇,1=>{let 会=l.map().valid.丕(七).丏()&七;乇.丕(七)|会}_=>乇.丕(七),};for
丆 in 1..=亍{let 丌=10*C::from(丆);if 下<丌{break;}let mut 丒=乴&丨[F::from(丆)];for 儆 in 1..丆{丒=丒.丕(丨[F::from(儆)]);}if let Some(丷)=偮{丒&=儎(l,丂,丷);}for q in 丒.iter(){let(x,y)=乘(q);丅.push(丫::仔(L::举{丆,x,y}));}}if
下>=互{let 丧=l.丐(丂)&乇&l.仙();if l.丿().但(){for q in(丧&l.map().丼).iter(){let(x,y)=乘(q);丅.push(丫::仔(L::亯{x,y}));}}if l.丿().仲(){let 会=l.map().valid.丕(l.丐(丂)).丏();let
丒=(丧&会).丕(l.map().丼);let 丩=丩(l,1-丂);let 伴=C::from((丩&l.七(丂)).count());let 丞=佋(l,1-丂,false,T[丂]);let 両=if 僂{let(亩,佒)=之(l,丂);亩.丕(佒)}else{G::U};for q in 丒.iter().filter(|&q|{let
伏=G::q(q)|G::q(q).丏();!(伏&両).is_empty()||僺(l,q,伴,丩,丞)}){let(x,y)=乘(q);丅.push(丫::仔(L::亀{x,y}));}}}let 價=match 俨{1=>互,2=>10,_=>C::MAX,};if 下>=價{let 丩=丩(l,1-丂);let
些=l.map().valid.丕(l.丐(丂)|l.丐(1-丂));for 三 in(l.乼()&丩).iter(){let Some((上,丆))=l.亾(三)else{continue;};let 丷=G::q(三).丏()&丨[F::from(丆)]&些;let(仝,仞)=乘(三);let 侟=if
俨==1{if l.map().丼.test(三){continue;}L::亀{x:仝,y:仞}}else{L::举{丆:1,x:仝,y:仞,}};for 两 in 丷.iter(){let(x,y)=乘(两);丅.push(丫::争(vec![L::乹{上,x,y},侟]));}}}if 俒!=0&&下>=互&&l.丿().仲(){僭(l,&丨,丅);}if
佁!=0&&下>=20{儫(l,丨[1],下,丅);}if 傥{僡(l,丨[1],下,丅);}for 串 in 倌(l,俏,下,傈){if 串.万.len()>1{丅.push(丫::争(串.万));}}if let Some(丞)=侎(l,T[1-丂],下){if 丞.万.len()>1{丅.push(丫::争(丞.万));}}}fn
僭(l:&J,丨:&[G;4],丅:&mut Vec<丫>){let 丂=l.丝();let 丸=1-丂;let 丩=丩(l,丸);let 严=l.丐(丂);let 丙=l.丐(丸);let 会=丙.丏();let 丼=l.map().丼;for 三 in(l.乼()&丩&l.七(丂)).iter(){if
丼.test(三){continue;}let Some((上,丆))=l.亾(三)else{continue;};let 僊=l.map().业(三,T[丸]);let 丷=G::q(三).丏()&丨[F::from(丆)];let(仝,仞)=乘(三);for 两 in 丷.iter(){let 來=match
俒{1=>严.test(两)&&l.map().业(两,T[丸])>僊,2=>严.test(两)&&会.test(两),_=>严.test(两)||(丙.test(两)&&两!=T[丸]),};if!來{continue;}let(x,y)=乘(两);丅.push(丫::争(vec![L::乹{上,x,y},L::亀{x:仝,y:仞},]));}}}fn
儫(l:&J,侧:G,下:C,丅:&mut Vec<丫>){let 丂=l.丝();let 並=l.map();let 些=並.valid.丕(l.丐(丂)|l.丐(1-丂));let 両=些&侧;let 乴:K<F> =(l.七(丂).丏()&両).iter().collect();if 乴.is_empty(){return;}let
侯=|与:F|{乴.iter().copied().min_by_key(|&q|(並.业(q,与),q))};let mut 侏:K<(F,F)> =K::new();if 傦{for 丗 in 儬(l,&乴){if let Some(乁)=侯(丗){侏.push((乁,丗));}}}else{let
与=T[1-丂];let mut 侾=乴.clone();侾.sort_by_key(|&q|(並.业(q,与),q));侾.truncate(佻);侏.extend(侾.into_iter().map(|乁|(乁,与)));}let 享=F::try_from(下/10).unwrap_or(0).min(佁);for(乁,与)in
侏{let mut 丞=vec![乁];let mut 亭=G::q(乁);while 丞.len()<享{let 乻=丞[丞.len()-1];if 乻==与{break;}let 丄=(G::q(乻).丏()&両).丕(亭).iter().min_by_key(|&q|(並.业(q,与),q));let
Some(丄)=丄 else{break;};丞.push(丄);亭.set(丄);let 万=丞.iter().map(|&q|{let(x,y)=乘(q);L::举{丆:1,x,y}}).collect();丅.push(丫::争(万));}if 傧{儃(l,&丞,丅);}}}fn 儃(l:&J,丞:&[F],丅:&mut
Vec<丫>){let 丂=l.丝();let 乁=丞[0];let 乽=l.乼()&l.丈(丂)&G::q(乁).丏();for 三 in 乽.iter(){let Some((上,丆))=l.亾(三)else{continue;};if!l.乍(丂,丆).test(乁){continue;}let(x,y)=乘(乁);let
mut 万=vec![L::乹{上,x,y}];for&q in&丞[1..]{let(x,y)=乘(q);万.push(L::举{丆:1,x,y});丅.push(丫::争(万.clone()));}}}fn 儬(l:&J,乴:&[F])->K<F>{let 丂=l.丝();let 並=l.map();let(乊,丯)=佑(l);let
严:K<F> =l.丈(丂).iter().collect();let 丙:K<F> =l.丈(1-丂).iter().collect();let 佐=|丗:F|{乴.iter().map(|&q|並.业(q,丗)).min().unwrap_or(I::MAX)};let 僙=|丗:F|{let 丩=佐(丗);严.iter().any(|&亽|{let
d=並.业(亽,丗);d<=丩&&丙.iter().all(|&丸|d<並.业(丸,丗))})};let 侪=|丗:F|{储&&(l.丐(1-丂).test(丗)||丙.iter().any(|&丸|並.业(丸,丗)<佐(丗)))};let 両=|丒:G|->K<F>{let mut 亅:K<F> =丒.丕(l.丐(丂)).iter().filter(|&丗|佐(丗)!=I::MAX&&!僙(丗)&&!侪(丗)).collect();亅.sort_by_key(|&丗|(佐(丗),丗));亅};let
mut 亅=両(丯);if 亅.is_empty(){亅=両(乊);}亅.truncate(佻);亅}fn 僛(l:&J)->K<F>{let 丂=l.丝();let 並=l.map();let(乊,丯)=佑(l);let 七=l.七(丂);let 严=l.丐(丂);let 丩=丩(l,1-丂);let
丙:K<F> =l.丈(1-丂).iter().collect();let 倕=|q:F|{七.iter().map(|a|並.业(a,q)).min().unwrap_or(I::MAX)};let mut 丒:K<F> =(乊|丯).iter().filter(|&q|{if 严.test(q){七.test(q)&&丩.test(q)}else{!l.丐(1-丂).test(q)&&丙.iter().any(|&e|並.业(e,q)<倕(q))}}).collect();丒.sort_by_key(|&q|(I::from(!丯.test(q)),倕(q),q));丒.truncate(
2*佻);丒}fn 僡(l:&J,侧:G,下:C,丅:&mut Vec<丫>){let 丂=l.丝();let 並=l.map();let 些=並.valid.丕(l.丐(丂)|l.丐(1-丂));let 両=些&侧;let 七=l.七(丂);let 丧=七&l.仙();let 儅=if 佁==0{3}else{佁};for
q in 僛(l){if!l.丐(丂).test(q){let 乁=(七.丏()&両).iter().min_by_key(|&c|(並.业(c,q),c));if let Some(乁)=乁{let mut 丞=vec![乁];let mut 亭=G::q(乁);while 丞[丞.len()-1]!=q&&丞.len()<儅{let
乻=丞[丞.len()-1];let 丄=(G::q(乻).丏()&両).丕(亭).iter().min_by_key(|&c|(並.业(c,q),c));let Some(丄)=丄 else{break;};丞.push(丄);亭.set(丄);}let 丌=10*C::try_from(丞.len()).unwrap_or(C::MAX);if
丞.len()>=2&&丞[丞.len()-1]==q&&丌<=下{let 万=丞.iter().map(|&c|{let(x,y)=乘(c);L::举{丆:1,x,y}}).collect();丅.push(丫::争(万));}}}if 下>=互&&l.丿().仲(){let 丼=(G::q(q)|G::q(q).丏())&丧;for
乭 in 丼.丕(並.丼).iter(){let(x,y)=乘(乭);丅.push(丫::仔(L::亀{x,y}));}}}}const 傕:I=2;const 傶:C=5;fn 乽(l:&J,丁:F)->G{侮(l,丁,傕)}fn 侮(l:&J,丁:F,三:I)->G{let mut 乽=G::U;if
三>3{return 乽;}for q in(l.丈(丁)&l.七(丁)).iter(){if l.亢(q).is_some_and(|丆|丆>=三){乽|=G::q(q);}}乽}fn 丩(l:&J,丶:F)->G{let 乏=l.下(丶)+l.丠(丶);儣(l,丶,乽(l,丶),乏)}fn 亨(l:&J,丁:F)->G{丩(l,1-丁)&l.七(丁)}fn
伳(l:&J,丁:F)->C{C::from(亨(l,丁).count())}const 俟:C=4;fn 儊(丒:C)->C{丒/俟}fn 为(l:&J,丁:F,亨:G)->C{let 七=l.七(丁);let 为=(l.为()&七).丕(亨);let 丒=C::from(七.count());let
侊=if l.乨()<=偭{丒/偬+偫}else{儊(丒)};侊*C::from(为.count())}const 偭:Q=15;const 偫:C=6;const 偬:C=俟;const 傭:C=2;const 佂:I=1;const 俧:I=3;const 傮:I=6;fn 儂(仈:I,乇:I,仛:I,d:I)->I{match
仈{0=>4,1 if d<=乇=>4,1=>0,_ if d<=乇=>4,_ if d>=仛=>0,_=>(4*Q::from(仛-d)/Q::from(仛-乇))as I,}}fn 偝(l:&J,丁:F)->C{let 七=l.七(丁);let 丸=if 佂==0{G::U}else{l.七(1-丁)};let[a,b,c]=七.伷();let[e,f,g]=丸.伷();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丁 as 丮).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丟=(hash as F)&(俱-1);僃.with(|侌|{let mut 侌=侌.borrow_mut();let 从=&mut 侌[丟];if 从.0==七&&从.1==丸&&从.2==丁 as I+1{return 从.3;}let 中=if 佂==0{兗(七,T[丁])}else if 佂==1{let
丬=l.map().valid;let mut 乇=丸&丬;for _ in 0..俧{乇=(乇|乇.丏())&丬;}兖(七,T[丁],乇)}else{let d=丳(l,丸);let mut 乒=[0u8;M];for(w,&丷)in 乒.iter_mut().zip(d.iter()){*w=儂(佂,俧,傮,丷);}偞(七,T[丁],&乒)};*从=(七,丸,丁
as I+1,中);中})}const 俱:F=1<<12;thread_local!{static 僃:std::cell::RefCell<K<(G,G,I,C)>> =std::cell::RefCell::new(vec![(G::U,G::U,0,0);俱]);}fn 儞(七:G)->G{let
n=七.佥();let s=七.佦();let e=七.偃();let w=七.偄();let 儍=e.佥();let 儏=w.佥();let 儭=e.佦();let 兀=w.佦();let 儁=[n&儍&e,e&儭&s,s&兀&w,w&儏&n];let 亿=|a:G,b:G|(a|b).丕(a&b);let
俳=|[a,b,c,d]:[G;4]|{let(倾,倀)=(亿(a,b),a&b);let(倿,倂)=(亿(c,d),c&d);let t=倾&倿;let 倁=亿(倀,倂);(亿(倾,倿),亿(倁,t),(倀&倂)|(t&倁))};let(僼,倛,僽)=俳([n,e,s,w]);let(僦,倏,僧)=俳(儁);let
俻=僦.丕(僼);let 偠=亿(倛,倏);let 僜=亿(偠,俻);let 僋=倏.丕(倛)|俻.丕(偠);let 僝=亿(亿(僽,僧),僋);七&(僜|僝)}fn 兖(七:G,乱:F,乇:G)->C{if!七.test(乱){return 0;}let 亇=七.count();let mut 二=0i64;let
mut 享=0i64;for v in(儞(七)&乇).iter(){if v==乱{continue;}let 乖=七.丕(G::q(v));let mut 乫=G::q(乱);loop{let 丄=(乫|乫.丏())&乖;if 丄==乫{break;}乫=丄;}let 串=C::from(亇-1-乫.count());if
串>0{二+=4;享=享.max(4*串);}}(二+享)/4}fn 兗(七:G,乱:F)->C{偞(七,乱,&[4;M])}fn 偞(七:G,乱:F,乒:&[I;M])->C{struct 伐<'a>{乒:&'a[I;M],七:G,乶:[I;M],乞:[I;M],size:[I;M],仐:I,二:C,享:C,}fn
仪(d:&mut 伐<'_>,v:F)->I{d.仐+=1;let 乶=d.仐;d.乶[v]=乶;let mut 乞=乶;let mut 侹=1u8;let mut 串=0u8;for w in(G::q(v).丏()&d.七).iter(){if d.乶[w]==0{let 偏=仪(d,w);侹+=偏;乞=乞.min(d.乞[w]);if
d.乞[w]>=乶{串+=偏;}}else{乞=乞.min(d.乶[w]);}}d.乞[v]=乞;d.size[v]=侹;if 串>0&&d.乶[v]>1{let w=C::from(d.乒[v]);d.二+=w;d.享=d.享.max(w*C::from(串));}侹}if!七.test(乱){return
0;}let mut d=伐{乒,七,乶:[0;M],乞:[0;M],size:[0;M],仐:0,二:0,享:0,};仪(&mut d,乱);(d.二+d.享)/4}fn 儯(l:&J,丁:F)->C{傭*(偝(l,1-丁)-偝(l,丁))}fn 倍(l:&J,丁:F)->C{if l.下(丁)+l.丠(丁)>=0{return
0;}(l.丈(丁)&l.七(丁)).iter().filter_map(|q|l.亢(q)).map(|丆|10*C::from(丆)).sum()}const 傇:C=15;const 傾:[C;2]=[3,1];const 傼:[C;2]=[4,20];fn 儾(l:&J,丁:F)->[C;2]{let
mut 丛=[0;2];for q in(l.丈(丁)&l.七(丁)).iter(){match l.亢(q){Some(2)=>丛[0]+=1,Some(3)=>丛[1]+=1,_=>{}}}丛}fn 偎(l:&J,丁:F)->C{let[偕,光]=儾(l,丁);let 俼=|丆:F|(3-傾[丆])*傼[丆];-傇*偕+俼(0)*偕+俼(1)*光}const
傀:C=0;const 傃:C=2;const 傂:C=100;fn 假(亪:C)->C{if 亪<0{return 0;}傀+傃*亪.min(傂)/2}const 俞:I=2;const 俛:C=2;const 俚:C=4;const 傄:I=3;fn 亸(l:&J,丁:F)->C{if 俛==0{return
0;}let 丬=l.map().valid;let mut 丈=G::U;for q in(l.丈(丁)&l.七(丁)).iter(){if l.亢(q).is_some_and(|丆|丆<=傄){丈|=G::q(q);}}let mut 佫=丬.丕(l.丐(丁));let mut 仅=G::U;let
mut 丷=0;for 业 in 1..=俚{佫=(佫|佫.丏())&丬;let 仆=(丈&佫).丕(仅);丷+=(业-1)*C::from(仆.count());仅|=仆;}丷+=俚*C::from(丈.丕(仅).count());俛*丷}const 俰:C=8;fn 之(l:&J,丁:F)->(G,G){let
七=l.七(丁);let 亩=七&l.丐(1-丁).丏();let 乗=l.乗()&七;(亩,亩&(乗|乗.丏()))}fn 做(l:&J,丁:F)->C{if 俰==0{return 0;}俰*C::from(之(l,丁).1 .count())}const 併:C=0;const 俗:I=1;const
俘:C=4;const 佶:C=0;const 俖:C=0;const 偷:Q=14;const 偶:C=2;const 俯:C=0;const 僁:Q=14;fn 仫(l:&J,丁:F)->C{if 俯==0||l.乨()>僁{return 0;}let 严=丳(l,l.七(丁));let 丙=丳(l,l.七(1-丁));let
mut 令=0;for c in l.map().valid.iter(){match 严[c].cmp(&丙[c]){std::cmp::Ordering::Less=>令+=1,std::cmp::Ordering::Greater=>令-=1,std::cmp::Ordering::Equal=>{}}}俯*令}struct
俙{valid:G,乊:G,丯:G,}thread_local!{static 偸:std::cell::RefCell<P<俙>> =const{std::cell::RefCell::new(None)};}fn 佑(l:&J)->(G,G){偸.with(|q|{let mut 乃=q.borrow_mut();let
丬=l.map().valid;if 乃.as_ref().is_none_or(|z|z.valid!=丬){let 仨=仨(l);let mut 乊=G::U;for c in 丬.iter(){if 仨[c]<=偶{乊.set(c);}}*乃=Some(俙{valid:丬,乊,丯:仢(l),});}乃.as_ref().map_or((G::U,G::U),|z|(z.乊,z.丯))})}fn
佒(l:&J,丁:F)->G{let 七=l.七(丁);let 乗=l.乗()&七;(乗|乗.丏())&七}fn 丗(l:&J,丁:F)->C{if(併==0&&俘==0&&佶==0&&俖==0)||l.乨()>偷{return 0;}let(乊,丯)=佑(l);let mut 亇=0;if 併!=0{let
严=丳(l,l.七(丁));let 丙=丳(l,l.七(1-丁));let mut 令=0;for c in 乊.iter(){let(a,b)=(严[c],丙[c]);if a!=I::MAX&&a.saturating_add(俗)<=b{令+=1;}else if b!=I::MAX&&b.saturating_add(俗)<=a{令-=1;}}亇+=併*令;}let
丛=|乓:G|C::from(乓.count());let(丹,丙)=(l.七(丁),l.七(1-丁));亇+=俘*(丛(丹&乊)-丛(丙&乊));if 佶!=0{亇+=佶*(丛(佒(l,丁)&乊)-丛(佒(l,1-丁)&乊));}亇+=俖*(丛(丹&丯)-丛(丙&丯));亇}const 俕:C=4;const
俔:C=0;const 偳:C=4;const 側:Q=12;fn 僲()->[G;2]{static 偲:std::sync::OnceLock<[G;2]> =std::sync::OnceLock::new();*偲.get_or_init(僳)}fn 僳()->[G;2]{let mut 丅=[G::U;2];for
c in 0..M{let(x,y)=((c%Y)as C,(c/Y)as C);if x-y>=俕{丅[0].set(c);}else if y-x>=俕{丅[1].set(c);}}丅}fn 停(l:&J,丁:F)->C{let[a,b]=僲();let 七=l.七(丁);C::from((七&a).count().min((七&b).count())).min(偳)}fn
僱(l:&J,丁:F)->C{if 俔==0||l.乨()>側{return 0;}俔*(停(l,丁)-停(l,1-丁))}fn 僩(l:&J,丁:F)->C{let 乽=[侮(l,丁,俞),侮(l,1-丁,俞),];let(丱,企)=僪(l,丁,乽);let 丱=丱+假(企[0])-假(企[1]);if
l.丢().is_some(){return 丱;}let 严=亨(l,丁);let 丙=亨(l,1-丁);let 伴=C::from(丙.count())-C::from(严.count());丱-2*倍(l,丁)+2*倍(l,1-丁)+儯(l,丁)+偎(l,丁)-偎(l,1-丁)-亸(l,丁)+亸(l,1-丁)+傶*伴/4+为(l,丁,严)-为(l,1-丁,丙)+做(l,丁)-做(l,1-丁)+丗(l,丁)+仫(l,丁)+僱(l,丁)}const
傑:F=8;const 傠:F=3;const 傉:F=32;const 傢:O=true;const 傰:P<F> =None;const 傱:F=3;const 傤:O=false;const 傡:丮=300;const 偪:亮=0.75;const 偽:P<Q> =Some(7);const 偻:O=false;const
偺:C=10;const 偼:Q=6;struct 亱{于:P<乸>,仟:P<乸>,伦:丮,}#[derive(Default)]struct 侀{万:K<L>,亗:F,亷:丮,东:F,主:F,倆:F,事:F,了:O,偋:O,仌:F,}const 僀:O=true;#[derive(Clone)]struct
亃{万:K<L>,侖:C,}struct 仭{万:K<L>,end:J,first:P<丮>,丘:P<K<亃>>,}type 休=HashMap<丮,C,BuildHasherDefault<DefaultHasher>>;struct 佾{丂:F,limit:亱,亷:丮,伣:休,仩:P<K<丫>>,俀:O,九:P<们>,}const
佼:C=0;const 俌:O=true;const 健:O=false;const 俋:P<I> =None;fn 僐(l:&J,东:&[亃],丅:&mut Vec<丫>){if!俌&&俋.is_none(){return;}let 侵=东.iter().map(|乬|乬.万.as_slice());僑(l,侵,(俌,健,俋),丅,);}fn
僑<'a>(l:&J,侵:impl Iterator<Item=&'a[L]>,(乗,丏,丆):(O,O,P<I>),丅:&mut Vec<丫>,){let 丂=l.丝();let 丹=l.七(丂);let mut 伡=G::U;for 乬 in 侵{for 专 in 乬{if let L::举{x,y,..}|L::乹{x,y,..}=*专{let
q=F::from(y)*Y+F::from(x);if 丹.test(q){伡|=G::q(q);}}}}if 伡.is_empty(){return;}let 丧=l.仙()&丹;let 下=l.下(丂);let 倷=|专:L,丅:&mut Vec<丫>|{if!丅.iter().any(|a|a.万()==[专]){丅.push(丫::仔(专));}};if
乗&&下>=互&&l.丿().仲(){let mut 丒=伡;if 丏{丒|=伡.丏();}for q in(丒&丧).丕(l.map().丼).iter(){let(x,y)=乘(q);倷(L::亀{x,y},丅);}}if let Some(丆)=丆.filter(|&丆|丆<=l.丿().亚()){if
下>=10*C::from(丆){for q in(伡&丧).iter(){let(x,y)=乘(q);倷(L::举{丆,x,y},丅);}}}}fn 侈(l:&mut J,万:&[L])->O{万.iter().all(|&专|l.义(专))}fn 亙(万:&[L])->丮{let mut hash:丮=0xcbf29ce484222325;for
专 in 万{let(佖,a,b,c)=match*专{L::乹{上,x,y}=>(1,丮::from(上),x,y),L::举{丆,x,y}=>(2,丮::from(丆),x,y),L::亯{x,y}=>(3,0,x,y),L::亀{x,y}=>(4,0,x,y),};let 伎=佖|a<<8|丮::from(b)<<40|丮::from(c)<<48;hash=(hash^伎).wrapping_mul(0x0100000001b3);}hash}fn
俵(乧:&J,乬:&[L])->J{let mut 丄=乧.clone();丄.仸();for&专 in 乬{丄.义(专);}丄.仸();丄}fn 佘(l:&J)->P<K<L>>{let 丂=l.丝();let 与=T[1-丂];let 伍=|丵:&J|丵.丢()==Some(乵::井(丂));for
q in l.乼().iter(){if l.map().业(q,与)!=1{continue;}let Some((上,_))=l.亾(q)else{continue;};let(x,y)=乘(与);let 专=L::乹{上,x,y};let mut 丵=l.clone();if 丵.义(专)&&伍(&丵){return
Some(vec![专]);}}let 丞=侎(l,与,l.下(丂))?;let mut 丵=l.clone();(侈(&mut 丵,&丞.万)&&伍(&丵)).then_some(丞.万)}fn 偍(l:&J,丁:F,个:C)->O{let 丌=侔(l,丁,l.丈(丁),T[1-丁]);丌!=亴&&l.下(丁)+个*l.丠(丁)>=丌}fn
儽(l:&J)->O{let 丂=l.丝();if 佼==0||l.丢().is_some(){return false;}let mut 丄=l.clone();丄.仸();let 伳=丄.丢().is_none()&&佋(&丄,1-丂,true,T[丂])<=丄.下(1-丂);!伳&&偍(l,丂,佼)}impl
佾{fn 俾(&self,乧:&J)->O{self.俀&&乧.丢()!=Some(乵::井(self.丂))&&!偍(乧,self.丂,佼)}fn 亏(&self)->O{self.亷>=self.limit.伦||self.limit.于.is_some_and(|t|乸::now()>=t)}fn
丱(&mut self,乧:&J,东:&[亃],例:C)->C{self.亷+=1;self.你(乧,东,例)}fn 你(&self,乧:&J,东:&[亃],例:C)->C{if 乧.丢()==Some(乵::井(self.丂)){return 佄;}if 东.is_empty(){return self.value(乧);}let
mut 佬=C::MAX;for 乬 in 东{佬=佬.min(self.value(&俵(乧,&乬.万)));if 佬<=例{break;}}佬}fn value(&self,l:&J)->C{let 伙=僩(l,self.丂);match&self.九{Some(九)if l.丢().is_none()=>伙+九.兄(l,self.丂),_=>伙,}}fn
俴(&self,习:&mut Vec<丫>){if let Some(九)=&self.九{for 佞 in&九.仌{习.push(丫::争(佞.clone()));}}}#[allow(clippy::too_many_arguments)]fn 儙(&mut self,乄:&J,价:C,习:&[丫],乕:&[(C,F)],书:&mut[O],东:&[亃],仹:&mut
休,侥:O,)->(P<(F,J,C)>,O){let mut 丑=None;let mut 佈=价;for&(佛,丟)in 乕{if 书[丟]{continue;}if 侥&&佛!=C::MAX&&佛<=佈-价{return(丑,true);}if self.亏(){return(丑,false);}书[丟]=true;let
万=习[丟].万();let mut 丄=乄.clone();if!侈(&mut 丄,万)||self.俾(&丄){仹.insert(亙(万),C::MIN/4);continue;}let 丱=self.丱(&丄,东,佈);仹.insert(亙(万),丱-价);if 丱>佈{佈=丱;丑=Some((丟,丄,丱));}}(丑,false)}fn
侓(&mut self,乱:&J,东:&[亃],仄:&[丮],僔:O)->仭{let 一=self;let mut 乮=None;let mut 乄=乱.clone();let mut 万=K::new();let mut 价=一.丱(&乄,东,C::MIN);let mut 仹=休::default();let
mut 习=K::new();let mut 乕=K::new();while 乄.丢().is_none()&&!一.亏(){if 万.is_empty()&&僔{match&一.仩{Some(仩)=>习.clone_from(仩),None=>{倗(&乄,&mut 习);if let Some(九)=一.九.as_ref().filter(|g|g.倯){习.push(丫::争(九.世.clone()));}一.俴(&mut
习);一.仩=Some(习.clone());}}}else{倗(&乄,&mut 习);一.俴(&mut 习);}僐(&乄,东,&mut 习);乕.clear();for(丟,亖)in 习.iter().enumerate(){let k=亙(亖.万());let 佛=仹.get(&k).or(一.伣.get(&k)).copied();乕.push((佛.unwrap_or(C::MAX),丟));}乕.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 书=vec![false;习.len()];if 万.is_empty(){for(丟,亖)in 习.iter().enumerate(){书[丟]=仄.contains(&亙(亖.万()));}}let mut 丑=None;for
侥 in[true,false]{let(乆,儴)=一.儙(&乄,价,&习,&乕,&mut 书,东,&mut 仹,侥,);丑=乆;if 丑.is_some()||!儴{break;}}let Some((丟,丄,丱))=丑 else{break;};if 万.is_empty(){乮=Some(亙(习[丟].万()));}万.extend_from_slice(习[丟].万());乄=丄;价=丱;}for(k,乐)in
仹{let 倜=一.伣.entry(k).or_insert(乐);*倜=(*倜).max(乐);}仭{万,end:乄,first:乮,丘:None,}}fn 儧(&self,乧:&J,亗:F)->K<K<L>>{let mut 丄=乧.clone();丄.仸();let mut 丅=K::new();if
丄.丢().is_some(){return 丅;}let 亜=丄.丝();let 下=丄.下(亜);if let Some(丞)=侎(&丄,T[self.丂],下){丅.push(丞.万);}for 串 in 倌(&丄,俏,下,傠){丅.push(串.万);}if 傢{let mut 候=K::new();僸(&丄,&mut
候);丅.push(候);}let mut 丙=佾{丂:亜,limit:亱{伦:傡*亗 as 丮,..self.limit},亷:0,伣:休::default(),仩:None,俀:false,九:None,};let mut 仄=K::new();let mut 乆:K<K<L>> =K::new();for
_ in 0..亗{let 世=丙.侓(&丄,&[],&仄,true);if!乆.contains(&世.万){乆.push(世.万.clone());丅.push(世.万);}match 世.first{Some(乮)if!丙.亏()=>仄.push(乮),_=>break,}}丅.retain(|万|!万.is_empty());丅}fn
俶(&self,乧:&J)->K<亃>{self.俷(乧,1)}fn 俷(&self,乧:&J,亗:F)->K<亃>{self.儧(乧,亗).into_iter().map(|万|{let 侖=self.value(&俵(乧,&万));亃{万,侖}}).collect()}fn 事(&mut self,乱:&J,亖:&丫,东:&[亃])->P<仭>{let
mut 乡=乱.clone();if!侈(&mut 乡,亖.万())||self.俾(&乡){return None;}self.亷+=1;let 乖=self.侓(&乡,东,&[],false);let mut 万=亖.万().to_vec();万.extend_from_slice(&乖.万);Some(仭{万,end:乖.end,first:Some(亙(亖.万())),丘:None,})}fn
侜(&self,世:&仭,东:&[亃])->C{let 丱=self.你(&世.end,东,C::MIN);match&世.丘{Some(丘)if!丘.is_empty()=>丱.min(self.你(&世.end,丘,C::MIN)),_=>丱,}}}fn 儥(乆:&[亃],东:&mut Vec<亃>)->O{let
乎:K<_> =东.iter().map(|r|r.万.clone()).collect();for 乬 in 乆{if 东.iter().any(|r|r.万==乬.万){continue;}东.push(乬.clone());}东.sort_by_key(|乬|乬.侖);东.truncate(傑);东.iter().map(|r|&r.万).ne(&乎)}fn
先(l:&J,乔:亱,了:&mut 亦)->侀{偑(l,乔,了,None)}fn 偑(l:&J,乔:亱,了:&mut 亦,九:P<们>,)->侀{let 丂=l.丝();let mut 乂=侀::default();if l.丢().is_some(){return 乂;}if let Some(万)=佘(l){乂.万=万;return
乂;}let 乜=|儱:亮|match(乔.于,乔.仟){(Some(于),Some(仟))=>Some(于+(仟-于).mul_f64(儱)),_=>None,};if 偽.is_some_and(|且|l.乨()>=且){了.end();}if 了.儝(l){let 万=了.且(l,乜(0.4));let
僓=万.iter().any(|专|{matches!(专,L::举{..}|L::亯{..}|L::亀{..})});let 亸=偻&&!僓&&l.乨()>=偼&&l.下(丂)>=偺;if!亸{乂.了=true;乂.万=万;return 乂;}了.end();}let mut 乑=佾{丂,limit:亱{于:乜(偪),..乔},亷:0,伣:休::default(),仩:None,俀:儽(l),九,};let
mut 东=K::new();let mut 主:K<仭> =K::new();let mut 仄=K::new();let mut 低=false;let mut 事:K<丫> =K::new();let mut 倽=false;let mut 侶=std::time::Duration::ZERO;let
mut 乁=乑.九.as_ref().filter(|g|g.start).map(|g|丫::争(g.世.clone()));while 乂.亗<傉&&(主.is_empty()||!乑.亏()){let 侽=match 乁.take(){Some(亖)=>乑.事(l,&亖,&东),None=>None,};let
世=if let Some(世)=侽{世}else if 低{if 事.is_empty()&&傤&&!倽{倽=true;let 丑=主.iter().max_by_key(|p|乑.你(&p.end,&东,C::MIN));if let Some(丑)=丑{for 儳 in 0..丑.万.len(){let
mut 乖=丑.万.clone();乖.remove(儳);if!乖.is_empty(){事.push(丫::争(乖));}}}}let Some(亖)=事.pop()else{break;};match 乑.事(l,&亖,&东){Some(世)=>世,None=>continue,}}else{乑.侓(l,&东,&仄,true)};乂.亗+=1;let
偁=主.iter().any(|p|p.万==世.万);let 乮=世.first;let 偊=乑.亏();let mut 世=世;let mut 佚=false;if!偊&&!偁{let 侽=乸::now();let 乆=乑.俶(&世.end);侶=侶.max(侽.elapsed());佚=儥(&乆,&mut
东);世.丘=Some(乆);}if!偁{主.push(世);}if 偊{break;}if!佚&&!低{match 乮{Some(乮)=>仄.push(乮),None=>{低=true;事=乑.仩.clone().unwrap_or_default();事.retain(|a|!仄.contains(&亙(a.万())));let
乐=|a:&丫|{乑.伣.get(&亙(a.万())).copied().unwrap_or(C::MIN)};事.sort_by_key(乐);}}}}if let Some(亗)=傰{let mut 乕:K<(C,F)> =主.iter().enumerate().map(|(i,p)|(乑.侜(p,&东),i)).collect();乕.sort_by(|a,b|b.0
.cmp(&a.0).then(b.1 .cmp(&a.1)));let 丌=侶.mul_f64(亗 as 亮);for&(_,丟)in 乕.iter().take(傱){let 僰=乑.limit.于.is_none_or(|于|乸::now()+丌<=于);if!僰{break;}let 佚=乑.俷(&主[丟].end,亗);let
丘=主[丟].丘.get_or_insert_with(K::new);for 乬 in 佚{if!丘.iter().any(|r|r.万==乬.万){丘.push(乬);}}}}乂.事=F::from(低);乂.亷=乑.亷;乂.东=东.len();乂.主=主.len();let 仇=||乔.仟.is_some_and(|t|乸::now()>=t);let
mut 伉:K<P<C>> =vec![None;主.len()];for 丟 in(0..主.len()).rev(){if 丟+1<主.len()&&仇(){break;}伉[丟]=Some(乑.侜(&主[丟],&东));}let 伇=|伉:&[P<C>]|{let mut 丑:P<(C,F)>
=None;for(丟,丱)in 伉.iter().enumerate().rev(){if let Some(丱)=*丱{if 丑.is_none_or(|(亠,_)|丱>亠){丑=Some((丱,丟));}}}丑};let mut 丑=伇(&伉);乑.limit.于=乔.仟;while let Some((_,丟))=丑{if!僀||主[丟].丘.is_some()||仇(){break;}let
乆=乑.俶(&主[丟].end);主[丟].丘=Some(乆);乂.倆+=1;伉[丟]=Some(乑.侜(&主[丟],&东));丑=伇(&伉);}if let Some((_,丟))=丑{乂.偋=乑.九.as_ref().is_some_and(|g|!g.世.is_empty()&&主[丟].万.starts_with(&g.世));乂.仌=乑.九.as_ref().map_or(0,|g|{g.仌.iter().filter(|p|p.iter().all(|c|主[丟].万.contains(c))).count()});let
mut 不=l.clone();for&专 in&主[丟].万{if 不.义(专){乂.万.push(专);}}}乂}fn 儜(乔:&dyn Fn(乸,Q)->亱,mut 了:亦,侤:impl BufRead,佝:impl Write,)->Result<(),乛>{let mut 京=伔::default();个(侤,佝,|仼,亐,且,乁|match
J::read(仼,亐,交::佯,且){Ok(l)=>{let 仿=九(&l,&mut 京);let 乂=match 仿{Some(九)=>{偑(&l,乔(乁,且),&mut 了,Some(九))}None=>先(&l,乔(乁,且),&mut 了),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",且,乂.亗,乂.亷,乂.东,乂.倆,乂.事,I::from(乂.偋),乂.仌,乁.elapsed());僈(&乂.万)}Err(仚)=>{eprintln!("plan: cannot read the input: {}"
,仚);"WAIT".to_string()}},)}const 傘:P<C> =None;const 傿:O=false;#[derive(Clone,Copy,PartialEq)]enum 伓{佱,偨,}#[derive(Clone,Copy)]struct 亳{q:F,kind:伓,仠:O,value:C,}fn
乪(l:&J,丁:F)->K<亳>{儐(l,丁,傿)}fn 儐(l:&J,丁:F,允:O)->K<亳>{let(乊,丯)=佑(l);let(丹,丙)=(l.七(丁),l.七(1-丁));let(伟,传)=(丳(l,丹),丳(l,丙));let 伙=伩(l,&伟,&伟,&传);let mut 丅=K::new();for
q in(乊|丯).iter(){let 侗=丳(l,G::q(q));let 仠=丹.test(q);let 中=if 允{伩(l,&伟,&侗,&传)-伩(l,&传,&侗,&伟)}else if 仠{let 偟=丳(l,僴(丹,丁,q));伙-伩(l,&偟,&偟,&传)}else{伩(l,&伟,&侗,&传)-伙};丅.push(亳{q,kind:if
丯.test(q){伓::佱}else{伓::偨},仠,value:中,});}丅.sort_by_key(|o|(-o.value,o.kind!=伓::佱,o.q));丅}fn 伩(l:&J,a:&[I;M],g:&[I;M],b:&[I;M])->C{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 仅(乪:&[亳])->K<亳>{乪.iter().filter(|o|o.value>=傘.unwrap_or(1)).copied().collect()}fn 僴(七:G,丁:F,q:F)->G{let
乖=七.丕(G::q(q));let mut 乯=G::q(T[丁])&乖;loop{let 丄=乯|(乯.丏()&乖);if 丄==乯{return 乯;}乯=丄;}}const 俪:C=10;const 傖:C=100;#[derive(Clone,Copy)]enum 伿{众(Q),举(F),}#[derive(Clone,Copy)]struct
乿{个:Q,侢:伿,}#[derive(Clone,Copy)]struct 伕{q:F,ours:P<乿>,作:P<乿>,theirs:P<乿>,亪:C,}fn 儢(l:&J,丁:F,乪:&[亳])->K<伕>{let mut 丅=K::new();for 临 in 乪.iter().filter(|o|!o.仠){let
q=临.q;let(儖,作)=俸(l,丁,q);let 严=丑(儖,作);let(兇,兆)=俸(l,1-丁,q);let 丙=丑(兇,兆);let 个=|a:P<乿>|a.map_or(傖,|a|C::from(a.个));丅.push(伕{q,ours:严,作,theirs:丙,亪:个(丙)-个(严),});}丅}fn
丑(亽:P<乿>,伶:P<乿>)->P<乿>{match(亽,伶){(Some(u),Some(t))=>Some(if t.个<u.个{t}else{u}),(u,t)=>u.or(t),}}fn 俸(l:&J,丶:F,q:F)->(P<乿>,P<乿>){let 並=l.map();let mut
亽:P<乿> =None;for 三 in l.丈(丶).iter(){let d=並.业(三,q);let Some(上)=l.云(三)else{continue;};if d!=I::MAX&&亽.is_none_or(|u|Q::from(d)<u.个){亽=Some(乿{个:Q::from(d),侢:伿::众(上),});}}let
下=l.下(丶)+if 丶==l.丝(){0}else{l.丠(丶)};let 佪=match(下,l.丠(丶)){(下,_)if 下>=俪=>Some(0),(_,丠)if 丠<=0=>None,(下,丠)=>Some((俪-下+丠-1)/丠),};let mut 伶:P<乿> =None;if let
Some(佪)=佪{let 乴=l.七(丶).丏()&並.valid&l.乍(丶,1);for 三 in 乴.iter(){let d=並.业(三,q);if d!=I::MAX&&伶.is_none_or(|t|1+佪 as Q+Q::from(d)<t.个){伶=Some(乿{个:1+佪 as Q+Q::from(d),侢:伿::举(三),});}}}(亽,伶)}const
俫:C=10;#[derive(Default)]struct 伔{倭:BTreeMap<Q,F>,}#[derive(Default)]struct 仂{仜:K<(Q,F)>,亡:K<(F,F)>,back:K<Q>,}fn 仳(l:&J,丁:F,乪:&[亳],仦:&[伕],东:&mut 伔,)->仂{俹(l,丁,乪,仦,东,l.丈(丁))}fn
俹(l:&J,丁:F,乪:&[亳],仦:&[伕],东:&mut 伔,侘:G,)->仂{let 並=l.map();let 丈:BTreeMap<Q,F> =(l.丈(丁)&侘).iter().filter_map(|c|l.云(c).map(|上|(上,c))).collect();let mut 両:K<(亳,伕)>
=K::new();for o in 仅(乪).into_iter().filter(|o|!o.仠){if let Some(r)=仦.iter().find(|r|r.q==o.q){if r.ours.is_some()&&r.亪>=0{両.push((o,*r));}}}両.sort_by_key(|(o,r)|{(-o.value,o.kind!=伓::佱,r.ours.map_or(Q::MAX,|a|a.个),o.q,)});let
久=|亽:Q,丗:F|Q::from(並.业(丈[&亽],丗));let 倡=|亘:&BTreeMap<F,Q>,伯:&BTreeSet<Q>,上:Q|{伯.contains(&上)||亘.values().any(|&u|u==上)};let mut 亘:BTreeMap<F,Q> =BTreeMap::new();for(&上,&丗)in&东.倭{if
丈.contains_key(&上)&&両.iter().any(|(o,_)|o.q==丗)&&!亘.contains_key(&丗){亘.insert(丗,上);}}let mut 伯:BTreeSet<Q> =BTreeSet::new();for(o,_)in&両{let Some(&倣)=亘.get(&o.q)else{continue;};let
侯=丈.keys().copied().filter(|&上|!倡(&亘,&伯,上)).map(|上|(久(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();if let Some((d,上))=侯{if d<久(倣,o.q){伯.insert(倣);亘.insert(o.q,上);}}}let
mut 亡=K::new();for(o,r)in&両{if 亘.contains_key(&o.q){continue;}let 丧=丈.keys().copied().filter(|&上|!倡(&亘,&伯,上)).map(|上|(久(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();let
偗=丧.map(|(d,_)|C::from(d));let 佨=r.作.and_then(|t|match t.侢{伿::举(q)if l.下(丁)-俫*亡.len()as C>=俫&&!亡.iter().any(|&(c,_)|c==q)=>{Some((q,C::from(t.个)))}_=>None,});let
偔=佨.map(|(_,t)|t);let 偘=match(偗,偔){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 僘=if 偘{偔}else{偗};let Some(个)=僘 else{continue;};if r.theirs.map_or(100,|a|C::from(a.个))<个{continue;}match(偘,佨,丧){(true,Some((q,_)),_)=>亡.push((q,o.q)),(false,_,Some((_,上)))=>{亘.insert(o.q,上);}_=>{}}}let
mut 丅=仂{亡,..仂::default()};for(o,_)in&両{if let Some(&上)=亘.get(&o.q){丅.仜.push((上,o.q));}}丅.back=l.丈(丁).iter().filter_map(|c|l.云(c)).filter(|上|!丅.仜.iter().any(|&(u,_)|u==*上)).collect();丅.back.sort_unstable();东.倭=丅.仜.iter().copied().collect();丅}const
俬:C=10;fn 倄(l:&J,丁:F,上:Q)->P<F>{l.丈(丁).iter().find(|&c|l.云(c)==Some(上))}fn 偉(l:&J,丁:F,三:F,丗:F)->K<F>{let 並=l.map();let 仆=並.业(三,丗);if 仆==I::MAX||仆==0{return
K::new();}let mut 丅:K<F> =(G::q(三).丏()&並.valid).iter().filter(|&n|並.业(n,丗)<仆).collect();丅.sort_by_key(|&n|(l.丐(丁).test(n),n));丅}fn 产(不:&mut J,丅:&mut Vec<L>,专:L)->O{let
倰=不.义(专);if 倰{丅.push(专);}倰}#[derive(Clone,Copy)]struct 仯{侷:O,guard:I,倒:O,偀:O,}fn 仌(l:&J,京:&仂,乷:仯)->K<K<L>>{let mut 丅=K::new();for&(上,丗)in&京.仜{let 儔=仂{仜:vec![(上,丗)],..仂::default()};let
佞=偌(l,&儔,仯{侷:false,..乷},);if!佞.is_empty(){丅.push(佞);}}丅}fn 丣(不:&J,上:Q,三:F,两:F,亨:O,乷:仯,)->P<K<L>>{let 丁=不.丝();let mut 丵=不.clone();let mut 万=K::new();let(x,y)=乘(两);if!产(&mut
丵,&mut 万,L::乹{上,x,y}){return None;}if 亨&&乷.guard!=0{let(仝,仞)=乘(三);let 侟=if 乷.guard==1{L::亀{x:仝,y:仞}}else{L::举{丆:1,x:仝,y:仞,}};if!产(&mut 丵,&mut 万,侟){return
None;}}if 乷.偀&&伳(&丵,丁)>伳(不,丁){return None;}Some(万)}fn 偌(l:&J,京:&仂,乷:仯)->K<L>{let 丁=l.丝();let 並=l.map();let mut 不=l.clone();let mut 丅=K::new();let 丩=丩(l,1-丁);let
mut 伀:K<(Q,F)> =K::new();for&(上,丗)in&京.仜{let Some(三)=倄(&不,丁,上)else{continue;};if!不.乼().test(三){continue;}let 亨=丩.test(三);for 两 in 偉(&不,丁,三,丗){if let Some(万)=丣(&不,上,三,两,亨,乷){for
专 in&万{不.义(*专);}if 亨&&乷.guard==0{伀.push((上,三));}丅.extend(万);break;}}}for&(上,丗)in&京.仜{if 不.下(丁)<俬{break;}let Some(乜)=倄(&不,丁,上)else{continue;};let 働=伀.iter().find(|&&(u,_)|u==上).map(|&(_,c)|c);let
丒=match 働{Some(q)if 乷.倒=>vec![q],_=>偉(&不,丁,乜,丗),};for 两 in 丒{let(x,y)=乘(两);if 产(&mut 不,&mut 丅,L::举{丆:1,x,y}){break;}}}if 乷.侷{for&(_,丗)in&京.亡{if 不.下(丁)<俬{break;}let
乴=不.七(丁).丏()&並.valid;let mut 丒:K<F> =乴.iter().filter(|&c|並.业(c,丗)!=I::MAX).collect();丒.sort_by_key(|&c|(並.业(c,丗),c));for 两 in 丒{let(x,y)=乘(两);if 产(&mut
不,&mut 丅,L::举{丆:1,x,y}){break;}}}}丅}const 俭:I=0;const 傹:I=1;const 俠:C=0;const 備:O=false;const 偹:I=0;const 偰:O=false;const 傪:O=false;const 偤:O=false;const
偵:Q=7;const 傻:Q=14;const 傳:O=俭!=0||俠!=0;struct 们{世:K<L>,倯:O,start:O,仌:K<K<L>>,乪:K<(F,C,C)>,伈:C,}impl 们{fn 兄(&self,l:&J,丁:F)->C{if self.伈==0{return 0;}let(丹,丙)=(l.七(丁),l.七(1-丁));let
mut 亇=0;for&(q,中,些)in&self.乪{亇+=if 丹.test(q){中}else if 丙.test(q){-中}else{些};}self.伈*亇/4}}fn 九(l:&J,东:&mut 伔)->P<们>{let 伪=l.乨();if!傳||!(偵..=傻).contains(&伪)||l.丢().is_some(){return
None;}僻(l,东,催)}#[derive(Clone,Copy)]struct 佃{仈:I,乷:仯,侉:O,伈:C,侳:O,}const 催:佃=佃{仈:俭,乷:仯{侷:傹==2,guard:偹,倒:偰,偀:傪,},侉:偤,伈:俠,侳:備,};fn 僻(l:&J,东:&mut 伔,么:佃)->P<们>{let
佃{仈,乷,侉,伈,侳,}=么;let 丁=l.丝();let 乪=乪(l,丁);let 仦=儢(l,丁,&乪);let 京=if 侉{let 侘=l.丈(丁).丕(丩(l,1-丁));俹(l,丁,&乪,&仦,东,侘)}else{仳(l,丁,&乪,&仦,东)};let 世=if 仈!=0{偌(l,&京,乷)}else{K::new()};let
仅=仅(&乪);let 充=仅.iter().map(|o|{let 亪=仦.iter().find(|r|r.q==o.q).map_or(0,|r|r.亪.clamp(-1,1));let 些=if 侳{o.value*亪}else{0};(o.q,o.value,些)}).collect();let
儘=if 仈&4!=0{仌(l,&京,乷)}else{K::new()};Some(们{仌:儘,倯:!世.is_empty()&&仈&1!=0,start:!世.is_empty()&&仈&2!=0,世,乪:充,伈,})}const 偦:亮=45.0;const 偱:亮=800.0;const 傯:亮=0.6;fn
乔(乁:乸,且:Q)->亱{if let Some(伦)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return 亱{于:None,仟:None,伦,};}let 佤=兊().min(100.0);let
儌=if 且==1{偱}else{偦};let 乏=Duration::from_secs_f64(儌/1000.0*佤);亱{于:Some(乁+乏.mul_f64(傯)),仟:Some(乁+乏),伦:丮::MAX,}}const 傛:F=9;const 傗:什=什{丒:3,丠:1,下:1,串:2,二:2,伵:2,丩:6,仫:1,丯:0,仳:false,乕:傜,};const
傟:Q=0;const 傜:亂=亂::佴{仴:8};const 俤:I=I::MAX;const 俢:C=2;const 俣:C=18;const 信:C=0;const 傚:F=32;const 偢:O=false;const 偣:Settings1=Settings1{侻:3,侺:1,侑:4,仇:2,侙:2,位:2,係:true,reserve:true,侚:true,乥:仁::乺,fill:true,乚:Z{佣:俤,件:俢,佔:俣,佅:信,..Z::仕},};fn
了()->亦{if 偢{return 亦::new(五::伻(偣),K::new(),Z::仕);}儕(傚)}fn 儕(亣:F)->亦{let 亟=五::伖{个:傛,亣,临:傗,佡:傟,};let 丰=傫.iter().map(|&乚|Z{佣:俤,件:俢,佔:俣,佅:信,..乚}).collect();亦::new(亟,丰,Z::仕)}fn
main(){let 儹=io::stdin();let 儺=io::stdout();if let Err(仚)=儜(&乔,了(),儹.lock(),儺.lock()){eprintln!("plan: {}",仚);std::process::exit(1);}}const 傷:&str="CG_TIME_SCALE";fn
兊()->亮{儗(std::env::var(傷).ok().as_deref())}fn 儗(中:P<&str>)->亮{中.and_then(|亐|亐.trim().parse::<亮>().ok()).filter(|佤|佤.is_finite()&&*佤>0.0).unwrap_or(1.0)}const
_:()=assert!(偯==16);const 偯:F=16;const 偩:亮=0.4;const 傒:I=4;#[derive(Clone,Copy)]struct 他{丯:G,倅:G,伧:G,}impl 他{fn of(l:&J)->他{let(a,伜)=倳(l,T[0]);let(b,佌)=倳(l,T[1]);let
亇=a[T[1]];let 億=伜[T[1]];let mut 乃=他{丯:G::U,倅:G::U,伧:G::U,};for q in l.map().valid.iter(){if 伜[q]==I::MAX||佌[q]==I::MAX{continue;}if 伜[q].abs_diff(佌[q])<=1{乃.倅.set(q);}if
伜[q].abs_diff(佌[q])<=傒{乃.伧.set(q);}let 儑=伺::from(伜[q])+伺::from(佌[q])==伺::from(億);if 亇>0.0&&儑&&q!=T[0]&&q!=T[1]&&a[q]*b[q]/亇>=偩{乃.丯.set(q);}}乃}fn 仢(&self)->G{self.丯&self.伧}}fn
儼(l:&J,儷:G)->[I;M]{let 丬=l.map().valid;let mut 丅=[I::MAX;M];let mut 乯=儷&丬;let mut 仿=乯;let mut 丣=0u8;while!仿.is_empty(){for q in 仿.iter(){丅[q]=丣;}let 丄=(仿.丏()&丬).丕(乯);乯|=丄;仿=丄;丣=丣.saturating_add(1);}丅}fn
倳(l:&J,三:F)->([亮;M],[I;M]){let 久=儼(l,G::q(三));let mut 乕:K<F> =l.map().valid.iter().filter(|&c|久[c]!=I::MAX).collect();乕.sort_by_key(|&c|久[c]);let mut 丛=[0.0;M];丛[三]=1.0;for
q in 乕{if q==三{continue;}let 乢=G::q(q).丏()&l.map().valid;丛[q]=乢.iter().filter(|&n|久[n]!=I::MAX&&久[n]+1==久[q]).map(|n|丛[n]).sum();}(丛,久)}const Y:F=12;const
M:F=Y*Y;const 予:F=4*Y;const 亥:丮=(1<<予)-1;const 似:丮=1|1<<12|1<<24|1<<36;const 佰:丮=似<<11;const 俦:丮=(1<<Y)-1;#[derive(Clone,Copy,Debug,PartialEq)]struct G([丮;3]);impl
G{const U:G=G([0;3]);const 偡:G=G([亥;3]);#[inline]fn q(q:F)->G{let mut 乓=G::U;乓.set(q);乓}#[inline]fn test(self,q:F)->O{(self.0[q/予]>>(q%予))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/予]|=1<<(q%予);}#[inline]fn clear(&mut self,q:F){self.0[q/予]&=!(1<<(q%予));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伷(self)->[丮;3]{self.0}#[inline]fn count(self)->Q{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丕(self,乀:G)->G{G([self.0[0]&!乀.0[0],self.0[1]&!乀.0[1],self.0[2]&!乀.0[2],])}#[inline]fn
iter(self)->佮{佮{侩:self.0,乜:0,}}#[inline]fn 佦(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&俦)<<(予-Y)),(b>>Y)|((c&俦)<<(予-Y)),c>>Y,])}#[inline]fn 佥(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亥,((b<<Y)&亥)|(a>>(予-Y)),((c<<Y)&亥)|(b>>(予-Y)),])}#[inline]fn
偃(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佰&亥,(b>>1)&!佰&亥,(c>>1)&!佰&亥,])}#[inline]fn 偄(self)->G{let[a,b,c]=self.0;G([(a<<1)&!似&亥,(b<<1)&!似&亥,(c<<1)&!似&亥,])}#[inline]fn
丏(self)->G{let 一=self;一.佦()|一.佥()|一.偃()|一.偄()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,乀:G)->G{G([self.0[0]&乀.0[0],self.0[1]&乀.0[1],self.0[2]&乀.0[2],])}}impl
BitAndAssign for G{#[inline]fn bitand_assign(&mut self,乀:G){*self=*self&乀;}}impl BitOr for G{type Output=G;#[inline]fn bitor(self,乀:G)->G{G([self.0[0]|乀.0[0],self.0[1]|乀.0[1],self.0[2]|乀.0[2],])}}impl
BitOrAssign for G{#[inline]fn bitor_assign(&mut self,乀:G){*self=*self|乀;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::偡.丕(self)}}struct 佮{侩:[丮;3],乜:F,}impl
Iterator for 佮{type Item=F;#[inline]fn next(&mut self)->P<F>{let 一=self;while 一.乜<3{let 余=一.侩[一.乜];if 余!=0{一.侩[一.乜]=余&(余-1);return Some(一.乜*予+余.trailing_zeros()as
F);}一.乜+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum L{乹{上:Q,x:I,y:I},举{丆:I,x:I,y:I},亯{x:I,y:I},亀{x:I,y:I},}impl L{fn 免(&self)->乛{match*self{L::乹{上,x,y}=>format!("MOVE {} {x} {y}",上),L::举{丆,x,y}=>format!("TRAIN {} {x} {y}",丆),L::亯{x,y}=>format!("BUILD MINE {x} {y}"),L::亀{x,y}=>format!("BUILD TOWER {x} {y}"
),}}}fn 僈(万:&[L])->乛{if 万.is_empty(){return "WAIT".to_string();}let mut line=乛::with_capacity(16*万.len());for(丟,专)in 万.iter().enumerate(){if 丟>0{line.push(';');}line.push_str(&专.免());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
交{俲,兞,僄,#[default]佯,}impl 交{fn 亚(self)->I{match self{交::俲=>1,_=>3,}}fn 但(self)->O{matches!(self,交::僄|交::佯)}fn 仲(self)->O{self==交::佯}}const 亓:I=I::MAX;struct
亲{乤:G,valid:G,丼:G,业:Box<[I]>,丏:[[[I;4];M];2],}thread_local!{static 傆:RefCell<P<Arc<亲>>> =const{RefCell::new(None)};}impl 亲{fn 儲(乤:G,丼:G)->Arc<亲>{傆.with(|乻|{let
mut 乻=乻.borrow_mut();if let Some(並)=乻.as_ref(){if 並.乤==乤&&並.丼==丼{return Arc::clone(並);}}let 並=Arc::new(亲::build(乤,丼));*乻=Some(Arc::clone(&並));並})}fn build(乤:G,丼:G)->亲{let
丬=!乤;const 傲:[[(伹,伹);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[亓;4];M];2];for(丁,丷)in 傲.iter().enumerate(){for q in
丬.iter(){let(x,y)=((q%Y)as 伹,(q/Y)as 伹);let mut 丛=0;for&(僤,僥)in 丷{let(倫,倬)=(x+僤,y+僥);if(0..Y as 伹).contains(&倫)&&(0..Y as 伹).contains(&倬){let 丄=倬 as F*Y+倫
as F;if 丬.test(丄){丏[丁][q][丛]=丄 as I;丛+=1;}}}}}let mut 业=vec![亓;M*M].into_boxed_slice();let mut 侲=[0u8;M];for 三 in 丬.iter(){let 乾=&mut 业[三*M..(三+1)*M];乾[三]=0;侲[0]=三
as I;let(mut 侠,mut 俁)=(0,1);while 侠<俁{let q=侲[侠]as F;侠+=1;for&丄 in&丏[0][q]{if 丄!=亓&&乾[丄 as F]==亓{乾[丄 as F]=乾[q]+1;侲[俁]=丄;俁+=1;}}}}亲{乤,valid:丬,丼,业,丏,}}#[inline]fn
业(&self,三:F,两:F)->I{self.业[三*M+两]}}const 傐:Q=100;const 互:C=15;const 傽:[C;3]=[1,4,20];const T:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 乵{井(F),俑,}enum
亁{佽,佹,佺,}#[derive(Clone)]struct J{map:Arc<亲>,丿:交,丘:[G;2],七:[G;2],丹:G,乭:G,丈:[G;2],丆:[G;3],今:G,云:[Q;M],下:[C;2],丠:[C;2],丝:F,乨:Q,伄:Q,丢:P<乵>,仡:O,}fn 倚(mut 乫:G,丘:G)->G{loop{let
丄=(乫|乫.丏())&丘;if 丄==乫{return 乫;}乫=丄;}}fn 亊(丘:G,任:F)->G{if!丘.test(任){return G::U;}let mut 乫=G::q(任);loop{let 丄=(乫|乫.丏())&丘;if 丄==乫{return 乫;}乫=丄;}}impl
J{fn 丝(&self)->F{self.丝}fn 乨(&self)->Q{self.乨}fn 丢(&self)->P<乵>{self.丢}fn 丿(&self)->交{self.丿}fn 伃(&self,丁:F)->C{let 一=self;let 丈=一.丈[丁];一.下[丁]+10*C::from((丈&一.丆[0]).count())+20*C::from((丈&一.丆[1]).count())+30*C::from((丈&一.丆[2]).count())}fn
下(&self,丁:F)->C{self.下[丁]}fn 丠(&self,丁:F)->C{self.丠[丁]}fn 俄(&self,丁:F)->F{self.丈[丁].count()as F}fn map(&self)->&亲{&self.map}fn 丐(&self,丁:F)->G{self.丘[丁]}fn
七(&self,丁:F)->G{self.七[丁]}fn 丈(&self,丁:F)->G{self.丈[丁]}fn 亾(&self,q:F)->P<(Q,I)>{let 一=self;(一.丈[0]|一.丈[1]).test(q).then(||(一.云[q],一.侨(q)))}fn 乼(&self)->G{self.丈[self.丝]&self.今}fn
为(&self)->G{self.丹}fn 乗(&self)->G{self.乭}fn 亢(&self,q:F)->P<I>{(q<M&&(self.丈[0]|self.丈[1]).test(q)).then(||self.侨(q))}fn 云(&self,q:F)->P<Q>{(q<M&&(self.丈[0]|self.丈[1]).test(q)).then(||self.云[q])}fn
儸(&mut self,丁:F){let 一=self;if 一.仡{debug_assert_eq!(一.七,[亊(一.丘[0],T[0]),亊(一.丘[1],T[1])]);}else{一.七=[亊(一.丘[0],T[0]),亊(一.丘[1],T[1])];一.仡=true;}let 僞=一.丈[丁].丕(一.七[丁]);一.侴(僞);一.倈();一.下[丁]+=一.丠[丁];if
一.下[丁]<0{一.下[丁]=0;一.侴(一.丈[丁]);}一.今|=一.丈[丁];}fn 倈(&mut self){let 一=self;for 丁 in 0..2{let 七=一.七[丁];let 丈=七&一.丈[丁];let mut 丠=C::from(七.count())+4*C::from((七&一.丹).count());for(丆,兓)in
傽.iter().enumerate(){丠-=兓*C::from((丈&一.丆[丆]).count());}一.丠[丁]=丠;}}fn 仸(&mut self){let 一=self;if 一.丢.is_some(){return;}if 一.丝==1{if 一.乨>=傐{一.丢=Some(match
一.伃(0).cmp(&一.伃(1)){std::cmp::Ordering::Greater=>乵::井(0),std::cmp::Ordering::Less=>乵::井(1),std::cmp::Ordering::Equal=>乵::俑,});return;}一.乨+=1;}一.丝=1-一.丝;一.儸(一.丝);}fn
佟(&mut self,万:&[L]){let 一=self;if 一.丢.is_some(){return;}let 依=一.伄;for&专 in 万{if 一.僣(专,依){continue;}一.义(专);if 一.丢.is_some(){return;}}一.仸();}fn 僣(&self,专:L,依:Q)->O{let
一=self;match 专{L::乹{上,x,y}=>{一.仵(x,y).is_none()||上>=依||一.倓(一.丝,上).is_none()}L::举{丆,x,y}=>{一.仵(x,y).is_none()||丆==0||丆>一.丿.亚()}L::亯{x,y}|L::亀{x,y}=>{一.仵(x,y).is_none()}}}#[inline]fn
仵(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<Y&&y<Y&&!self.map.乤.test(y*Y+x)).then_some(y*Y+x)}fn 倓(&self,丁:F,上:Q)->P<F>{self.丈[丁].iter().find(|&q|self.云[q]==上)}#[inline]fn
侨(&self,q:F)->I{if self.丆[0].test(q){1}else if self.丆[1].test(q){2}else{3}}fn 像(&self,亜:F)->G{let 乗=self.乭&self.丘[亜];let 七=self.七[亜];乗|(七&(七&乗).丏())}fn
乍(&self,丁:F,丆:I)->G{let 一=self;debug_assert!((1..=3).contains(&丆),"unit level {} is not 1 to 3",丆);let 亜=1-丁;let 丈=一.丈[0]|一.丈[1];let 僨=一.仙();let 兑=match
丆{1 if 一.丿==交::俲=>一.丆[1]|一.丆[2],1=>丈,2=>一.丆[1]|一.丆[2],_=>G::U,};let 乍=僨|一.丘[亜].丕(兑);if 丆<3{乍.丕(一.像(亜))}else{乍}}fn 介(&self,丁:F)->G{self.七[丁]|self.七[丁].丏()}fn
仙(&self)->G{let 一=self;let 丈=一.丈[0]|一.丈[1];let 僾=G::q(T[0])|G::q(T[1]);一.map.valid.丕(丈|一.丹|一.乭|僾)}fn 义(&mut self,专:L)->O{let 一=self;if 一.丢.is_some(){return
false;}let 丁=一.丝;let 仗=match 专{L::举{丆,x,y}=>一.佨(丁,丆,x,y),L::乹{上,x,y}=>一.儋(丁,上,x,y),L::亯{x,y}=>一.build(丁,true,x,y),L::亀{x,y}=>一.build(丁,false,x,y),};let
Some(仗)=仗 else{return false;};match 仗{亁::佽=>{}亁::佹 if 一.仡=>{一.七[丁]=倚(一.七[丁],一.丘[丁]);}亁::佺 if 一.仡=>{一.七[丁]=倚(一.七[丁],一.丘[丁]);一.七[1-丁]=亊(一.丘[1-丁],T[1-丁]);}亁::佹=>一.七[丁]=亊(一.丘[丁],T[丁]),亁::佺=>{一.七=[亊(一.丘[0],T[0]),亊(一.丘[1],T[1])];}}debug_assert!(!一.仡||一.七==[亊(一.丘[0],T[0]),亊(一.丘[1],T[1])]);一.倈();if!一.丘[0].test
(T[0]){一.丢=Some(乵::井(1));}else if!一.丘[1].test(T[1]){一.丢=Some(乵::井(0));}true}fn 侴(&mut self,丒:G){let 一=self;一.丈[0]=一.丈[0].丕(丒);一.丈[1]=一.丈[1].丕(丒);for 丆
in&mut 一.丆{*丆=丆.丕(丒);}一.今=一.今.丕(丒);}fn 偐(&mut self,丁:F,q:F)->亁{let 一=self;一.侴(G::q(q));一.丹.clear(q);一.乭.clear(q);let 亜=1-丁;if 一.丘[亜].test(q){一.丘[亜].clear(q);一.丘[丁].set(q);亁::佺}else
if 一.丘[丁].test(q){亁::佽}else{一.丘[丁].set(q);亁::佹}}fn 佨(&mut self,丁:F,丆:I,x:I,y:I)->P<亁>{let 一=self;if 丆==0||丆>一.丿.亚(){return None;}let q=一.仵(x,y)?;let 丌=10*C::from(丆);if
一.下[丁]<丌||!一.介(丁).test(q)||!一.乍(丁,丆).test(q){return None;}let 仗=一.偐(丁,q);一.倴(丁,丆,q,一.伄);一.伄=一.伄.saturating_add(1);一.下[丁]-=丌;Some(仗)}fn 倴(&mut self,丁:F,丆:I,q:F,上:Q){self.丈[丁].set(q);self.丆[F::from(丆)-1].set(q);self.云[q]=上;}fn
儋(&mut self,丁:F,上:Q,x:I,y:I)->P<亁>{let 一=self;let 与=一.仵(x,y)?;let 三=一.倓(丁,上)?;if!一.今.test(三){return None;}let 丆=一.侨(三);let 丨=一.乍(丁,丆);let 乢=一.map.丏[丁][三];if
乢.contains(&(与 as I))&&!丨.test(与){return None;}let mut 丑=一.map.业(三,与);let mut 伞=None;for&丄 in 乢.iter().take_while(|&&丄|丄!=亓){let 丄=F::from(丄);let 业=一.map.业(丄,与);if
业<丑&&丨.test(丄){丑=业;伞=Some(丄);}}let 伞=伞?;let 仗=一.偐(丁,伞);一.丈[丁].clear(三);一.丆[F::from(丆)-1].clear(三);一.今.clear(三);一.倴(丁,丆,伞,上);Some(仗)}fn build(&mut self,丁:F,丹:O,x:I,y:I)->P<亁>{let
一=self;let 來=if 丹{一.丿.但()}else{一.丿.仲()};let q=一.仵(x,y)?;let 儒=一.map.丼.test(q);if!來||一.下[丁]<互||儒!=丹||!一.丘[丁].test(q)||!一.介(丁).test(q)||!一.仙().test(q){return
None;}if 丹{一.丹.set(q);}else{一.乭.set(q);}一.下[丁]-=互;Some(亁::佽)}}const 傋:C=1000000000;const 佷:C=0;const 傓:C=1;const 傸:C=2;impl J{fn read(仼:&str,且:&str,丿:交,乨:Q,)->Result<J,乛>{let
mut 丼=K::new();let mut 乌=仼.lines();let 丛=侕(&mut 乌,"mine spot count")?;for _ in 0..丛{let[x,y]=亻(伅(&mut 乌,"a mine spot")?,"a mine spot")?;丼.push((佊(x)?,佊(y)?));}倐(乌,"the initial input")?;let
mut 乌=且.lines();let mut 価=[0i64;4];for(中,也)in 価.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[侰]=亻(伅(&mut 乌,也)?,也)?;if 侰.abs()>傋{return
Err(format!("{}: {} is out of range",也,侰));}*中=侰;}let mut 乤=G::U;let mut 丒=[G::U;2];let mut 七=[G::U;2];for y in 0..Y{let 乾=伅(&mut 乌,"a map row")?.trim();if
乾.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,乾));}for(x,伱)in 乾.bytes().enumerate(){let q=y*Y+x;match 伱{b'#'=>乤.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
买=F::from(伱==b'X'||伱==b'x');丒[买].set(q);if 伱.is_ascii_uppercase(){七[买].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伱 as char))}}}}let
僒=侕(&mut 乌,"building count")?;let mut 亄=K::new();for _ in 0..僒{let 亻=亻(伅(&mut 乌,"a building")?,"a building")?;亄.push(亻);}let 俄=侕(&mut 乌,"unit count")?;let
mut 丈=K::new();for _ in 0..俄{let 亻=亻(伅(&mut 乌,"a unit")?,"a unit")?;丈.push(亻);}倐(乌,"the turn's input")?;let 倱=亄.iter().find(|俿|俿[0]==0&&俿[1]==佷).ok_or("the bot's own HQ is not among the buildings")?;let
倞=侍(倱[2],倱[3],乤)?;let 丁=T.iter().position(|&任|任==倞).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",倞))?;let mut 偅=G::U;for(x,y)in 丼{if
乤.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}偅.set(y*Y+x);}let 侄=|买:F|if 买==0{丁}else{1-丁};let mut l=J{map:亲::儲(乤,偅),丿,丘:[G::U;2],七:[G::U;2],丹:G::U,乭:G::U,丈:[G::U;2],丆:[G::U;3],今:G::U,云:[0;M],下:[0;2],丠:[0;2],丝:丁,乨,伄:1,丢:None,仡:false,};for
买 in 0..2{let 亞=侄(买);l.丘[亞]=丒[买];l.七[亞]=七[买];l.下[亞]=価[2*买];l.丠[亞]=価[2*买+1];}let mut 倮=G::U;for[亞,佖,x,y]in 亄{let 买=债(亞)?;let q=侍(x,y,乤)?;if!丒[买].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 倮.test(q){return Err(format!("two buildings at ({x}, {y})"));}倮.set(q);match 佖{佷
if q==T[侄(买)]=>{}佷=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}傓=>l.丹.set(q),傸=>l.乭.set(q),_=>return Err(format!("unknown building type {}",佖)),}}let
mut 乲=G::U;let mut 佗=0u32;for[亞,上,丆,x,y]in 丈{let 买=债(亞)?;let q=侍(x,y,乤)?;if!丒[买].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乲.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丆){return Err(format!("unit level {} is not 1, 2 or 3",丆));}let 上=Q::try_from(上).map_err(|_|format!("unit id {} is out of range",上))?;if
乲.iter().any(|乀|l.云[乀]==上){return Err(format!("two units have id {}",上));}乲.set(q);l.丈[侄(买)].set(q);l.丆[丆 as F-1].set(q);l.云[q]=上;佗=佗.max(上);}if!乲.is_empty(){l.伄=佗.checked_add(1).ok_or_else(||format!("unit id {} is out of range",佗))?;}l.今=l.丈[丁];l.仡=l.七==[亊(l.丘[0],T[0]),亊(l.丘[1],T[1])];Ok(l)}}fn
伅<'a>(乌:&mut Lines<'a>,也:&str)->Result<&'a str,乛>{乌.next().ok_or_else(||format!("the text ends before {}",也))}fn 倐(mut 乌:Lines<'_>,也:&str)->Result<(),乛>{match
乌.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",也)),None=>Ok(()),}}fn 亻<const N:F>(line:&str,也:&str)->Result<[C;N],乛>{let
mut 伷=line.split_whitespace();let mut 偙=[0i64;N];for 中 in&mut 偙{let 伎=伷.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",也))?;*中=伎.parse().map_err(|_|format!("{}: {:?} is not an integer",也,伎))?;}if
伷.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",也));}Ok(偙)}fn 侕(乌:&mut Lines<'_>,也:&str)->Result<F,乛>{let[丛]=亻(伅(乌,也)?,也)?;F::try_from(丛).ok().filter(|&丛|丛<=M).ok_or_else(||format!("{}: {} is out of range",也,丛))}fn
佊(中:C)->Result<F,乛>{F::try_from(中).ok().filter(|&中|中<Y).ok_or_else(||format!("coordinate {} is off the map",中))}fn 侍(x:C,y:C,乤:G)->Result<F,乛>{let q=佊(y)?*Y+佊(x)?;if
乤.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 债(亞:C)->Result<F,乛>{match 亞{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亞)),}}#[derive(Clone,Copy,PartialEq)]enum
仁{乺,俊,}#[derive(Clone,Copy)]struct Settings1{侻:I,侺:I,侑:C,仇:C,侙:F,位:I,係:O,reserve:O,侚:O,乥:仁,fill:O,乚:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{侻:3,侺:1,侑:4,仇:2,侙:2,位:2,係:true,reserve:true,侚:true,乥:仁::乺,fill:true,乚:Z::仕,};}const
侃:C=1000;const 侂:C=10;fn 俇(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侱(l:&mut J,专:L,丅:&mut Vec<L>)->O{let 产=l.义(专);if 产{丅.push(专);}产}fn 便(l:&mut J,三:F,两:F,丅:&mut
Vec<L>)->O{let Some((上,_))=l.亾(三)else{return false;};let(x,y)=俇(两);侱(l,L::乹{上,x,y},丅)}fn 元(l:&J,乃:&他,么:&Settings1,乔:F,)->K<F>{let 丂=l.丝();let 严=丳(l,l.丐(丂));let
丙=丳(l,l.丐(1-丂));let 些=l.map().valid.丕(l.丐(0)|l.丐(1));let 倸=|c:F|严[c]!=I::MAX&&丙[c]!=I::MAX;let mut 丯:K<F> =乃.仢().丕(l.丐(丂)).iter().filter(|&c|倸(c)).collect();丯.sort_by_key(|&c|(严[c],c));let
mut 之:K<F> =些.iter().filter(|&c|倸(c)&&严[c].abs_diff(丙[c])<=么.侺).collect();之.sort_by_key(|&c|(严[c].abs_diff(丙[c]),严[c],c));let mut 丅:K<F> =K::new();for
q in 丯.into_iter().chain(之){if 丅.len()>=乔{break;}let 仛=丅.iter().all(|&t|l.map().业(t,q)>=么.侻);let 僗=乃.仢().test(q);if!丅.contains(&q)&&(僗||仛){丅.push(q);}}丅}fn
僿(丌:&[K<C>])->K<F>{let n=丌.len();let 倢=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 偛=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亹=0usize;let mut 伨=vec![倢;n+1];let mut 俅=vec![false;n+1];loop{俅[亹]=true;let 倠=p[亹];let mut 伝=倢;let mut 伤=0usize;for j in 1..=n{if!俅[j]{let
乄=丌[倠-1][j-1]-u[倠]-v[j];if 乄<伨[j]{伨[j]=乄;偛[j]=亹;}if 伨[j]<伝{伝=伨[j];伤=j;}}}for j in 0..=n{if 俅[j]{u[p[j]]+=伝;v[j]-=伝;}else{伨[j]-=伝;}}亹=伤;if p[亹]==0{break;}}loop{let
伤=偛[亹];p[亹]=p[伤];亹=伤;if 亹==0{break;}}}let mut 丅=vec![0usize;n];for j in 1..=n{if p[j]>0{丅[p[j]-1]=j-1;}}丅}#[derive(Clone,Copy)]enum 仒{众(F),侁,}fn 乴(l:&J)->G{let
丂=l.丝();(l.介(丂)&l.乍(丂,1)).丕(l.丐(丂))}fn 儚(l:&J,乚:&Z)->F{佩(l,乚,乙::乺).iter().filter(|c|matches!(c,L::举{..})).count()}fn 両(q:F,丧:G)->Q{(G::q(q).丏()&丧).count()}fn
倖(l:&J,乐:G,与:F,侦:I)->K<[C;M]>{let 丬=l.map().valid;let mut 丑=[C::MIN;M];丑[与]=0;let mut 乩=vec![丑];for _ in 0..侦{let mut 丄=丑;for q in 丬.iter(){for n in(G::q(q).丏()&丬).iter(){if
丑[n]>C::MIN{let 中=丑[n]+C::from(乐.test(n));if 中>丄[q]{丄[q]=中;}}}}丑=丄;乩.push(丑);}乩}fn 倲(l:&J,乩:&[[C;M]],乐:G,三:F,k:F,)->G{let 丬=l.map().valid;let mut 丅=G::U;let
mut q=三;let mut k=k.min(乩.len()-1);while k>0&&乩[k][q]>0{let Some(丄)=(G::q(q).丏()&丬).iter().filter(|&n|乩[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(乐.test(n))+乩[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乐.test(丄){丅.set(丄);}q=丄;k-=1;}丅}fn 且(l:&J,么:&Settings1)->K<L>{let 丂=l.丝();let 乃=他::of(l);let mut 不=l.clone();let mut 丅=K::new();let 丙=丳(l,l.丐(1-丂));let
丈:K<F> =l.乼().iter().collect();let 倘=(l.下(丂)/侂).max(0)as F;let 亡=match 么.乥{仁::乺=>儚(l,&么.乚).min(倘),仁::俊=>倘,};let mut 优:K<仒> =丈.iter().map(|&c|仒::众(c)).collect();优.extend(std::iter::repeat_n(仒::侁,亡));let
亅=元(l,&乃,么,优.len()+么.侙);let 兏=乴(l);let 丯=乃.仢();let n=优.len().max(亅.len());let mut 丌=vec![vec![0i64;n];n];for(乾,&与)in 亅.iter().enumerate(){for(伛,从)in 丌[乾].iter_mut().enumerate(){let
Some(&僇)=优.get(伛)else{*从=侃;continue;};let 佇=match 僇{仒::众(q)=>{let d=l.map().业(q,与);if d==I::MAX{None}else{Some(C::from(d)-1)}}仒::侁=>兏.iter().map(|c|l.map().业(c,与)).filter(|&d|d!=I::MAX).min().map(C::from),};*从=match
佇{None=>侃,Some(佇)=>{let 丸=C::from(丙[与])-1;let 仇=么.仇*(佇-丸).max(0);let 侊=if 丯.test(与){么.侑}else{0};(佇+仇-侊).max(0)}};}}let 僉=僿(&丌);let mut 侞:K<(F,F)> =K::new();let
mut 促:K<(C,F)> =K::new();for(乾,&与)in 亅.iter().enumerate(){let 伛=僉[乾];if 丌[乾][伛]>=侃{continue;}match 优.get(伛){Some(仒::众(q))=>侞.push((*q,与)),Some(仒::侁)=>促.push((丌[乾][伛],与)),None=>{}}}let
mut 丧=l.map().valid.丕(l.丐(0)|l.丐(1));let mut 倩:K<F> =K::new();let mut 佢=G::U;let 亝=|q:F|{let 丆=l.亢(q).unwrap_or(1);let 伲=丧|l.丐(1-丂);(G::q(q).丏()&伲&l.乍(丂,丆)).count()};侞.sort_by_key(|&(q,与)|{let
僮=if 么.侚{亝(q)}else{0};(僮,l.map().业(q,与),q)});for&(三,与)in&侞{let Some(丆)=不.亢(三)else{continue;};let 丨=不.乍(丂,丆);let d=不.map().业(三,与);let 乐=(丧|不.丐(1-丂)).丕(佢);let
侦=d.saturating_add(么.位);let 乩=倖(&不,乐,与,侦.saturating_sub(1));let 仱=乩[乩.len()-1];let 丄=(G::q(三).丏()&丨).iter().filter(|&c|仱[c]>C::MIN).max_by_key(|&c|{let
僚=d.saturating_sub(不.map().业(c,与));let 伲=丧.test(c)||不.丐(1-丂).test(c);(伲&&么.係,C::from(乐.test(c))+仱[c],僚,両(c,丧),std::cmp::Reverse(c),)}).filter(|&c|{丧.test(c)||不.丐(1-丂).test(c)||不.map().业(c,与)<d});if
let Some(两)=丄{if 便(&mut 不,三,两,&mut 丅){丧.clear(两);倩.push(三);if 么.reserve{佢|=倲(&不,&乩,乐,两,乩.len()-1);}}}}let 严=丳(l,l.丐(丂));let mut 丶=G::U;for c in 丧.iter(){if
严[c]<=丙[c]{丶.set(c);}}for&三 in&丈{if 倩.contains(&三)||!不.乼().test(三){continue;}let Some(丆)=不.亢(三)else{continue;};let 丨=不.乍(丂,丆);let 伲=(G::q(三).丏()&丧&丨).iter().max_by_key(|&c|(両(c,丧),丙[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(两)=伲{if 便(&mut 不,三,两,&mut 丅){丧.clear(两);}continue;}let 丗=[丶&丧,丧].into_iter().find_map(|丒|{丒.iter().min_by_key(|&c|(不.map().业(三,c),c))});if let
Some(丗)=丗{let d=不.map().业(三,丗);let 丄=(G::q(三).丏()&丨).iter().filter(|&c|不.map().业(c,丗)<d).min_by_key(|&c|c);if let Some(两)=丄{便(&mut 不,三,两,&mut 丅);}}}促.sort();let
mut 侫=0usize;for&(_,与)in&促{if 不.下(丂)<侂{break;}let 丒=乴(&不);let Some(乇)=丒.iter().map(|c|不.map().业(c,与)).min()else{break;};let 乐=(丧|不.丐(1-丂)).丕(佢);let 乩=倖(&不,乐,与,乇.saturating_add(么.位));let
仱=乩[乩.len()-1];let q=丒.iter().filter(|&c|不.map().业(c,与)==乇).max_by_key(|&c|(仱[c],両(c,丧),std::cmp::Reverse(c)));let Some(q)=q else{break;};let(x,y)=俇(q);if
侱(&mut 不,L::举{丆:1,x,y},&mut 丅){丧.clear(q);侫+=1;if 么.reserve{佢|=倲(&不,&乩,乐,q,乩.len()-1);}}}if 么.fill&&么.乥==仁::乺{while 侫<亡&&不.下(丂)>=侂{let q=(乴(&不)&丧).iter().max_by_key(|&c|{(両(c,丧),std::cmp::Reverse(丙[c]),std::cmp::Reverse(c),)});let
Some(q)=q else{break;};let(x,y)=俇(q);if!侱(&mut 不,L::举{丆:1,x,y},&mut 丅){break;}丧.clear(q);侫+=1;}}丅}const 偿:C=15;const 傔:C=6;const 修:C=10;const 傩:I=3;#[derive(Clone,Copy)]struct
Z{open:C,佉:C,個:I,侣:C,佔:C,倦:C,佣:I,倊:C,倉:C,佅:C,仺:C,伋:C,乣:C,份:C,仑:C,件:C,}impl Z{const 仕:Z=Z{open:3,佉:2,個:12,侣:4,佔:偿,倦:傔,佣:傩,倊:2,倉:2,佅:0,仺:0,伋:0,乣:0,份:0,仑:0,件:0,};const
fn 兜(self,仺:C,伋:C,乣:C)->Z{Z{仺,伋,乣,..self}}const fn 兛(self,份:C,仑:C)->Z{Z{份,仑,..self}}}fn 伸(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 丳(l:&J,三:G)->[I;M]{let mut
久=[I::MAX;M];let 丬=l.map().valid;let mut 之=三&丬;let mut 乲=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){久[q]=d;}let 丄=(之.丏()&丬).丕(乲);乲|=丄;之=丄;d=d.saturating_add(1);}久}fn
僶(l:&J)->I{let 久=丳(l,l.丐(0));l.丐(1).iter().map(|q|久[q]).min().unwrap_or(I::MAX)}struct 仃<'a>{乚:&'a Z,丂:F,theirs:[I;M],伌:[I;M],free:G,丶:G,ours:G,丼:G,丈:G,}impl
仃<'_>{fn new<'a>(l:&J,乚:&'a Z)->仃<'a>{let 丂=l.丝();let 丹=丳(l,l.丐(丂));let 丙=丳(l,l.丐(1-丂));let 伌=丳(l,G::q(T[1-丂]));let 丧=l.map().valid.丕(l.丐(0)|l.丐(1));let
mut 丶=G::U;for q in 丧.iter(){if 丹[q]<=丙[q]{丶.set(q);}}仃{乚,丂,theirs:丙,伌,free:丧,丶,ours:l.丐(丂),丼:l.map().丼,丈:l.丈(丂),}}fn value(&self,q:F)->C{let 一=self;let
p=一.乚;let 両=C::from((G::q(q).丏()&一.free).count());let 仴=p.個;let 佉=C::from(仴)-C::from(一.theirs[q].min(仴));let 任=if p.侣>0{C::from(一.伌[q].min(30))/p.侣}else{0};let
丘=C::from((G::q(q).丏()&一.ours).count());let line=if 丘<=1{p.伋}else{0};let 乣=if 一.丼.test(q){p.乣}else{0};let 儿=if 両==0{p.份}else{0};let 仑=p.仑*(22-C::from(一.伌[q].min(22)));let
件=p.件*C::from((G::q(q).丏()&一.丈).count());p.open*両+p.佉*佉-任+p.仺*丘-line+乣-儿+仑-件}fn 丑(&self,丒:G)->P<F>{丒.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
侒(&mut self,q:F){let 一=self;一.free.clear(q);一.丶.clear(q);一.ours.set(q);一.丈.set(q);}}fn 伭(l:&mut J,专:L,丅:&mut Vec<L>)->O{let 产=l.义(专);if 产{丅.push(专);}产}fn
偈(l:&mut J,三:F,两:F,丅:&mut Vec<L>)->O{let Some((上,_))=l.亾(三)else{return false;};let(x,y)=伸(两);伭(l,L::乹{上,x,y},丅)}fn 兒(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-修}fn
優(l:&J,乚:&Z)->K<L>{佩(l,乚,乙::乺)}#[derive(Clone,Copy,PartialEq)]enum 乙{乺,亰{为:I,丈:I},}fn 佩(l:&J,乚:&Z,乥:乙)->K<L>{let mut 不=l.clone();let mut 且=仃::new(l,乚);let
mut 丅=K::new();人(&mut 不,&mut 且,&mut 丅);match 乥{乙::乺=>儡(&mut 不,&mut 且,&mut 丅),乙::亰{为,丈}=>{僬(&mut 不,&mut 且,为,丈,&mut 丅)}}丅}fn 人(l:&mut J,且:&mut 仃<'_>,丅:&mut
Vec<L>){let 丂=且.丂;let mut 亸:K<F> =K::new();loop{let 丨=l.乍(丂,1);let mut 伇:P<(Q,F)> =None;for 三 in l.乼().iter(){if 亸.contains(&三){continue;}let 亝=(G::q(三).丏()&且.free&丨).count();if
亝==0{亸.push(三);}else if 伇.is_none_or(|(n,_)|亝<n){伇=Some((亝,三));}}let Some((_,三))=伇 else{break;};且.丈.clear(三);match 且.丑(G::q(三).丏()&且.free&丨){Some(两)if
偈(l,三,两,丅)=>且.侒(两),_=>{且.丈.set(三);亸.push(三);}}}for 三 in 亸{if!l.乼().test(三){continue;}let 与=[且.丶,且.free].into_iter().find_map(|丒|{丒.iter().min_by_key(|&q|(l.map().业(三,q),q))});if
let Some(与)=与{偈(l,三,与,丅);}}}fn 儡(l:&mut J,且:&mut 仃<'_>,丅:&mut Vec<L>){let 丂=且.丂;let p=且.乚;let h=(p.佔-C::from(l.乨())).max(p.倦);let 克=(C::from(僶(l))-p.倊).max(0)/p.倉.max(1)+p.佅;loop{let
下=l.下(丂);let 丈=l.俄(丂)as C;let 儩=C::from(且.丶.count());let q=且.丑(l.介(丂)&且.free&l.乍(丂,1));let 偖=兒(h,(儩/(丈+1)).min(克));let 倃=q.is_some()&&下>=修&&偖>0;let 倧=4*h-互;let
乣=倨(l,且);let 僕=乣.is_some()&&下>=互&&倧>0;if 僕&&(!倃||倧>=偖){let(x,y)=伸(乣.unwrap_or_default());if 伭(l,L::亯{x,y},丅){continue;}break;}let Some(q)=q.filter(|_|倃)else{break;};let(x,y)=伸(q);if!伭(l,L::举{丆:1,x,y},丅){break;}且.侒(q);}}fn
倨(l:&J,且:&仃<'_>)->P<F>{let 丂=且.丂;(l.map().丼&l.七(丂)&l.仙()).iter().filter(|&乣|且.theirs[乣]>且.乚.佣).min_by_key(|&乣|(l.map().业(T[丂],乣),乣))}fn 僬(l:&mut J,且:&mut
仃<'_>,为:I,丈:I,丅:&mut Vec<L>,){let 丂=且.丂;for _ in 0..为{let Some(乣)=倨(l,且)else{break;};let(x,y)=伸(乣);if!伭(l,L::亯{x,y},丅){break;}}for _ in 0..丈{let Some(q)=且.丑(l.介(丂)&且.free&l.乍(丂,1))else{break;};let(x,y)=伸(q);if!伭(l,L::举{丆:1,x,y},丅){break;}且.侒(q);}}fn
乯(l:&J)->O{if l.丢().is_some(){return false;}let 丂=l.丝();let 下=l.下(丂);let 丩=仮::new(l,丂,true);l.丐(1-丂).iter().any(|q|丩.丌[q]<=下)}#[derive(Clone,Copy)]struct
丽{乒:I,乥:乙,}const 俈:I=I::MAX;const 偧:[乙;8]=[乙::乺,乙::亰{为:0,丈:0},乙::亰{为:0,丈:1},乙::亰{为:0,丈:2},乙::亰{为:0,丈:俈,},乙::亰{为:1,丈:0},乙::亰{为:1,丈:1},乙::亰{为:1,丈:俈,},];struct
佲<'a>{丰:&'a[Z],仉:Z,rest:丽,亶:P<乸>,么:Settings2,}impl 佲<'_>{fn 乚(&self,丣:丽)->&Z{&self.丰[F::from(丣.乒)%self.丰.len()]}fn 万(&self,l:&J,丣:丽)->K<L>{佩(l,self.乚(丣),丣.乥)}}fn
僅(l:&J,万:&[L],仉:&Z)->(J,O){let mut 丄=l.clone();丄.佟(万);if 丄.丢().is_some()||乯(&丄){return(丄,true);}let 乬=優(&丄,仉);丄.佟(&乬);let 亼=丄.丢().is_some()||乯(&丄)||丄.乨()>俎;(丄,亼)}fn
儠(l:&J,世:&[丽],付:&佲<'_>)->K<丮>{let 丂=l.丝();let mut 伥=vec![估(l,丂)];let mut 乳=l.clone();for 且 in 0..俎 as F{let 丣=世.get(且).copied().unwrap_or(付.rest);let 万=付.万(&乳,丣);let(丄,亼)=僅(&乳,&万,&付.仉);if
亼{break;}乳=丄;伥.push(估(&乳,丂));}伥}fn 估(l:&J,丂:F)->丮{let mut hash:丮=0xcbf29ce484222325;let mut 侬=|伎:丮|hash=(hash^伎).wrapping_mul(0x0100000001b3);for(兂,乓)in[l.丐(丂),l.丈(丂),l.为()].into_iter().enumerate(){for
q in 乓.iter(){侬(((兂 as 丮)<<8)|q as 丮);}侬(0xff);}侬(l.下(丂)as 丮);hash}enum 五{伖{个:F,亣:F,临:什,佡:Q,},伻(Settings1),}struct 亦{亟:五,丰:K<Z>,仉:Z,世:K<丽>,住:K<丮>,以:F,主:Q,亼:O,}impl
亦{fn new(亟:五,丰:K<Z>,仉:Z)->亦{亦{亟,丰,仉,世:K::new(),住:K::new(),以:0,主:0,亼:false,}}fn 儝(&mut self,l:&J)->O{let 一=self;let 儵=matches!(一.亟,五::伖{..}|五::伻(_));let
償=!儵&&一.主>0&&一.以>=一.住.len();if!一.亼&&(償||乯(l)){一.亼=true;}!一.亼}fn end(&mut self){self.亼=true;}fn 且(&mut self,l:&J,亶:P<乸>)->K<L>{let 一=self;let 付=佲{丰:&一.丰,仉:一.仉,rest:丽{乒:0,乥:乙::乺,},亶,么:Settings2::DEFAULT,};if
let 五::伻(么)=一.亟{一.主+=1;一.以+=1;return 且(l,&么);}let 丂=l.丝();let 儓=一.住.get(一.以)==Some(&估(l,丂));let 僫=matches!(一.亟,五::伖{佡,..}if 佡>0&&l.乨()>=佡);if!儓||僫{let
乮=一.主==0;一.世=match 一.亟{五::伖{个,亣,临,..}=>{let 临=&临;let 乖=傣;let mut 丷=世(l,&一.丰,乖,个,亣,亶,临,);丷.resize(个,乖);丷}五::伻(_)=>K::new(),};一.住=match 一.亟{五::伖{..}=>儶(l,&一.丰,&一.世),_=>儠(l,&一.世,&付),};一.以=0;一.主+=1;}let
丣=一.世.get(一.以).copied().unwrap_or(付.rest);一.以+=1;付.万(l,丣)}}fn 儶(l:&J,丰:&[Z],世:&[丽])->K<丮>{let 丂=l.丝();let mut 伥=K::with_capacity(世.len());let mut 乳=l.clone();for&丣
in 世{if 乳.丢().is_some(){break;}伥.push(估(&乳,丂));乳=侅(&乳,&万(&乳,丰,丣));}伥}const 俎:Q=20;const 傺:C=3;#[derive(Clone,Copy)]struct Settings2{个:C,}impl Settings2{const
DEFAULT:Settings2=Settings2{个:傺};}fn 丷(l:&J,三:G)->[I;M]{let 丬=l.map().valid;let mut 久=[I::MAX;M];let mut 之=三&丬;let mut 乲=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){久[q]=d;}let 丄=(之.丏()&丬).丕(乲);乲|=丄;之=丄;d=d.saturating_add(1);}久}fn 兕(l:&J,丁:F)->C{let 严=丷(l,l.七(丁));let 丙=丷(l,l.七(1-丁));l.map().valid.iter().map(|q|match
严[q].cmp(&丙[q]){_ if 严[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 傫:[Z;27]=丰();const
傣:丽=丽{乒:16,乥:乙::乺,};const fn 丰()->[Z;27]{let 仺=[0,2,4];let 伋=[0,3,6];let 仑=[0,1,2];let mut 丅=[Z::仕;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
份=if g>0||t>0{4}else{0};丅[i]=Z::仕.兜(仺[g],伋[t],0).兛(份,仑[w]);i+=1;}丅}#[derive(Default)]struct 使{丒:C,丠:C,下:C,二:C,串:C,伵:C,丩:C,仫:C,}#[derive(Clone,Copy)]struct
什{丒:C,丠:C,下:C,串:C,二:C,伵:C,丩:C,仫:C,丯:C,仳:O,乕:亂,}#[derive(Clone,Copy,PartialEq)]enum 亂{俜,俐(佳),佴{仴:C},仓(仓),俍,}fn 兌(l:&J,丁:F)->[I;M]{let 严=丳(l,G::q(T[丁]));let
丙=丳(l,G::q(T[1-丁]));let mut 亩=G::U;let mut 丶=G::U;for c in l.map().valid.iter(){if 严[c]==I::MAX||丙[c]==I::MAX{continue;}if 严[c]==丙[c]{亩.set(c);}if 严[c]<=丙[c]{丶.set(c);}}let
d=丳(l,亩);let mut 丅=[I::MAX;M];for c in 丶.iter(){丅[c]=d[c];}丅}fn 僎(l:&J,丁:F,亩:&[I;M])->C{let d=丳(l,l.七(丁));(0..M).filter(|&c|亩[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&亩[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 仓{傞,}impl
仓{fn 俆(self,a:C,b:C,d:C)->C{let 仧=(a-b).abs()/2;match self{仓::傞=>a.min(d/2),}}}fn 僯(l:&J,丁:F,供:仓)->[C;M]{let 严=丳(l,G::q(T[丁]));let 丙=丳(l,G::q(T[1-丁]));let
d=C::from(严[T[1-丁]]);let mut 丅=[0;M];for c in 0..M{if 严[c]!=I::MAX&&丙[c]!=I::MAX{丅[c]=供.俆(C::from(严[c]),C::from(丙[c]),d);}}丅}#[derive(Clone,Copy,PartialEq)]enum
佳{傴(C),}impl 佳{fn 俆(self,仧:C)->C{match self{佳::傴(亠)=>(亠-仧).max(0).pow(2),}}}type 佀=[C;4];fn 儮(l:&J,丁:F)->[C;M]{struct 伐{七:G,乶:[伺;M],乞:[伺;M],size:[C;M],串:[C;M],仐:伺,}fn
仪(d:&mut 伐,v:F){d.仐+=1;d.乶[v]=d.仐;d.乞[v]=d.仐;d.size[v]=1;for w in(G::q(v).丏()&d.七).iter(){if d.乶[w]==0{仪(d,w);d.size[v]+=d.size[w];d.乞[v]=d.乞[v].min(d.乞[w]);if
d.乞[w]>=d.乶[v]{d.串[v]+=d.size[w];}}else{d.乞[v]=d.乞[v].min(d.乶[w]);}}}let mut d=伐{七:l.七(丁),乶:[0;M],乞:[0;M],size:[0;M],串:[0;M],仐:0,};let 乱=T[丁];if d.七.test(乱){仪(&mut
d,乱);}d.串[乱]=0;d.串}fn 儰(l:&J,丁:F)->使{let 七=l.七(丁);let 串=儮(l,丁);let 伌=丳(l,G::q(T[1-丁]));let mut 丅=使{丒:C::from(七.count()),丠:l.丠(丁),下:l.下(丁),丩:C::MAX,仫:兕(l,丁)/2,..使::default()};for
q in 七.iter(){if 串[q]>0{丅.二+=1;丅.串=丅.串.max(串[q]);}if q!=T[丁]&&(G::q(q).丏()&七).count()<=1{丅.伵+=1;}丅.丩=丅.丩.min(C::from(伌[q]));}丅}fn 兔(l:&J,丁:F,临:&什,伧:G)->C{let
仠=C::from((l.丐(丁)&伧).count());let s=儰(l,丁);let o=临;o.丒*s.丒+o.丠*s.丠+o.下*s.下-o.串*s.串-o.二*s.二-o.伵*s.伵-o.丩*s.丩.min(30)+o.仫*s.仫+o.丯*仠}fn 仢(l:&J)->G{他::of(l).仢()}fn
儉(l:&J)->G{let 严=丳(l,G::q(T[0]));let 丙=丳(l,G::q(T[1]));let mut 乊=G::U;for c in l.map().valid.iter(){if 严[c]!=I::MAX&&丙[c]!=I::MAX&&严[c].abs_diff(丙[c])<=2{乊.set(c);}}乊}fn
仨(l:&J)->[C;M]{let a=丳(l,G::q(T[0]));let b=丳(l,G::q(T[1]));let mut 丅=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丅[c]=C::from(a[c].abs_diff(b[c]))/2;}}丅}#[derive(Clone,Copy)]struct
仰{丯:G,乊:G,乒:[C;M],仨:[C;M],之:G,front:[I;M],}impl 仰{fn of(l:&J,临:&什)->仰{let 値=matches!(临.乕,亂::俜);let mut 乒=[0;M];if let 亂::俐(僟)=临.乕{for(w,仧)in 乒.iter_mut().zip(仨(l)){*w=if
仧==C::MAX{0}else{僟.俆(仧)};}}if let 亂::仓(供)=临.乕{乒=僯(l,l.丝(),供);}let(儨,之)=match 临.乕{亂::佴{..}=>{let r=仨(l);let mut f=G::U;for(c,&仧)in r.iter().enumerate(){if
仧==0{f.set(c);}}(r,f)}_=>([C::MAX;M],G::U),};仰{乒,仨:儨,之,front:if 临.乕==亂::俍{兌(l,l.丝())}else{[I::MAX;M]},丯:if 临.丯!=0||値{他::of(l).仢()}else{G::U},乊:if 値{儉(l)}else{G::U},}}}fn
仾(l:&J,丁:F,临:&什,乃:仰)->佀{let 佧=兔(l,丁,临,乃.丯);match 临.乕{亂::俐(_)|亂::仓(_)=>{let 七=l.七(丁);let 兘:C=七.iter().map(|c|乃.乒[c]).sum();[C::from(七.count()),兘,佧,0]}亂::佴{仴}=>{let
七=l.七(丁);let d=丳(l,七);let 仛:C=乃.之.iter().map(|c|C::from(d[c]).min(仴)).sum();[C::from(七.count()),-仛,佧,0]}亂::俍=>[C::from(l.七(丁).count()),僎(l,丁,&乃.front),佧,0,],亂::俜=>{let
丐=l.丐(丁);let 丯=C::from((丐&乃.丯).count());let 乊=C::from((丐&乃.乊).count());match 临.乕{_=>[C::from(l.七(丁).count()),丯,乊,佧],}}}}fn 侅(l:&J,万:&[L])->J{let mut 丄=l.clone();丄.佟(万);if
丄.丢().is_none(){丄.佟(&[]);}丄}const 俉:I=I::MAX;fn 万(l:&J,丰:&[Z],丣:丽)->K<L>{if 丣.乒==俉{let 么=Settings1{乚:丰.first().copied().unwrap_or(Z::仕),乥:if 丣.乥==乙::乺{仁::乺}else{仁::俊},..Settings1::DEFAULT};return
且(l,&么);}佩(l,&丰[F::from(丣.乒)%丰.len()],丣.乥,)}fn 倵(l:&J,丰:&[Z],世:&[丽],乖:丽,个:F,)->J{let mut 乳=l.clone();for 且 in 0..个{if 乳.丢().is_some(){break;}let 丣=世.get(且).copied().unwrap_or(乖);乳=侅(&乳,&万(&乳,丰,丣));}乳}fn
儻(l:&J,丰:&[Z],仳:O)->K<(丽,K<L>)>{let mut 丅:K<(丽,K<L>)> =K::new();if 仳{for 乥 in[乙::乺,乙::亰{为:0,丈:I::MAX,},]{let 丣=丽{乒:俉,乥,};let c=万(l,丰,丣);if!丅.iter().any(|(_,乀)|*乀==c){丅.push((丣,c));}}}for
乒 in 0..丰.len().min(F::from(I::MAX)){for&乥 in&偧{let 丣=丽{乒:乒 as I,乥,};let c=万(l,丰,丣);if!丅.iter().any(|(_,乀)|*乀==c){丅.push((丣,c));}}}丅}fn 僌(l:&J,丰:&[Z],乖:丽,个:F,亣:F,亶:P<乸>,临:&什,)->K<丽>{let
丂=l.丝();let 伬=||亶.is_some_and(|t|乸::now()>=t);let 乃=仰::of(l,临);let 伮=|世:&[丽]|仾(&倵(l,丰,世,乖,个),丂,临,乃);let mut 丑:(佀,K<丽>)=(伮(&[]),K::new());let mut 之:K<(J,K<丽>)>
=vec![(l.clone(),K::new())];for 僢 in 0..个{let mut 亵:K<(佀,J,K<丽>)> =K::new();let mut 乲:HashMap<丮,F> =HashMap::new();for(倪,世)in&之{if 伬(){break;}for(丣,c)in
儻(倪,丰,临.仳){let 丄=侅(倪,&c);let mut 侐=世.clone();侐.push(丣);let v=仾(&丄,丂,临,乃);let 仾=估(&丄,丂);match 乲.get(&仾){Some(&乜)if 亵[乜].0>=v=>{}Some(&乜)=>亵[乜]=(v,丄,侐),None=>{乲.insert(仾,亵.len());亵.push((v,丄,侐));}}}}if
亵.is_empty(){break;}亵.sort_by(|a,b|b.0 .cmp(&a.0));亵.truncate(亣);let 乻=僢+1==个||伬();for(i,(_,_,世))in 亵.iter().take(if 乻{亣}else{1}).enumerate(){if i>0&&伬(){break;}let
v=伮(世);if v>丑.0{丑=(v,世.clone());}}if 乻{break;}之=亵.into_iter().map(|(_,s,p)|(s,p)).collect();}丑.1}fn 世(l:&J,丰:&[Z],乖:丽,个:F,儈:F,亶:P<乸>,临:&什,)->K<丽>{let 丂=l.丝();let
乃=仰::of(l,临);let 伮=|世:&[丽]|仾(&倵(l,丰,世,乖,个),丂,临,乃);let mut 丑:(佀,K<丽>)=(伮(&[]),K::new());let mut 亣=2;loop{let 乆=僌(l,丰,乖,个,亣,亶,临);let v=伮(&乆);if v>丑.0{丑=(v,乆);}let
伬=亶.is_some_and(|t|乸::now()>=t);if 伬||(亶.is_none()&&亣>=儈){break;}亣*=2;}丑.1}const 偾:F=4+12;fn 个(侤:impl BufRead,mut 佝:impl Write,mut 侇:impl FnMut(&str,&str,Q,乸)->乛,)->Result<(),乛>{let
mut 乌=保{倹:侤};let Some(乮)=乌.line()?else{return Ok(());};let mut 仼=format!("{}\n",乮);乌.take(丛(&乮,"mine spot count")?,&mut 仼)?;let mut 且=0;loop{let Some(乮)=乌.line()?else{return
Ok(());};let 乁=乸::now();且+=1;let mut 亐=format!("{}\n",乮);乌.take(偾-1,&mut 亐)?;for 也 in["building count","unit count"]{let line=乌.倻()?;let n=丛(&line,也)?;亐.push_str(&line);亐.push('\n');乌.take(n,&mut
亐)?;}let 侇=侇(&仼,&亐,且,乁);writeln!(佝,"{}",侇).and_then(|()|佝.flush()).map_err(|仚|format!("turn {}: cannot write the answer: {}",且,仚))?;}}struct 保<R>{倹:R,}impl<R:BufRead>保<R>{fn
line(&mut self)->Result<P<乛>,乛>{let mut line=乛::new();match self.倹.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(仚)=>Err(format!("cannot read input: {}",仚)),}}fn
倻(&mut self)->Result<乛,乛>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,亐:&mut String)->Result<(),乛>{for
_ in 0..n{亐.push_str(&self.倻()?);亐.push('\n');}Ok(())}}fn 丛(line:&str,也:&str)->Result<F,乛>{match line.trim().parse::<F>(){Ok(n)if n<=傊=>Ok(n),_=>Err(format!("bad {} {line:?}",也)),}}const
傊:F=144;const 債:C=3;const 傅:C=2;const 傏:F=16;fn 儛(l:&J,丅:&mut Vec<L>){let 丁=l.丝();let mut 亟=俥{s:l.clone(),丁,丸:1-丁,与:T[1-丁],体:T[丁],且:l.乨(),丅,};亟.run();}struct
俥<'a>{s:J,丁:F,丸:F,与:F,体:F,且:Q,丅:&'a mut Vec<L>,}fn 佭(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侭(上:Q,q:F)->L{let(x,y)=佭(q);L::乹{上,x,y}}fn 偓(丆:I,q:F)->L{let(x,y)=佭(q);L::举{丆,x,y}}impl
俥<'_>{fn run(&mut self){let 一=self;if 一.佘(){return;}一.人();if 一.佘(){return;}一.亄();一.亡();}fn 久(&self,三:F,两:F)->C{C::from(self.s.map().业(三,两))}fn 义(&mut self,专:L)->O{let
书=self.s.义(专);if 书{self.丅.push(专);}书}fn 伍(&self,丵:&J)->O{丵.丢()==Some(乵::井(self.丁))}fn 佘(&mut self)->O{let 一=self;if 一.s.丢().is_some(){return false;}for
q in 一.s.乼().iter(){if 一.久(q,一.与)!=1{continue;}let Some((上,_))=一.s.亾(q)else{continue;};let 专=侭(上,一.与);let mut 丵=一.s.clone();if 丵.义(专)&&一.伍(&丵){return 一.义(专);}}let
Some(丞)=一.僖()else{return false;};let mut 丵=一.s.clone();if!丞.iter().all(|&专|丵.义(专))||!一.伍(&丵){return false;}for 专 in 丞{一.义(专);}true}fn 僖(&self)->P<K<L>>{let
一=self;const 俓:C=C::MAX;let 亍=一.s.丿().亚();let 丨:K<G> =(1..=亍).map(|丆|一.s.乍(一.丁,丆)).collect();let 伢=|q:F|{丨.iter().position(|乓|乓.test(q)).map(|丟|(丟 as I+1,10*(丟
as C+1)))};let 七=一.s.七(一.丁);let mut 丌=[俓;M];let mut 乎=[F::MAX;M];let mut 书=[false;M];for q in 七.iter(){丌[q]=0;}loop{let mut 丄:P<F> =None;for q in 0..M{if!书[q]&&丌[q]<俓&&丄.is_none_or(|n|丌[q]<丌[n]){丄=Some(q);}}let
Some(q)=丄 else{break;};书[q]=true;if q==一.与{break;}for&乢 in&一.s.map().丏[0][q]{if 乢==亓{break;}let 乢=F::from(乢);if 书[乢]||七.test(乢){continue;}let Some((_,仍))=伢(乢)else{continue;};if
丌[q]+仍<丌[乢]{丌[乢]=丌[q]+仍;乎[乢]=q;}}}if 丌[一.与]>一.s.下(一.丁){return None;}let mut 丞=K::new();let mut q=一.与;while!七.test(q){let(丆,_)=伢(q)?;丞.push(偓(丆,q));q=乎[q];}丞.reverse();Some(丞)}fn
乐(&self,乡:&J)->C{let(丂,仏)=(self.丁,self.丸);let 乎=&self.s;let 丛=|乓:G|C::from(乓.count());let 丘=丛(乡.丐(丂))-丛(乎.丐(丂));let 亭=丛(乎.丐(仏))-丛(乡.丐(仏));let 侪=丛(乎.七(仏))-丛(乡.七(仏));let
何=乎.伃(仏)-乡.伃(仏);let 为=丛(乎.为()&乎.丐(仏))-丛(乡.为()&乡.丐(仏));12*丘+20*亭+8*(侪-亭).max(0)+5*何+25*为}fn 偆(&self,q:F)->C{let 一=self;let 乣=一.s.map().丼.test(q);if 乣&&一.s.丿().但()&&!一.s.丐(一.丁).test(q){20}else{0}}fn
們(&self,乔:C)->O{let 一=self;一.s.丈(一.丸).iter().any(|q|一.久(q,一.体)<=乔)}fn 人(&mut self){let 一=self;let mut 丈:K<(C,F)> =一.s.乼().iter().map(|q|(一.久(q,一.与),q)).collect();丈.sort_unstable();for(仆,q)in
丈{if 一.s.丢().is_some(){return;}let Some((上,丆))=一.s.亾(q)else{continue;};let 丨=一.s.乍(一.丁,丆);let 乢=一.s.map().丏[一.丁][q];let mut 丑:P<(C,F)> =None;for&丄 in 乢.iter().take_while(|&&丄|丄!=亓){let
丄=F::from(丄);if!丨.test(丄){continue;}let mut 丵=一.s.clone();if!丵.义(侭(上,丄)){continue;}let 中=if 一.伍(&丵){100000}else{一.乐(&丵)+10*(仆-一.久(丄,一.与))+一.偆(丄)};if 丑.is_none_or(|(亠,_)|中>亠){丑=Some((中,丄));}}if
let Some((中,丄))=丑{if 中>0{一.义(侭(上,丄));}}}}fn 亄(&mut self){let 一=self;if 一.s.丢().is_some(){return;}if 一.們(債){一.兎();}else{一.为();}}fn 兎(&mut self){let 一=self;if!一.s.丿().仲()||一.s.下(一.丁)<互{return;}let
乢=一.s.map().丏[一.丁][一.体];let 丒:K<F> =乢.iter().take_while(|&&q|q!=亓).map(|&q|F::from(q)).collect();let 丹=一.s.丐(一.丁);if 丒.iter().any(|&q|丹.test(q)&&一.s.乗().test(q)){return;}for
q in 丒{let(x,y)=佭(q);if 一.义(L::亀{x,y}){return;}}}fn 为(&mut self){let 一=self;if!一.s.丿().但(){return;}while 一.s.下(一.丁)>=互{let 丧=一.s.map().丼&一.s.丐(一.丁)&一.s.七(一.丁)&一.s.仙();let
Some(q)=丧.iter().min_by_key(|&q|(一.久(q,一.体),q))else{return;};let(x,y)=佭(q);if!一.义(L::亯{x,y}){return;}}}fn 亡(&mut self){let 一=self;let 值=if 一.且<=3{0}else{5};for
_ in 0..傏{if 一.s.丢().is_some(){return;}let 下=一.s.下(一.丁);if 下<10{return;}let 儀=一.們(傅);let 乇=一.s.介(一.丁);let mut 丑:P<(C,L)> =None;let mut 侼:P<(C,L)> =None;let
mut 俽=G::U;for 丆 in 1..=一.s.丿().亚(){let 丨=一.s.乍(一.丁,丆);let 亝=(乇&丨).丕(俽);俽=丨;let 仍=10*C::from(丆);if 仍>下{continue;}for q in 亝.iter(){let 专=偓(丆,q);let mut
丵=一.s.clone();if!丵.义(专){continue;}let 伀=丵.下(一.丁);let 丠=丵.丠(一.丁);let 何=一.s.伃(一.丸)-丵.伃(一.丸);let 僆=if 何>0&&儀{伀+丠>=0}else{丠>=0&&(伀>=值||何>0)};if!僆{continue;}let
乐=一.乐(&丵);let 中=乐+一.偆(q)-6*一.久(q,一.与)-10*(C::from(丆)-1);if 乐>0{if 丑.is_none_or(|(亠,_)|中>亠){丑=Some((中,专));}}else if 丠>=3&&伀>=值+10&&丆==1{if 侼.is_none_or(|(亠,_)|中>亠){侼=Some((中,专));}}}}let
Some((_,专))=丑.or(侼)else{return;};if!一.义(专){return;}}}}fn 僸(l:&J,丅:&mut Vec<L>){丅.clear();儛(l,丅);}const 亴:C=C::MAX;const 傎:F=8;struct 仮{丌:[C;M],乎:[I;M],}fn
佋(l:&J,丁:F,人:O,与:F)->C{侔(l,丁,代(l,丁,人),与)}fn 侔(l:&J,丁:F,代:G,与:F)->C{let mut 丌=亴;倔(l,丁,代,|丣,new|{if new.test(与){丌=10*丣 as C;}丌!=亴});丌}fn 儣(l:&J,丁:F,代:G,乏:C)->G{let
mut 乯=G::U;倔(l,丁,代,|丣,new|{if 10*丣 as C>乏{return true;}乯|=new;false});乯.丕(l.七(丁))}fn 代(l:&J,丁:F,人:O)->G{if 人&&丁==l.丝(){l.乼()}else{G::U}}fn 倔(l:&J,丁:F,代:G,mut
仪:impl FnMut(F,G)->O,){let 亍=l.丿().亚();let mut 伢=[G::U;3];let mut 乲=G::U;for 丆 in 1..=亍{let 乓=l.乍(丁,丆);伢[F::from(丆)-1]=乓.丕(乲);乲|=乓;}let mut 両=[l.七(丁),G::U,G::U,G::U];for
q in(代&l.丈(丁)).iter(){if let Some(丆)=l.亢(q).filter(|&丆|丆<=亍){両[0]|=G::q(q).丏()&l.乍(丁,丆);}}let mut 书=G::U;for 丣 in 0..{let new=両[丣%4].丕(书);両[丣%4]=G::U;if
仪(丣,new){return;}书|=new;let 丄=new.丏().丕(书);for(仱,丒)in 伢.iter().enumerate(){両[(丣+仱+1)%4]|=丄&*丒;}if 両.iter().all(|乓|乓.丕(书).is_empty()){break;}}}impl 仮{fn
new(l:&J,丁:F,人:O)->仮{let 亍=F::from(l.丿().亚());let mut 丨=[G::U;3];for(丟,乓)in 丨.iter_mut().enumerate().take(亍){*乓=l.乍(丁,丟 as I+1);}let 七=l.七(丁);let 丏=&l.map().丏[0];let
mut 丌=[亴;M];let mut 乎=[亓;M];let mut 両=[0u8;M];let mut 丛=0;let mut 偂=G::U;for q in 七.iter(){丌[q]=0;両[丛]=q as I;丛+=1;}if 人&&丁==l.丝(){for 三 in l.乼().iter(){let
Some(丆)=l.亢(三)else{continue;};let 丷=G::q(三).丏()&丨[F::from(丆)-1];for q in 丷.iter(){if 丌[q]!=0{丌[q]=0;両[丛]=q as I;丛+=1;}}}}while 丛>0{let mut 丑=0;for 丟 in
1..丛{let(a,b)=(F::from(両[丟]),F::from(両[丑]));if(丌[a],a)<(丌[b],b){丑=丟;}}let q=F::from(両[丑]);丛-=1;両[丑]=両[丛];偂.set(q);for&丄 in&丏[q]{if 丄==亓{break;}let 丄=F::from(丄);if
偂.test(丄)||七.test(丄){continue;}let Some(丆)=丨[..亍].iter().position(|乓|乓.test(丄))else{continue;};let 偒=丌[q]+10*(丆 as C+1);if 偒<丌[丄]{if 丌[丄]==亴{両[丛]=丄 as
I;丛+=1;}丌[丄]=偒;乎[丄]=q as I;}}}仮{丌,乎}}fn path(&self,与:F)->K<F>{let mut path=K::new();if 与>=M||self.丌[与]==亴{return path;}let mut q=与;while self.乎[q]!=亓{path.push(q);q=F::from(self.乎[q]);}path.reverse();path}}struct
伽{万:K<L>,丌:C,}fn 儇(l:&J,丁:F,q:F)->P<I>{(1..=l.丿().亚()).find(|&丆|l.乍(丁,丆).test(q))}fn 儦(不:&mut J,丂:F,丗:F,乏:C,万:&mut Vec<L>,亏:&mut i64,)->O{loop{if 不.丢()==Some(乵::井(丂)){return
true;}if 不.丐(丂).test(丗)&&不.七(丂).test(丗){return true;}let 丩=仮::new(不,丂,false);let 丌=丩.丌[丗];if 丌==亴||*亏+丌>乏{return false;}let Some(&q)=丩.path(丗).first()else{return
false;};let Some(丆)=儇(不,丂,q)else{return false;};let 专=L::举{丆,x:(q%Y)as I,y:(q/Y)as I,};if!不.义(专){return false;}*亏+=10*C::from(丆);万.push(专);}}fn 俺(l:&J,丂:F,亅:&[F],乏:C)->P<伽>{let
mut 不=l.clone();let mut 万=K::new();let mut 亏=0;for&丗 in 亅{if!儦(&mut 不,丂,丗,乏,&mut 万,&mut 亏){return None;}}(亏<=乏).then_some(伽{万,丌:亏,})}fn 児(l:&J,丂:F,与:F)->C{let
丬=l.map().valid;let 丐=l.丐(丂)&丬;let 倇=|mut 乄:G|loop{let 丄=乄|(乄.丏()&丐);if 丄==乄{return 乄;}乄=丄;};let mut 乄=倇(l.七(丂));let mut 丷=0;while!乄.test(与){let new=乄.丏().丕(乄)&丬;if
new.is_empty(){return C::MAX/20;}乄=倇(乄|new);丷+=1;}丷}fn 侎(l:&J,与:F,乏:C)->P<伽>{if 乏<0||l.丢().is_some()||与>=M||l.map().乤.test(与){return None;}let 丂=l.丝();if
l.丐(丂).test(与)&&l.七(丂).test(与){return Some(伽{万:K::new(),丌:0,});}if 10*児(l,丂,与)>乏{return None;}let mut 丑=俺(l,丂,&[与],乏);let 乮=仮::new(l,丂,false);let mut 乗:K<F>
=(l.乗()&l.丐(1-丂)).iter().filter(|&乭|乭!=与&&乮.丌[乭]!=亴).collect();乗.sort_by_key(|&乭|(l.map().业(乭,与),乭));乗.truncate(傎);for 乭 in 乗{if let Some(丞)=俺(l,丂,&[乭,与],乏){if
丑.as_ref().is_none_or(|亠|丞.丌<亠.丌){丑=Some(丞);}}}丑}const 傌:F=20000;struct 伾{万:K<L>,丌:C,丒:C,丈:C,亄:C,丠:C,丱:C,}struct 佸{丒:C,丈:C,亄:C,}fn 倝(l:&J,丁:F)->佸{let 七=l.七(丁);let
丈=l.丈(丁)&七;let 中=丈.iter().filter_map(|q|l.亢(q)).map(|丆|10*C::from(丆)).sum();佸{丒:C::from(七.count()),丈:中,亄:C::from(((l.为()|l.乗())&七).count()),}}struct 俩{丂:F,伏:G,乎:佸,丠:C,伂:F,乏:C,伆:F,path:K<L>,乆:K<伾>,index:HashMap<伒,F,BuildHasherDefault<DefaultHasher>>,}type
伒=[丮;3];fn 党(mut 亙:伒,q:F)->伒{亙[q/64]|=1<<(q%64);亙}impl 俩{fn 倎(&mut self,不:&J,乻:P<F>,亭:伒,丌:C,)->O{let 一=self;let 丂=一.丂;let 亝=match 乻{None=>不.介(丂),Some(q)=>G::q(q).丏(),}&一.伏;let
亝=亝.丕(不.丐(丂));let mut 丨=[G::U;3];for 丆 in 1..=不.丿().亚(){丨[F::from(丆)-1]=不.乍(丂,丆);}for q in 亝.iter(){let 亍=F::from(不.丿().亚());let Some(丟)=丨[..亍].iter().position(|乓|乓.test(q))else{continue;};let
丆=丟 as I+1;let 仍=10*C::from(丆);if 丌+仍>一.乏{continue;}if 一.伆>=傌{return false;}一.伆+=1;let mut 丄=不.clone();let 专=L::举{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丄.义(专){continue;}一.path.push(专);let
伊=党(亭,q);一.儤(&丄,伊,丌+仍);let 僠=一.path.len()<一.伂&&丄.丢().is_none();let 僷=!僠||一.倎(&丄,Some(q),伊,丌+仍);一.path.pop();if!僷{return false;}}true}fn 儤(&mut self,乡:&J,伊:伒,丌:C){let
一=self;let 伫=1-一.丂;let 伪=倝(乡,伫);let 丒=一.乎.丒-伪.丒;let 丈=一.乎.丈-伪.丈;if 丒<=0&&丈<=0{return;}let 亄=一.乎.亄-伪.亄;let 丠=乡.丠(一.丂)-一.丠;let 兝=乡.丢()==Some(乵::井(一.丂));let
丱=if 兝{C::MAX/2}else{3*丒+丈+10*亄+2*丠-丌};if let Some(&乜)=一.index.get(&伊){if 丌>=一.乆[乜].丌{return;}}let 串=伾{万:一.path.clone(),丌,丒,丈,亄,丠,丱,};match 一.index.get(&伊){Some(&乜)=>一.乆[乜]=串,None=>{一.index.insert(伊,一.乆.len());一.乆.push(串);}}}}fn
倌(l:&J,伂:F,乏:C,乔:F)->K<伾>{乑(l,伂,乏,乔).0}fn 乑(l:&J,伂:F,乏:C,乔:F)->(K<伾>,F){if l.丢().is_some()||伂==0||乔==0{return(K::new(),0);}let 丂=l.丝();let 伫=1-丂;let mut
侸=俩{丂,伏:l.丐(伫)|l.丐(伫).丏(),乎:倝(l,伫),丠:l.丠(丂),伂,乏,伆:0,path:K::new(),乆:K::new(),index:HashMap::default(),};侸.倎(l,None,[0;3],0);let 伆=侸.伆;let mut 乆=侸.乆;乆.sort_by(|a,b|b.丱.cmp(&a.丱).then(a.丌.cmp(&b.丌)));乆.truncate(乔);(乆,伆)}const
佄:C=1000000;const 傍:C=200;fn 丶(l:&J,丁:F)->C{let 七=l.七(丁);let 丈:C=(l.丈(丁)&七).iter().filter_map(|q|l.亢(q)).map(|丆|10*C::from(丆)).sum();l.下(丁)+3*l.丠(丁)+4*C::from(七.count())+丈+10*C::from((l.乗()&七).count())}fn
倥(l:&J,丁:F,乽:G)->C{僵(l,丁,侔(l,丁,乽,T[1-丁]),)}fn 僵(l:&J,丁:F,丌:C)->C{if 丌==亴{return C::MIN;}l.下(丁)+l.丠(丁)-丌}fn 倶(亪:C)->C{亪.clamp(-傍,0)}fn 僪(l:&J,丁:F,乽:[G;2],)->(C,[C;2]){let
乀=1-丁;match l.丢(){Some(乵::井(兙))=>{return(if 兙==丁{佄}else{-佄},[0,0])}Some(乵::俑)=>return(0,[0,0]),None=>{}}let mut 丱=丶(l,丁)-丶(l,乀);if 丁==l.丝(){let mut 丄=l.clone();丄.仸();if
丄.丢().is_none()&&佋(&丄,乀,true,T[丁])<=丄.下(乀){丱-=佄/2;}}let 企=[倥(l,丁,乽[0]),倥(l,乀,乽[1]),];(丱+倶(企[0])-倶(企[1]),企)}
