// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伻=u16;type Q=u32;type 丯=u64;type 伺=i32;type C=i64;type 亿=f64;type O=bool;type 乙=String;type 亇=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
傁:O=true;fn 僶(l:&J,q:F,伵:C,丩:G,丝:C)->O{let 丂=l.东();let 丸=1-丂;let 倠=傁&&丝<=l.下(丸)+2*l.丟(丸).max(0);let 伐=G::q(q)|G::q(q).丏();if!倠&&(伐&丩).is_empty(){return
false;}let(x,y)=乕(q);let mut 乑=l.clone();if!乑.乇(L::乿{x,y}){return false;}if 伴(&乑,丂)<伵{return true;}倠&&低(&乑,丸,false,T[丂])>丝}const 傿:O=false;const 傟:I=2;const
俪:I=1;const 俒:I=2;const 佂:F=0;const 使:F=2;const 傥:O=false;const 傧:O=false;const 傦:O=false;const 傤:O=false;const 傈:F=4;const 俏:F=5;#[derive(Clone)]enum
丫{仓(L),争(K<L>),}impl 丫{fn 万(&self)->&[L]{match self{丫::仓(丒)=>std::slice::from_ref(丒),丫::争(万)=>万,}}}fn 乕(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 候(l:&J,丅:&mut
Vec<丫>){丅.clear();if l.丢().is_some(){return;}let 丂=l.东();let 亍=l.丿().亘();let mut 丨=[G::U;4];for 丆 in 1..=亍{丨[F::from(丆)]=l.乌(丂,丆);}for 三 in l.乺().iter(){let
Some((上,丆))=l.亽(三)else{continue;};let 之=G::q(三).丏()&丨[F::from(丆)];for 两 in 之.iter(){let(x,y)=乕(两);丅.push(丫::仓(L::乷{上,x,y}));}}let 下=l.下(丂);let 七=l.七(丂);let
乗=l.今(丂);let 乲=match 傟{0=>乗,1=>{let 伛=l.map().valid.丗(七).丏()&七;乗.丗(七)|伛}_=>乗.丗(七),};for 丆 in 1..=亍{let 与=10*C::from(丆);if 下<与{break;}let mut 丑=乲&丨[F::from(丆)];for
儂 in 1..丆{丑=丑.丗(丨[F::from(儂)]);}for q in 丑.iter(){let(x,y)=乕(q);丅.push(丫::仓(L::举{丆,x,y}));}}if 下>=云{let 丧=l.丐(丂)&乗&l.付();if l.丿().佇(){for q in(丧&l.map().丼).iter(){let(x,y)=乕(q);丅.push(丫::仓(L::亭{x,y}));}}if
l.丿().仲(){let 伛=l.map().valid.丗(l.丐(丂)).丏();let 丑=(丧&伛).丗(l.map().丼);let 丩=丩(l,1-丂);let 伵=C::from((丩&l.七(丂)).count());let 丝=低(l,1-丂,false,T[丂]);let 両=if
傿{let(亨,何)=义(l,丂);亨.丗(何)}else{G::U};for q in 丑.iter().filter(|&q|{let 伐=G::q(q)|G::q(q).丏();!(伐&両).is_empty()||僶(l,q,伵,丩,丝)}){let(x,y)=乕(q);丅.push(丫::仓(L::乿{x,y}));}}}let
僵=match 俪{1=>云,2=>10,_=>C::MAX,};if 下>=僵{let 丩=丩(l,1-丂);let 亙=l.map().valid.丗(l.丐(丂)|l.丐(1-丂));for 三 in(l.乺()&丩).iter(){let Some((上,丆))=l.亽(三)else{continue;};let
之=G::q(三).丏()&丨[F::from(丆)]&亙;let(仜,仝)=乕(三);let 価=if 俪==1{if l.map().丼.test(三){continue;}L::乿{x:仜,y:仝}}else{L::举{丆:1,x:仜,y:仝,}};for 两 in 之.iter(){let(x,y)=乕(两);丅.push(丫::争(vec![L::乷{上,x,y},価]));}}}if
俒!=0&&下>=云&&l.丿().仲(){僪(l,&丨,丅);}if 佂!=0&&下>=20{儥(l,丨[1],下,丅);}if 傤{僞(l,丨[1],下,丅);}for 串 in 倍(l,俏,下,傈){if 串.万.len()>1{丅.push(丫::争(串.万));}}if let Some(丝)=侑(l,T[1-丂],下){if
丝.万.len()>1{丅.push(丫::争(丝.万));}}}fn 僪(l:&J,丨:&[G;4],丅:&mut Vec<丫>){let 丂=l.东();let 丸=1-丂;let 丩=丩(l,丸);let 严=l.丐(丂);let 且=l.丐(丸);let 伛=且.丏();let 丼=l.map().丼;for
三 in(l.乺()&丩&l.七(丂)).iter(){if 丼.test(三){continue;}let Some((上,丆))=l.亽(三)else{continue;};let 僇=l.map().丙(三,T[丸]);let 之=G::q(三).丏()&丨[F::from(丆)];let(仜,仝)=乕(三);for
两 in 之.iter(){let 侊=match 俒{1=>严.test(两)&&l.map().丙(两,T[丸])>僇,2=>严.test(两)&&伛.test(两),_=>严.test(两)||(且.test(两)&&两!=T[丸]),};if!侊{continue;}let(x,y)=乕(两);丅.push(丫::争(vec![L::乷{上,x,y},L::乿{x:仜,y:仝},]));}}}fn
儥(l:&J,侩:G,下:C,丅:&mut Vec<丫>){let 丂=l.东();let 並=l.map();let 亙=並.valid.丗(l.丐(丂)|l.丐(1-丂));let 両=亙&侩;let 乲:K<F> =(l.七(丂).丏()&両).iter().collect();if 乲.is_empty(){return;}let
侱=|不:F|{乲.iter().copied().min_by_key(|&q|(並.丙(q,不),q))};let mut 侒:K<(F,F)> =K::new();if 傥{for 丘 in 儦(l,&乲){if let Some(乀)=侱(丘){侒.push((乀,丘));}}}else{let
不=T[1-丂];let mut 侾=乲.clone();侾.sort_by_key(|&q|(並.丙(q,不),q));侾.truncate(使);侒.extend(侾.into_iter().map(|乀|(乀,不)));}let 亪=F::try_from(下/10).unwrap_or(0).min(佂);for(乀,不)in
侒{let mut 丝=vec![乀];let mut 京=G::q(乀);while 丝.len()<亪{let 乹=丝[丝.len()-1];if 乹==不{break;}let 丄=(G::q(乹).丏()&両).丗(京).iter().min_by_key(|&q|(並.丙(q,不),q));let
Some(丄)=丄 else{break;};丝.push(丄);京.set(丄);let 万=丝.iter().map(|&q|{let(x,y)=乕(q);L::举{丆:1,x,y}}).collect();丅.push(丫::争(万));}if 傦{僿(l,&丝,丅);}}}fn 僿(l:&J,丝:&[F],丅:&mut
Vec<丫>){let 丂=l.东();let 乀=丝[0];let 乻=l.乺()&l.丈(丂)&G::q(乀).丏();for 三 in 乻.iter(){let Some((上,丆))=l.亽(三)else{continue;};if!l.乌(丂,丆).test(乀){continue;}let(x,y)=乕(乀);let
mut 万=vec![L::乷{上,x,y}];for&q in&丝[1..]{let(x,y)=乕(q);万.push(L::举{丆:1,x,y});丅.push(丫::争(万.clone()));}}}fn 儦(l:&J,乲:&[F])->K<F>{let 丂=l.东();let 並=l.map();let(么,中)=佔(l);let
严:K<F> =l.丈(丂).iter().collect();let 且:K<F> =l.丈(1-丂).iter().collect();let 体=|丘:F|{乲.iter().map(|&q|並.丙(q,丘)).min().unwrap_or(I::MAX)};let 僖=|丘:F|{let 丩=体(丘);严.iter().any(|&亼|{let
d=並.丙(亼,丘);d<=丩&&且.iter().all(|&丸|d<並.丙(丸,丘))})};let 侬=|丘:F|{傧&&(l.丐(1-丂).test(丘)||且.iter().any(|&丸|並.丙(丸,丘)<体(丘)))};let 両=|丑:G|->K<F>{let mut 亃:K<F> =丑.丗(l.丐(丂)).iter().filter(|&丘|体(丘)!=I::MAX&&!僖(丘)&&!侬(丘)).collect();亃.sort_by_key(|&丘|(体(丘),丘));亃};let
mut 亃=両(中);if 亃.is_empty(){亃=両(么);}亃.truncate(使);亃}fn 僘(l:&J)->K<F>{let 丂=l.东();let 並=l.map();let(么,中)=佔(l);let 七=l.七(丂);let 严=l.丐(丂);let 丩=丩(l,1-丂);let
且:K<F> =l.丈(1-丂).iter().collect();let 倗=|q:F|{七.iter().map(|a|並.丙(a,q)).min().unwrap_or(I::MAX)};let mut 丑:K<F> =(么|中).iter().filter(|&q|{if 严.test(q){七.test(q)&&丩.test(q)}else{!l.丐(1-丂).test(q)&&且.iter().any(|&e|並.丙(e,q)<倗(q))}}).collect();丑.sort_by_key(|&q|(I::from(!中.test(q)),倗(q),q));丑.truncate(
2*使);丑}fn 僞(l:&J,侩:G,下:C,丅:&mut Vec<丫>){let 丂=l.东();let 並=l.map();let 亙=並.valid.丗(l.丐(丂)|l.丐(1-丂));let 両=亙&侩;let 七=l.七(丂);let 丧=七&l.付();let 儁=if 佂==0{3}else{佂};for
q in 僘(l){if!l.丐(丂).test(q){let 乀=(七.丏()&両).iter().min_by_key(|&c|(並.丙(c,q),c));if let Some(乀)=乀{let mut 丝=vec![乀];let mut 京=G::q(乀);while 丝[丝.len()-1]!=q&&丝.len()<儁{let
乹=丝[丝.len()-1];let 丄=(G::q(乹).丏()&両).丗(京).iter().min_by_key(|&c|(並.丙(c,q),c));let Some(丄)=丄 else{break;};丝.push(丄);京.set(丄);}let 与=10*C::try_from(丝.len()).unwrap_or(C::MAX);if
丝.len()>=2&&丝[丝.len()-1]==q&&与<=下{let 万=丝.iter().map(|&c|{let(x,y)=乕(c);L::举{丆:1,x,y}}).collect();丅.push(丫::争(万));}}}if 下>=云&&l.丿().仲(){let 丼=(G::q(q)|G::q(q).丏())&丧;for
乫 in 丼.丗(並.丼).iter(){let(x,y)=乕(乫);丅.push(丫::仓(L::乿{x,y}));}}}}const 傕:I=2;const 傳:C=5;fn 乻(l:&J,丁:F)->G{侰(l,丁,傕)}fn 侰(l:&J,丁:F,三:I)->G{let mut 乻=G::U;if
三>3{return 乻;}for q in(l.丈(丁)&l.七(丁)).iter(){if l.亠(q).is_some_and(|丆|丆>=三){乻|=G::q(q);}}乻}fn 丩(l:&J,丷:F)->G{let 乎=l.下(丷)+l.丟(丷);儞(l,丷,乻(l,丷),乎)}fn 产(l:&J,丁:F)->G{丩(l,1-丁)&l.七(丁)}fn
伴(l:&J,丁:F)->C{C::from(产(l,丁).count())}const 俟:C=4;fn 儆(丑:C)->C{丑/俟}fn 为(l:&J,丁:F,产:G)->C{let 七=l.七(丁);let 为=(l.为()&七).丗(产);let 丑=C::from(七.count());let
侍=if l.书()<=偮{丑/偭+偬}else{儆(丑)};侍*C::from(为.count())}const 偮:Q=15;const 偬:C=6;const 偭:C=俟;const 催:C=2;const 佃:I=1;const 俩:I=3;const 傭:I=6;fn 僾(仇:I,乗:I,仚:I,d:I)->I{match
仇{0=>4,1 if d<=乗=>4,1=>0,_ if d<=乗=>4,_ if d>=仚=>0,_=>(4*Q::from(仚-d)/Q::from(仚-乗))as I,}}fn 偞(l:&J,丁:F)->C{let 七=l.七(丁);let 丸=if 佃==0{G::U}else{l.七(1-丁)};let[a,b,c]=七.伸();let[e,f,g]=丸.伸();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丁 as 丯).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丠=(hash as F)&(俳-1);僀.with(|侏|{let mut 侏=侏.borrow_mut();let 仍=&mut 侏[丠];if 仍.0==七&&仍.1==丸&&仍.2==丁 as I+1{return 仍.3;}let 丬=if 佃==0{兒(七,T[丁])}else if 佃==1{let
丱=l.map().valid;let mut 乗=丸&丱;for _ in 0..俩{乗=(乗|乗.丏())&丱;}兑(七,T[丁],乗)}else{let d=丳(l,丸);let mut 乐=[0u8;M];for(w,&之)in 乐.iter_mut().zip(d.iter()){*w=僾(佃,俩,傭,之);}偟(七,T[丁],&乐)};*仍=(七,丸,丁
as I+1,丬);丬})}const 俳:F=1<<12;thread_local!{static 僀:std::cell::RefCell<K<(G,G,I,C)>> =std::cell::RefCell::new(vec![(G::U,G::U,0,0);俳]);}fn 儙(七:G)->G{let
n=七.佨();let s=七.佩();let e=七.偄();let w=七.偅();let 儉=e.佨();let 儊=w.佨();let 儧=e.佩();let 儺=w.佩();let 僽=[n&儉&e,e&儧&s,s&儺&w,w&儊&n];let 亾=|a:G,b:G|(a|b).丗(a&b);let
俵=|[a,b,c,d]:[G;4]|{let(倿,倁)=(亾(a,b),a&b);let(偀,倃)=(亾(c,d),c&d);let t=倿&偀;let 倂=亾(倁,倃);(亾(倿,偀),亾(倂,t),(倁&倃)|(t&倂))};let(僸,倝,價)=俵([n,e,s,w]);let(僣,倐,僤)=俵(僽);let
俼=僣.丗(僸);let 偡=亾(倝,倐);let 僙=亾(偡,俼);let 僈=倐.丗(倝)|俼.丗(偡);let 僚=亾(亾(價,僤),僈);七&(僙|僚)}fn 兑(七:G,乯:F,乗:G)->C{if!七.test(乯){return 0;}let 了=七.count();let mut 二=0i64;let
mut 亪=0i64;for v in(儙(七)&乗).iter(){if v==乯{continue;}let 乪=七.丗(G::q(v));let mut 乩=G::q(乯);loop{let 丄=(乩|乩.丏())&乪;if 丄==乩{break;}乩=丄;}let 串=C::from(了-1-乩.count());if
串>0{二+=4;亪=亪.max(4*串);}}(二+亪)/4}fn 兒(七:G,乯:F)->C{偟(七,乯,&[4;M])}fn 偟(七:G,乯:F,乐:&[I;M])->C{struct 休<'a>{乐:&'a[I;M],七:G,乴:[I;M],乜:[I;M],size:[I;M],仏:I,二:C,亪:C,}fn
仩(d:&mut 休<'_>,v:F)->I{d.仏+=1;let 乴=d.仏;d.乴[v]=乴;let mut 乜=乴;let mut 侺=1u8;let mut 串=0u8;for w in(G::q(v).丏()&d.七).iter(){if d.乴[w]==0{let 偐=仩(d,w);侺+=偐;乜=乜.min(d.乜[w]);if
d.乜[w]>=乴{串+=偐;}}else{乜=乜.min(d.乴[w]);}}d.乜[v]=乜;d.size[v]=侺;if 串>0&&d.乴[v]>1{let w=C::from(d.乐[v]);d.二+=w;d.亪=d.亪.max(w*C::from(串));}侺}if!七.test(乯){return
0;}let mut d=休{乐,七,乴:[0;M],乜:[0;M],size:[0;M],仏:0,二:0,亪:0,};仩(&mut d,乯);(d.二+d.亪)/4}fn 儩(l:&J,丁:F)->C{催*(偞(l,1-丁)-偞(l,丁))}fn 倎(l:&J,丁:F)->C{if l.下(丁)+l.丟(丁)>=0{return
0;}(l.丈(丁)&l.七(丁)).iter().filter_map(|q|l.亠(q)).map(|丆|10*C::from(丆)).sum()}const 傇:C=15;const 傻:[C;2]=[3,1];const 傹:[C;2]=[4,20];fn 儸(l:&J,丁:F)->[C;2]{let
mut 业=[0;2];for q in(l.丈(丁)&l.七(丁)).iter(){match l.亠(q){Some(2)=>业[0]+=1,Some(3)=>业[1]+=1,_=>{}}}业}fn 偏(l:&J,丁:F)->C{let[偖,元]=儸(l,丁);let 俽=|丆:F|(3-傻[丆])*傹[丆];-傇*偖+俽(0)*偖+俽(1)*元}const
傀:C=0;const 傃:C=2;const 傂:C=100;fn 偈(亩:C)->C{if 亩<0{return 0;}傀+傃*亩.min(傂)/2}const 俞:I=2;const 俛:C=2;const 俚:C=4;const 傄:I=3;fn 亶(l:&J,丁:F)->C{if 俛==0{return
0;}let 丱=l.map().valid;let mut 丈=G::U;for q in(l.丈(丁)&l.七(丁)).iter(){if l.亠(q).is_some_and(|丆|丆<=傄){丈|=G::q(q);}}let mut 佮=丱.丗(l.丐(丁));let mut 仄=G::U;let
mut 之=0;for 丙 in 1..=俚{佮=(佮|佮.丏())&丱;let 仅=(丈&佮).丗(仄);之+=(丙-1)*C::from(仅.count());仄|=仅;}之+=俚*C::from(丈.丗(仄).count());俛*之}const 俲:C=8;fn 义(l:&J,丁:F)->(G,G){let
七=l.七(丁);let 亨=七&l.丐(1-丁).丏();let 乔=l.乔()&七;(亨,亨&(乔|乔.丏()))}fn 偛(l:&J,丁:F)->C{if 俲==0{return 0;}俲*C::from(义(l,丁).1 .count())}const 佸:C=0;const 俗:I=1;const
俘:C=4;const 佹:C=0;const 俖:C=0;const 偷:Q=14;const 偶:C=2;const 俱:C=0;const 傾:Q=14;fn 仪(l:&J,丁:F)->C{if 俱==0||l.书()>傾{return 0;}let 严=丳(l,l.七(丁));let 且=丳(l,l.七(1-丁));let
mut 代=0;for c in l.map().valid.iter(){match 严[c].cmp(&且[c]){std::cmp::Ordering::Less=>代+=1,std::cmp::Ordering::Greater=>代-=1,std::cmp::Ordering::Equal=>{}}}俱*代}struct
俙{valid:G,么:G,中:G,}thread_local!{static 偸:std::cell::RefCell<P<俙>> =const{std::cell::RefCell::new(None)};}fn 佔(l:&J)->(G,G){偸.with(|q|{let mut 乂=q.borrow_mut();let
丱=l.map().valid;if 乂.as_ref().is_none_or(|z|z.valid!=丱){let 仧=仧(l);let mut 么=G::U;for c in 丱.iter(){if 仧[c]<=偶{么.set(c);}}*乂=Some(俙{valid:丱,么,中:仡(l),});}乂.as_ref().map_or((G::U,G::U),|z|(z.么,z.中))})}fn
何(l:&J,丁:F)->G{let 七=l.七(丁);let 乔=l.乔()&七;(乔|乔.丏())&七}fn 丘(l:&J,丁:F)->C{if(佸==0&&俘==0&&佹==0&&俖==0)||l.书()>偷{return 0;}let(么,中)=佔(l);let mut 了=0;if 佸!=0{let
严=丳(l,l.七(丁));let 且=丳(l,l.七(1-丁));let mut 代=0;for c in 么.iter(){let(a,b)=(严[c],且[c]);if a!=I::MAX&&a.saturating_add(俗)<=b{代+=1;}else if b!=I::MAX&&b.saturating_add(俗)<=a{代-=1;}}了+=佸*代;}let
业=|乒:G|C::from(乒.count());let(丹,且)=(l.七(丁),l.七(1-丁));了+=俘*(业(丹&么)-业(且&么));if 佹!=0{了+=佹*(业(何(l,丁)&么)-业(何(l,1-丁)&么));}了+=俖*(业(丹&中)-业(且&中));了}const 俕:C=4;const
俔:C=0;const 偳:C=4;const 側:Q=12;fn 僮()->[G;2]{static 偲:std::sync::OnceLock<[G;2]> =std::sync::OnceLock::new();*偲.get_or_init(僯)}fn 僯()->[G;2]{let mut 丅=[G::U;2];for
c in 0..M{let(x,y)=((c%Y)as C,(c/Y)as C);if x-y>=俕{丅[0].set(c);}else if y-x>=俕{丅[1].set(c);}}丅}fn 偝(l:&J,丁:F)->C{let[a,b]=僮();let 七=l.七(丁);C::from((七&a).count().min((七&b).count())).min(偳)}fn
僭(l:&J,丁:F)->C{if 俔==0||l.书()>側{return 0;}俔*(偝(l,丁)-偝(l,1-丁))}fn 僦(l:&J,丁:F)->C{let 乻=[侰(l,丁,俞),侰(l,1-丁,俞),];let(丮,企)=僧(l,丁,乻);let 丮=丮+偈(企[0])-偈(企[1]);if
l.丢().is_some(){return 丮;}let 严=产(l,丁);let 且=产(l,1-丁);let 伵=C::from(且.count())-C::from(严.count());丮-2*倎(l,丁)+2*倎(l,1-丁)+儩(l,丁)+偏(l,丁)-偏(l,1-丁)-亶(l,丁)+亶(l,1-丁)+傳*伵/4+为(l,丁,严)-为(l,1-丁,且)+偛(l,丁)-偛(l,1-丁)+丘(l,丁)+仪(l,丁)+僭(l,丁)}const
傑:F=8;const 傢:F=3;const 傉:F=32;const 俧:丯=300;const 俥:O=true;const 佽:O=true;const 傜:O=false;const 傝:F=16;const 偫:亿=0.75;const 偽:P<Q> =Some(7);const 偻:O=false;const
偺:C=10;const 偼:Q=6;struct 亯{亻:P<亇>,仞:P<亇>,伂:丯,}#[derive(Default)]struct 侄{万:K<L>,侖:F,亦:丯,丞:F,丶:F,倇:F,亖:F,亅:O,偌:O,介:F,}const 傽:O=true;#[derive(Clone)]struct
亊{万:K<L>,侙:C,}struct 仭{万:K<L>,end:J,first:P<丯>,丛:P<K<亊>>,}type 仫=HashMap<丯,C,BuildHasherDefault<DefaultHasher>>;struct 侂{丂:F,limit:亯,亦:丯,亵:仫,仨:P<K<丫>>,俀:O,乛:P<们>,佞:std::cell::RefCell<仫>,佟:std::cell::RefCell<K<K<L>>>,}const
侀:C=0;const 俌:O=true;const 偦:O=false;const 俋:P<I> =None;fn 働(l:&J,丞:&[亊],丅:&mut Vec<丫>){if!俌&&俋.is_none(){return;}let 估=丞.iter().map(|乼|乼.万.as_slice());僎(l,估,(俌,偦,俋),丅,);}fn
僎<'a>(l:&J,估:impl Iterator<Item=&'a[L]>,(乔,丏,丆):(O,O,P<I>),丅:&mut Vec<丫>,){let 丂=l.东();let 丹=l.七(丂);let mut 伢=G::U;for 乼 in 估{for 丒 in 乼{if let L::举{x,y,..}|L::乷{x,y,..}=*丒{let
q=F::from(y)*Y+F::from(x);if 丹.test(q){伢|=G::q(q);}}}}if 伢.is_empty(){return;}let 丧=l.付()&丹;let 下=l.下(丂);let 倹=|丒:L,丅:&mut Vec<丫>|{if!丅.iter().any(|a|a.万()==[丒]){丅.push(丫::仓(丒));}};if
乔&&下>=云&&l.丿().仲(){let mut 丑=伢;if 丏{丑|=伢.丏();}for q in(丑&丧).丗(l.map().丼).iter(){let(x,y)=乕(q);倹(L::乿{x,y},丅);}}if let Some(丆)=丆.filter(|&丆|丆<=l.丿().亘()){if
下>=10*C::from(丆){for q in(伢&丧).iter(){let(x,y)=乕(q);倹(L::举{丆,x,y},丅);}}}}fn 佈(l:&mut J,万:&[L])->O{万.iter().all(|&丒|l.乇(丒))}fn 亄(万:&[L])->丯{let mut hash:丯=0xcbf29ce484222325;for
丒 in 万{let(余,a,b,c)=match*丒{L::乷{上,x,y}=>(1,丯::from(上),x,y),L::举{丆,x,y}=>(2,丯::from(丆),x,y),L::亭{x,y}=>(3,0,x,y),L::乿{x,y}=>(4,0,x,y),};let 伏=余|a<<8|丯::from(b)<<40|丯::from(c)<<48;hash=(hash^伏).wrapping_mul(0x0100000001b3);}hash}fn
俷(乵:&J,乼:&[L])->J{let mut 丄=乵.clone();丄.仸();for&丒 in 乼{丄.乇(丒);}丄.仸();丄}fn 佛(l:&J)->P<K<L>>{let 丂=l.东();let 不=T[1-丂];let 伎=|丵:&J|丵.丢()==Some(乳::五(丂));for
q in l.乺().iter(){if l.map().丙(q,不)!=1{continue;}let Some((上,_))=l.亽(q)else{continue;};let(x,y)=乕(不);let 丒=L::乷{上,x,y};let mut 丵=l.clone();if 丵.乇(丒)&&伎(&丵){return
Some(vec![丒]);}}let 丝=侑(l,不,l.下(丂))?;let mut 丵=l.clone();(佈(&mut 丵,&丝.万)&&伎(&丵)).then_some(丝.万)}fn 偎(l:&J,丁:F,个:C)->O{let 与=侗(l,丁,l.丈(丁),T[1-丁]);与!=亲&&l.下(丁)+个*l.丟(丁)>=与}fn
儷(l:&J)->O{let 丂=l.东();if 侀==0||l.丢().is_some(){return false;}let mut 丄=l.clone();丄.仸();let 伴=丄.丢().is_none()&&低(&丄,1-丂,true,T[丂])<=丄.下(1-丂);!伴&&偎(l,丂,侀)}impl
侂{fn 俿(&self,乵:&J)->O{self.俀&&乵.丢()!=Some(乳::五(self.丂))&&!偎(乵,self.丂,侀)}fn 于(&self)->O{self.亦>=self.limit.伂||self.limit.亻.is_some_and(|t|亇::now()>=t)}fn
丮(&mut self,乵:&J,丞:&[亊],侎:C)->C{self.亦+=1;self.侶(乵,丞,侎)}fn 侶(&self,乵:&J,丞:&[亊],侎:C)->C{if 乵.丢()==Some(乳::五(self.丂)){return 佅;}if 丞.is_empty(){return self.value(乵);}let
mut 佯=C::MAX;for 乼 in 丞{佯=佯.min(self.value(&俷(乵,&乼.万)));if 佯<=侎{break;}}佯}fn value(&self,l:&J)->C{let 会=僦(l,self.丂);match&self.乛{Some(乛)if l.丢().is_none()=>会+乛.儾(l,self.丂),_=>会,}}fn
俶(&self,乞:&mut Vec<丫>){if let Some(乛)=&self.乛{for 佢 in&乛.介{乞.push(丫::争(佢.clone()));}}}#[allow(clippy::too_many_arguments)]fn 儔(&mut self,乃:&J,价:C,乞:&[丫],习:&[(C,F)],乥:&mut[O],丞:&[亊],仹:&mut
仫,侧:O,)->(P<(F,J,C)>,O){let mut 专=None;let mut 佊=价;for&(佝,丠)in 习{if 乥[丠]{continue;}if 侧&&佝!=C::MAX&&佝<=佊-价{return(专,true);}if self.于(){return(专,false);}乥[丠]=true;let
万=乞[丠].万();let mut 丄=乃.clone();if!佈(&mut 丄,万)||self.俿(&丄){仹.insert(亄(万),C::MIN/4);continue;}let 丮=self.丮(&丄,丞,佊);仹.insert(亄(万),丮-价);if 丮>佊{佊=丮;专=Some((丠,丄,丮));}}(专,false)}fn
佋(&mut self,乯:&J,丞:&[亊],伙:&[丯],僑:O)->仭{let 一=self;let mut 乬=None;let mut 乃=乯.clone();let mut 万=K::new();let mut 价=一.丮(&乃,丞,C::MIN);let mut 仹=仫::default();let
mut 乞=K::new();let mut 习=K::new();while 乃.丢().is_none()&&!一.于(){if 万.is_empty()&&僑{match&一.仨{Some(仨)=>乞.clone_from(仨),None=>{候(&乃,&mut 乞);if let Some(乛)=一.乛.as_ref().filter(|g|g.倱){乞.push(丫::争(乛.世.clone()));}一.俶(&mut
乞);一.仨=Some(乞.clone());}}}else{候(&乃,&mut 乞);一.俶(&mut 乞);}働(&乃,丞,&mut 乞);习.clear();for(丠,井)in 乞.iter().enumerate(){let k=亄(井.万());let 佝=仹.get(&k).or(一.亵.get(&k)).copied();习.push((佝.unwrap_or(C::MAX),丠));}习.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乥=vec![false;乞.len()];if 万.is_empty(){for(丠,井)in 乞.iter().enumerate(){乥[丠]=伙.contains(&亄(井.万()));}}let mut 专=None;for
侧 in[true,false]{let(乏,儭)=一.儔(&乃,价,&乞,&习,&mut 乥,丞,&mut 仹,侧,);专=乏;if 专.is_some()||!儭{break;}}let Some((丠,丄,丮))=专 else{break;};if 万.is_empty(){乬=Some(亄(乞[丠].万()));}万.extend_from_slice(乞[丠].万());乃=丄;价=丮;}for(k,久)in
仹{let 伤=一.亵.entry(k).or_insert(久);*伤=(*伤).max(久);}仭{万,end:乃,first:乬,丛:None,}}fn 估(&self,乵:&J)->K<K<L>>{let 一=self;let mut 丄=乵.clone();丄.仸();let mut 丅=K::new();if
丄.丢().is_some(){return 丅;}let 亚=丄.东();let 下=丄.下(亚);if let Some(丝)=侑(&丄,T[一.丂],下){丅.push(丝.万);}for 串 in 倍(&丄,俏,下,傢){丅.push(串.万);}let mut 倛=K::new();僴(&丄,&mut
倛);丅.push(倛);let mut 且=侂{丂:亚,limit:亯{伂:俧,..一.limit},亦:0,亵:仫::default(),仨:None,俀:false,乛:None,佞:std::cell::RefCell::default(),佟:std::cell::RefCell::default(),};if
俥{且.亵=一.佞.borrow().clone();}let 児=if 佽{一.佟.borrow().iter().filter_map(|万|{let mut 乑=丄.clone();佈(&mut 乑,万).then(||(且.value(&乑),万.clone()))}).max_by_key(|(丮,_)|*丮).map(|(_,万)|万)}else{None};let
世=match 児.and_then(|万|且.亖(&丄,&丫::争(万),&[])){Some(世)=>世,None=>且.佋(&丄,&[],&[],true),};if 傜&&佽{if let Some(乬)=世.first.filter(|_|!且.于()){且.limit.伂=且.亦+俧;let
主=且.佋(&丄,&[],&[乬],true);if 主.万!=世.万{丅.push(主.万);}}}if 佽&&!世.万.is_empty(){let mut 丶=一.佟.borrow_mut();if!丶.contains(&世.万){if 丶.len()>=傝{丶.remove(0);}丶.push(世.万.clone());}}if
俥{let mut 亵=一.佞.borrow_mut();for(亄,久)in 且.亵.iter(){let 伤=亵.entry(*亄).or_insert(*久);*伤=(*伤).max(*久);}}丅.push(世.万);丅.retain(|万|!万.is_empty());丅}fn 俸(&self,乵:&J)->K<亊>{self.估(乵).into_iter().map(|万|{let
侙=self.value(&俷(乵,&万));亊{万,侙}}).collect()}fn 亖(&mut self,乯:&J,井:&丫,丞:&[亊])->P<仭>{let mut 乑=乯.clone();if!佈(&mut 乑,井.万())||self.俿(&乑){return None;}self.亦+=1;let
乪=self.佋(&乑,丞,&[],false);let mut 万=井.万().to_vec();万.extend_from_slice(&乪.万);Some(仭{万,end:乪.end,first:Some(亄(井.万())),丛:None,})}fn 倔(&self,世:&仭,丞:&[亊])->C{let
丮=self.侶(&世.end,丞,C::MIN);match&世.丛{Some(丛)if!丛.is_empty()=>丮.min(self.侶(&世.end,丛,C::MIN)),_=>丮,}}}fn 儠(乏:&[亊],丞:&mut Vec<亊>)->O{let 乍:K<_> =丞.iter().map(|r|r.万.clone()).collect();for
乼 in 乏{if 丞.iter().any(|r|r.万==乼.万){continue;}丞.push(乼.clone());}丞.sort_by_key(|乼|乼.侙);丞.truncate(傑);丞.iter().map(|r|&r.万).ne(&乍)}fn 兂(l:&J,乓:亯,亅:&mut
交)->侄{偒(l,乓,亅,None)}fn 偒(l:&J,乓:亯,亅:&mut 交,乛:P<们>,)->侄{let 丂=l.东();let mut 乁=侄::default();if l.丢().is_some(){return 乁;}if let Some(万)=佛(l){乁.万=万;return
乁;}let 乚=|儫:亿|match(乓.亻,乓.仞){(Some(亻),Some(仞))=>Some(亻+(仞-亻).mul_f64(儫)),_=>None,};if 偽.is_some_and(|丕|l.书()>=丕){亅.end();}if 亅.儘(l){let 万=亅.丕(l,乚(0.4));let
僐=万.iter().any(|丒|{matches!(丒,L::举{..}|L::亭{..}|L::乿{..})});let 亶=偻&&!僐&&l.书()>=偼&&l.下(丂)>=偺;if!亶{乁.亅=true;乁.万=万;return 乁;}亅.end();}let mut 乡=侂{丂,limit:亯{亻:乚(偫),..乓},亦:0,亵:仫::default(),仨:None,俀:儷(l),乛,佞:std::cell::RefCell::default(),佟:std::cell::RefCell::default(),};let
mut 丞=K::new();let mut 丶:K<仭> =K::new();let mut 伙=K::new();let mut 佑=false;let mut 亖:K<丫> =K::new();let mut 乀=乡.乛.as_ref().filter(|g|g.start).map(|g|丫::争(g.世.clone()));while
乁.侖<傉&&(丶.is_empty()||!乡.于()){let 儲=match 乀.take(){Some(井)=>乡.亖(l,&井,&丞),None=>None,};let 世=if let Some(世)=儲{世}else if 佑{let Some(井)=亖.pop()else{break;};match
乡.亖(l,&井,&丞){Some(世)=>世,None=>continue,}}else{乡.佋(l,&丞,&伙,true)};乁.侖+=1;let 偂=丶.iter().any(|p|p.万==世.万);let 乬=世.first;let 偋=乡.于();let mut 世=世;let mut 倪=false;if!偋&&!偂{let
乏=乡.俸(&世.end);倪=儠(&乏,&mut 丞);世.丛=Some(乏);}if!偂{丶.push(世);}if 偋{break;}if!倪&&!佑{match 乬{Some(乬)=>伙.push(乬),None=>{佑=true;亖=乡.仨.clone().unwrap_or_default();亖.retain(|a|!伙.contains(&亄(a.万())));let
久=|a:&丫|{乡.亵.get(&亄(a.万())).copied().unwrap_or(C::MIN)};亖.sort_by_key(久);}}}}乁.亖=F::from(佑);乁.亦=乡.亦;乁.丞=丞.len();乁.丶=丶.len();let 仆=||乓.仞.is_some_and(|t|亇::now()>=t);let
mut 伊:K<P<C>> =vec![None;丶.len()];for 丠 in(0..丶.len()).rev(){if 丠+1<丶.len()&&仆(){break;}伊[丠]=Some(乡.倔(&丶[丠],&丞));}let 伈=|伊:&[P<C>]|{let mut 专:P<(C,F)>
=None;for(丠,丮)in 伊.iter().enumerate().rev(){if let Some(丮)=*丮{if 专.is_none_or(|(亞,_)|丮>亞){专=Some((丮,丠));}}}专};let mut 专=伈(&伊);乡.limit.亻=乓.仞;while let Some((_,丠))=专{if!傽||丶[丠].丛.is_some()||仆(){break;}let
乏=乡.俸(&丶[丠].end);丶[丠].丛=Some(乏);乁.倇+=1;伊[丠]=Some(乡.倔(&丶[丠],&丞));专=伈(&伊);}if let Some((_,丠))=专{乁.偌=乡.乛.as_ref().is_some_and(|g|!g.世.is_empty()&&丶[丠].万.starts_with(&g.世));乁.介=乡.乛.as_ref().map_or(0,|g|{g.介.iter().filter(|p|p.iter().all(|c|丶[丠].万.contains(c))).count()});let
mut 丌=l.clone();for&丒 in&丶[丠].万{if 丌.乇(丒){乁.万.push(丒);}}}乁}fn 儗(乓:&dyn Fn(亇,Q)->亯,mut 亅:交,侦:impl BufRead,佡:impl Write,)->Result<(),乙>{let mut 享=伔::default();个(侦,佡,|仼,亏,丕,乀|match
J::read(仼,亏,亢::佲,丕){Ok(l)=>{let 仿=乛(&l,&mut 享);let 乁=match 仿{Some(乛)=>{偒(&l,乓(乀,丕),&mut 亅,Some(乛))}None=>兂(&l,乓(乀,丕),&mut 亅),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",丕,乁.侖,乁.亦,乁.丞,乁.倇,乁.亖,I::from(乁.偌),乁.介,乀.elapsed());僅(&乁.万)}Err(仙)=>{eprintln!("plan: cannot read the input: {}"
,仙);"WAIT".to_string()}},)}const 傘:P<C> =None;const 傼:O=false;#[derive(Clone,Copy,PartialEq)]enum 伓{佴,偩,}#[derive(Clone,Copy)]struct 亱{q:F,kind:伓,仟:O,value:C,}fn
乨(l:&J,丁:F)->K<亱>{儋(l,丁,傼)}fn 儋(l:&J,丁:F,儻:O)->K<亱>{let(么,中)=佔(l);let(丹,且)=(l.七(丁),l.七(1-丁));let(传,伡)=(丳(l,丹),丳(l,且));let 会=伩(l,&传,&传,&伡);let mut 丅=K::new();for
q in(么|中).iter(){let 侚=丳(l,G::q(q));let 仟=丹.test(q);let 丬=if 儻{伩(l,&传,&侚,&伡)-伩(l,&伡,&侚,&传)}else if 仟{let 偠=丳(l,僰(丹,丁,q));会-伩(l,&偠,&偠,&伡)}else{伩(l,&传,&侚,&伡)-会};丅.push(亱{q,kind:if
中.test(q){伓::佴}else{伓::偩},仟,value:丬,});}丅.sort_by_key(|o|(-o.value,o.kind!=伓::佴,o.q));丅}fn 伩(l:&J,a:&[I;M],g:&[I;M],b:&[I;M])->C{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 仄(乨:&[亱])->K<亱>{乨.iter().filter(|o|o.value>=傘.unwrap_or(1)).copied().collect()}fn 僰(七:G,丁:F,q:F)->G{let
乪=七.丗(G::q(q));let mut 乭=G::q(T[丁])&乪;loop{let 丄=乭|(乭.丏()&乪);if 丄==乭{return 乭;}乭=丄;}}const 俬:C=10;const 傖:C=100;#[derive(Clone,Copy)]enum 佀{众(Q),举(F),}#[derive(Clone,Copy)]struct
乾{个:Q,侤:佀,}#[derive(Clone,Copy)]struct 伕{q:F,ours:P<乾>,你:P<乾>,theirs:P<乾>,亩:C,}fn 儝(l:&J,丁:F,乨:&[亱])->K<伕>{let mut 丅=K::new();for 临 in 乨.iter().filter(|o|!o.仟){let
q=临.q;let(儑,你)=俹(l,丁,q);let 严=专(儑,你);let(允,兀)=俹(l,1-丁,q);let 且=专(允,兀);let 个=|a:P<乾>|a.map_or(傖,|a|C::from(a.个));丅.push(伕{q,ours:严,你,theirs:且,亩:个(且)-个(严),});}丅}fn
专(亼:P<乾>,伷:P<乾>)->P<乾>{match(亼,伷){(Some(u),Some(t))=>Some(if t.个<u.个{t}else{u}),(u,t)=>u.or(t),}}fn 俹(l:&J,丷:F,q:F)->(P<乾>,P<乾>){let 並=l.map();let mut
亼:P<乾> =None;for 三 in l.丈(丷).iter(){let d=並.丙(三,q);let Some(上)=l.亐(三)else{continue;};if d!=I::MAX&&亼.is_none_or(|u|Q::from(d)<u.个){亼=Some(乾{个:Q::from(d),侤:佀::众(上),});}}let
下=l.下(丷)+if 丷==l.东(){0}else{l.丟(丷)};let 佭=match(下,l.丟(丷)){(下,_)if 下>=俬=>Some(0),(_,丟)if 丟<=0=>None,(下,丟)=>Some((俬-下+丟-1)/丟),};let mut 伷:P<乾> =None;if let
Some(佭)=佭{let 乲=l.七(丷).丏()&並.valid&l.乌(丷,1);for 三 in 乲.iter(){let d=並.丙(三,q);if d!=I::MAX&&伷.is_none_or(|t|1+佭 as Q+Q::from(d)<t.个){伷=Some(乾{个:1+佭 as Q+Q::from(d),侤:佀::举(三),});}}}(亼,伷)}const
俭:C=10;#[derive(Default)]struct 伔{倯:BTreeMap<Q,F>,}#[derive(Default)]struct 仂{仛:K<(Q,F)>,亟:K<(F,F)>,back:K<Q>,}fn 仳(l:&J,丁:F,乨:&[亱],以:&[伕],丞:&mut 伔,)->仂{俺(l,丁,乨,以,丞,l.丈(丁))}fn
俺(l:&J,丁:F,乨:&[亱],以:&[伕],丞:&mut 伔,供:G,)->仂{let 並=l.map();let 丈:BTreeMap<Q,F> =(l.丈(丁)&供).iter().filter_map(|c|l.亐(c).map(|上|(上,c))).collect();let mut 両:K<(亱,伕)>
=K::new();for o in 仄(乨).into_iter().filter(|o|!o.仟){if let Some(r)=以.iter().find(|r|r.q==o.q){if r.ours.is_some()&&r.亩>=0{両.push((o,*r));}}}両.sort_by_key(|(o,r)|{(-o.value,o.kind!=伓::佴,r.ours.map_or(Q::MAX,|a|a.个),o.q,)});let
乄=|亼:Q,丘:F|Q::from(並.丙(丈[&亼],丘));let 倢=|亗:&BTreeMap<F,Q>,伯:&BTreeSet<Q>,上:Q|{伯.contains(&上)||亗.values().any(|&u|u==上)};let mut 亗:BTreeMap<F,Q> =BTreeMap::new();for(&上,&丘)in&丞.倯{if
丈.contains_key(&上)&&両.iter().any(|(o,_)|o.q==丘)&&!亗.contains_key(&丘){亗.insert(丘,上);}}let mut 伯:BTreeSet<Q> =BTreeSet::new();for(o,_)in&両{let Some(&値)=亗.get(&o.q)else{continue;};let
侱=丈.keys().copied().filter(|&上|!倢(&亗,&伯,上)).map(|上|(乄(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();if let Some((d,上))=侱{if d<乄(値,o.q){伯.insert(値);亗.insert(o.q,上);}}}let
mut 亟=K::new();for(o,r)in&両{if 亗.contains_key(&o.q){continue;}let 丧=丈.keys().copied().filter(|&上|!倢(&亗,&伯,上)).map(|上|(乄(上,o.q),上)).filter(|&(d,_)|d!=Q::from(I::MAX)).min();let
偘=丧.map(|(d,_)|C::from(d));let 佫=r.你.and_then(|t|match t.侤{佀::举(q)if l.下(丁)-俭*亟.len()as C>=俭&&!亟.iter().any(|&(c,_)|c==q)=>{Some((q,C::from(t.个)))}_=>None,});let
偕=佫.map(|(_,t)|t);let 偙=match(偘,偕){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 僕=if 偙{偕}else{偘};let Some(个)=僕 else{continue;};if r.theirs.map_or(100,|a|C::from(a.个))<个{continue;}match(偙,佫,丧){(true,Some((q,_)),_)=>亟.push((q,o.q)),(false,_,Some((_,上)))=>{亗.insert(o.q,上);}_=>{}}}let
mut 丅=仂{亟,..仂::default()};for(o,_)in&両{if let Some(&上)=亗.get(&o.q){丅.仛.push((上,o.q));}}丅.back=l.丈(丁).iter().filter_map(|c|l.亐(c)).filter(|上|!丅.仛.iter().any(|&(u,_)|u==*上)).collect();丅.back.sort_unstable();丞.倯=丅.仛.iter().copied().collect();丅}const
修:C=10;fn 倅(l:&J,丁:F,上:Q)->P<F>{l.丈(丁).iter().find(|&c|l.亐(c)==Some(上))}fn 偊(l:&J,丁:F,三:F,丘:F)->K<F>{let 並=l.map();let 仅=並.丙(三,丘);if 仅==I::MAX||仅==0{return
K::new();}let mut 丅:K<F> =(G::q(三).丏()&並.valid).iter().filter(|&n|並.丙(n,丘)<仅).collect();丅.sort_by_key(|&n|(l.丐(丁).test(n),n));丅}fn 亥(丌:&mut J,丅:&mut Vec<L>,丒:L)->O{let
倲=丌.乇(丒);if 倲{丅.push(丒);}倲}#[derive(Clone,Copy)]struct 仯{侸:O,guard:I,倓:O,偁:O,}fn 介(l:&J,享:&仂,乶:仯)->K<K<L>>{let mut 丅=K::new();for&(上,丘)in&享.仛{let 儏=仂{仛:vec![(上,丘)],..仂::default()};let
佢=偍(l,&儏,仯{侸:false,..乶},);if!佢.is_empty(){丅.push(佢);}}丅}fn 丣(丌:&J,上:Q,三:F,两:F,产:O,乶:仯,)->P<K<L>>{let 丁=丌.东();let mut 丵=丌.clone();let mut 万=K::new();let(x,y)=乕(两);if!亥(&mut
丵,&mut 万,L::乷{上,x,y}){return None;}if 产&&乶.guard!=0{let(仜,仝)=乕(三);let 価=if 乶.guard==1{L::乿{x:仜,y:仝}}else{L::举{丆:1,x:仜,y:仝,}};if!亥(&mut 丵,&mut 万,価){return
None;}}if 乶.偁&&伴(&丵,丁)>伴(丌,丁){return None;}Some(万)}fn 偍(l:&J,享:&仂,乶:仯)->K<L>{let 丁=l.东();let 並=l.map();let mut 丌=l.clone();let mut 丅=K::new();let 丩=丩(l,1-丁);let
mut 伀:K<(Q,F)> =K::new();for&(上,丘)in&享.仛{let Some(三)=倅(&丌,丁,上)else{continue;};if!丌.乺().test(三){continue;}let 产=丩.test(三);for 两 in 偊(&丌,丁,三,丘){if let Some(万)=丣(&丌,上,三,两,产,乶){for
丒 in&万{丌.乇(*丒);}if 产&&乶.guard==0{伀.push((上,三));}丅.extend(万);break;}}}for&(上,丘)in&享.仛{if 丌.下(丁)<修{break;}let Some(乚)=倅(&丌,丁,上)else{continue;};let 僊=伀.iter().find(|&&(u,_)|u==上).map(|&(_,c)|c);let
丑=match 僊{Some(q)if 乶.倓=>vec![q],_=>偊(&丌,丁,乚,丘),};for 两 in 丑{let(x,y)=乕(两);if 亥(&mut 丌,&mut 丅,L::举{丆:1,x,y}){break;}}}if 乶.侸{for&(_,丘)in&享.亟{if 丌.下(丁)<修{break;}let
乲=丌.七(丁).丏()&並.valid;let mut 丑:K<F> =乲.iter().filter(|&c|並.丙(c,丘)!=I::MAX).collect();丑.sort_by_key(|&c|(並.丙(c,丘),c));for 两 in 丑{let(x,y)=乕(两);if 亥(&mut
丌,&mut 丅,L::举{丆:1,x,y}){break;}}}}丅}const 俯:I=0;const 傶:I=1;const 俠:C=0;const 備:O=false;const 偹:I=0;const 偰:O=false;const 傩:O=false;const 健:O=false;const
偵:Q=7;const 傸:Q=14;const 傰:O=俯!=0||俠!=0;struct 们{世:K<L>,倱:O,start:O,介:K<K<L>>,乨:K<(F,C,C)>,伉:C,}impl 们{fn 儾(&self,l:&J,丁:F)->C{if self.伉==0{return 0;}let(丹,且)=(l.七(丁),l.七(1-丁));let
mut 了=0;for&(q,丬,亙)in&self.乨{了+=if 丹.test(q){丬}else if 且.test(q){-丬}else{亙};}self.伉*了/4}}fn 乛(l:&J,丞:&mut 伔)->P<们>{let 伪=l.书();if!傰||!(偵..=傸).contains(&伪)||l.丢().is_some(){return
None;}僷(l,丞,傫)}#[derive(Clone,Copy)]struct 佄{仇:I,乶:仯,侌:O,伉:C,侵:O,}const 傫:佄=佄{仇:俯,乶:仯{侸:傶==2,guard:偹,倓:偰,偁:傩,},侌:健,伉:俠,侵:備,};fn 僷(l:&J,丞:&mut 伔,乆:佄)->P<们>{let
佄{仇,乶,侌,伉,侵,}=乆;let 丁=l.东();let 乨=乨(l,丁);let 以=儝(l,丁,&乨);let 享=if 侌{let 供=l.丈(丁).丗(丩(l,1-丁));俺(l,丁,&乨,&以,丞,供)}else{仳(l,丁,&乨,&以,丞)};let 世=if 仇!=0{偍(l,&享,乶)}else{K::new()};let
仄=仄(&乨);let 儿=仄.iter().map(|o|{let 亩=以.iter().find(|r|r.q==o.q).map_or(0,|r|r.亩.clamp(-1,1));let 亙=if 侵{o.value*亩}else{0};(o.q,o.value,亙)}).collect();let
儓=if 仇&4!=0{介(l,&享,乶)}else{K::new()};Some(们{介:儓,倱:!世.is_empty()&&仇&1!=0,start:!世.is_empty()&&仇&2!=0,世,乨:儿,伉,})}const 偧:亿=45.0;const 偱:亿=800.0;const 傮:亿=0.6;fn
乓(乀:亇,丕:Q)->亯{if let Some(伂)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return 亯{亻:None,仞:None,伂,};}let 佧=兄().min(100.0);let
儈=if 丕==1{偱}else{偧};let 乎=Duration::from_secs_f64(儈/1000.0*佧);亯{亻:Some(乀+乎.mul_f64(傮)),仞:Some(乀+乎),伂:丯::MAX,}}const 傛:F=9;const 傗:什=什{丑:3,丟:1,下:1,串:2,二:2,伶:2,丩:6,仪:1,中:0,仳:false,习:傞,};const
傡:Q=0;const 傞:亁=亁::佷{仴:8};const 俤:I=I::MAX;const 俢:C=2;const 俣:C=18;const 信:C=0;const 傚:F=32;const 偣:O=false;const 偤:Settings1=Settings1{侼:3,侻:1,侔:4,仆:2,侜:2,佐:2,係:true,reserve:true,依:true,乤:仁::乸,fill:true,乘:Z{佦:俤,件:俢,佗:俣,但:信,..Z::仔},};fn
亅()->交{if 偣{return 交::new(亓::似(偤),K::new(),Z::仔);}儐(傚)}fn 儐(亡:F)->交{let 亝=亓::伖{个:傛,亡,临:傗,佤:傡,};let 丰=傪.iter().map(|&乘|Z{佦:俤,件:俢,佗:俣,但:信,..乘}).collect();交::new(亝,丰,Z::仔)}fn
main(){let 儳=io::stdin();let 儴=io::stdout();if let Err(仙)=儗(&乓,亅(),儳.lock(),儴.lock()){eprintln!("plan: {}",仙);std::process::exit(1);}}const 傴:&str="CG_TIME_SCALE";fn
兄()->亿{儒(std::env::var(傴).ok().as_deref())}fn 儒(丬:P<&str>)->亿{丬.and_then(|亏|亏.trim().parse::<亿>().ok()).filter(|佧|佧.is_finite()&&*佧>0.0).unwrap_or(1.0)}const
_:()=assert!(偯==16);const 偯:F=16;const 偪:亿=0.4;const 傒:I=4;#[derive(Clone,Copy)]struct 仕{中:G,倆:G,伧:G,}impl 仕{fn of(l:&J)->仕{let(a,伝)=倵(l,T[0]);let(b,住)=倵(l,T[1]);let
了=a[T[1]];let 儀=伝[T[1]];let mut 乂=仕{中:G::U,倆:G::U,伧:G::U,};for q in l.map().valid.iter(){if 伝[q]==I::MAX||住[q]==I::MAX{continue;}if 伝[q].abs_diff(住[q])<=1{乂.倆.set(q);}if
伝[q].abs_diff(住[q])<=傒{乂.伧.set(q);}let 儌=伻::from(伝[q])+伻::from(住[q])==伻::from(儀);if 了>0.0&&儌&&q!=T[0]&&q!=T[1]&&a[q]*b[q]/了>=偪{乂.中.set(q);}}乂}fn 仡(&self)->G{self.中&self.伧}}fn
儶(l:&J,儰:G)->[I;M]{let 丱=l.map().valid;let mut 丅=[I::MAX;M];let mut 乭=儰&丱;let mut 仿=乭;let mut 丣=0u8;while!仿.is_empty(){for q in 仿.iter(){丅[q]=丣;}let 丄=(仿.丏()&丱).丗(乭);乭|=丄;仿=丄;丣=丣.saturating_add(1);}丅}fn
倵(l:&J,三:F)->([亿;M],[I;M]){let 乄=儶(l,G::q(三));let mut 习:K<F> =l.map().valid.iter().filter(|&c|乄[c]!=I::MAX).collect();习.sort_by_key(|&c|乄[c]);let mut 业=[0.0;M];业[三]=1.0;for
q in 习{if q==三{continue;}let 也=G::q(q).丏()&l.map().valid;业[q]=也.iter().filter(|&n|乄[n]!=I::MAX&&乄[n]+1==乄[q]).map(|n|业[n]).sum();}(业,乄)}const Y:F=12;const
M:F=Y*Y;const 予:F=4*Y;const 亣:丯=(1<<予)-1;const 伽:丯=1|1<<12|1<<24|1<<36;const 佳:丯=伽<<11;const 俨:丯=(1<<Y)-1;#[derive(Clone,Copy,Debug,PartialEq)]struct G([丯;3]);impl
G{const U:G=G([0;3]);const 偢:G=G([亣;3]);#[inline]fn q(q:F)->G{let mut 乒=G::U;乒.set(q);乒}#[inline]fn test(self,q:F)->O{(self.0[q/予]>>(q%予))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/予]|=1<<(q%予);}#[inline]fn clear(&mut self,q:F){self.0[q/予]&=!(1<<(q%予));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伸(self)->[丯;3]{self.0}#[inline]fn count(self)->Q{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丗(self,主:G)->G{G([self.0[0]&!主.0[0],self.0[1]&!主.0[1],self.0[2]&!主.0[2],])}#[inline]fn
iter(self)->佱{佱{侫:self.0,乚:0,}}#[inline]fn 佩(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&俨)<<(予-Y)),(b>>Y)|((c&俨)<<(予-Y)),c>>Y,])}#[inline]fn 佨(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亣,((b<<Y)&亣)|(a>>(予-Y)),((c<<Y)&亣)|(b>>(予-Y)),])}#[inline]fn
偄(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佳&亣,(b>>1)&!佳&亣,(c>>1)&!佳&亣,])}#[inline]fn 偅(self)->G{let[a,b,c]=self.0;G([(a<<1)&!伽&亣,(b<<1)&!伽&亣,(c<<1)&!伽&亣,])}#[inline]fn
丏(self)->G{let 一=self;一.佩()|一.佨()|一.偄()|一.偅()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,主:G)->G{G([self.0[0]&主.0[0],self.0[1]&主.0[1],self.0[2]&主.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,主:G)->G{G([self.0[0]|主.0[0],self.0[1]|主.0[1],self.0[2]|主.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,主:G){*self=*self|主;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::偢.丗(self)}}struct 佱{侫:[丯;3],乚:F,}impl Iterator for 佱{type Item=F;#[inline]fn
next(&mut self)->P<F>{let 一=self;while 一.乚<3{let 作=一.侫[一.乚];if 作!=0{一.侫[一.乚]=作&(作-1);return Some(一.乚*予+作.trailing_zeros()as F);}一.乚+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乷{上:Q,x:I,y:I},举{丆:I,x:I,y:I},亭{x:I,y:I},乿{x:I,y:I},}impl L{fn 兇(&self)->乙{match*self{L::乷{上,x,y}=>format!("MOVE {} {x} {y}",上),L::举{丆,x,y}=>format!("TRAIN {} {x} {y}",丆),L::亭{x,y}=>format!("BUILD MINE {x} {y}"),L::乿{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
僅(万:&[L])->乙{if 万.is_empty(){return "WAIT".to_string();}let mut line=乙::with_capacity(16*万.len());for(丠,丒)in 万.iter().enumerate(){if 丠>0{line.push(';');}line.push_str(&丒.兇());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亢{俴,兙,僁,#[default]佲,}impl 亢{fn 亘(self)->I{match self{亢::俴=>1,_=>3,}}fn 佇(self)->O{matches!(self,亢::僁|亢::佲)}fn 仲(self)->O{self==亢::佲}}const 互:I=I::MAX;struct
亰{乣:G,valid:G,丼:G,丙:Box<[I]>,丏:[[[I;4];M];2],}thread_local!{static 傆:RefCell<P<Arc<亰>>> =const{RefCell::new(None)};}impl 亰{fn 儬(乣:G,丼:G)->Arc<亰>{傆.with(|乹|{let
mut 乹=乹.borrow_mut();if let Some(並)=乹.as_ref(){if 並.乣==乣&&並.丼==丼{return Arc::clone(並);}}let 並=Arc::new(亰::build(乣,丼));*乹=Some(Arc::clone(&並));並})}fn build(乣:G,丼:G)->亰{let
丱=!乣;const 傯:[[(伺,伺);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[互;4];M];2];for(丁,之)in 傯.iter().enumerate(){for q in
丱.iter(){let(x,y)=((q%Y)as 伺,(q/Y)as 伺);let mut 业=0;for&(僡,僢)in 之{let(倭,倮)=(x+僡,y+僢);if(0..Y as 伺).contains(&倭)&&(0..Y as 伺).contains(&倮){let 丄=倮 as F*Y+倭
as F;if 丱.test(丄){丏[丁][q][业]=丄 as I;业+=1;}}}}}let mut 丙=vec![互;M*M].into_boxed_slice();let mut 侴=[0u8;M];for 三 in 丱.iter(){let 乽=&mut 丙[三*M..(三+1)*M];乽[三]=0;侴[0]=三
as I;let(mut 侢,mut 俁)=(0,1);while 侢<俁{let q=侴[侢]as F;侢+=1;for&丄 in&丏[0][q]{if 丄!=互&&乽[丄 as F]==互{乽[丄 as F]=乽[q]+1;侴[俁]=丄;俁+=1;}}}}亰{乣,valid:丱,丼,丙,丏,}}#[inline]fn
丙(&self,三:F,两:F)->I{self.丙[三*M+两]}}const 傐:Q=100;const 云:C=15;const 傺:[C;3]=[1,4,20];const T:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 乳{五(F),俑,}enum
亀{侁,佼,佾,}#[derive(Clone)]struct J{map:Arc<亰>,丿:亢,丛:[G;2],七:[G;2],丹:G,乫:G,丈:[G;2],丆:[G;3],仉:G,亐:[Q;M],下:[C;2],丟:[C;2],东:F,书:Q,伅:Q,丢:P<乳>,仠:O,}fn 倜(mut 乩:G,丛:G)->G{loop{let
丄=(乩|乩.丏())&丛;if 丄==乩{return 乩;}乩=丄;}}fn 事(丛:G,任:F)->G{if!丛.test(任){return G::U;}let mut 乩=G::q(任);loop{let 丄=(乩|乩.丏())&丛;if 丄==乩{return 乩;}乩=丄;}}impl
J{fn 东(&self)->F{self.东}fn 书(&self)->Q{self.书}fn 丢(&self)->P<乳>{self.丢}fn 丿(&self)->亢{self.丿}fn 伄(&self,丁:F)->C{let 一=self;let 丈=一.丈[丁];一.下[丁]+10*C::from((丈&一.丆[0]).count())+20*C::from((丈&一.丆[1]).count())+30*C::from((丈&一.丆[2]).count())}fn
下(&self,丁:F)->C{self.下[丁]}fn 丟(&self,丁:F)->C{self.丟[丁]}fn 俄(&self,丁:F)->F{self.丈[丁].count()as F}fn map(&self)->&亰{&self.map}fn 丐(&self,丁:F)->G{self.丛[丁]}fn
七(&self,丁:F)->G{self.七[丁]}fn 丈(&self,丁:F)->G{self.丈[丁]}fn 亽(&self,q:F)->P<(Q,I)>{let 一=self;(一.丈[0]|一.丈[1]).test(q).then(||(一.亐[q],一.侪(q)))}fn 乺(&self)->G{self.丈[self.东]&self.仉}fn
为(&self)->G{self.丹}fn 乔(&self)->G{self.乫}fn 亠(&self,q:F)->P<I>{(q<M&&(self.丈[0]|self.丈[1]).test(q)).then(||self.侪(q))}fn 亐(&self,q:F)->P<Q>{(q<M&&(self.丈[0]|self.丈[1]).test(q)).then(||self.亐[q])}fn
儱(&mut self,丁:F){let 一=self;if 一.仠{debug_assert_eq!(一.七,[事(一.丛[0],T[0]),事(一.丛[1],T[1])]);}else{一.七=[事(一.丛[0],T[0]),事(一.丛[1],T[1])];一.仠=true;}let 僛=一.丈[丁].丗(一.七[丁]);一.侷(僛);一.倉();一.下[丁]+=一.丟[丁];if
一.下[丁]<0{一.下[丁]=0;一.侷(一.丈[丁]);}一.仉|=一.丈[丁];}fn 倉(&mut self){let 一=self;for 丁 in 0..2{let 七=一.七[丁];let 丈=七&一.丈[丁];let mut 丟=C::from(七.count())+4*C::from((七&一.丹).count());for(丆,免)in
傺.iter().enumerate(){丟-=免*C::from((丈&一.丆[丆]).count());}一.丟[丁]=丟;}}fn 仸(&mut self){let 一=self;if 一.丢.is_some(){return;}if 一.东==1{if 一.书>=傐{一.丢=Some(match
一.伄(0).cmp(&一.伄(1)){std::cmp::Ordering::Greater=>乳::五(0),std::cmp::Ordering::Less=>乳::五(1),std::cmp::Ordering::Equal=>乳::俑,});return;}一.书+=1;}一.东=1-一.东;一.儱(一.东);}fn
佣(&mut self,万:&[L]){let 一=self;if 一.丢.is_some(){return;}let 侟=一.伅;for&丒 in 万{if 一.僠(丒,侟){continue;}一.乇(丒);if 一.丢.is_some(){return;}}一.仸();}fn 僠(&self,丒:L,侟:Q)->O{let
一=self;match 丒{L::乷{上,x,y}=>{一.仵(x,y).is_none()||上>=侟||一.倕(一.东,上).is_none()}L::举{丆,x,y}=>{一.仵(x,y).is_none()||丆==0||丆>一.丿.亘()}L::亭{x,y}|L::乿{x,y}=>{一.仵(x,y).is_none()}}}#[inline]fn
仵(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<Y&&y<Y&&!self.map.乣.test(y*Y+x)).then_some(y*Y+x)}fn 倕(&self,丁:F,上:Q)->P<F>{self.丈[丁].iter().find(|&q|self.亐[q]==上)}#[inline]fn
侪(&self,q:F)->I{if self.丆[0].test(q){1}else if self.丆[1].test(q){2}else{3}}fn 僌(&self,亚:F)->G{let 乔=self.乫&self.丛[亚];let 七=self.七[亚];乔|(七&(七&乔).丏())}fn
乌(&self,丁:F,丆:I)->G{let 一=self;debug_assert!((1..=3).contains(&丆),"unit level {} is not 1 to 3",丆);let 亚=1-丁;let 丈=一.丈[0]|一.丈[1];let 僥=一.付();let 克=match
丆{1 if 一.丿==亢::俴=>一.丆[1]|一.丆[2],1=>丈,2=>一.丆[1]|一.丆[2],_=>G::U,};let 乌=僥|一.丛[亚].丗(克);if 丆<3{乌.丗(一.僌(亚))}else{乌}}fn 今(&self,丁:F)->G{self.七[丁]|self.七[丁].丏()}fn
付(&self)->G{let 一=self;let 丈=一.丈[0]|一.丈[1];let 僺=G::q(T[0])|G::q(T[1]);一.map.valid.丗(丈|一.丹|一.乫|僺)}fn 乇(&mut self,丒:L)->O{let 一=self;if 一.丢.is_some(){return
false;}let 丁=一.东;let 他=match 丒{L::举{丆,x,y}=>一.佫(丁,丆,x,y),L::乷{上,x,y}=>一.儇(丁,上,x,y),L::亭{x,y}=>一.build(丁,true,x,y),L::乿{x,y}=>一.build(丁,false,x,y),};let
Some(他)=他 else{return false;};match 他{亀::侁=>{}亀::佼 if 一.仠=>{一.七[丁]=倜(一.七[丁],一.丛[丁]);}亀::佾 if 一.仠=>{一.七[丁]=倜(一.七[丁],一.丛[丁]);一.七[1-丁]=事(一.丛[1-丁],T[1-丁]);}亀::佼=>一.七[丁]=事(一.丛[丁],T[丁]),亀::佾=>{一.七=[事(一.丛[0],T[0]),事(一.丛[1],T[1])];}}debug_assert!(!一.仠||一.七==[事(一.丛[0],T[0]),事(一.丛[1],T[1])]);一.倉();if!一.丛[0].test
(T[0]){一.丢=Some(乳::五(1));}else if!一.丛[1].test(T[1]){一.丢=Some(乳::五(0));}true}fn 侷(&mut self,丑:G){let 一=self;一.丈[0]=一.丈[0].丗(丑);一.丈[1]=一.丈[1].丗(丑);for 丆
in&mut 一.丆{*丆=丆.丗(丑);}一.仉=一.仉.丗(丑);}fn 偑(&mut self,丁:F,q:F)->亀{let 一=self;一.侷(G::q(q));一.丹.clear(q);一.乫.clear(q);let 亚=1-丁;if 一.丛[亚].test(q){一.丛[亚].clear(q);一.丛[丁].set(q);亀::佾}else
if 一.丛[丁].test(q){亀::侁}else{一.丛[丁].set(q);亀::佼}}fn 佫(&mut self,丁:F,丆:I,x:I,y:I)->P<亀>{let 一=self;if 丆==0||丆>一.丿.亘(){return None;}let q=一.仵(x,y)?;let 与=10*C::from(丆);if
一.下[丁]<与||!一.今(丁).test(q)||!一.乌(丁,丆).test(q){return None;}let 他=一.偑(丁,q);一.倶(丁,丆,q,一.伅);一.伅=一.伅.saturating_add(1);一.下[丁]-=与;Some(他)}fn 倶(&mut self,丁:F,丆:I,q:F,上:Q){self.丈[丁].set(q);self.丆[F::from(丆)-1].set(q);self.亐[q]=上;}fn
儇(&mut self,丁:F,上:Q,x:I,y:I)->P<亀>{let 一=self;let 不=一.仵(x,y)?;let 三=一.倕(丁,上)?;if!一.仉.test(三){return None;}let 丆=一.侪(三);let 丨=一.乌(丁,丆);let 也=一.map.丏[丁][三];if
也.contains(&(不 as I))&&!丨.test(不){return None;}let mut 专=一.map.丙(三,不);let mut 伟=None;for&丄 in 也.iter().take_while(|&&丄|丄!=互){let 丄=F::from(丄);let 丙=一.map.丙(丄,不);if
丙<专&&丨.test(丄){专=丙;伟=Some(丄);}}let 伟=伟?;let 他=一.偑(丁,伟);一.丈[丁].clear(三);一.丆[F::from(丆)-1].clear(三);一.仉.clear(三);一.倶(丁,丆,伟,上);Some(他)}fn build(&mut self,丁:F,丹:O,x:I,y:I)->P<亀>{let
一=self;let 侊=if 丹{一.丿.佇()}else{一.丿.仲()};let q=一.仵(x,y)?;let 儍=一.map.丼.test(q);if!侊||一.下[丁]<云||儍!=丹||!一.丛[丁].test(q)||!一.今(丁).test(q)||!一.付().test(q){return
None;}if 丹{一.丹.set(q);}else{一.乫.set(q);}一.下[丁]-=云;Some(亀::侁)}}const 傋:C=1000000000;const 佺:C=0;const 傓:C=1;const 債:C=2;impl J{fn read(仼:&str,丕:&str,丿:亢,书:Q,)->Result<J,乙>{let
mut 丼=K::new();let mut 乊=仼.lines();let 业=侘(&mut 乊,"mine spot count")?;for _ in 0..业{let[x,y]=亹(伆(&mut 乊,"a mine spot")?,"a mine spot")?;丼.push((位(x)?,位(y)?));}們(乊,"the initial input")?;let
mut 乊=丕.lines();let mut 侣=[0i64;4];for(丬,九)in 侣.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[侲]=亹(伆(&mut 乊,九)?,九)?;if 侲.abs()>傋{return
Err(format!("{}: {} is out of range",九,侲));}*丬=侲;}let mut 乣=G::U;let mut 丑=[G::U;2];let mut 七=[G::U;2];for y in 0..Y{let 乽=伆(&mut 乊,"a map row")?.trim();if
乽.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,乽));}for(x,伲)in 乽.bytes().enumerate(){let q=y*Y+x;match 伲{b'#'=>乣.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乮=F::from(伲==b'X'||伲==b'x');丑[乮].set(q);if 伲.is_ascii_uppercase(){七[乮].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伲 as char))}}}}let
像=侘(&mut 乊,"building count")?;let mut 亂=K::new();for _ in 0..像{let 亹=亹(伆(&mut 乊,"a building")?,"a building")?;亂.push(亹);}let 俄=侘(&mut 乊,"unit count")?;let
mut 丈=K::new();for _ in 0..俄{let 亹=亹(伆(&mut 乊,"a unit")?,"a unit")?;丈.push(亹);}們(乊,"the turn's input")?;let 倳=亂.iter().find(|倀|倀[0]==0&&倀[1]==佺).ok_or("the bot's own HQ is not among the buildings")?;let
借=侐(倳[2],倳[3],乣)?;let 丁=T.iter().position(|&任|任==借).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",借))?;let mut 偆=G::U;for(x,y)in 丼{if
乣.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}偆.set(y*Y+x);}let 侈=|乮:F|if 乮==0{丁}else{1-丁};let mut l=J{map:亰::儬(乣,偆),丿,丛:[G::U;2],七:[G::U;2],丹:G::U,乫:G::U,丈:[G::U;2],丆:[G::U;3],仉:G::U,亐:[0;M],下:[0;2],丟:[0;2],东:丁,书,伅:1,丢:None,仠:false,};for
乮 in 0..2{let 亜=侈(乮);l.丛[亜]=丑[乮];l.七[亜]=七[乮];l.下[亜]=侣[2*乮];l.丟[亜]=侣[2*乮+1];}let mut 倰=G::U;for[亜,余,x,y]in 亂{let 乮=值(亜)?;let q=侐(x,y,乣)?;if!丑[乮].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 倰.test(q){return Err(format!("two buildings at ({x}, {y})"));}倰.set(q);match 余{佺
if q==T[侈(乮)]=>{}佺=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}傓=>l.丹.set(q),債=>l.乫.set(q),_=>return Err(format!("unknown building type {}",余)),}}let
mut 买=G::U;let mut 佚=0u32;for[亜,上,丆,x,y]in 丈{let 乮=值(亜)?;let q=侐(x,y,乣)?;if!丑[乮].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
买.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丆){return Err(format!("unit level {} is not 1, 2 or 3",丆));}let 上=Q::try_from(上).map_err(|_|format!("unit id {} is out of range",上))?;if
买.iter().any(|主|l.亐[主]==上){return Err(format!("two units have id {}",上));}买.set(q);l.丈[侈(乮)].set(q);l.丆[丆 as F-1].set(q);l.亐[q]=上;佚=佚.max(上);}if!买.is_empty(){l.伅=佚.checked_add(1).ok_or_else(||format!("unit id {} is out of range",佚))?;}l.仉=l.丈[丁];l.仠=l.七==[事(l.丛[0],T[0]),事(l.丛[1],T[1])];Ok(l)}}fn
伆<'a>(乊:&mut Lines<'a>,九:&str)->Result<&'a str,乙>{乊.next().ok_or_else(||format!("the text ends before {}",九))}fn 們(mut 乊:Lines<'_>,九:&str)->Result<(),乙>{match
乊.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",九)),None=>Ok(()),}}fn 亹<const N:F>(line:&str,九:&str)->Result<[C;N],乙>{let
mut 伸=line.split_whitespace();let mut 做=[0i64;N];for 丬 in&mut 做{let 伏=伸.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",九))?;*丬=伏.parse().map_err(|_|format!("{}: {:?} is not an integer",九,伏))?;}if
伸.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",九));}Ok(做)}fn 侘(乊:&mut Lines<'_>,九:&str)->Result<F,乙>{let[业]=亹(伆(乊,九)?,九)?;F::try_from(业).ok().filter(|&业|业<=M).ok_or_else(||format!("{}: {} is out of range",九,业))}fn
位(丬:C)->Result<F,乙>{F::try_from(丬).ok().filter(|&丬|丬<Y).ok_or_else(||format!("coordinate {} is off the map",丬))}fn 侐(x:C,y:C,乣:G)->Result<F,乙>{let q=位(y)?*Y+位(x)?;if
乣.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 值(亜:C)->Result<F,乙>{match 亜{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亜)),}}#[derive(Clone,Copy,PartialEq)]enum
仁{乸,俊,}#[derive(Clone,Copy)]struct Settings1{侼:I,侻:I,侔:C,仆:C,侜:F,佐:I,係:O,reserve:O,依:O,乤:仁,fill:O,乘:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{侼:3,侻:1,侔:4,仆:2,侜:2,佐:2,係:true,reserve:true,依:true,乤:仁::乸,fill:true,乘:Z::仔,};}const
侇:C=1000;const 來:C=10;fn 俇(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侳(l:&mut J,丒:L,丅:&mut Vec<L>)->O{let 亥=l.乇(丒);if 亥{丅.push(丒);}亥}fn 便(l:&mut J,三:F,两:F,丅:&mut
Vec<L>)->O{let Some((上,_))=l.亽(三)else{return false;};let(x,y)=俇(两);侳(l,L::乷{上,x,y},丅)}fn 儽(l:&J,乂:&仕,乆:&Settings1,乓:F,)->K<F>{let 丂=l.东();let 严=丳(l,l.丐(丂));let
且=丳(l,l.丐(1-丂));let 亙=l.map().valid.丗(l.丐(0)|l.丐(1));let 债=|c:F|严[c]!=I::MAX&&且[c]!=I::MAX;let mut 中:K<F> =乂.仡().丗(l.丐(丂)).iter().filter(|&c|债(c)).collect();中.sort_by_key(|&c|(严[c],c));let
mut 义:K<F> =亙.iter().filter(|&c|债(c)&&严[c].abs_diff(且[c])<=乆.侻).collect();义.sort_by_key(|&c|(严[c].abs_diff(且[c]),严[c],c));let mut 丅:K<F> =K::new();for
q in 中.into_iter().chain(义){if 丅.len()>=乓{break;}let 仚=丅.iter().all(|&t|l.map().丙(t,q)>=乆.侼);let 僔=乂.仡().test(q);if!丅.contains(&q)&&(僔||仚){丅.push(q);}}丅}fn
僻(与:&[K<C>])->K<F>{let n=与.len();let 倣=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 停=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亷=0usize;let mut 伨=vec![倣;n+1];let mut 俅=vec![false;n+1];loop{俅[亷]=true;let 倡=p[亷];let mut 伞=倣;let mut 伥=0usize;for j in 1..=n{if!俅[j]{let
乃=与[倡-1][j-1]-u[倡]-v[j];if 乃<伨[j]{伨[j]=乃;停[j]=亷;}if 伨[j]<伞{伞=伨[j];伥=j;}}}for j in 0..=n{if 俅[j]{u[p[j]]+=伞;v[j]-=伞;}else{伨[j]-=伞;}}亷=伥;if p[亷]==0{break;}}loop{let
伥=停[亷];p[亷]=p[伥];亷=伥;if 亷==0{break;}}}let mut 丅=vec![0usize;n];for j in 1..=n{if p[j]>0{丅[p[j]-1]=j-1;}}丅}#[derive(Clone,Copy)]enum 仑{众(F),侅,}fn 乲(l:&J)->G{let
丂=l.东();(l.今(丂)&l.乌(丂,1)).丗(l.丐(丂))}fn 儕(l:&J,乘:&Z)->F{佬(l,乘,乖::乸).iter().filter(|c|matches!(c,L::举{..})).count()}fn 両(q:F,丧:G)->Q{(G::q(q).丏()&丧).count()}fn
倘(l:&J,久:G,不:F,侨:I)->K<[C;M]>{let 丱=l.map().valid;let mut 专=[C::MIN;M];专[不]=0;let mut 乧=vec![专];for _ in 0..侨{let mut 丄=专;for q in 丱.iter(){for n in(G::q(q).丏()&丱).iter(){if
专[n]>C::MIN{let 丬=专[n]+C::from(久.test(n));if 丬>丄[q]{丄[q]=丬;}}}}专=丄;乧.push(专);}乧}fn 倴(l:&J,乧:&[[C;M]],久:G,三:F,k:F,)->G{let 丱=l.map().valid;let mut 丅=G::U;let
mut q=三;let mut k=k.min(乧.len()-1);while k>0&&乧[k][q]>0{let Some(丄)=(G::q(q).丏()&丱).iter().filter(|&n|乧[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(久.test(n))+乧[k-1][n],std::cmp::Reverse(n),)})else{break;};if
久.test(丄){丅.set(丄);}q=丄;k-=1;}丅}fn 丕(l:&J,乆:&Settings1)->K<L>{let 丂=l.东();let 乂=仕::of(l);let mut 丌=l.clone();let mut 丅=K::new();let 且=丳(l,l.丐(1-丂));let
丈:K<F> =l.乺().iter().collect();let 倚=(l.下(丂)/來).max(0)as F;let 亟=match 乆.乤{仁::乸=>儕(l,&乆.乘).min(倚),仁::俊=>倚,};let mut 优:K<仑> =丈.iter().map(|&c|仑::众(c)).collect();优.extend(std::iter::repeat_n(仑::侅,亟));let
亃=儽(l,&乂,乆,优.len()+乆.侜);let 光=乲(l);let 中=乂.仡();let n=优.len().max(亃.len());let mut 与=vec![vec![0i64;n];n];for(乽,&不)in 亃.iter().enumerate(){for(伜,仍)in 与[乽].iter_mut().enumerate(){let
Some(&僄)=优.get(伜)else{*仍=侇;continue;};let 佉=match 僄{仑::众(q)=>{let d=l.map().丙(q,不);if d==I::MAX{None}else{Some(C::from(d)-1)}}仑::侅=>光.iter().map(|c|l.map().丙(c,不)).filter(|&d|d!=I::MAX).min().map(C::from),};*仍=match
佉{None=>侇,Some(佉)=>{let 丸=C::from(且[不])-1;let 仆=乆.仆*(佉-丸).max(0);let 侍=if 中.test(不){乆.侔}else{0};(佉+仆-侍).max(0)}};}}let 僆=僻(&与);let mut 侠:K<(F,F)> =K::new();let
mut 促:K<(C,F)> =K::new();for(乽,&不)in 亃.iter().enumerate(){let 伜=僆[乽];if 与[乽][伜]>=侇{continue;}match 优.get(伜){Some(仑::众(q))=>侠.push((*q,不)),Some(仑::侅)=>促.push((与[乽][伜],不)),None=>{}}}let
mut 丧=l.map().valid.丗(l.丐(0)|l.丐(1));let mut 倫:K<F> =K::new();let mut 佥=G::U;let 些=|q:F|{let 丆=l.亠(q).unwrap_or(1);let 伳=丧|l.丐(1-丂);(G::q(q).丏()&伳&l.乌(丂,丆)).count()};侠.sort_by_key(|&(q,不)|{let
僫=if 乆.依{些(q)}else{0};(僫,l.map().丙(q,不),q)});for&(三,不)in&侠{let Some(丆)=丌.亠(三)else{continue;};let 丨=丌.乌(丂,丆);let d=丌.map().丙(三,不);let 久=(丧|丌.丐(1-丂)).丗(佥);let
侨=d.saturating_add(乆.佐);let 乧=倘(&丌,久,不,侨.saturating_sub(1));let 仱=乧[乧.len()-1];let 丄=(G::q(三).丏()&丨).iter().filter(|&c|仱[c]>C::MIN).max_by_key(|&c|{let
僗=d.saturating_sub(丌.map().丙(c,不));let 伳=丧.test(c)||丌.丐(1-丂).test(c);(伳&&乆.係,C::from(久.test(c))+仱[c],僗,両(c,丧),std::cmp::Reverse(c),)}).filter(|&c|{丧.test(c)||丌.丐(1-丂).test(c)||丌.map().丙(c,不)<d});if
let Some(两)=丄{if 便(&mut 丌,三,两,&mut 丅){丧.clear(两);倫.push(三);if 乆.reserve{佥|=倴(&丌,&乧,久,两,乧.len()-1);}}}}let 严=丳(l,l.丐(丂));let mut 丷=G::U;for c in 丧.iter(){if
严[c]<=且[c]{丷.set(c);}}for&三 in&丈{if 倫.contains(&三)||!丌.乺().test(三){continue;}let Some(丆)=丌.亠(三)else{continue;};let 丨=丌.乌(丂,丆);let 伳=(G::q(三).丏()&丧&丨).iter().max_by_key(|&c|(両(c,丧),且[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(两)=伳{if 便(&mut 丌,三,两,&mut 丅){丧.clear(两);}continue;}let 丘=[丷&丧,丧].into_iter().find_map(|丑|{丑.iter().min_by_key(|&c|(丌.map().丙(三,c),c))});if let
Some(丘)=丘{let d=丌.map().丙(三,丘);let 丄=(G::q(三).丏()&丨).iter().filter(|&c|丌.map().丙(c,丘)<d).min_by_key(|&c|c);if let Some(两)=丄{便(&mut 丌,三,两,&mut 丅);}}}促.sort();let
mut 侭=0usize;for&(_,不)in&促{if 丌.下(丂)<來{break;}let 丑=乲(&丌);let Some(乗)=丑.iter().map(|c|丌.map().丙(c,不)).min()else{break;};let 久=(丧|丌.丐(1-丂)).丗(佥);let 乧=倘(&丌,久,不,乗.saturating_add(乆.佐));let
仱=乧[乧.len()-1];let q=丑.iter().filter(|&c|丌.map().丙(c,不)==乗).max_by_key(|&c|(仱[c],両(c,丧),std::cmp::Reverse(c)));let Some(q)=q else{break;};let(x,y)=俇(q);if
侳(&mut 丌,L::举{丆:1,x,y},&mut 丅){丧.clear(q);侭+=1;if 乆.reserve{佥|=倴(&丌,&乧,久,q,乧.len()-1);}}}if 乆.fill&&乆.乤==仁::乸{while 侭<亟&&丌.下(丂)>=來{let q=(乲(&丌)&丧).iter().max_by_key(|&c|{(両(c,丧),std::cmp::Reverse(且[c]),std::cmp::Reverse(c),)});let
Some(q)=q else{break;};let(x,y)=俇(q);if!侳(&mut 丌,L::举{丆:1,x,y},&mut 丅){break;}丧.clear(q);侭+=1;}}丅}const 偿:C=15;const 傔:C=6;const 俰:C=10;const 储:I=3;#[derive(Clone,Copy)]struct
Z{open:C,佌:C,倌:I,侥:C,佗:C,倧:C,佦:I,個:C,倊:C,但:C,仺:C,伌:C,乢:C,份:C,仐:C,件:C,}impl Z{const 仔:Z=Z{open:3,佌:2,倌:12,侥:4,佗:偿,倧:傔,佦:储,個:2,倊:2,但:0,仺:0,伌:0,乢:0,份:0,仐:0,件:0,};const
fn 兗(self,仺:C,伌:C,乢:C)->Z{Z{仺,伌,乢,..self}}const fn 兖(self,份:C,仐:C)->Z{Z{份,仐,..self}}}fn 伹(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 丳(l:&J,三:G)->[I;M]{let mut
乄=[I::MAX;M];let 丱=l.map().valid;let mut 义=三&丱;let mut 买=义;let mut d=0u8;while!义.is_empty(){for q in 义.iter(){乄[q]=d;}let 丄=(义.丏()&丱).丗(买);买|=丄;义=丄;d=d.saturating_add(1);}乄}fn
僲(l:&J)->I{let 乄=丳(l,l.丐(0));l.丐(1).iter().map(|q|乄[q]).min().unwrap_or(I::MAX)}struct 仃<'a>{乘:&'a Z,丂:F,theirs:[I;M],伍:[I;M],free:G,丷:G,ours:G,丼:G,丈:G,}impl
仃<'_>{fn new<'a>(l:&J,乘:&'a Z)->仃<'a>{let 丂=l.东();let 丹=丳(l,l.丐(丂));let 且=丳(l,l.丐(1-丂));let 伍=丳(l,G::q(T[1-丂]));let 丧=l.map().valid.丗(l.丐(0)|l.丐(1));let
mut 丷=G::U;for q in 丧.iter(){if 丹[q]<=且[q]{丷.set(q);}}仃{乘,丂,theirs:且,伍,free:丧,丷,ours:l.丐(丂),丼:l.map().丼,丈:l.丈(丂),}}fn value(&self,q:F)->C{let 一=self;let
p=一.乘;let 両=C::from((G::q(q).丏()&一.free).count());let 仴=p.倌;let 佌=C::from(仴)-C::from(一.theirs[q].min(仴));let 任=if p.侥>0{C::from(一.伍[q].min(30))/p.侥}else{0};let
丛=C::from((G::q(q).丏()&一.ours).count());let line=if 丛<=1{p.伌}else{0};let 乢=if 一.丼.test(q){p.乢}else{0};let 儹=if 両==0{p.份}else{0};let 仐=p.仐*(22-C::from(一.伍[q].min(22)));let
件=p.件*C::from((G::q(q).丏()&一.丈).count());p.open*両+p.佌*佌-任+p.仺*丛-line+乢-儹+仐-件}fn 专(&self,丑:G)->P<F>{丑.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn
侕(&mut self,q:F){let 一=self;一.free.clear(q);一.丷.clear(q);一.ours.set(q);一.丈.set(q);}}fn 伭(l:&mut J,丒:L,丅:&mut Vec<L>)->O{let 亥=l.乇(丒);if 亥{丅.push(丒);}亥}fn
偉(l:&mut J,三:F,两:F,丅:&mut Vec<L>)->O{let Some((上,_))=l.亽(三)else{return false;};let(x,y)=伹(两);伭(l,L::乷{上,x,y},丅)}fn 兌(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-俰}fn
儤(l:&J,乘:&Z)->K<L>{佬(l,乘,乖::乸)}#[derive(Clone,Copy,PartialEq)]enum 乖{乸,亮{为:I,丈:I},}fn 佬(l:&J,乘:&Z,乤:乖)->K<L>{let mut 丌=l.clone();let mut 丕=仃::new(l,乘);let
mut 丅=K::new();亸(&mut 丌,&mut 丕,&mut 丅);match 乤{乖::乸=>儜(&mut 丌,&mut 丕,&mut 丅),乖::亮{为,丈}=>{僩(&mut 丌,&mut 丕,为,丈,&mut 丅)}}丅}fn 亸(l:&mut J,丕:&mut 仃<'_>,丅:&mut
Vec<L>){let 丂=丕.丂;let mut 亶:K<F> =K::new();loop{let 丨=l.乌(丂,1);let mut 伈:P<(Q,F)> =None;for 三 in l.乺().iter(){if 亶.contains(&三){continue;}let 些=(G::q(三).丏()&丕.free&丨).count();if
些==0{亶.push(三);}else if 伈.is_none_or(|(n,_)|些<n){伈=Some((些,三));}}let Some((_,三))=伈 else{break;};丕.丈.clear(三);match 丕.专(G::q(三).丏()&丕.free&丨){Some(两)if
偉(l,三,两,丅)=>丕.侕(两),_=>{丕.丈.set(三);亶.push(三);}}}for 三 in 亶{if!l.乺().test(三){continue;}let 不=[丕.丷,丕.free].into_iter().find_map(|丑|{丑.iter().min_by_key(|&q|(l.map().丙(三,q),q))});if
let Some(不)=不{偉(l,三,不,丅);}}}fn 儜(l:&mut J,丕:&mut 仃<'_>,丅:&mut Vec<L>){let 丂=丕.丂;let p=丕.乘;let h=(p.佗-C::from(l.书())).max(p.倧);let 充=(C::from(僲(l))-p.個).max(0)/p.倊.max(1)+p.但;loop{let
下=l.下(丂);let 丈=l.俄(丂)as C;let 儣=C::from(丕.丷.count());let q=丕.专(l.今(丂)&丕.free&l.乌(丂,1));let 偗=兌(h,(儣/(丈+1)).min(充));let 倄=q.is_some()&&下>=俰&&偗>0;let 倨=4*h-云;let
乢=倩(l,丕);let 僒=乢.is_some()&&下>=云&&倨>0;if 僒&&(!倄||倨>=偗){let(x,y)=伹(乢.unwrap_or_default());if 伭(l,L::亭{x,y},丅){continue;}break;}let Some(q)=q.filter(|_|倄)else{break;};let(x,y)=伹(q);if!伭(l,L::举{丆:1,x,y},丅){break;}丕.侕(q);}}fn
倩(l:&J,丕:&仃<'_>)->P<F>{let 丂=丕.丂;(l.map().丼&l.七(丂)&l.付()).iter().filter(|&乢|丕.theirs[乢]>丕.乘.佦).min_by_key(|&乢|(l.map().丙(T[丂],乢),乢))}fn 僩(l:&mut J,丕:&mut
仃<'_>,为:I,丈:I,丅:&mut Vec<L>,){let 丂=丕.丂;for _ in 0..为{let Some(乢)=倩(l,丕)else{break;};let(x,y)=伹(乢);if!伭(l,L::亭{x,y},丅){break;}}for _ in 0..丈{let Some(q)=丕.专(l.今(丂)&丕.free&l.乌(丂,1))else{break;};let(x,y)=伹(q);if!伭(l,L::举{丆:1,x,y},丅){break;}丕.侕(q);}}fn
乭(l:&J)->O{if l.丢().is_some(){return false;}let 丂=l.东();let 下=l.下(丂);let 丩=仮::new(l,丂,true);l.丐(1-丂).iter().any(|q|丩.与[q]<=下)}#[derive(Clone,Copy)]struct
丽{乐:I,乤:乖,}const 俈:I=I::MAX;const 偨:[乖;8]=[乖::乸,乖::亮{为:0,丈:0},乖::亮{为:0,丈:1},乖::亮{为:0,丈:2},乖::亮{为:0,丈:俈,},乖::亮{为:1,丈:0},乖::亮{为:1,丈:1},乖::亮{为:1,丈:俈,},];struct
併<'a>{丰:&'a[Z],仈:Z,rest:丽,亴:P<亇>,乆:Settings2,}impl 併<'_>{fn 乘(&self,丣:丽)->&Z{&self.丰[F::from(丣.乐)%self.丰.len()]}fn 万(&self,l:&J,丣:丽)->K<L>{佬(l,self.乘(丣),丣.乤)}}fn
僂(l:&J,万:&[L],仈:&Z)->(J,O){let mut 丄=l.clone();丄.佣(万);if 丄.丢().is_some()||乭(&丄){return(丄,true);}let 乼=儤(&丄,仈);丄.佣(&乼);let 人=丄.丢().is_some()||乭(&丄)||丄.书()>俎;(丄,人)}fn
儛(l:&J,世:&[丽],仗:&併<'_>)->K<丯>{let 丂=l.东();let mut 伦=vec![伱(l,丂)];let mut 乱=l.clone();for 丕 in 0..俎 as F{let 丣=世.get(丕).copied().unwrap_or(仗.rest);let 万=仗.万(&乱,丣);let(丄,人)=僂(&乱,&万,&仗.仈);if
人{break;}乱=丄;伦.push(伱(&乱,丂));}伦}fn 伱(l:&J,丂:F)->丯{let mut hash:丯=0xcbf29ce484222325;let mut 侮=|伏:丯|hash=(hash^伏).wrapping_mul(0x0100000001b3);for(儼,乒)in[l.丐(丂),l.丈(丂),l.为()].into_iter().enumerate(){for
q in 乒.iter(){侮(((儼 as 丯)<<8)|q as 丯);}侮(0xff);}侮(l.下(丂)as 丯);hash}enum 亓{伖{个:F,亡:F,临:什,佤:Q,},似(Settings1),}struct 交{亝:亓,丰:K<Z>,仈:Z,世:K<丽>,佒:K<丯>,令:F,丶:Q,人:O,}impl
交{fn new(亝:亓,丰:K<Z>,仈:Z)->交{交{亝,丰,仈,世:K::new(),佒:K::new(),令:0,丶:0,人:false,}}fn 儘(&mut self,l:&J)->O{let 一=self;let 儮=matches!(一.亝,亓::伖{..}|亓::似(_));let
儚=!儮&&一.丶>0&&一.令>=一.佒.len();if!一.人&&(儚||乭(l)){一.人=true;}!一.人}fn end(&mut self){self.人=true;}fn 丕(&mut self,l:&J,亴:P<亇>)->K<L>{let 一=self;let 仗=併{丰:&一.丰,仈:一.仈,rest:丽{乐:0,乤:乖::乸,},亴,乆:Settings2::DEFAULT,};if
let 亓::似(乆)=一.亝{一.丶+=1;一.令+=1;return 丕(l,&乆);}let 丂=l.东();let 儎=一.佒.get(一.令)==Some(&伱(l,丂));let 僨=matches!(一.亝,亓::伖{佤,..}if 佤>0&&l.书()>=佤);if!儎||僨{let
乬=一.丶==0;一.世=match 一.亝{亓::伖{个,亡,临,..}=>{let 临=&临;let 乪=傣;let mut 之=世(l,&一.丰,乪,个,亡,亴,临,);之.resize(个,乪);之}亓::似(_)=>K::new(),};一.佒=match 一.亝{亓::伖{..}=>儯(l,&一.丰,&一.世),_=>儛(l,&一.世,&仗),};一.令=0;一.丶+=1;}let
丣=一.世.get(一.令).copied().unwrap_or(仗.rest);一.令+=1;仗.万(l,丣)}}fn 儯(l:&J,丰:&[Z],世:&[丽])->K<丯>{let 丂=l.东();let mut 伦=K::with_capacity(世.len());let mut 乱=l.clone();for&丣
in 世{if 乱.丢().is_some(){break;}伦.push(伱(&乱,丂));乱=侉(&乱,&万(&乱,丰,丣));}伦}const 俎:Q=20;const 傷:C=3;#[derive(Clone,Copy)]struct Settings2{个:C,}impl Settings2{const
DEFAULT:Settings2=Settings2{个:傷};}fn 之(l:&J,三:G)->[I;M]{let 丱=l.map().valid;let mut 乄=[I::MAX;M];let mut 义=三&丱;let mut 买=义;let mut d=0u8;while!义.is_empty(){for
q in 义.iter(){乄[q]=d;}let 丄=(义.丏()&丱).丗(买);买|=丄;义=丄;d=d.saturating_add(1);}乄}fn 兏(l:&J,丁:F)->C{let 严=之(l,l.七(丁));let 且=之(l,l.七(1-丁));l.map().valid.iter().map(|q|match
严[q].cmp(&且[q]){_ if 严[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 傪:[Z;27]=丰();const
傣:丽=丽{乐:16,乤:乖::乸,};const fn 丰()->[Z;27]{let 仺=[0,2,4];let 伌=[0,3,6];let 仐=[0,1,2];let mut 丅=[Z::仔;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
份=if g>0||t>0{4}else{0};丅[i]=Z::仔.兗(仺[g],伌[t],0).兖(份,仐[w]);i+=1;}丅}#[derive(Default)]struct 侃{丑:C,丟:C,下:C,二:C,串:C,伶:C,丩:C,仪:C,}#[derive(Clone,Copy)]struct
什{丑:C,丟:C,下:C,串:C,二:C,伶:C,丩:C,仪:C,中:C,仳:O,习:亁,}#[derive(Clone,Copy,PartialEq)]enum 亁{俜,俐(佶),佷{仴:C},仒(仒),俍,}fn 兆(l:&J,丁:F)->[I;M]{let 严=丳(l,G::q(T[丁]));let
且=丳(l,G::q(T[1-丁]));let mut 亨=G::U;let mut 丷=G::U;for c in l.map().valid.iter(){if 严[c]==I::MAX||且[c]==I::MAX{continue;}if 严[c]==且[c]{亨.set(c);}if 严[c]<=且[c]{丷.set(c);}}let
d=丳(l,亨);let mut 丅=[I::MAX;M];for c in 丷.iter(){丅[c]=d[c];}丅}fn 僋(l:&J,丁:F,亨:&[I;M])->C{let d=丳(l,l.七(丁));(0..M).filter(|&c|亨[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&亨[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 仒{傠,}impl
仒{fn 俆(self,a:C,b:C,d:C)->C{let 仦=(a-b).abs()/2;match self{仒::傠=>a.min(d/2),}}}fn 僬(l:&J,丁:F,侞:仒)->[C;M]{let 严=丳(l,G::q(T[丁]));let 且=丳(l,G::q(T[1-丁]));let
d=C::from(严[T[1-丁]]);let mut 丅=[0;M];for c in 0..M{if 严[c]!=I::MAX&&且[c]!=I::MAX{丅[c]=侞.俆(C::from(严[c]),C::from(且[c]),d);}}丅}#[derive(Clone,Copy,PartialEq)]enum
佶{傱(C),}impl 佶{fn 俆(self,仦:C)->C{match self{佶::傱(亞)=>(亞-仦).max(0).pow(2),}}}type 佁=[C;4];fn 儨(l:&J,丁:F)->[C;M]{struct 休{七:G,乴:[伻;M],乜:[伻;M],size:[C;M],串:[C;M],仏:伻,}fn
仩(d:&mut 休,v:F){d.仏+=1;d.乴[v]=d.仏;d.乜[v]=d.仏;d.size[v]=1;for w in(G::q(v).丏()&d.七).iter(){if d.乴[w]==0{仩(d,w);d.size[v]+=d.size[w];d.乜[v]=d.乜[v].min(d.乜[w]);if
d.乜[w]>=d.乴[v]{d.串[v]+=d.size[w];}}else{d.乜[v]=d.乜[v].min(d.乴[w]);}}}let mut d=休{七:l.七(丁),乴:[0;M],乜:[0;M],size:[0;M],串:[0;M],仏:0,};let 乯=T[丁];if d.七.test(乯){仩(&mut
d,乯);}d.串[乯]=0;d.串}fn 優(l:&J,丁:F)->侃{let 七=l.七(丁);let 串=儨(l,丁);let 伍=丳(l,G::q(T[1-丁]));let mut 丅=侃{丑:C::from(七.count()),丟:l.丟(丁),下:l.下(丁),丩:C::MAX,仪:兏(l,丁)/2,..侃::default()};for
q in 七.iter(){if 串[q]>0{丅.二+=1;丅.串=丅.串.max(串[q]);}if q!=T[丁]&&(G::q(q).丏()&七).count()<=1{丅.伶+=1;}丅.丩=丅.丩.min(C::from(伍[q]));}丅}fn 兎(l:&J,丁:F,临:&什,伧:G)->C{let
仟=C::from((l.丐(丁)&伧).count());let s=優(l,丁);let o=临;o.丑*s.丑+o.丟*s.丟+o.下*s.下-o.串*s.串-o.二*s.二-o.伶*s.伶-o.丩*s.丩.min(30)+o.仪*s.仪+o.中*仟}fn 仡(l:&J)->G{仕::of(l).仡()}fn
儅(l:&J)->G{let 严=丳(l,G::q(T[0]));let 且=丳(l,G::q(T[1]));let mut 么=G::U;for c in l.map().valid.iter(){if 严[c]!=I::MAX&&且[c]!=I::MAX&&严[c].abs_diff(且[c])<=2{么.set(c);}}么}fn
仧(l:&J)->[C;M]{let a=丳(l,G::q(T[0]));let b=丳(l,G::q(T[1]));let mut 丅=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丅[c]=C::from(a[c].abs_diff(b[c]))/2;}}丅}#[derive(Clone,Copy)]struct
仰{中:G,么:G,乐:[C;M],仧:[C;M],义:G,front:[I;M],}impl 仰{fn of(l:&J,临:&什)->仰{let 倥=matches!(临.习,亁::俜);let mut 乐=[0;M];if let 亁::俐(僜)=临.习{for(w,仦)in 乐.iter_mut().zip(仧(l)){*w=if
仦==C::MAX{0}else{僜.俆(仦)};}}if let 亁::仒(侞)=临.习{乐=僬(l,l.东(),侞);}let(儢,义)=match 临.习{亁::佷{..}=>{let r=仧(l);let mut f=G::U;for(c,&仦)in r.iter().enumerate(){if
仦==0{f.set(c);}}(r,f)}_=>([C::MAX;M],G::U),};仰{乐,仧:儢,义,front:if 临.习==亁::俍{兆(l,l.东())}else{[I::MAX;M]},中:if 临.中!=0||倥{仕::of(l).仡()}else{G::U},么:if 倥{儅(l)}else{G::U},}}}fn
仾(l:&J,丁:F,临:&什,乂:仰)->佁{let 佪=兎(l,丁,临,乂.中);match 临.习{亁::俐(_)|亁::仒(_)=>{let 七=l.七(丁);let 兓:C=七.iter().map(|c|乂.乐[c]).sum();[C::from(七.count()),兓,佪,0]}亁::佷{仴}=>{let
七=l.七(丁);let d=丳(l,七);let 仚:C=乂.义.iter().map(|c|C::from(d[c]).min(仴)).sum();[C::from(七.count()),-仚,佪,0]}亁::俍=>[C::from(l.七(丁).count()),僋(l,丁,&乂.front),佪,0,],亁::俜=>{let
丐=l.丐(丁);let 中=C::from((丐&乂.中).count());let 么=C::from((丐&乂.么).count());match 临.习{_=>[C::from(l.七(丁).count()),中,么,佪],}}}}fn 侉(l:&J,万:&[L])->J{let mut 丄=l.clone();丄.佣(万);if
丄.丢().is_none(){丄.佣(&[]);}丄}const 俉:I=I::MAX;fn 万(l:&J,丰:&[Z],丣:丽)->K<L>{if 丣.乐==俉{let 乆=Settings1{乘:丰.first().copied().unwrap_or(Z::仔),乤:if 丣.乤==乖::乸{仁::乸}else{仁::俊},..Settings1::DEFAULT};return
丕(l,&乆);}佬(l,&丰[F::from(丣.乐)%丰.len()],丣.乤,)}fn 倷(l:&J,丰:&[Z],世:&[丽],乪:丽,个:F,)->J{let mut 乱=l.clone();for 丕 in 0..个{if 乱.丢().is_some(){break;}let 丣=世.get(丕).copied().unwrap_or(乪);乱=侉(&乱,&万(&乱,丰,丣));}乱}fn
儵(l:&J,丰:&[Z],仳:O)->K<(丽,K<L>)>{let mut 丅:K<(丽,K<L>)> =K::new();if 仳{for 乤 in[乖::乸,乖::亮{为:0,丈:I::MAX,},]{let 丣=丽{乐:俉,乤,};let c=万(l,丰,丣);if!丅.iter().any(|(_,主)|*主==c){丅.push((丣,c));}}}for
乐 in 0..丰.len().min(F::from(I::MAX)){for&乤 in&偨{let 丣=丽{乐:乐 as I,乤,};let c=万(l,丰,丣);if!丅.iter().any(|(_,主)|*主==c){丅.push((丣,c));}}}丅}fn 僉(l:&J,丰:&[Z],乪:丽,个:F,亡:F,亴:P<亇>,临:&什,)->K<丽>{let
丂=l.东();let 伬=||亴.is_some_and(|t|亇::now()>=t);let 乂=仰::of(l,临);let 伮=|世:&[丽]|仾(&倷(l,丰,世,乪,个),丂,临,乂);let mut 专:(佁,K<丽>)=(伮(&[]),K::new());let mut 义:K<(J,K<丽>)>
=vec![(l.clone(),K::new())];for 僟 in 0..个{let mut 亳:K<(佁,J,K<丽>)> =K::new();let mut 买:HashMap<丯,F> =HashMap::new();for(倬,世)in&义{if 伬(){break;}for(丣,c)in
儵(倬,丰,临.仳){let 丄=侉(倬,&c);let mut 侓=世.clone();侓.push(丣);let v=仾(&丄,丂,临,乂);let 仾=伱(&丄,丂);match 买.get(&仾){Some(&乚)if 亳[乚].0>=v=>{}Some(&乚)=>亳[乚]=(v,丄,侓),None=>{买.insert(仾,亳.len());亳.push((v,丄,侓));}}}}if
亳.is_empty(){break;}亳.sort_by(|a,b|b.0 .cmp(&a.0));亳.truncate(亡);let 乹=僟+1==个||伬();for(i,(_,_,世))in 亳.iter().take(if 乹{亡}else{1}).enumerate(){if i>0&&伬(){break;}let
v=伮(世);if v>专.0{专=(v,世.clone());}}if 乹{break;}义=亳.into_iter().map(|(_,s,p)|(s,p)).collect();}专.1}fn 世(l:&J,丰:&[Z],乪:丽,个:F,億:F,亴:P<亇>,临:&什,)->K<丽>{let 丂=l.东();let
乂=仰::of(l,临);let 伮=|世:&[丽]|仾(&倷(l,丰,世,乪,个),丂,临,乂);let mut 专:(佁,K<丽>)=(伮(&[]),K::new());let mut 亡=2;loop{let 乏=僉(l,丰,乪,个,亡,亴,临);let v=伮(&乏);if v>专.0{专=(v,乏);}let
伬=亴.is_some_and(|t|亇::now()>=t);if 伬||(亴.is_none()&&亡>=億){break;}亡*=2;}专.1}const 偾:F=4+12;fn 个(侦:impl BufRead,mut 佡:impl Write,mut 例:impl FnMut(&str,&str,Q,亇)->乙,)->Result<(),乙>{let
mut 乊=保{倻:侦};let Some(乬)=乊.line()?else{return Ok(());};let mut 仼=format!("{}\n",乬);乊.take(业(&乬,"mine spot count")?,&mut 仼)?;let mut 丕=0;loop{let Some(乬)=乊.line()?else{return
Ok(());};let 乀=亇::now();丕+=1;let mut 亏=format!("{}\n",乬);乊.take(偾-1,&mut 亏)?;for 九 in["building count","unit count"]{let line=乊.倽()?;let n=业(&line,九)?;亏.push_str(&line);亏.push('\n');乊.take(n,&mut
亏)?;}let 例=例(&仼,&亏,丕,乀);writeln!(佡,"{}",例).and_then(|()|佡.flush()).map_err(|仙|format!("turn {}: cannot write the answer: {}",丕,仙))?;}}struct 保<R>{倻:R,}impl<R:BufRead>保<R>{fn
line(&mut self)->Result<P<乙>,乙>{let mut line=乙::new();match self.倻.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(仙)=>Err(format!("cannot read input: {}",仙)),}}fn
倽(&mut self)->Result<乙,乙>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,亏:&mut String)->Result<(),乙>{for
_ in 0..n{亏.push_str(&self.倽()?);亏.push('\n');}Ok(())}}fn 业(line:&str,九:&str)->Result<F,乙>{match line.trim().parse::<F>(){Ok(n)if n<=傊=>Ok(n),_=>Err(format!("bad {} {line:?}",九)),}}const
傊:F=144;const 傲:C=3;const 傅:C=2;const 傏:F=16;fn 儖(l:&J,丅:&mut Vec<L>){let 丁=l.东();let mut 亝=俦{s:l.clone(),丁,丸:1-丁,不:T[1-丁],佖:T[丁],丕:l.书(),丅,};亝.run();}struct
俦<'a>{s:J,丁:F,丸:F,不:F,佖:F,丕:Q,丅:&'a mut Vec<L>,}fn 佰(q:F)->(I,I){((q%Y)as I,(q/Y)as I)}fn 侯(上:Q,q:F)->L{let(x,y)=佰(q);L::乷{上,x,y}}fn 偔(丆:I,q:F)->L{let(x,y)=佰(q);L::举{丆,x,y}}impl
俦<'_>{fn run(&mut self){let 一=self;if 一.佛(){return;}一.亸();if 一.佛(){return;}一.亂();一.亟();}fn 乄(&self,三:F,两:F)->C{C::from(self.s.map().丙(三,两))}fn 乇(&mut self,丒:L)->O{let
乥=self.s.乇(丒);if 乥{self.丅.push(丒);}乥}fn 伎(&self,丵:&J)->O{丵.丢()==Some(乳::五(self.丁))}fn 佛(&mut self)->O{let 一=self;if 一.s.丢().is_some(){return false;}for
q in 一.s.乺().iter(){if 一.乄(q,一.不)!=1{continue;}let Some((上,_))=一.s.亽(q)else{continue;};let 丒=侯(上,一.不);let mut 丵=一.s.clone();if 丵.乇(丒)&&一.伎(&丵){return 一.乇(丒);}}let
Some(丝)=一.僓()else{return false;};let mut 丵=一.s.clone();if!丝.iter().all(|&丒|丵.乇(丒))||!一.伎(&丵){return false;}for 丒 in 丝{一.乇(丒);}true}fn 僓(&self)->P<K<L>>{let
一=self;const 俓:C=C::MAX;let 亍=一.s.丿().亘();let 丨:K<G> =(1..=亍).map(|丆|一.s.乌(一.丁,丆)).collect();let 伣=|q:F|{丨.iter().position(|乒|乒.test(q)).map(|丠|(丠 as I+1,10*(丠
as C+1)))};let 七=一.s.七(一.丁);let mut 与=[俓;M];let mut 乍=[F::MAX;M];let mut 乥=[false;M];for q in 七.iter(){与[q]=0;}loop{let mut 丄:P<F> =None;for q in 0..M{if!乥[q]&&与[q]<俓&&丄.is_none_or(|n|与[q]<与[n]){丄=Some(q);}}let
Some(q)=丄 else{break;};乥[q]=true;if q==一.不{break;}for&也 in&一.s.map().丏[0][q]{if 也==互{break;}let 也=F::from(也);if 乥[也]||七.test(也){continue;}let Some((_,仌))=伣(也)else{continue;};if
与[q]+仌<与[也]{与[也]=与[q]+仌;乍[也]=q;}}}if 与[一.不]>一.s.下(一.丁){return None;}let mut 丝=K::new();let mut q=一.不;while!七.test(q){let(丆,_)=伣(q)?;丝.push(偔(丆,q));q=乍[q];}丝.reverse();Some(丝)}fn
久(&self,乑:&J)->C{let(丂,从)=(self.丁,self.丸);let 乍=&self.s;let 业=|乒:G|C::from(乒.count());let 丛=业(乑.丐(丂))-业(乍.丐(丂));let 京=业(乍.丐(从))-业(乑.丐(从));let 侬=业(乍.七(从))-业(乑.七(从));let
佘=乍.伄(从)-乑.伄(从);let 为=业(乍.为()&乍.丐(从))-业(乑.为()&乑.丐(从));12*丛+20*京+8*(侬-京).max(0)+5*佘+25*为}fn 假(&self,q:F)->C{let 一=self;let 乢=一.s.map().丼.test(q);if 乢&&一.s.丿().佇()&&!一.s.丐(一.丁).test(q){20}else{0}}fn
倒(&self,乓:C)->O{let 一=self;一.s.丈(一.丸).iter().any(|q|一.乄(q,一.佖)<=乓)}fn 亸(&mut self){let 一=self;let mut 丈:K<(C,F)> =一.s.乺().iter().map(|q|(一.乄(q,一.不),q)).collect();丈.sort_unstable();for(仅,q)in
丈{if 一.s.丢().is_some(){return;}let Some((上,丆))=一.s.亽(q)else{continue;};let 丨=一.s.乌(一.丁,丆);let 也=一.s.map().丏[一.丁][q];let mut 专:P<(C,F)> =None;for&丄 in 也.iter().take_while(|&&丄|丄!=互){let
丄=F::from(丄);if!丨.test(丄){continue;}let mut 丵=一.s.clone();if!丵.乇(侯(上,丄)){continue;}let 丬=if 一.伎(&丵){100000}else{一.久(&丵)+10*(仅-一.乄(丄,一.不))+一.假(丄)};if 专.is_none_or(|(亞,_)|丬>亞){专=Some((丬,丄));}}if
let Some((丬,丄))=专{if 丬>0{一.乇(侯(上,丄));}}}}fn 亂(&mut self){let 一=self;if 一.s.丢().is_some(){return;}if 一.倒(傲){一.先();}else{一.为();}}fn 先(&mut self){let 一=self;if!一.s.丿().仲()||一.s.下(一.丁)<云{return;}let
也=一.s.map().丏[一.丁][一.佖];let 丑:K<F> =也.iter().take_while(|&&q|q!=互).map(|&q|F::from(q)).collect();let 丹=一.s.丐(一.丁);if 丑.iter().any(|&q|丹.test(q)&&一.s.乔().test(q)){return;}for
q in 丑{let(x,y)=佰(q);if 一.乇(L::乿{x,y}){return;}}}fn 为(&mut self){let 一=self;if!一.s.丿().佇(){return;}while 一.s.下(一.丁)>=云{let 丧=一.s.map().丼&一.s.丐(一.丁)&一.s.七(一.丁)&一.s.付();let
Some(q)=丧.iter().min_by_key(|&q|(一.乄(q,一.佖),q))else{return;};let(x,y)=佰(q);if!一.乇(L::亭{x,y}){return;}}}fn 亟(&mut self){let 一=self;let 倾=if 一.丕<=3{0}else{5};for
_ in 0..傏{if 一.s.丢().is_some(){return;}let 下=一.s.下(一.丁);if 下<10{return;}let 僼=一.倒(傅);let 乗=一.s.今(一.丁);let mut 专:P<(C,L)> =None;let mut 侽:P<(C,L)> =None;let
mut 俾=G::U;for 丆 in 1..=一.s.丿().亘(){let 丨=一.s.乌(一.丁,丆);let 些=(乗&丨).丗(俾);俾=丨;let 仌=10*C::from(丆);if 仌>下{continue;}for q in 些.iter(){let 丒=偔(丆,q);let mut
丵=一.s.clone();if!丵.乇(丒){continue;}let 伀=丵.下(一.丁);let 丟=丵.丟(一.丁);let 佘=一.s.伄(一.丸)-丵.伄(一.丸);let 僃=if 佘>0&&僼{伀+丟>=0}else{丟>=0&&(伀>=倾||佘>0)};if!僃{continue;}let
久=一.久(&丵);let 丬=久+一.假(q)-6*一.乄(q,一.不)-10*(C::from(丆)-1);if 久>0{if 专.is_none_or(|(亞,_)|丬>亞){专=Some((丬,丒));}}else if 丟>=3&&伀>=倾+10&&丆==1{if 侽.is_none_or(|(亞,_)|丬>亞){侽=Some((丬,丒));}}}}let
Some((_,丒))=专.or(侽)else{return;};if!一.乇(丒){return;}}}}fn 僴(l:&J,丅:&mut Vec<L>){丅.clear();儖(l,丅);}const 亲:C=C::MAX;const 傎:F=8;struct 仮{与:[C;M],乍:[I;M],}fn
低(l:&J,丁:F,亸:O,不:F)->C{侗(l,丁,仢(l,丁,亸),不)}fn 侗(l:&J,丁:F,仢:G,不:F)->C{let mut 与=亲;倖(l,丁,仢,|丣,new|{if new.test(不){与=10*丣 as C;}与!=亲});与}fn 儞(l:&J,丁:F,仢:G,乎:C)->G{let
mut 乭=G::U;倖(l,丁,仢,|丣,new|{if 10*丣 as C>乎{return true;}乭|=new;false});乭.丗(l.七(丁))}fn 仢(l:&J,丁:F,亸:O)->G{if 亸&&丁==l.东(){l.乺()}else{G::U}}fn 倖(l:&J,丁:F,仢:G,mut
仩:impl FnMut(F,G)->O,){let 亍=l.丿().亘();let mut 伣=[G::U;3];let mut 买=G::U;for 丆 in 1..=亍{let 乒=l.乌(丁,丆);伣[F::from(丆)-1]=乒.丗(买);买|=乒;}let mut 両=[l.七(丁),G::U,G::U,G::U];for
q in(仢&l.丈(丁)).iter(){if let Some(丆)=l.亠(q).filter(|&丆|丆<=亍){両[0]|=G::q(q).丏()&l.乌(丁,丆);}}let mut 乥=G::U;for 丣 in 0..{let new=両[丣%4].丗(乥);両[丣%4]=G::U;if
仩(丣,new){return;}乥|=new;let 丄=new.丏().丗(乥);for(仱,丑)in 伣.iter().enumerate(){両[(丣+仱+1)%4]|=丄&*丑;}if 両.iter().all(|乒|乒.丗(乥).is_empty()){break;}}}impl 仮{fn
new(l:&J,丁:F,亸:O)->仮{let 亍=F::from(l.丿().亘());let mut 丨=[G::U;3];for(丠,乒)in 丨.iter_mut().enumerate().take(亍){*乒=l.乌(丁,丠 as I+1);}let 七=l.七(丁);let 丏=&l.map().丏[0];let
mut 与=[亲;M];let mut 乍=[互;M];let mut 両=[0u8;M];let mut 业=0;let mut 偃=G::U;for q in 七.iter(){与[q]=0;両[业]=q as I;业+=1;}if 亸&&丁==l.东(){for 三 in l.乺().iter(){let
Some(丆)=l.亠(三)else{continue;};let 之=G::q(三).丏()&丨[F::from(丆)-1];for q in 之.iter(){if 与[q]!=0{与[q]=0;両[业]=q as I;业+=1;}}}}while 业>0{let mut 专=0;for 丠 in
1..业{let(a,b)=(F::from(両[丠]),F::from(両[专]));if(与[a],a)<(与[b],b){专=丠;}}let q=F::from(両[专]);业-=1;両[专]=両[业];偃.set(q);for&丄 in&丏[q]{if 丄==互{break;}let 丄=F::from(丄);if
偃.test(丄)||七.test(丄){continue;}let Some(丆)=丨[..亍].iter().position(|乒|乒.test(丄))else{continue;};let 偓=与[q]+10*(丆 as C+1);if 偓<与[丄]{if 与[丄]==亲{両[业]=丄 as
I;业+=1;}与[丄]=偓;乍[丄]=q as I;}}}仮{与,乍}}fn path(&self,不:F)->K<F>{let mut path=K::new();if 不>=M||self.与[不]==亲{return path;}let mut q=不;while self.乍[q]!=互{path.push(q);q=F::from(self.乍[q]);}path.reverse();path}}struct
伾{万:K<L>,与:C,}fn 儃(l:&J,丁:F,q:F)->P<I>{(1..=l.丿().亘()).find(|&丆|l.乌(丁,丆).test(q))}fn 儡(丌:&mut J,丂:F,丘:F,乎:C,万:&mut Vec<L>,于:&mut i64,)->O{loop{if 丌.丢()==Some(乳::五(丂)){return
true;}if 丌.丐(丂).test(丘)&&丌.七(丂).test(丘){return true;}let 丩=仮::new(丌,丂,false);let 与=丩.与[丘];if 与==亲||*于+与>乎{return false;}let Some(&q)=丩.path(丘).first()else{return
false;};let Some(丆)=儃(丌,丂,q)else{return false;};let 丒=L::举{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丌.乇(丒){return false;}*于+=10*C::from(丆);万.push(丒);}}fn 俻(l:&J,丂:F,亃:&[F],乎:C)->P<伾>{let
mut 丌=l.clone();let mut 万=K::new();let mut 于=0;for&丘 in 亃{if!儡(&mut 丌,丂,丘,乎,&mut 万,&mut 于){return None;}}(于<=乎).then_some(伾{万,与:于,})}fn 兊(l:&J,丂:F,不:F)->C{let
丱=l.map().valid;let 丐=l.丐(丂)&丱;let 倈=|mut 乃:G|loop{let 丄=乃|(乃.丏()&丐);if 丄==乃{return 乃;}乃=丄;};let mut 乃=倈(l.七(丂));let mut 之=0;while!乃.test(不){let new=乃.丏().丗(乃)&丱;if
new.is_empty(){return C::MAX/20;}乃=倈(乃|new);之+=1;}之}fn 侑(l:&J,不:F,乎:C)->P<伾>{if 乎<0||l.丢().is_some()||不>=M||l.map().乣.test(不){return None;}let 丂=l.东();if
l.丐(丂).test(不)&&l.七(丂).test(不){return Some(伾{万:K::new(),与:0,});}if 10*兊(l,丂,不)>乎{return None;}let mut 专=俻(l,丂,&[不],乎);let 乬=仮::new(l,丂,false);let mut 乔:K<F>
=(l.乔()&l.丐(1-丂)).iter().filter(|&乫|乫!=不&&乬.与[乫]!=亲).collect();乔.sort_by_key(|&乫|(l.map().丙(乫,不),乫));乔.truncate(傎);for 乫 in 乔{if let Some(丝)=俻(l,丂,&[乫,不],乎){if
专.as_ref().is_none_or(|亞|丝.与<亞.与){专=Some(丝);}}}专}const 傌:F=20000;struct 伿{万:K<L>,与:C,丑:C,丈:C,亂:C,丟:C,丮:C,}struct 佻{丑:C,丈:C,亂:C,}fn 倞(l:&J,丁:F)->佻{let 七=l.七(丁);let
丈=l.丈(丁)&七;let 丬=丈.iter().filter_map(|q|l.亠(q)).map(|丆|10*C::from(丆)).sum();佻{丑:C::from(七.count()),丈:丬,亂:C::from(((l.为()|l.乔())&七).count()),}}struct 俫{丂:F,伐:G,乍:佻,丟:C,伃:F,乎:C,伇:F,path:K<L>,乏:K<伿>,index:HashMap<伒,F,BuildHasherDefault<DefaultHasher>>,}type
伒=[丯;3];fn 兕(mut 亄:伒,q:F)->伒{亄[q/64]|=1<<(q%64);亄}impl 俫{fn 倏(&mut self,丌:&J,乹:P<F>,京:伒,与:C,)->O{let 一=self;let 丂=一.丂;let 些=match 乹{None=>丌.今(丂),Some(q)=>G::q(q).丏(),}&一.伐;let
些=些.丗(丌.丐(丂));let mut 丨=[G::U;3];for 丆 in 1..=丌.丿().亘(){丨[F::from(丆)-1]=丌.乌(丂,丆);}for q in 些.iter(){let 亍=F::from(丌.丿().亘());let Some(丠)=丨[..亍].iter().position(|乒|乒.test(q))else{continue;};let
丆=丠 as I+1;let 仌=10*C::from(丆);if 与+仌>一.乎{continue;}if 一.伇>=傌{return false;}一.伇+=1;let mut 丄=丌.clone();let 丒=L::举{丆,x:(q%Y)as I,y:(q/Y)as I,};if!丄.乇(丒){continue;}一.path.push(丒);let
伋=兕(京,q);一.償(&丄,伋,与+仌);let 僝=一.path.len()<一.伃&&丄.丢().is_none();let 僳=!僝||一.倏(&丄,Some(q),伋,与+仌);一.path.pop();if!僳{return false;}}true}fn 償(&mut self,乑:&J,伋:伒,与:C){let
一=self;let 伫=1-一.丂;let 伪=倞(乑,伫);let 丑=一.乍.丑-伪.丑;let 丈=一.乍.丈-伪.丈;if 丑<=0&&丈<=0{return;}let 亂=一.乍.亂-伪.亂;let 丟=乑.丟(一.丂)-一.丟;let 兘=乑.丢()==Some(乳::五(一.丂));let
丮=if 兘{C::MAX/2}else{3*丑+丈+10*亂+2*丟-与};if let Some(&乚)=一.index.get(&伋){if 与>=一.乏[乚].与{return;}}let 串=伿{万:一.path.clone(),与,丑,丈,亂,丟,丮,};match 一.index.get(&伋){Some(&乚)=>一.乏[乚]=串,None=>{一.index.insert(伋,一.乏.len());一.乏.push(串);}}}}fn
倍(l:&J,伃:F,乎:C,乓:F)->K<伿>{乡(l,伃,乎,乓).0}fn 乡(l:&J,伃:F,乎:C,乓:F)->(K<伿>,F){if l.丢().is_some()||伃==0||乓==0{return(K::new(),0);}let 丂=l.东();let 伫=1-丂;let mut
侹=俫{丂,伐:l.丐(伫)|l.丐(伫).丏(),乍:倞(l,伫),丟:l.丟(丂),伃,乎,伇:0,path:K::new(),乏:K::new(),index:HashMap::default(),};侹.倏(l,None,[0;3],0);let 伇=侹.伇;let mut 乏=侹.乏;乏.sort_by(|a,b|b.丮.cmp(&a.丮).then(a.与.cmp(&b.与)));乏.truncate(乓);(乏,伇)}const
佅:C=1000000;const 傍:C=200;fn 丷(l:&J,丁:F)->C{let 七=l.七(丁);let 丈:C=(l.丈(丁)&七).iter().filter_map(|q|l.亠(q)).map(|丆|10*C::from(丆)).sum();l.下(丁)+3*l.丟(丁)+4*C::from(七.count())+丈+10*C::from((l.乔()&七).count())}fn
倦(l:&J,丁:F,乻:G)->C{僱(l,丁,侗(l,丁,乻,T[1-丁]),)}fn 僱(l:&J,丁:F,与:C)->C{if 与==亲{return C::MIN;}l.下(丁)+l.丟(丁)-与}fn 倸(亩:C)->C{亩.clamp(-傍,0)}fn 僧(l:&J,丁:F,乻:[G;2],)->(C,[C;2]){let
主=1-丁;match l.丢(){Some(乳::五(兔))=>{return(if 兔==丁{佅}else{-佅},[0,0])}Some(乳::俑)=>return(0,[0,0]),None=>{}}let mut 丮=丷(l,丁)-丷(l,主);if 丁==l.东(){let mut 丄=l.clone();丄.仸();if
丄.丢().is_none()&&低(&丄,主,true,T[丁])<=丄.下(主){丮-=佅/2;}}let 企=[倦(l,丁,乻[0]),倦(l,主,乻[1]),];(丮+倸(企[0])-倸(企[1]),企)}
