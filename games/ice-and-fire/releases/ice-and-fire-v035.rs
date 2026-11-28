// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type C=usize;type I=u8;type 佩=u16;type M=u32;type Z=u64;type 似=u128;type 佨=i32;type F=i64;type 亅=f64;type O=bool;type 乙=String;type 乩=Instant;type K<T>=Vec<T>;type
P<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::{Arc,OnceLock};use std::time::{Duration,Instant};const
僙:O=true;fn 全(l:&J,丁:C,佣:F,丰:G,両:F)->O{let 七=l.丠();let 丮=1-七;let 偩=僙&&両<=l.下(丮)+2*l.丢(丮).max(0);let 伻=G::丁(丁)|G::丁(丁).丏();if!偩&&(伻&丰).is_empty(){return
false;}let(x,y)=乨(丁);let mut 乕=l.clone();if!乕.乏(L::亝{x,y}){return false;}if 佢(&乕,七)<佣{return true;}偩&&伟(&乕,丮,false,T[七])>両}const 儨:O=false;const 價:I=2;const
倩:I=1;const 倐:I=2;const 佴:C=0;const 侴:C=2;const 儂:O=false;const 億:O=false;const 儃:O=false;const 儁:O=false;const 僣:C=4;const 倌:C=5;#[derive(Clone)]enum
中{仜(L),亱(K<L>),}impl 中{fn 三(&self)->&[L]{match self{中::仜(丕)=>std::slice::from_ref(丕),中::亱(三)=>三,}}}fn 乨(丁:C)->(I,I){((丁%Y)as I,(丁/Y)as I)}fn 偡(l:&J,万:&mut
Vec<中>){万.clear();if l.丣().is_some(){return;}let 七=l.丠();let 享=l.乇().亷();let mut 丯=[G::U;4];for 丆 in 1..=享{丯[C::from(丆)]=l.乐(七,丆);}for 上 in l.互().iter(){let
Some((丌,丆))=l.介(上)else{continue;};let 乔=G::丁(上).丏()&丯[C::from(丆)];for 丟 in 乔.iter(){let(x,y)=乨(丟);万.push(中::仜(L::于{丌,x,y}));}}let 下=l.下(七);let 丄=l.丄(七);let
么=l.仫(七);let 亍=match 價{0=>么,1=>{let 佋=l.map().valid.丗(丄).丏()&丄;么.丗(丄)|佋}_=>么.丗(丄),};for 丆 in 1..=享{let 专=10*F::from(丆);if 下<专{break;}let mut 且=亍&丯[C::from(丆)];for
养 in 1..丆{且=且.丗(丯[C::from(养)]);}for 丁 in 且.iter(){let(x,y)=乨(丁);万.push(中::仜(L::乆{丆,x,y}));}}if 下>=亰{let 丫=l.丐(七)&么&l.份();if l.乇().佺(){for 丁 in(丫&l.map().乃).iter(){let(x,y)=乨(丁);万.push(中::仜(L::仌{x,y}));}}if
l.乇().伜(){let 佋=l.map().valid.丗(l.丐(七)).丏();let 且=(丫&佋).丗(l.map().乃);let 丰=丰(l,1-七);let 佣=F::from((丰&l.丄(七)).count());let 両=伟(l,1-七,false,T[七]);let 丨=if
儨{let(仅,侈)=乑(l,七);仅.丗(侈)}else{G::U};for 丁 in 且.iter().filter(|&丁|{let 伻=G::丁(丁)|G::丁(丁).丏();!(伻&丨).is_empty()||全(l,丁,佣,丰,両)}){let(x,y)=乨(丁);万.push(中::仜(L::亝{x,y}));}}}let
內=match 倩{1=>亰,2=>10,_=>F::MAX,};if 下>=內{let 丰=丰(l,1-七);let 亸=l.map().valid.丗(l.丐(七)|l.丐(1-七));for 上 in(l.互()&丰).iter(){let Some((丌,丆))=l.介(上)else{continue;};let
乔=G::丁(上).丏()&丯[C::from(丆)]&亸;let(伂,伃)=乨(上);let 代=if 倩==1{if l.map().乃.test(上){continue;}L::亝{x:伂,y:伃}}else{L::乆{丆:1,x:伂,y:伃,}};for 丟 in 乔.iter(){let(x,y)=乨(丟);万.push(中::亱(vec![L::于{丌,x,y},代]));}}}if
倐!=0&&下>=亰&&l.乇().伜(){兛(l,&丯,万);}if 佴!=0&&下>=20{冟(l,丯[1],下,万);}if 儁{克(l,丯[1],下,万);}for 丶 in 偓(l,倌,下,僣){if 丶.三.len()>1{万.push(中::亱(丶.三));}}if let Some(両)=俉(l,T[1-七],下){if
両.三.len()>1{万.push(中::亱(両.三));}}}fn 兛(l:&J,丯:&[G;4],万:&mut Vec<中>){let 七=l.丠();let 丮=1-七;let 丰=丰(l,丮);let 並=l.丐(七);let 丛=l.丐(丮);let 佋=丛.丏();let 乃=l.map().乃;for
上 in(l.互()&丰&l.丄(七)).iter(){if 乃.test(上){continue;}let Some((丌,丆))=l.介(上)else{continue;};let 儲=l.map().东(上,T[丮]);let 乔=G::丁(上).丏()&丯[C::from(丆)];let(伂,伃)=乨(上);for
丟 in 乔.iter(){let 便=match 倐{1=>並.test(丟)&&l.map().东(丟,T[丮])>儲,2=>並.test(丟)&&佋.test(丟),_=>並.test(丟)||(丛.test(丟)&&丟!=T[丮]),};if!便{continue;}let(x,y)=乨(丟);万.push(中::亱(vec![L::于{丌,x,y},L::亝{x:伂,y:伃},]));}}}fn
冟(l:&J,信:G,下:F,万:&mut Vec<中>){let 七=l.丠();let 丬=l.map();let 亸=丬.valid.丗(l.丐(七)|l.丐(1-七));let 丨=亸&信;let 亍:K<C> =(l.丄(七).丏()&丨).iter().collect();if 亍.is_empty(){return;}let
俨=|丒:C|{亍.iter().copied().min_by_key(|&丁|(丬.东(丁,丒),丁))};let mut 俊:K<(C,C)> =K::new();if 儂{for 丞 in 冠(l,&亍){if let Some(乄)=俨(丞){俊.push((乄,丞));}}}else{let
丒=T[1-七];let mut 俶=亍.clone();俶.sort_by_key(|&丁|(丬.东(丁,丒),丁));俶.truncate(侴);俊.extend(俶.into_iter().map(|乄|(乄,丒)));}let 仇=C::try_from(下/10).unwrap_or(0).min(佴);for(乄,丒)in
俊{let mut 両=vec![乄];let mut 今=G::丁(乄);while 両.len()<仇{let 亇=両[両.len()-1];if 亇==丒{break;}let 丅=(G::丁(亇).丏()&丨).丗(今).iter().min_by_key(|&丁|(丬.东(丁,丒),丁));let
Some(丅)=丅 else{break;};両.push(丅);今.set(丅);let 三=両.iter().map(|&丁|{let(x,y)=乨(丁);L::乆{丆:1,x,y}}).collect();万.push(中::亱(三));}if 儃{具(l,&両,万);}}}fn 具(l:&J,両:&[C],万:&mut
Vec<中>){let 七=l.丠();let 乄=両[0];let 亚=l.互()&l.丈(七)&G::丁(乄).丏();for 上 in 亚.iter(){let Some((丌,丆))=l.介(上)else{continue;};if!l.乐(七,丆).test(乄){continue;}let(x,y)=乨(乄);let
mut 三=vec![L::于{丌,x,y}];for&丁 in&両[1..]{let(x,y)=乨(丁);三.push(L::乆{丆:1,x,y});万.push(中::亱(三.clone()));}}}fn 冠(l:&J,亍:&[C])->K<C>{let 七=l.丠();let 丬=l.map();let(为,丵)=侇(l);let
並:K<C> =l.丈(七).iter().collect();let 丛:K<C> =l.丈(1-七).iter().collect();let 來=|丞:C|{亍.iter().map(|&丁|丬.东(丁,丞)).min().unwrap_or(I::MAX)};let 允=|丞:C|{let 丰=來(丞);並.iter().any(|&仙|{let
d=丬.东(仙,丞);d<=丰&&丛.iter().all(|&丮|d<丬.东(丮,丞))})};let 俣=|丞:C|{億&&(l.丐(1-七).test(丞)||丛.iter().any(|&丮|丬.东(丮,丞)<來(丞)))};let 丨=|且:G|->K<C>{let mut 亠:K<C> =且.丗(l.丐(七)).iter().filter(|&丞|來(丞)!=I::MAX&&!允(丞)&&!俣(丞)).collect();亠.sort_by_key(|&丞|(來(丞),丞));亠};let
mut 亠=丨(丵);if 亠.is_empty(){亠=丨(为);}亠.truncate(侴);亠}fn 兄(l:&J)->K<C>{let 七=l.丠();let 丬=l.map();let(为,丵)=侇(l);let 丄=l.丄(七);let 並=l.丐(七);let 丰=丰(l,1-七);let
丛:K<C> =l.丈(1-七).iter().collect();let 偟=|丁:C|{丄.iter().map(|a|丬.东(a,丁)).min().unwrap_or(I::MAX)};let mut 且:K<C> =(为|丵).iter().filter(|&丁|{if 並.test(丁){丄.test(丁)&&丰.test(丁)}else{!l.丐(1-七).test(丁)&&丛.iter().any(|&e|丬.东(e,丁)<偟(丁))}}).collect();且.sort_by_key(|&丁|(I::from(!丵.test(丁)),偟(丁),丁));且.truncate(
2*侴);且}fn 克(l:&J,信:G,下:F,万:&mut Vec<中>){let 七=l.丠();let 丬=l.map();let 亸=丬.valid.丗(l.丐(七)|l.丐(1-七));let 丨=亸&信;let 丄=l.丄(七);let 丫=丄&l.份();let 兺=if 佴==0{3}else{佴};for
丁 in 兄(l){if!l.丐(七).test(丁){let 乄=(丄.丏()&丨).iter().min_by_key(|&c|(丬.东(c,丁),c));if let Some(乄)=乄{let mut 両=vec![乄];let mut 今=G::丁(乄);while 両[両.len()-1]!=丁&&両.len()<兺{let
亇=両[両.len()-1];let 丅=(G::丁(亇).丏()&丨).丗(今).iter().min_by_key(|&c|(丬.东(c,丁),c));let Some(丅)=丅 else{break;};両.push(丅);今.set(丅);}let 专=10*F::try_from(両.len()).unwrap_or(F::MAX);if
両.len()>=2&&両[両.len()-1]==丁&&专<=下{let 三=両.iter().map(|&c|{let(x,y)=乨(c);L::乆{丆:1,x,y}}).collect();万.push(中::亱(三));}}}if 下>=亰&&l.乇().伜(){let 乃=(G::丁(丁)|G::丁(丁).丏())&丫;for
亃 in 乃.丗(丬.乃).iter(){let(x,y)=乨(亃);万.push(中::仜(L::亝{x,y}));}}}}const 僱:I=2;const 儙:F=5;fn 亚(l:&J,丂:C)->G{俧(l,丂,僱)}fn 俧(l:&J,丂:C,上:I)->G{let mut 亚=G::U;if
上>3{return 亚;}for 丁 in(l.丈(丂)&l.丄(丂)).iter(){if l.亯(丁).is_some_and(|丆|丆>=上){亚|=G::丁(丁);}}亚}fn 丰(l:&J,乀:C)->G{let 九=l.下(乀)+l.丢(乀);写(l,乀,亚(l,乀),九)}fn 仄(l:&J,丂:C)->G{丰(l,1-丂)&l.丄(丂)}fn
佢(l:&J,丂:C)->F{F::from(仄(l,丂).count())}const 倠:F=4;fn 冀(且:F)->F{且/倠}fn 乂(l:&J,丂:C,仄:G)->F{let 丄=l.丄(丂);let 乂=(l.乂()&丄).丗(仄);let 且=F::from(丄.count());let
俄=if l.乬()<=僅{且/僄+僃}else{冀(且)};俄*F::from(乂.count())}const 僅:M=15;const 僃:F=6;const 僄:F=倠;const 儋:F=2;const 併:I=1;const 倨:I=3;const 儌:I=6;fn 共(仨:I,么:I,仿:I,d:I)->I{match
仨{0=>4,1 if d<=么=>4,1=>0,_ if d<=么=>4,_ if d>=仿=>0,_=>(4*M::from(仿-d)/M::from(仿-么))as I,}}fn 傳(l:&J,丂:C)->F{let 丄=l.丄(丂);let 丮=if 併==0{G::U}else{l.丄(1-丂)};let[a,b,c]=丄.佦();let[e,f,g]=丮.佦();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丂 as Z).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as C)&(倷-1);儩.with(|俆|{let mut 俆=俆.borrow_mut();let 仰=&mut 俆[与];if 仰.0==丄&&仰.1==丮&&仰.2==丂 as I+1{return 仰.3;}let 丩=if 併==0{凈(丄,T[丂])}else if 併==1{let
临=l.map().valid;let mut 么=丮&临;for _ in 0..倨{么=(么|么.丏())&临;}凇(丄,T[丂],么)}else{let d=丼(l,丮);let mut 乤=[0u8;Q];for(w,&乔)in 乤.iter_mut().zip(d.iter()){*w=共(併,倨,儌,乔);}傴(丄,T[丂],&乤)};*仰=(丄,丮,丂
as I+1,丩);丩})}const 倷:C=1<<12;thread_local!{static 儩:std::cell::RefCell<K<(G,G,I,F)>> =std::cell::RefCell::new(vec![(G::U,G::U,0,0);倷]);}fn 冔(丄:G)->G{let
n=丄.侙();let s=丄.侚();let e=丄.傓();let w=丄.傔();let 冃=e.侙();let 冄=w.侙();let 冢=e.侚();let 冲=w.侚();let 兰=[n&冃&e,e&冢&s,s&冲&w,w&冄&n];let 仚=|a:G,b:G|(a|b).丗(a&b);let
倹=|[a,b,c,d]:[G;4]|{let(傍,偆)=(仚(a,b),a&b);let(傎,偈)=(仚(c,d),c&d);let t=傍&傎;let 假=仚(偆,偈);(仚(傍,傎),仚(假,t),(偆&偈)|(t&假))};let(兪,健,八)=倹([n,e,s,w]);let(児,偗,兑)=倹(兰);let
偀=児.丗(兪);let 傶=仚(健,偗);let 充=仚(傶,偀);let 儳=偗.丗(健)|偀.丗(傶);let 兆=仚(仚(八,兑),儳);丄&(充|兆)}fn 凇(丄:G,乴:C,么:G)->F{if!丄.test(乴){return 0;}let 乵=丄.count();let mut 乭=0i64;let
mut 仇=0i64;for v in(冔(丄)&么).iter(){if v==乴{continue;}let 乱=丄.丗(G::丁(v));let mut 亁=G::丁(乴);loop{let 丅=(亁|亁.丏())&乱;if 丅==亁{break;}亁=丅;}let 丶=F::from(乵-1-亁.count());if
丶>0{乭+=4;仇=仇.max(4*丶);}}(乭+仇)/4}fn 凈(丄:G,乴:C)->F{傴(丄,乴,&[4;Q])}fn 傴(丄:G,乴:C,乤:&[I;Q])->F{struct 伽<'a>{乤:&'a[I;Q],丄:G,云:[I;Q],乳:[I;Q],size:[I;Q],仲:I,乭:F,仇:F,}fn
休(d:&mut 伽<'_>,v:C)->I{d.仲+=1;let 云=d.仲;d.云[v]=云;let mut 乳=云;let mut 俱=1u8;let mut 丶=0u8;for w in(G::丁(v).丏()&d.丄).iter(){if d.云[w]==0{let 傢=休(d,w);俱+=傢;乳=乳.min(d.乳[w]);if
d.乳[w]>=云{丶+=傢;}}else{乳=乳.min(d.云[w]);}}d.乳[v]=乳;d.size[v]=俱;if 丶>0&&d.云[v]>1{let w=F::from(d.乤[v]);d.乭+=w;d.仇=d.仇.max(w*F::from(丶));}俱}if!丄.test(乴){return
0;}let mut d=伽{乤,丄,云:[0;Q],乳:[0;Q],size:[0;Q],仲:0,乭:0,仇:0,};休(&mut d,乴);(d.乭+d.仇)/4}fn 冤(l:&J,丂:C)->F{儋*(傳(l,1-丂)-傳(l,丂))}fn 偔(l:&J,丂:C)->F{if l.下(丂)+l.丢(丂)>=0{return
0;}(l.丈(丂)&l.丄(丂)).iter().filter_map(|丁|l.亯(丁)).map(|丆|10*F::from(丆)).sum()}const 僢:F=15;const 儢:[F;2]=[3,1];const 儠:[F;2]=[4,20];fn 冰(l:&J,丂:C)->[F;2]{let
mut 丙=[0;2];for 丁 in(l.丈(丂)&l.丄(丂)).iter(){match l.亯(丁){Some(2)=>丙[0]+=1,Some(3)=>丙[1]+=1,_=>{}}}丙}fn 傡(l:&J,丂:C)->F{let[傪,冹]=冰(l,丂);let 偁=|丆:C|(3-儢[丆])*儠[丆];-僢*傪+偁(0)*傪+偁(1)*冹}const
僘:F=0;const 僛:F=2;const 僚:F=100;fn 傗(仆:F)->F{if 仆<0{return 0;}僘+僛*仆.min(僚)/2}const 借:I=2;const 倜:F=2;const 倛:F=4;const 僜:I=3;fn 仓(l:&J,丂:C)->F{if 倜==0{return
0;}let 临=l.map().valid;let mut 丈=G::U;for 丁 in(l.丈(丂)&l.丄(丂)).iter(){if l.亯(丁).is_some_and(|丆|丆<=僜){丈|=G::丁(丁);}}let mut 価=临.丗(l.丐(丂));let mut 仡=G::U;let
mut 乔=0;for 东 in 1..=倛{価=(価|価.丏())&临;let 令=(丈&価).丗(仡);乔+=(东-1)*F::from(令.count());仡|=令;}乔+=倛*F::from(丈.丗(仡).count());倜*乔}const 倶:F=8;fn 乑(l:&J,丂:C)->(G,G){let
丄=l.丄(丂);let 仅=丄&l.丐(1-丂).丏();let 乣=l.乣()&丄;(仅,仅&(乣|乣.丏()))}fn 傰(l:&J,丂:C)->F{if 倶==0{return 0;}倶*F::from(乑(l,丂).1 .count())}const 侮:F=0;const 倘:I=1;const
候:F=4;const 侯:F=0;const 倗:F=0;const 像:M=14;const 僎:F=2;const 倵:F=0;const 儧:M=14;fn 伒(l:&J,丂:C)->F{if 倵==0||l.乬()>儧{return 0;}let 並=丼(l,l.丄(丂));let 丛=丼(l,l.丄(1-丂));let
mut 伊=0;for c in l.map().valid.iter(){match 並[c].cmp(&丛[c]){std::cmp::Ordering::Less=>伊+=1,std::cmp::Ordering::Greater=>伊-=1,std::cmp::Ordering::Equal=>{}}}倵*伊}struct
倚{valid:G,为:G,丵:G,}thread_local!{static 僐:std::cell::RefCell<P<倚>> =const{std::cell::RefCell::new(None)};}fn 侇(l:&J)->(G,G){僐.with(|丁|{let mut 乊=丁.borrow_mut();let
临=l.map().valid;if 乊.as_ref().is_none_or(|z|z.valid!=临){let 伏=伏(l);let mut 为=G::U;for c in 临.iter(){if 伏[c]<=僎{为.set(c);}}*乊=Some(倚{valid:临,为,丵:伈(l),});}乊.as_ref().map_or((G::U,G::U),|z|(z.为,z.丵))})}fn
侈(l:&J,丂:C)->G{let 丄=l.丄(丂);let 乣=l.乣()&丄;(乣|乣.丏())&丄}fn 丞(l:&J,丂:C)->F{if(侮==0&&候==0&&侯==0&&倗==0)||l.乬()>像{return 0;}let(为,丵)=侇(l);let mut 乵=0;if 侮!=0{let
並=丼(l,l.丄(丂));let 丛=丼(l,l.丄(1-丂));let mut 伊=0;for c in 为.iter(){let(a,b)=(並[c],丛[c]);if a!=I::MAX&&a.saturating_add(倘)<=b{伊+=1;}else if b!=I::MAX&&b.saturating_add(倘)<=a{伊-=1;}}乵+=侮*伊;}let
丙=|乖:G|F::from(乖.count());let(乁,丛)=(l.丄(丂),l.丄(1-丂));乵+=候*(丙(乁&为)-丙(丛&为));if 侯!=0{乵+=侯*(丙(侈(l,丂)&为)-丙(侈(l,1-丂)&为));}乵+=倗*(丙(乁&丵)-丙(丛&丵));乵}const 倓:F=4;const
倒:F=0;const 僊:F=4;const 僋:M=12;fn 兟()->[G;2]{static 僉:std::sync::OnceLock<[G;2]> =std::sync::OnceLock::new();*僉.get_or_init(兠)}fn 兠()->[G;2]{let mut 万=[G::U;2];for
c in 0..Q{let(x,y)=((c%Y)as F,(c/Y)as F);if x-y>=倓{万[0].set(c);}else if y-x>=倓{万[1].set(c);}}万}fn 傲(l:&J,丂:C)->F{let[a,b]=兟();let 丄=l.丄(丂);F::from((丄&a).count().min((丄&b).count())).min(僊)}fn
兞(l:&J,丂:C)->F{if 倒==0||l.乬()>僋{return 0;}倒*(傲(l,丂)-傲(l,1-丂))}const 倮:C=17;const 優:[F;倮]=[82,110,124,86,81,97,123,92,96,107,126,121,109,88,100,100,100,];fn
亢(侜:F,伸:F)->F{if 伸==100{侜}else{侜*伸/100}}fn 兖(l:&J,丂:C)->F{兗(l,丂,&優)}#[inline(always)]fn 兗(l:&J,丂:C,w:&[F;倮])->F{let 亚=[俧(l,丂,借),俧(l,1-丂,借),];let(丧,仧)=兘(l,丂,亚);let
傘=傗(仧[0])-傗(仧[1]);if l.丣().is_some(){return 丧+傘;}let mut 丧=丧+亢(傘,w[6]);if w[..6].iter().any(|&x|x!=100){let 並=俰(l,丂);let 丛=俰(l,1-丂);for i in 0..5{let 伋=並[i]-丛[i];丧+=亢(伋,w[i])-伋;}let
傅=侔(仧[0])-侔(仧[1]);丧+=亢(傅,w[5])-傅;}let 並=仄(l,丂);let 丛=仄(l,1-丂);let 佣=F::from(丛.count())-F::from(並.count());丧+亢(-2*偔(l,丂)+2*偔(l,1-丂),w[7])+亢(冤(l,丂),w[8])+亢(傡(l,丂)-傡(l,1-丂),w[9],)+亢(-仓(l,丂)+仓(l,1-丂),w[10])+亢(儙*佣/4,w[11])+亢(乂(l,丂,並)-乂(l,1-丂,丛),w[12],)+亢(傰(l,丂)-傰(l,1-丂),w[13])+亢(丞(l,丂),w[14])+亢(伒(l,丂),w[
15])+亢(兞(l,丂),w[16])}struct 佅{侟:但,丈:K<(M,C,[I;亦])>,}impl 佅{fn new(l:&J)->佅{let 侟=但::new(l);let 丈=l.互().iter().filter_map(|丁|{let(丌,_)=l.介(丁)?;let 书=侟.key(l,丁)?;Some((丌,丁,伱(&书)))}).collect();佅{侟,丈}}fn
偮(&self,l:&J,乼:&中)->P<(乚,[I;亦])>{let 中::仜(L::于{丌,x,y})=乼 else{return None;};let&(_,上,p)=self.丈.iter().find(|u|u.0==*丌)?;let 丟=C::from(*y)*Y+C::from(*x);Some((self.侟.kind(l,上,丟),p))}fn
伱(&self,l:&J,乼:&中)->I{self.偮(l,乼).map_or(0,|(乸,p)|p[乸 as C])}fn filter(&self,l:&J,k:C,乛:&mut Vec<中>){乛.retain(|乼|match self.偮(l,乼){Some((乸,p))=>{let 丸=兹(&p);丸.is_empty()||丸.iter().take(k).any(|&交|交==乸)}None=>true,});}}const
佫:C=1;const 僾:C=0;const 倎:P<C> =None;const 僂:亅=2000.0;#[derive(Clone,Copy,PartialEq)]enum 五{优,佮,倔,}#[derive(Clone,Copy)]struct 佳{亣:M,亗:C,仮:C,伅:五,free:O,}const
僯:亅=1000.0;const 僌:似=50;const 僠:P<佳> =Some(佳{亣:100,亗:2,仮:2,伅:五::优,free:true,});const 僬:C=8*佫;const 僽:C=3*佫;const 僤:C=32*佫;const 僿:Z=300*佫 as Z;const 僁:亅=0.75;const
僕:P<M> =Some(7);const 僓:O=false;const 僒:F=10;const 僔:M=6;const 侸:P<C> =None;const 侹:O=false;const 儞:M=7;#[derive(Clone,Copy)]struct 乫{乧:P<乩>,乌:P<乩>,乞:P<乩>,亙:Z,}#[derive(Default)]struct
佷{三:K<L>,俏:C,仃:Z,丝:C,串:C,偌:C,亪:C,丧:P<F>,乍:O,傝:O,们:C,}const 儦:O=true;#[derive(Clone)]struct 亖{三:K<L>,低:F,}struct 仝{三:K<L>,end:J,first:P<Z>,业:P<K<亖>>,}type
伓=HashMap<Z,F,BuildHasherDefault<DefaultHasher>>;struct 众{七:C,争:O,limit:乫,仃:Z,伧:伓,仯:P<K<中>>,供:O,习:P<件>,伢:(C,C),}const 侵:F=0;const 倇:O=true;const 傻:O=false;const
倆:P<I> =None;fn 儸(l:&J,丝:&[亖],万:&mut Vec<中>){if!倇&&倆.is_none(){return;}let 仮=丝.iter().map(|乢|乢.三.as_slice());儹(l,仮,(倇,傻,倆),万,);}fn 儹<'a>(l:&J,仮:impl Iterator<Item=&'a[L]>,(乣,丏,丆):(O,O,P<I>),万:&mut
Vec<中>,){let 七=l.丠();let 乁=l.丄(七);let mut 佑=G::U;for 乢 in 仮{for 丕 in 乢{if let L::乆{x,y,..}|L::于{x,y,..}=*丕{let 丁=C::from(y)*Y+C::from(x);if 乁.test(丁){佑|=G::丁(丁);}}}}if
佑.is_empty(){return;}let 丫=l.份()&乁;let 下=l.下(七);let 傇=|丕:L,万:&mut Vec<中>|{if!万.iter().any(|a|a.三()==[丕]){万.push(中::仜(丕));}};if 乣&&下>=亰&&l.乇().伜(){let mut
且=佑;if 丏{且|=佑.丏();}for 丁 in(且&丫).丗(l.map().乃).iter(){let(x,y)=乨(丁);傇(L::亝{x,y},万);}}if let Some(丆)=丆.filter(|&丆|丆<=l.乇().亷()){if 下>=10*F::from(丆){for 丁
in(佑&丫).iter(){let(x,y)=乨(丁);傇(L::乆{丆,x,y},万);}}}}fn 俁(l:&mut J,三:&[L])->O{三.iter().all(|&丕|l.乏(丕))}fn 书(三:&[L])->Z{let mut hash:Z=0xcbf29ce484222325;for
丕 in 三{let(乸,a,b,c)=match*丕{L::于{丌,x,y}=>(1,Z::from(丌),x,y),L::乆{丆,x,y}=>(2,Z::from(丆),x,y),L::仌{x,y}=>(3,0,x,y),L::亝{x,y}=>(4,0,x,y),};let 伺=乸|a<<8|Z::from(b)<<40|Z::from(c)<<48;hash=(hash^伺).wrapping_mul(0x0100000001b3);}hash}fn
佹(乗:&J,乢:&[L])->J{let mut 丅=乗.clone();丅.伣();for&丕 in 乢{丅.乏(丕);}丅.伣();丅}fn 佗(l:&J)->P<K<L>>{let 七=l.丠();let 丒=T[1-七];let 伹=|举:&J|举.丣()==Some(亏::亴(七));for
丁 in l.互().iter(){if l.map().东(丁,丒)!=1{continue;}let Some((丌,_))=l.介(丁)else{continue;};let(x,y)=乨(丒);let 丕=L::于{丌,x,y};let mut 举=l.clone();if 举.乏(丕)&&伹(&举){return
Some(vec![丕]);}}let 両=俉(l,丒,l.下(七))?;let mut 举=l.clone();(俁(&mut 举,&両.三)&&伹(&举)).then_some(両.三)}fn 傠(l:&J,丂:C,丱:F)->O{let 专=俐(l,丂,l.丈(丂),T[1-丂]);专!=仁&&l.下(丂)+丱*l.丢(丂)>=专}fn
傟(l:&J)->O{let 七=l.丠();if 侵==0||l.丣().is_some(){return false;}let mut 丅=l.clone();丅.伣();let 佢=丅.丣().is_none()&&伟(&丅,1-七,true,T[七])<=丅.下(1-七);!佢&&傠(l,七,侵)}impl
众{fn 偃(&self,乗:&J)->O{self.供&&乗.丣()!=Some(亏::亴(self.七))&&!傠(乗,self.七,侵)}fn 京(&self)->O{self.仃>=self.limit.亙||self.limit.乧.is_some_and(|t|乩::now()>=t)}fn
丧(&mut self,乗:&J,丝:&[亖],俅:F)->F{self.仃+=1;self.侕(乗,丝,俅)}fn 侕(&self,乗:&J,丝:&[亖],俅:F)->F{if 乗.丣()==Some(亏::亴(self.七)){return 伙;}if 丝.is_empty(){return self.value(乗);}let
mut 侢=F::MAX;for 乢 in 丝{侢=侢.min(self.value(&佹(乗,&乢.三)));if 侢<=俅{break;}}侢}fn value(&self,l:&J)->F{let 佊=兖(l,self.七);match&self.习{Some(习)if l.丣().is_none()=>佊+习.侜(l,self.七),_=>佊,}}fn
二(&self,乴:&J,主:&J)->P<佅>{let 円=(侸.is_some()||侹)&&!self.争&&乴.乬()>=儞;円.then(||佅::new(主))}fn 债(&self,乛:&mut Vec<中>){if let Some(习)=&self.习{for 侒 in&习.们{乛.push(中::亱(侒.clone()));}}}#[allow(clippy::too_many_arguments)]fn
冏(&mut self,主:&J,伡:F,乛:&[中],丸:&[(F,C)],乾:&mut[O],丝:&[亖],伤:&mut 伓,俠:O,)->(P<(C,J,F)>,O){let mut 不=None;let mut 佼=伡;for&(侎,与)in 丸{if 乾[与]{continue;}if 俠&&侎!=F::MAX&&侎<=佼-伡{return(不,true);}if
self.京(){return(不,false);}乾[与]=true;let 三=乛[与].三();let mut 丅=主.clone();if!俁(&mut 丅,三)||self.偃(&丅){伤.insert(书(三),F::MIN/4);continue;}let 丧=self.丧(&丅,丝,佼);伤.insert(书(三),丧-伡);if
丧>佼{佼=丧;不=Some((与,丅,丧));}}(不,false)}fn 佾(&mut self,乴:&J,丝:&[亖],佉:&[Z],儼:O)->仝{let 一=self;let mut 亘=None;let mut 主=乴.clone();let mut 三=K::new();let mut
伡=一.丧(&主,丝,F::MIN);let mut 伤=伓::default();let mut 乛=K::new();let mut 丸=K::new();while 主.丣().is_none()&&!一.京(){if 三.is_empty()&&儼{match&一.仯{Some(仯)=>乛.clone_from(仯),None=>{偡(&主,&mut
乛);if let Some(习)=一.习.as_ref().filter(|g|g.偼){乛.push(中::亱(习.丘.clone()));}一.债(&mut 乛);一.仯=Some(乛.clone());}}}else{偡(&主,&mut 乛);一.债(&mut 乛);}儸(&主,丝,&mut
乛);let 二=一.二(乴,&主);if let(Some(二),Some(k))=(&二,侸){二.filter(&主,k,&mut 乛);}丸.clear();for(与,乼)in 乛.iter().enumerate(){let k=书(乼.三());let 侎=伤.get(&k).or(一.伧.get(&k)).copied();丸.push((侎.unwrap_or(F::MAX),与));}match
二.as_ref().filter(|_|侹){Some(二)=>{let 傆:K<I> =乛.iter().map(|a|二.伱(&主,a)).collect();丸.sort_by(|a,b|{(b.0 .cmp(&a.0)).then(傆[b.1].cmp(&傆[a.1])).then(a.1
.cmp(&b.1))});}None=>丸.sort_by(|a,b|b.0 .cmp(&a.0).then(a.1 .cmp(&b.1))),}let mut 乾=vec![false;乛.len()];if 三.is_empty(){for(与,乼)in 乛.iter().enumerate(){乾[与]=佉.contains(&书(乼.三()));}}let
mut 不=None;for 俠 in[true,false]{let(也,冧)=一.冏(&主,伡,&乛,&丸,&mut 乾,丝,&mut 伤,俠,);不=也;if 不.is_some()||!冧{break;}}let Some((与,丅,丧))=不 else{break;};if 三.is_empty(){亘=Some(书(乛[与].三()));}三.extend_from_slice(乛[与].三());主=丅;伡=丧;}for(k,乘)in
伤{let 偦=一.伧.entry(k).or_insert(乘);*偦=(*偦).max(乘);}仝{三,end:主,first:亘,业:None,}}fn 仮(&self,乗:&J)->K<K<L>>{let mut 丅=乗.clone();丅.伣();let mut 万=K::new();if
丅.丣().is_some(){return 万;}let 争=丅.丠();let 下=丅.下(争);if let Some(両)=俉(&丅,T[self.七],下){万.push(両.三);}for 丶 in 偓(&丅,倌+僾,下,僽){万.push(丶.三);}let mut 偣=K::new();兦(&丅,&mut
偣);万.push(偣);let mut 丛=众{七:争,争:true,limit:乫{亙:僿,..self.limit},仃:0,伧:伓::default(),仯:None,供:false,习:None,伢:(0,0),};万.push(丛.佾(&丅,&[],&[],true).三);万.retain(|三|!三.is_empty());万}fn
值(&self,乗:&J)->K<亖>{self.仮(乗).into_iter().map(|三|{let 低=self.value(&佹(乗,&三));亖{三,低}}).collect()}fn 亪(&mut self,乴:&J,乼:&中,丝:&[亖])->P<仝>{let mut 乕=乴.clone();if!俁(&mut
乕,乼.三())||self.偃(&乕){return None;}self.仃+=1;let 乱=self.佾(&乕,丝,&[],false);let mut 三=乼.三().to_vec();三.extend_from_slice(&乱.三);Some(仝{三,end:乱.end,first:Some(书(乼.三())),业:None,})}fn
体(&self,丘:&仝,丝:&[亖])->F{let 丧=self.侕(&丘.end,丝,F::MIN);match&丘.业{Some(业)if!业.is_empty()=>丧.min(self.侕(&丘.end,业,F::MIN)),_=>丧,}}}impl 众{fn 光(&mut self,串:&[仝],亂:&[P<F>],丝:&[亖],(乺,亄):(P<C>,C),严:乫,乍:&mut
亐,)->P<C>{let 乺=乺?;let 冡=|与:C|亂[与].unwrap_or(F::MIN);let mut 丸:K<C> =(0..串.len()).collect();丸.sort_by_key(|&与|std::cmp::Reverse(冡(与)));let mut 亗:K<C> =丸.into_iter().take(亄).collect();if!亗.contains(&乺){亗.push(乺);}let
mut 亶:K<K<J>> =K::new();for&与 in&亗{let 丘=&串[与];let mut 備=K::new();if 丘.end.丣().is_none(){let mut 义:K<&[L]> =K::new();let 业=丘.业.iter().flatten();for 乢 in
丝.iter().chain(业){if!义.contains(&乢.三.as_slice()){义.push(&乢.三);備.push(佹(&丘.end,&乢.三));}}}亶.push(備);}let 乵:C=亶.iter().map(K::len).sum();self.伢=(亗.len(),乵);let
mut 乮=乵;let mut 不:P<(F,O,C)> =None;for(买,&与)in 亗.iter().enumerate(){let mut 乞=if 亶[买].is_empty(){self.体(&串[与],丝)}else{F::MAX};for 伅 in&亶[买]{let 丩=self.关(伅,严,乮,乍);乮-=1;乞=乞.min(丩);}let
书=(乞,与==乺,std::cmp::Reverse(买));let 促=不.is_none_or(|(d,t,r)|书>(d,t,std::cmp::Reverse(r)));if 促{不=Some((乞,与==乺,买));}}不.map(|(_,_,买)|亗[买])}fn 关(&self,l:&J,严:乫,乮:C,乍:&mut
亐,)->F{if l.丣().is_some(){return self.value(l);}let 亣=乮.max(1);let 伫=match 严.乞{Some(乞)=>{let 予=乩::now();let 伐=乞.saturating_duration_since(予).div_f64(亣
as 亅);乫{乧:Some(予+伐.mul_f64(0.6)),乌:Some(予+伐),乞:None,亙:Z::MAX,}}None=>乫{乧:None,乌:None,乞:None,亙:(严.亙/亣 as Z).max(1),},};let 丷=俻(l,伫,乍,None,(None,None));丷.丧.unwrap_or_else(||self.value(l))}}fn
兵(严:乫,亣:M)->乫{let 予=乩::now();let 伋=亅::from(亣)/100.0;let 丶=|t:乩|予+t.saturating_duration_since(予).mul_f64(伋);乫{乧:严.乧.map(丶),乌:严.乌.map(丶),乞:None,亙:(似::from(严.亙)*似::from(亣)/100).max(1)as
Z,}}impl 众{fn 其(&self,串:&[仝],亂:&[P<F>],丝:&[亖],(乺,乲):(P<C>,佳),(企,乍):(乫,&mut 亐),)->P<C>{let 乺=乺?;let 乸=if 乲.伅==五::佮{五::佮}else{五::优};let mut 丸:K<C> =(0..串.len()).filter(|&与|与!=乺&&亂[与].is_some()).collect();丸.sort_by_key(|&与|std::cmp::Reverse(亂[与]));let
亗:K<C> =std::iter::once(乺).chain(丸).take(乲.亗.max(1)).collect();let mut 亶:K<K<J>> =K::new();for&与 in&亗{let 丘=&串[与];let mut 仈:K<(F,J)> =K::new();if 丘.end.丣().is_none(){let
mut 义:K<&[L]> =K::new();let 业=丘.业.iter().flatten();for 乢 in 丝.iter().chain(业){if!义.contains(&乢.三.as_slice()){义.push(&乢.三);let 乕=佹(&丘.end,&乢.三);仈.push((self.value(&乕),乕));}}}仈.sort_by_key(|(低,_)|*低);仈.truncate(乲.仮.max(1));亶.push(仈.into_iter().map(|(_,乕)|乕).collect());}let
乵:C=亶.iter().map(K::len).sum();let 净=if 乲.free{似::from(企.亙)*僌/100}else{似::from(企.亙)*似::from(100-乲.亣.min(100))/100};let 兿=Duration::from_secs_f64(僯/1e6*俼());let
兒=(净/乵.max(1)as 似).max(1)as Z;let mut 乮=乵;let mut 不:P<(F,O,C)> =None;for(买,&与)in 亗.iter().enumerate(){let mut 乞=if 亶[买].is_empty(){self.体(&串[与],丝)}else{F::MAX};let
偝=|乮:C|match 企.乌{Some(乌)if 乲.free=>{乌.saturating_duration_since(乩::now()).div_f64(乮.max(1)as 亅)>=兿}_=>true,};let mut 乸=乸;let mut 偎=true;if 乲.free&&乸==五::优&&!亶[买].is_empty()&&!偝(乮){if
乲.伅==五::倔{乸=五::佮;}else{乮-=亶[买].len();continue;}}for 伅 in&亶[买]{if 企.乌.is_some_and(|乌|乩::now()>=乌){return 不.map(|(_,_,买)|亗[买]);}if 乸==五::优&&!偝(乮){偎=false;乮-=1;continue;}let
mut 伫=乫{乧:None,乌:None,乞:None,亙:兒,};if let Some(乌)=企.乌{let 予=乩::now();let 伐=乌.saturating_duration_since(予).div_f64(乮.max(1)as 亅);let 乧=if 乸==五::优{0.6}else{0.9};伫.乧=Some(予+伐.mul_f64(乧));伫.乌=Some(予+伐);伫.亙=Z::MAX;}let
丩=self.兴(伅,乸,伫,乍);乮-=1;乞=乞.min(丩);}if!偎{continue;}let 书=(乞,与==乺,std::cmp::Reverse(买));let 促=不.is_none_or(|(d,t,r)|书>(d,t,std::cmp::Reverse(r)));if 促{不=Some((乞,与==乺,买));}}不.map(|(_,_,买)|亗[买])}fn
兴(&self,l:&J,乸:五,严:乫,乍:&mut 亐)->F{if l.丣().is_some(){return self.value(l);}match 乸{五::优=>{let 丷=俻(l,严,乍,None,(None,None));丷.丧.unwrap_or_else(||self.value(l))}五::佮|五::倔=>{if
佗(l).is_some(){return 伙;}let mut 丿=众{七:self.七,争:false,limit:严,仃:0,伧:伓::default(),仯:None,供:傟(l),习:None,伢:(0,0),};let 丘=丿.佾(l,&[],&[],true);丿.侕(&丘.end,&[],F::MIN)}}}}fn
军(也:&[亖],丝:&mut Vec<亖>)->O{let 乜:K<_> =丝.iter().map(|r|r.三.clone()).collect();for 乢 in 也{if 丝.iter().any(|r|r.三==乢.三){continue;}丝.push(乢.clone());}丝.sort_by_key(|乢|乢.低);丝.truncate(僬);丝.iter().map(|r|&r.三).ne(&乜)}fn
俺(l:&J,严:乫,乍:&mut 亐)->佷{傥(l,严,乍,None)}fn 傥(l:&J,严:乫,乍:&mut 亐,习:P<件>,)->佷{俻(l,严,乍,习,(倎,僠))}fn 俻(l:&J,严:乫,乍:&mut 亐,习:P<件>,(乞,乲):(P<C>,P<佳>),)->佷{let 七=l.丠();let
mut 丷=佷::default();if l.丣().is_some(){return 丷;}if let Some(三)=佗(l){丷.三=三;丷.丧=Some(伙);return 丷;}let 丳=|亣:亅|match(严.乧,严.乌){(Some(乧),Some(乌))=>Some(乧+(乌-乧).mul_f64(亣)),_=>None,};if
僕.is_some_and(|世|l.乬()>=世){乍.end();}if 乍.冓(l){let 三=乍.世(l,丳(0.4));let 儻=三.iter().any(|丕|{matches!(丕,L::乆{..}|L::仌{..}|L::亝{..})});let 仓=僓&&!儻&&l.乬()>=僔&&l.下(七)>=僒;if!仓{丷.乍=true;丷.三=三;return
丷;}乍.end();}let 企=严;let 严=乲.filter(|乲|!乲.free).map_or(严,|乲|兵(严,乲.亣));let 丳=|亣:亅|match(严.乧,严.乌){(Some(乧),Some(乌))=>Some(乧+(乌-乧).mul_f64(亣)),_=>None,};let
mut 丿=众{七,争:false,limit:乫{乧:丳(僁),..严},仃:0,伧:伓::default(),仯:None,供:傟(l),习,伢:(0,0),};let mut 丝=K::new();let mut 串:K<仝> =K::new();let mut 佉=K::new();let mut
侃=false;let mut 亪:K<中> =K::new();let mut 乄=丿.习.as_ref().filter(|g|g.start).map(|g|中::亱(g.丘.clone()));while 丷.俏<僤&&(串.is_empty()||!丿.京()){let 俵=match 乄.take(){Some(乼)=>丿.亪(l,&乼,&丝),None=>None,};let
丘=if let Some(丘)=俵{丘}else if 侃{let Some(乼)=亪.pop()else{break;};match 丿.亪(l,&乼,&丝){Some(丘)=>丘,None=>continue,}}else{丿.佾(l,&丝,&佉,true)};丷.俏+=1;let 傐=串.iter().any(|p|p.三==丘.三);let
亘=丘.first;let 傜=丿.京();let mut 丘=丘;let mut 偶=false;if!傜&&!傐{let 也=丿.值(&丘.end);偶=军(&也,&mut 丝);丘.业=Some(也);}if!傐{串.push(丘);}if 傜{break;}if!偶&&!侃{match 亘{Some(亘)=>佉.push(亘),None=>{侃=true;亪=丿.仯.clone().unwrap_or_default();亪.retain(|a|!佉.contains(&书(a.三())));let
二=丿.二(l,l);if let(Some(二),Some(k))=(&二,侸){二.filter(l,k,&mut 亪);}let 乘=|a:&中|{丿.伧.get(&书(a.三())).copied().unwrap_or(F::MIN)};match 二.as_ref().filter(|_|侹){Some(二)=>{亪.sort_by_key(|a|(乘(a),二.伱(l,a)));}None=>亪.sort_by_key(乘),}}}}}let
兕=!丿.京();丷.亪=C::from(侃);丷.仃=丿.仃;丷.丝=丝.len();丷.串=串.len();let 仦=||严.乌.is_some_and(|t|乩::now()>=t);let mut 亂:K<P<F>> =vec![None;串.len()];for 与 in(0..串.len()).rev(){if
与+1<串.len()&&仦(){break;}亂[与]=Some(丿.体(&串[与],&丝));}let 伌=|亂:&[P<F>]|{let mut 不:P<(F,C)> =None;for(与,丧)in 亂.iter().enumerate().rev(){if let Some(丧)=*丧{if
不.is_none_or(|(交,_)|丧>交){不=Some((丧,与));}}}不};let mut 不=伌(&亂);丿.limit.乧=严.乌;while let Some((_,与))=不{if!儦||串[与].业.is_some()||仦(){break;}let 也=丿.值(&串[与].end);串[与].业=Some(也);丷.偌+=1;亂[与]=Some(丿.体(&串[与],&丝));不=伌(&亂);}if
let Some(乲)=乲.filter(|乲|!乲.free||兕){let 乺=不.map(|(_,与)|与);let 俍=丿.其(&串,&亂,&丝,(乺,乲),(企,乍));if let Some(与)=俍{不=Some((亂[与].unwrap_or(F::MIN),与));}}else if
let Some(亄)=乞{let 俵=乩::now();for(与,丧)in 亂.iter_mut().enumerate(){丧.get_or_insert_with(||丿.体(&串[与],&丝));}不=伌(&亂);let 乺=不.map(|(_,与)|与);let 俒=丿.光(&串,&亂,&丝,(乺,亄),严,乍);if
let Some(与)=俒{不=Some((亂[与].unwrap_or(F::MIN),与));}eprintln!("DEEP turn={} candidates={} leaves={} differs={} ms={:.1}",l.乬(),丿.伢.0,丿.伢.1,I::from(俒.is_some()&&俒!=乺),俵.elapsed().as_secs_f64()*1000.0);}丷.丧=不.map(|(丧,_)|丧);if
let Some((_,与))=不{丷.傝=丿.习.as_ref().is_some_and(|g|!g.丘.is_empty()&&串[与].三.starts_with(&g.丘));丷.们=丿.习.as_ref().map_or(0,|g|{g.们.iter().filter(|p|p.iter().all(|c|串[与].三.contains(c))).count()});let
mut 丑=l.clone();for&丕 in&串[与].三{if 丑.乏(丕){丷.三.push(丕);}}}丷}fn 冒(严:&dyn Fn(乩,M)->乫,mut 乍:亐,俟:impl BufRead,侐:impl Write,)->Result<(),乙>{let mut 仉=佂::default();丱(俟,侐,|伨,亭,世,乄|match
J::read(伨,亭,亾::侧,世){Ok(l)=>{let 亡=习(&l,&mut 仉);let 丷=match 亡{Some(习)=>{傥(&l,严(乄,世),&mut 乍,Some(习))}None=>俺(&l,严(乄,世),&mut 乍),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",世,丷.俏,丷.仃,丷.丝,丷.偌,丷.亪,I::from(丷.傝),丷.们,乄.elapsed());儰(&丷.三)}Err(仾)=>{eprintln!("plan: cannot read the input: {}"
,仾);"WAIT".to_string()}},)}const 僴:P<F> =None;const 儥:O=false;#[derive(Clone,Copy,PartialEq)]enum 佁{侩,亥,}#[derive(Clone,Copy)]struct 仏{丁:C,kind:佁,伄:O,value:F,}fn
亀(l:&J,丂:C)->K<仏>{内(l,丂,儥)}fn 内(l:&J,丂:C,决:O)->K<仏>{let(为,丵)=侇(l);let(乁,丛)=(l.丄(丂),l.丄(1-丂));let(佐,仼)=(丼(l,乁),丼(l,丛));let 佊=佚(l,&佐,&佐,&仼);let mut 万=K::new();for
丁 in(为|丵).iter(){let 俓=丼(l,G::丁(丁));let 伄=乁.test(丁);let 丩=if 决{佚(l,&佐,&俓,&仼)-佚(l,&仼,&俓,&佐)}else if 伄{let 債=丼(l,兡(乁,丂,丁));佊-佚(l,&債,&債,&仼)}else{佚(l,&佐,&俓,&仼)-佊};万.push(仏{丁,kind:if
丵.test(丁){佁::侩}else{佁::亥},伄,value:丩,});}万.sort_by_key(|o|(-o.value,o.kind!=佁::侩,o.丁));万}fn 佚(l:&J,a:&[I;Q],g:&[I;Q],b:&[I;Q])->F{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 仡(亀:&[仏])->K<仏>{亀.iter().filter(|o|o.value>=僴.unwrap_or(1)).copied().collect()}fn 兡(丄:G,丂:C,丁:C)->G{let
乱=丄.丗(G::丁(丁));let mut 乡=G::丁(T[丂])&乱;loop{let 丅=乡|(乡.丏()&乱);if 丅==乡{return 乡;}乡=丅;}}const 倰:F=10;const 僲:F=100;#[derive(Clone,Copy)]enum 佰{佇(M),乆(C),}#[derive(Clone,Copy)]struct
亜{丱:M,保:佰,}#[derive(Clone,Copy)]struct 佃{丁:C,ours:P<亜>,侏:P<亜>,theirs:P<亜>,仆:F,}fn 冘(l:&J,丂:C,亀:&[仏])->K<佃>{let mut 万=K::new();for 丽 in 亀.iter().filter(|o|!o.伄){let
丁=丽.丁;let(册,侏)=倽(l,丂,丁);let 並=不(册,侏);let(冸,冷)=倽(l,1-丂,丁);let 丛=不(冸,冷);let 丱=|a:P<亜>|a.map_or(僲,|a|F::from(a.丱));万.push(佃{丁,ours:並,侏,theirs:丛,仆:丱(丛)-丱(並),});}万}fn
不(仙:P<亜>,佥:P<亜>)->P<亜>{match(仙,佥){(Some(u),Some(t))=>Some(if t.丱<u.丱{t}else{u}),(u,t)=>u.or(t),}}fn 倽(l:&J,乀:C,丁:C)->(P<亜>,P<亜>){let 丬=l.map();let mut
仙:P<亜> =None;for 上 in l.丈(乀).iter(){let d=丬.东(上,丁);let Some(丌)=l.亮(上)else{continue;};if d!=I::MAX&&仙.is_none_or(|u|M::from(d)<u.丱){仙=Some(亜{丱:M::from(d),保:佰::佇(丌),});}}let
下=l.下(乀)+if 乀==l.丠(){0}else{l.丢(乀)};let 侠=match(下,l.丢(乀)){(下,_)if 下>=倰=>Some(0),(_,丢)if 丢<=0=>None,(下,丢)=>Some((倰-下+丢-1)/丢),};let mut 佥:P<亜> =None;if let
Some(侠)=侠{let 亍=l.丄(乀).丏()&丬.valid&l.乐(乀,1);for 上 in 亍.iter(){let d=丬.东(上,丁);if d!=I::MAX&&佥.is_none_or(|t|1+侠 as M+M::from(d)<t.丱){佥=Some(亜{丱:1+侠 as M+M::from(d),保:佰::乆(上),});}}}(仙,佥)}const
倱:F=10;#[derive(Default)]struct 佂{偻:BTreeMap<M,C>,}#[derive(Default)]struct 仟{伀:K<(M,C)>,亽:K<(C,C)>,back:K<M>,}fn 伝(l:&J,丂:C,亀:&[仏],伎:&[佃],丝:&mut 佂,)->仟{倾(l,丂,亀,伎,丝,l.丈(丂))}fn
倾(l:&J,丂:C,亀:&[仏],伎:&[佃],丝:&mut 佂,俕:G,)->仟{let 丬=l.map();let 丈:BTreeMap<M,C> =(l.丈(丂)&俕).iter().filter_map(|c|l.亮(c).map(|丌|(丌,c))).collect();let mut 丨:K<(仏,佃)>
=K::new();for o in 仡(亀).into_iter().filter(|o|!o.伄){if let Some(r)=伎.iter().find(|r|r.丁==o.丁){if r.ours.is_some()&&r.仆>=0{丨.push((o,*r));}}}丨.sort_by_key(|(o,r)|{(-o.value,o.kind!=佁::侩,r.ours.map_or(M::MAX,|a|a.丱),o.丁,)});let
之=|仙:M,丞:C|M::from(丬.东(丈[&仙],丞));let 偫=|亵:&BTreeMap<C,M>,佞:&BTreeSet<M>,丌:M|{佞.contains(&丌)||亵.values().any(|&u|u==丌)};let mut 亵:BTreeMap<C,M> =BTreeMap::new();for(&丌,&丞)in&丝.偻{if
丈.contains_key(&丌)&&丨.iter().any(|(o,_)|o.丁==丞)&&!亵.contains_key(&丞){亵.insert(丞,丌);}}let mut 佞:BTreeSet<M> =BTreeSet::new();for(o,_)in&丨{let Some(&偭)=亵.get(&o.丁)else{continue;};let
俨=丈.keys().copied().filter(|&丌|!偫(&亵,&佞,丌)).map(|丌|(之(丌,o.丁),丌)).filter(|&(d,_)|d!=M::from(I::MAX)).min();if let Some((d,丌))=俨{if d<之(偭,o.丁){佞.insert(偭);亵.insert(o.丁,丌);}}}let
mut 亽=K::new();for(o,r)in&丨{if 亵.contains_key(&o.丁){continue;}let 丫=丈.keys().copied().filter(|&丌|!偫(&亵,&佞,丌)).map(|丌|(之(丌,o.丁),丌)).filter(|&(d,_)|d!=M::from(I::MAX)).min();let
催=丫.map(|(d,_)|F::from(d));let 依=r.侏.and_then(|t|match t.保{佰::乆(丁)if l.下(丂)-倱*亽.len()as F>=倱&&!亽.iter().any(|&(c,_)|c==丁)=>{Some((丁,F::from(t.丱)))}_=>None,});let
储=依.map(|(_,t)|t);let 傭=match(催,储){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 俍=if 傭{储}else{催};let Some(丱)=俍 else{continue;};if r.theirs.map_or(100,|a|F::from(a.丱))<丱{continue;}match(傭,依,丫){(true,Some((丁,_)),_)=>亽.push((丁,o.丁)),(false,_,Some((_,丌)))=>{亵.insert(o.丁,丌);}_=>{}}}let
mut 万=仟{亽,..仟::default()};for(o,_)in&丨{if let Some(&丌)=亵.get(&o.丁){万.伀.push((丌,o.丁));}}万.back=l.丈(丂).iter().filter_map(|c|l.亮(c)).filter(|丌|!万.伀.iter().any(|&(u,_)|u==*丌)).collect();万.back.sort_unstable();丝.偻=万.伀.iter().copied().collect();万}const
倲:F=10;fn 偊(l:&J,丂:C,丌:M)->P<C>{l.丈(丂).iter().find(|&c|l.亮(c)==Some(丌))}fn 傛(l:&J,丂:C,上:C,丞:C)->K<C>{let 丬=l.map();let 令=丬.东(上,丞);if 令==I::MAX||令==0{return
K::new();}let mut 万:K<C> =(G::丁(上).丏()&丬.valid).iter().filter(|&n|丬.东(n,丞)<令).collect();万.sort_by_key(|&n|(l.丐(丂).test(n),n));万}fn 仂(丑:&mut J,万:&mut Vec<L>,丕:L)->O{let
偽=丑.乏(丕);if 偽{万.push(丕);}偽}#[derive(Clone,Copy)]struct 伖{修:O,guard:I,偛:O,傏:O,}fn 们(l:&J,仉:&仟,亓:伖)->K<K<L>>{let mut 万=K::new();for&(丌,丞)in&仉.伀{let 冊=仟{伀:vec![(丌,丞)],..仟::default()};let
侒=傞(l,&冊,伖{修:false,..亓},);if!侒.is_empty(){万.push(侒);}}万}fn 两(丑:&J,丌:M,上:C,丟:C,仄:O,亓:伖,)->P<K<L>>{let 丂=丑.丠();let mut 举=丑.clone();let mut 三=K::new();let(x,y)=乨(丟);if!仂(&mut
举,&mut 三,L::于{丌,x,y}){return None;}if 仄&&亓.guard!=0{let(伂,伃)=乨(上);let 代=if 亓.guard==1{L::亝{x:伂,y:伃}}else{L::乆{丆:1,x:伂,y:伃,}};if!仂(&mut 举,&mut 三,代){return
None;}}if 亓.傏&&佢(&举,丂)>佢(丑,丂){return None;}Some(三)}fn 傞(l:&J,仉:&仟,亓:伖)->K<L>{let 丂=l.丠();let 丬=l.map();let mut 丑=l.clone();let mut 万=K::new();let 丰=丰(l,1-丂);let
mut 乮:K<(M,C)> =K::new();for&(丌,丞)in&仉.伀{let Some(上)=偊(&丑,丂,丌)else{continue;};if!丑.互().test(上){continue;}let 仄=丰.test(上);for 丟 in 傛(&丑,丂,上,丞){if let Some(三)=两(&丑,丌,上,丟,仄,亓){for
丕 in&三{丑.乏(*丕);}if 仄&&亓.guard==0{乮.push((丌,上));}万.extend(三);break;}}}for&(丌,丞)in&仉.伀{if 丑.下(丂)<倲{break;}let Some(丳)=偊(&丑,丂,丌)else{continue;};let 儵=乮.iter().find(|&&(u,_)|u==丌).map(|&(_,c)|c);let
且=match 儵{Some(丁)if 亓.偛=>vec![丁],_=>傛(&丑,丂,丳,丞),};for 丟 in 且{let(x,y)=乨(丟);if 仂(&mut 丑,&mut 万,L::乆{丆:1,x,y}){break;}}}if 亓.修{for&(_,丞)in&仉.亽{if 丑.下(丂)<倲{break;}let
亍=丑.丄(丂).丏()&丬.valid;let mut 且:K<C> =亍.iter().filter(|&c|丬.东(c,丞)!=I::MAX).collect();且.sort_by_key(|&c|(丬.东(c,丞),c));for 丟 in 且{let(x,y)=乨(丟);if 仂(&mut
丑,&mut 万,L::乆{丆:1,x,y}){break;}}}}万}const 倳:I=0;const 儜:I=1;const 倡:F=0;const 僵:O=false;const 僑:I=0;const 僇:O=false;const 儈:O=false;const 傹:O=false;const
働:M=7;const 償:M=14;const 儏:O=倳!=0||倡!=0;struct 件{丘:K<L>,偼:O,start:O,们:K<K<L>>,亀:K<(C,F,F)>,伲:F,}impl 件{fn 侜(&self,l:&J,丂:C)->F{if self.伲==0{return 0;}let(乁,丛)=(l.丄(丂),l.丄(1-丂));let
mut 乵=0;for&(丁,丩,亸)in&self.亀{乵+=if 乁.test(丁){丩}else if 丛.test(丁){-丩}else{亸};}self.伲*乵/4}}fn 习(l:&J,丝:&mut 佂)->P<件>{let 予=l.乬();if!儏||!(働..=償).contains(&予)||l.丣().is_some(){return
None;}兩(l,丝,儊)}#[derive(Clone,Copy)]struct 佶{仨:I,亓:伖,係:O,伲:F,俬:O,}const 儊:佶=佶{仨:倳,亓:伖{修:儜==2,guard:僑,偛:僇,傏:儈,},係:傹,伲:倡,俬:僵,};fn 兩(l:&J,丝:&mut 佂,乎:佶)->P<件>{let
佶{仨,亓,係,伲,俬,}=乎;let 丂=l.丠();let 亀=亀(l,丂);let 伎=冘(l,丂,&亀);let 仉=if 係{let 俕=l.丈(丂).丗(丰(l,1-丂));倾(l,丂,&亀,&伎,丝,俕)}else{伝(l,丂,&亀,&伎,丝)};let 丘=if 仨!=0{傞(l,&仉,亓)}else{K::new()};let
仡=仡(&亀);let 冶=仡.iter().map(|o|{let 仆=伎.iter().find(|r|r.丁==o.丁).map_or(0,|r|r.仆.clamp(-1,1));let 亸=if 俬{o.value*仆}else{0};(o.丁,o.value,亸)}).collect();let
冎=if 仨&4!=0{们(l,&仉,亓)}else{K::new()};Some(件{们:冎,偼:!丘.is_empty()&&仨&1!=0,start:!丘.is_empty()&&仨&2!=0,丘,亀:冶,伲,})}const 儗:&str="׳椮浦粜顯漏煵波螲〴ꁜ却⬖ﾊ瀄颡뷻쓓퟊㧉鵃ꨠ渠诡懕豐퀊틞ⴂ⦃맷꽩灹칁쿜埓㞄뾢ῂए蕰穱茄⻯逎黷䒌䝷㐺亮φ䨋ᢞꦏ㠬Տᜅ᷊Ⴒ꺬뱌蔆悸ቝ蘱ꅺ쏉묇踗榴뾵碴ꀊ˯᮴莈繄探ꉷ襩鋊㍄탊赍䮟ᳺ囔ᬡ狮죇ꥻ镵葛⤈齢⸫ءؚ矹쭐쨒걕钣有㔑畳짉谶漰᷄纼뜺剷Ĩ翾쳉鍞ꋤ`柫볘뇣駟氯襟弸僸洂౓먨৛罆쯙㒫銷驯Ღ꽖據䋉ྏ护뷨䌞鎡篻ꦆ轛ડ蹮ኌ팱眄얜ƻ㖪ࣩ樖讞魞컙旻ꯚ찧鳄㯹ť캹剪詨댯画帙᪒爠盭늓ਲ켃ࢪ琳ೡ懊吾᭹睆줾䡋獙㴉봂➜삜푎郆થ괬⿬ṓ悊ឩ堤湹潰੬뻹틫⭸耣ધᔉﴱ垭䯁걙ꊒ边氤Ƭử㵒觷쉴䏉宎患䭟숾嗏橣Ⱃ鼼廚鮒鍶ⷭ求㻅穻빃咤㸱皪䑲ﾃ穔︓컝됶㜰ぅ妼梩鮯ਚ梣닏謏척漥⇽阔ﬞ廧䋐旟昛ಉ⬉㤀픲㒒薑间砰闈䛌Ĳ올媯ᩩ泈朡㕵饚ו䜛熋谷➭쌳떾쉻暻䒭ꅵࣳ᡺顗䎞᳃习뺰⡻樸❲笞ẟ㩴⛐䱿뽆喞ﰘ袃涸泮Ḇ㕴គ＂₶㑮툀혣힓ᤀ繨ᑧ벳娘ⲧ렎⃽䦖˺ಆ殜涝齫ף茒ꚙ䙫ꤖ諓此≕봤∲㣾㹥팤ྟ펀ᜱ荬犅䊠鲎셎㯢똺㣫쵵ⷍ绕뙁ඍ㾞톴骨Ӛ널㞼쫄貶㐍ᘇ㼮퓌㨚婠䦇⩣ǜ႑譇瘺䟖煦䠷۠ђⲤꬣ镯挎䒎좿ᙼ㷹꣊⮐槢뉷ຣ힥䶏ꄈ睍Ꮷሮ䎽㩭굗彦橞䆨❡ᡵ澭䭼ꦅ宒鐸옖回雸岟礔錯Ż⺾⢸꾻䪖턉᧟떞醌ꓘ힓ⴔᒒ鞙睼쥟ꐼࣾ쾄Āħ؁몱▏ꄧ䑎㳨獼뷮ꎀ찿ﰈ厓਌珜ⲵ啫뻫灕湘ⲓ㜓曩ẋ燛絣瑭㣽鳮〮羌鏥∶˞琓੆걄⋰㲉ᮩ햀涣瑏渐骩᳦谊뽬⾅浇થ⴨Ą釔틯ꁶ캫㐜㇓Ό灕妚ꎆꁦ袈粹旯䆿鎮矛뙰첦䫳ꆍ薿馻Ⴤ徚ꁍ窳ꮌ轞ɕ⠸ꗤ䷸ꇈ밙ꌭᚈ꿑헦碗୞쟨ₛ㣚援篴⃢㞠⿐崜쨼△쮿♌ઇ填ċ깵莑퉜娴凌쉉ᡆ紐惲Ꮷ⛆筄䆓ﺳ磐菀ऀ簉ܝ豻꧿쀇瓢ᙄ╸禗䊅ࠣ싈벑㪒ﺜ쾵흛岂ͰⷫԊ㚘扙嗝⦦蹰뎄ꖤꃱɖ泡誳冡韬嶁礀㙐ﾷ졗ꤟퟚĊ⨈쑭៽廆喆旮燔ⲳί枖㪸캔啟㔶ⲉಧ밋쿌펍搒캋洋轺膹궯⛏턥褈俚㺱┷݋䠧꘢旾쎜鼾팅뒎䫦怫풴씰杼眸礤ᥐ뗌꛶͞뿸㋚孃衫ｭĲ鼀䳖䩎캢ꊩ耭堠裶᠖榉灾뿼楎ⶦ⟭௯쵍悊邷鮤⊝炭卺ີ噕뷯뛧뷴뇔ᆼ艆줠䞓푱嚛䱴禿⼮燨天䡘ꀀ뉙슯왦哎꘏✚寷枘圫꿇┫籍㒆Ĕ妰㤻뵝霂Ṻ뇔숌ų颃ᦣ돶꘣ᗲ㨰쎕䕉ⓥ⩕霂ᦕꊡ齾Ꮆ઄ɺ觢쮉缯因蒾㩳㵟䲤悵쒀⏥ओ泀ⶲ돦瘩葑點獇蕑걱嚆뇌찿땀滐鼁ӀƢૹᡂ⫈稍理䍆狑힊쩳䗁쒏焿ⷐඉ혖㕟ﲌ枠极養䝕믩臘熎籩齚눳氁ꞩ링쟽珨㝐ף湑ꆜĒ뎃덟该覕呋竹囎ꕎ㈕倭毒嚁䏠ﴹ鑺퀘᳟䟓渺䪁懩Ċ侂췴펡誂萪ⶅڷ贇枠ﾔ帺阷勯銫릅붸ᝢɳ䎛腓ƃ⇉혵倚鉄龚┨曞肛ᬊ洏횷䀷璟뺽픻躔탡騂諯㎂犍ឩ륃㚃낉皕엽멽呐嶦딏掠伎罹芸˦ᙎ咙ぱ䟹ਈ苕迫ⲷ爭ﭽ뙿롏ﱽ᜽ⵉ晚ޤ偤艹꼈숹⵾൓ྠഡ杁㩰㩓ꀘ얝愋⓮휦炘Თ㕑溞Ʋ埓骣㬋࡚༙ꡯ彵곘ఔ၎誌뤻鰩톂ꦢŦ螠哠喐붗趓鎚贰ᚒ꠿扣桃䔿׋쌢뜢녢떯⾤乞ꓺ廙橬視秳亠㶲金띅绒ᑊѰ푿鹮牱䶠⥌Ɵ뫲塰瞽ප譓곽龄㈚ꍈ洱ꠊ嵂妩擿㕊钺摠霦᥅Ċ揗荁㑕ὒ婧勉뇁⤡퀁὘絥闅譡ቜᒴ耊隘顑솞溣䈧喐鐡具怉ㆍ䄻↉氍᳕巏㿀锵⾺畝ﺓ㌘ﻀ䀧政塱鵭뱲瓬퉩꫄庙གᄃᅃ挂饼ᤵ䜦㾎힂ᵠ໛Ĕ俦孇囬酵赋ᦳ줰⠴ꯐ錂祜歍᭍튓혀뚠⑋乷ᵱ圤屌媇ꇯ頊䳞♝耥朁❨じఘ⬮ʑ᎕ꬦ交ಚῧ鑾氭䯷鿭퀤념蕿䧾긏嬗ᢜ獚ሏ摒休ᑫĞ睒ⶲ䉮䁬䗹ᥒ셐ᅉ㨓驽ﻊ鯞砉铏埱ꟃ㥋钳돮㰪҃ໞ흿運㈩⥴鑸嬴拽㒽髀麶⢄왬䄫孻㫷ഀ蚵揟邏ᣋఅ訤樌遹袎询榙걒啄퀚㧰襄⹟ɮڦ嵍슾ȹݰ晣毁Ŗ貱跭ᤁ臝㚋術紧득㥽੾軌๩빽깜ᤈ竾䑻橣౗帉欞팵趽삸歳灙ㇵ䞜磖長熱츊鹫઱↞懇䪥◺䒗㳆ꀫ㸢뫩⸶똤粦ࢉ進뛘裡힭Ǟ껝걒箁줊뉽恹陊丬患㠇痖⇊탔誉욯⹨밙뵡勘Ƕ㏝쬥씪爍ﱮ뮫䔎ꩢ抾묡薡涅氂䅄栙⫊㵮蜾㠃牙埯ꢤ⁆띏伴館犽혌꒕㰠봽栺횊觽숐䗭뿝縅䄑Ċ鏆䢮᛿凚懅ᔆ㺮捆嶴༂쌎䉕ʹᓛ勅ᵝἙ噰䖛셷ᜅ㺐汸番⩜⚘젎叉䗹腓ꞇ퇝늿軽ꭢ궰➋헢⤳戸햖峛鼫乑ㄜ妐韍纏ﲪ涊世뒼Ọ靭顭ྍ㚨탩ㇵ돇ƃ늭ㅝ稷Ⱝ┹獚経쥩ꀥ헟鈚ᶪ푬ᡦ䡨㊕㼤೺鲞ꢵ뙈Ἃ熿첁⪌ޢ杢魖兂짼迊欔ﻩ蟢洴䞈簔皽槧彲澣擝镧欭뽅鼄䕈뜷᷺洇ﳱ愘䂛볕ሦ㿷蟶鹓䰽Ĕ묳眷Ř䫦䁈㹼Ⓑ牉梩娮ↄ뒊囶⇷꽶뫉韭睥읷썊謣澻쳩ச㤂늢ǀ裯巴돏胫囎돰캎㉥氫籬ꡲ澑唙蓮뼏얳ꮞԾ➍뭞귭襗⨁⽜亼黮睾锗ꘗᗝ憎䚃䙕㤶Ğ硛Ұᴋ됲쥫矵鸬䫂糮骶⻬䡣ᢧ驄ꨝ䴊ࠒ爋ᜎ侢悴辧添घ惙査턪칿圐筢ऴ㤟䔬륿ꛀ婾銷诰렡奥睛폏宣鏰맭ꬠ渽欈ㆴ帞ባϷ寰郷䄯픞牭ᙑ붨أ딩擾夡噻㢷뱜漍緫㴪⌡並䈷덆ꑥ苂獳춆ힽ≞适딉咛挘鶳뵨﬈゜ᕯ㯮넰⼗৅䷱Ꮒ稰裴키賬栬옧ǥ外䓴㸂ꆐ毭ﲺ磐⪶몞ㇾᰊꉘ஍釋礻ର還境ĩ坵ᵬഅ▋绳篓ꃱퟆߟퟳ넹駕챭騛규嫤ㅮ듨뢠剽媡㞃넍뱅䧛㙟洄޸㗴᜵헭귎㸀폂蝜⇧减뭙빻뢐꒽쀦厅뛑쬑鲌푇㖅࢐㈼듨흍릷⵴졺劆Ĩ製ජൽ陾킛娭ﾸꁎ玮뜌癙缃童໘输溵⋴Ⴌ갟ﱖ㯇烘飨枬⌿᚟䀅䒡蚽䂶㲂高₾㹧瀣ࣧ荋䐩둙觅鹛鸭霒㺍땭齍៓孔뎥ˉ긡ꊁ䷌ꨮ擋䥁ǞퟘĽ쵞捶䳁墨Ď矠૏䑣᠕畋鬋槻䳔紮秠鬆閊ᯋ䣿ꝟ散雟鑀㪚琱⚰鍌⢲楋謯屡䩙됟ᢊၠ躴嫛鏆삟★୑罭⤍˙ꗩ巔ץꛘ忶ꪧ騅⺠㆛ᘧ∐凋Ğ睒蘀呗㭚ꑍ飡渋눾둈坛㔨褸ⴙ轖긺赲봇킏⮀⠡極舓鐃⺗⣺蹱欮팁쎀꣘뚟尊䖈⯀౉哬컐贆䎖톏࿽澈财⬾羊㪷钼蜝繼↰꒨ꪨ尘┎較懻㱎Ğ뇚ⷄ栈쯖冬鋰蔈橵♙䟧ƀ煄뵏ﻊ䎑㪞숾鸥垸ㅃ㙭᷽ﲭ늑谂ꖎ鈌㭷ᗬ蕻훢털ꉼ쪕㍂줾왐瑍篇笍厙〧⮀䳋༞픨悙ꎙ㳒큠䴞厤뭒㓤콭붴߂躺爬␨酫ⓟ䒘꧘錋흇垸輻퇙筂鰞ᶿ⎍味豜Ⲫ궤᭓黫ྂၝ༷鮅ⱹ懞奈䏪衶惤⩤␍㣪쀇쩔湃攄崣ꊮ䙊麏䝠携ꦋ蕋䆳鯒憾譆襚잯땧룗䰵Ċ侚騳⯰謝ॼ駕ꮦ폽儽䓢퇟Ἢ疖평늞躶廈㇅㾽㥭⏦൥楝짊峝껅꿙飃ꩣ⪨랆澰⍞骐䮓믔척띎ᙎ稢؇ഃ촥㔄ﳰ䧇格➰ບ撩༱͕Ῥമ劘㷲パ囫᭖▾㱦ෑ튛馎㑢종︰韙퀏㌢븣䇾聆襆觀뚓짖㪳釤ꦱჱ稙䍣黇錓䜡橂럽讁碲ʮ㏷좌ꃟ첓崶燝쳅ꭣ꟦㻧収햁ꧦ됚鄇咷૘ʁ鲹ो鐍周慧灢簍㱣餡쾚⚫갲銖̍媞ⴹ뫴㫤뀡絠්則ᘼ峈䪨쇁ᔨ줮紶띎著餢朚ዽ띚삫샲雥纭䨋涫䄵ᅼ뽌吋㝍袭ꄳᘷ⸟嶿䓏⒢ꕤꇘ쬽栛祯Ĩ齝綱鸕엁昪勅폦娳톣竰誾ꊫ걨왱쁎픾ꇆ톀秹௔䛲Ϗ澧姝᤼僳椐帀ീ蝛൯㝗✨㑼卟涍쑞杴싹㡶톿杈翽ƴп㟆ꖿ튤밻墔䍵♘ꟺ㊘玳觏ﺫ쥂湔亼Ƥꇐᇚ驳씜ﭞ曝ഩ嬳薚ﻓ樇犎앐廽᪛⑌簪띶嶡渄ḯ⬗錹샤풜⿸㚂묒䳄ⲗ䌅㊦黩儅ॼ掻ℝ蜑㽒Б혆ܷ蓩᫕碭⾾≡䶘ᖴ첊ҷ䞌몞妼휆ꕲű㉣뼋汞苛Ĕ俩穨贞옟佩ꌁ쳍鰿㲴럖賶ᖲ쬁壇ꦧ숍鴲┇鎷⌼뉫囆︃졈㋈ⲷ唬ꘇ鋾䫗ꢛ㺇疯⡇梖澷稅욟㉳৵織뛖ホꖾ骎䲺蘭灖斏목⫌ऍ앩昁Ƙ觙畻΋雿᫗ꔃ嶐⒥￧襱摒쭳䝞뼝瘙ᘅᰠ﨧蝜큌蠏緶⤬봭䐡䱃෥삫뎖햄븴㧇迡忪뉺앬⚰煃ഐᷝ휰膦뀭厌珈ߧ켃Ώ᧯ᾐ븻ğ畆蛜ณ༳宫ᆒ鳟粜⟔ご蜵ⴏ̨鰊姷挊荰囆廋ㆁ퇼﹍ᓭ쭷泎潯都⿌忨욎儉쓓᝖泳盘봧瓟帔嘟⹲╷뿤䌶顝餃藉㼸깤⸅⇣䦜Ⰳ땒ӕ惚磖฽ސ៝䆰넣施땉尋欯弛䵅懠垩쨫鴂㱒ᦥ誜䏆緭磫㤆ᩭ䕈羡粿虓麬뗅ꓰ㧺ർ彌৚ꄊ啛γ兞붘碒̗녡칪퀚竡닖゚齛౓ꦉ솊坻᜙遍㼣銂◉ἝĲ올嫔쯫䶏އ醩俄ₔ녰骨䀑鼠퀲뜸⽵㋦쌽泄䁷걀慡鉌列讂쒏ᏹ䄲驼憷철﮵⺢硖ꖁ됊笥廚쪹씡拵諶澈楋庯襾玏硢ꨥ׶쵏딝훨涝Ỳ㮨挼瀌뛓全縘Ĕ兾헆굊⢴籁Ͱ筣瓢笗꼎呣➤橦훣ᇗ噫닣䄤楢뱻ⱘ휕헙魻䯗璿㔆颙ᓢ䱳⯗쉒籥᾽텨끆얄䱪혰㬴ꗨ⋉崒컂໾汨鞟㬤챑䨉读䈻輂ᵝ몏˛뎔膤꿓ᒷ죦㴑⩆墊儙鴐몎滵ꯃ㹈茏찙终褋僵◓隅鼿뢚䰼攒쭙옳騤趤⊯ኲ哣꾐嬺樀ᔖ뎧魯꠶䒽耫簒㿂洁㹴䥢酄﫳啭騽爝隯큮ࢦ⡠Ċ넍ធ樳릕术綢︩ザ뚋ꋥ猺谙말ഐ漫ᇰⱓﯙ芛晰赢Ą熡Ⱳப▝ｱ萨跕༰쭹휍趦ᇩ䲇㬎핗ｯ뿎鵋껢쵹ᵍ뎸襮㬠尨뺅ﵪ톼र액䒇燜ퟕ鈘쑳ăᚽ잽冋爿ᆟ蠔申眆뤏㝏삽◙諺殳傗噊鵃뎆ೋፓ묦뼃虧홠剱ƌ餳깒ࡡ鷀Ǉ↹몗鯴グ૽⌜鍙㶔廾ᤨ਀菗㹤㲿孆龹⋁㢗碭㸻곢䩡賙࡯᝝”귅ῧ㣾竵罫孍땴⭜툾명⨧ﷸ郂ゆ㱔ũ昼㱥䘒甋꘿绻布曮鼀쎤뜍낒ᄿꉪ鼳ł鱟䄢ꀉ兕ꍻቺ仁耱⧤㱾훩㳮悆⍙瓼뺼꓁ᯘ᭪濾頺掜ㅬőĴ㬺쩢䍗ᒄ㬔娙䶗寜퇦רႭ䧓띾䠈≳ⱕᥜ흍骕▥ἵ㜸䕹懠巾ᐔⶤ扊쬏௏ᜑஒ䌰눁⾞䢗彵훝塛װ怂鱍鋊鼦损ᖇ몛찥踁쪆틮䣿솆佁⼩뀍⌦Ġ栜錮쀅挣Ⰾꙛ秭竳蝓돈嬎ศⱈゝ맯뤂㤈‘㫔ᐖ畨㏐䡍拰ퟴ⢒䬗ದጸ浃ྫഁ弧䕡幘ꡐ㡡廴䉊ᷱ亱㻊艊᫶ܶ뼌췵ꂦ橴濭Ⱉ버㉅빗돟࿃嬨Ԝ赀⹫䰶뾘㮞菽伸ⵀᾷ祝㻧ｻ籼鿯쪩瘾ٹ秹嬳ᇼⶎ쎓ꋽꎕ㌩캿㈵㵂Ͻ胅￠嫤♲텶驫는꺗矷뫮橤ᎏ䂢椛䯾傿偽Ỿ嫄釟ᥲ甩謥땀₶痍葂♾쵈蛌편̀藣拫蹀톛召ⅶ㶏䜎䩥ೲߝ妚㓵渠Ḋ릓꜏౐뛲悴㽤ꗤꦝ퇨멐ꙇ홊揵ܴ跬㪝舓䃪⥰鸹廪细儀﹦朩䋙︊숞溤꽥뾂仉㫖㽮ᒀ룏뒉毀額ㅩ☓햋ฤĪ咮댆䊗⩁ꄶ쵪䋫㴺閜◆왫滳罣㼴ɤ竘髐檜갖⊃꒱ꆤႠ㪻皘슀팰難ầ㕌睉쏧㱍ჷ姶୯姀窷ຽ耭ᘽ⌠鑦쀢霪倁ၙ췞츃㞑㶠醯Ĩ䮼㵓⿍ޤ᳴ሗ槅︸攸ꏽ⫧庌᩿⧲怹竿䤂㓨䙲꛿抢郶봣执걇왴Ꝛ궸似끘釡膅䁨ሶ⫺韑ᖹ淼즪㯻兛隤灵츂퀝ᜂ턣相炁팤뚴扦囫Ĩꢊ喞饈瑸䗛뜂೩苮斣㪇ᐜ縒냣쿳啲撶㕭郾활ᘔ출懚戗볻駂걶輕٘ۛ旟ශ꫼癔ఀ彜겂雿㩲납臂ꇥ칤ᜥ㐹睕剭︊慈ކ듒蒦Ү쒝쒀⨕ꗉⓂ談኱꛿䵴Ė㚜ʹ紁䐶槞㕁쫄諿㉣孿ꆦ궗夙肬斔ﬄ辀ᢇ瑫Ⓢ徳稗辭嚃朠馽⮧匭ꬎힾ㑩⥈퓘瘺ፊ녧䪍쯂쪡쩸萼Ჯ抇㟻ᇭ᭢ྭ笕靹엃서⮮㓔텚袁⓲ᒒ졗Ĩ黁텚콰旓䙏姵੒䵖⢒桳톼릮蚂鑖䑥薴ၾ씖묚䔋ᱹງ竽럤ꀲ㕀‾놻⽨ݡ岗铏ܻ耽맍ⷬ쀐叹䶻侚腺㉍섛葠ࡽ蔘㗑㾵侅紹끺〺೑哔얬೜ᩀߵⰽ뢶㱺㎐掗滣ꥺϳ誐鎸ﯔ垅ᾳ呑ڹ㙑⎎要易鈜赵轍䦠瀫춓魵袖﹪쑟篎ꑪ㛦鄲䗩쒞䵧孴䖰ꀣ蒐蘺饒閤᭫ᚦᢢ⫝̸蟕昶睫耧胇畠Ĳ욄⬓칱郸䵱᷁ꔷ䁢⢐๔ﲶ슝秓嫌⼆彁끾⊌轂눂下不ͺ뒆蟠풂牝릹隡뚀芖混鍠酃䎐℟㛅✡ﲡᾘ⪍뇮物瑴源ਹ彠Ꞽу㰗❘뱶鑫槣Ƈ몢ꖩﲤ쎡㦩៩᭤䋖䃝荾罆죔镏ⴡ梜铓␔萵ꦊᦔ͎豊㇙銴㿿脮髞⑦銣羸部匽橥瑴垉䓢齦撎랚辶ￂօꏡṊ㥪ⶻ⟷扈ᥭ뾾얐澢〔㍅ၶꭴ翊ť俾皇硷ꖄ갽⻅튶ꁢ䁶襇涳獭灵턙榑璼兀䡃宀福ﴺි⡘➑ᐆ鉞숦妌쾡뢝舘☚ￎ콋菪䯤嫹⼮刋甆譶鄵ތ㍘⦞怿寕쬋迍筬쉽虯ꉭꬷꃲꝃ去ꯏĲ옭⧧녳禪ꉲ斉нݪꎍ膘睞㥉勴귥ŧ쟃⻱琙䶴ⵣ䀲庈貾Ꭼ닾眐蛯㣿붺镈昚꺰攀剢頻ሴ킛벼崝郋憜쀭ྟ侪㚌퉪탆ꪍ蟈猲塞诘戥韔Ĕ倕䬎ⳉ㑙ꎊ糃퀠콠逍ᰕ륝햫䆩㔚茩徇೘懈Ꮠ߾퓛떠ׯ읹膘谲ɳ㲼⢰屢в榵Ｃཏၐ籴爠务뽈掏㏳蜶妸杴뀻標霌੉࿁좬銕ҙ價傯濨䴺鮗畏䭋蠅Ἣ릷썾㞭麆켔ᮖ㖉ͳ᏿㹩葐枃좢迉䭃옙赥य귟鎮㘛ꞑꠌ怢ؤ⯔셃揽ḛ鸃ﶀᕥᛎץッቃ擆ള㶬攗륃峕㈕➥垧왚硐Ċ⯹蜭伢껷郣䒘殕욻ї⋻꾴٩챊탪茟╞⤽腍㻤궸픢쒚妕囇늱廪睡䖑벧ڮ耉❶븭጗狧믟淳ᏹ๳돸⳿ꋑ홫厾慝歇柔噻閆燭씑⤘媧͟ﭢ羀ɳ袄♑㬒짦꣇旇䥤袷琍첧験⭝ԇ瘛콝禢焧踏펖ｖה㑗㵑惼ዹΧ먹顱䏬⍶ᣵ机뒺蘝줙塱ᒘ祩쿀穟➻ꕕ鞜䠧ᑅ뷪뻢ㅈ葑䭱띹鵞ꨞ筚㶤Ĳ쥌䘛䝸䦠喝Ꞻ縓徕䫀긎㪅⻝䔳亱▇ꃣ嘊⿺肉祘垷ↀ嬀귲髎͓牏閾탯ゼ೒拉灪✶绐磨姇ꭘ쌺껥耾ꚲ퐀ɀ練骘優弞쾃副셡訧❸묣ߤ塖倝︉꽌쯭맧㯕埍㽴﮻욅ꓩ䟴⹞俞긮咉腀쓭帤뇈薈趍봢쓪䯯랤뉨할텔壳ꩆ褴얷຀렺㤳⚣떫칝靕㴦騪嵉ㄬ䨋秡꾦䯣咷溾ḹ䕩彿컆쾡謭⃦Ğ瞂ﻄᬼ쉲阺᝱ሒ玊뀙틶똹㋬軥ɀꪅὑ嘿巃の皰윥㕸⧊廠呎ޚ㣨ﾧ㼧횰ᴍ璒᪇싒묣㛹玓쀝朐렣䬜灯䕄虍俢⒇狞䔗ퟄ馄輁䶍⛧Ĩ鼛틜㭳㋹ⴓ횺椮Ⰹ吰ꈇ뱼拓ថ≀㫰齁ﾈៀᥪℍ괍躜䈬㊻놳蜾⧆洸銬処㫛鉿襉㨗탟仆웄꣱⠰䛆ﵱ櫰녟돧슗ᑃ盦ၑℭ餣︕ⶺኩ⠶秲䌯抛Ĳ뱡䞂꥛·ﵨㄯ껲ㆬ覵刱轇㞋叡ꦮ薯㳦ᴂ伂칹㽛ඃᠫꅝᜊ璤餏ﰴᘗ඾뼊꓇㻏冫䷣穃鈏醅உ옽츝ꖹ궼饓酳㻔ᨭ笂ঢ଎짻ᣏᡸﭕﬗ껄ꭲ႗Ğ睒鹙㞊ꛎ쁒ⴣ狖簄飛✀茜籋篇оⷈ಍ꥢ͜剎ᯑ䯨ￕ貃捒雴웯㾘ⴶ児팦쵳놰ꑬ鲻岩ꎮ䬔쯜衏긠迿굉䠮郲顇㖬釆㛇ᩁ䠳먍𢡄ⱘ蝧䒏ȑ旂칞㡓䏤쪟耤葸㾗ﻐ褐ظᗂ挱ቒ愁㡾ﲽ憎늂徯䖛Ὥ㓽탏伄ꭗ涡ⅉᾴ藚᜙羙਩・紻土ḿ䓷菻䔻뀴嫍㠞܀ꯛ㟨蘐炉宨ᴃⴙ柗䈿Ĩꄈ鳝筆ᨇ痀㯳솧삯붜ᥠ牀硑揬燭莄↨뢰雍兦콊镧哬ཀꡗ쇆券먢뼒伭麵燧〙矩匯鼪䶅፮⯮䒥뢘࿰껜僴㺘腧ゃ샜첎䓻贏嗟柩遜ﯮ⾎봾菔ĳ鱄髣凃ᄧ䙫㫓ꆪᙉ慭憕ꒊ貮鿨穀叜隫猺ᐪ悂㙸᫵尝햊㒏⛫뾱涫颉林吃⺃ⰱ㺇魓뫪料밂史斍뙴ɗ爾၆Ἳ鏫炖᝷槽㾷晄ෙৱ崙ᙾ죜얪Ĕ冥넰␭즕씺搱ў㍱赬䲣峐咶༿釃㣮璯硽䔦冫ꊐ⏼繵ܒ㹨뜧披潷퇀뗊胒섞秋餡㧜씮๚蜫피ⱷ唗㼽ꌝ癋푘尿쵧␋唚녭〚ㅜⲻᔯ좢蓙俰稦Ĩ麿䣟ꍟꞎꂚꖂ懲ꂯ㒓씘뎂ʬ᥸뮥䄝벚ޣݖ褏͜蔥㗒偻뜋熴ꄬ抦㳔ᗨӽꬥ猳杕홴曁烿蠹ल驝ᆭ鷚慊卆Խ扣漰㯃ꔦꤸ戜廑褵惵ꯒ⢛꧲䳞벝튓Ĕ俥⛔ᇞ褛脈服啀菇떢憍瑍㙷褆왹軬쭢殄尹ꌴ⟧쿿ꄉ塜̚蒒㬗⪚猌ᢔ몗̚ꀬ⩚츜ﰧ콱틛땭ᔑ侶䏖悪犋迓僘⧿䊣慱鹟콖랞ਏۆ䲼儦逤﩮홪荁Ĩ麿鶁耷䭲阕邳튑䦫㽆痠䨵蜴⫪ᢍ춿悎丕뤦榏㑼抩ຍ骦䵤ⴠ새啉䭍諝ﴪ폧蝒٧ෘ鱰꿊᩸㓡弜㬿驤혞䚯폟的艩넟藵햚﯂甥ꀦꟉ醼뙯䟬뾆彀ݓ䝎괟唁췻䎦Ꝝ柑흎凮峁ጇ냶냨ᅲਐ⚳덣ﱐ１ᬓ饒뺘ഽ䄑貺⬐Ṉ瑈趈퇁ㅻꎜఎ뤇劳Ϋ兟岓굌뽛є泥䝥鶣季쬠ꠣо䦌䝖統ᑰވ殘ꁆٮ搁ĬꟄ屜므揋䥵똬甊飼폇볍徙థﺼ寚琴谗⠔쟹俢䰁涎あ倳̷ļ꠲阊鵳宓ꀫ㫪˳⦪Პ䅠ᵺ䤺竪䀙쾏ꊛ鞒뒘᫟Ģ慙瘨瞉珮顁뼴ヲ哬Ĳ올寥脽拉쫻魯㾭膕▍銰䆊䤕쌶﫪褵ꘟ甧鑩巟杯癙垶㧹↾ۏ濪惮㳍쥛ꕆꡅ픝ᔉᬤ忽៘⌛쵙풤ᴐ匥⭩ꟹ᤭쪫רꊸ띡慙蟶⒭羒猄௪䇥鞩꒜Ĳ옳稻ਉ＾ꄇᓴᷔ큕伭䰇殉耮㩟庳༟⡍ꛇ᭵渾錓ླྀ唃駴ﺓꊕ鸤갍Ȧ嗝읎僳䧦쭁맱曏ᏻꨙꍞ瀱覆䟁鯦䔕Ẋ쬰ヲ긮里⴫媽謖了把尒㿩壱ꔶġ閬ᜂꊄ展䄶偯蚰ↅ뇻Ⱏ贍彸萓笙ȳ毚伒诖녖悸烼⺴ﯭ쀶ໟ᩶윓옼팕臧略宆뿢뀨좢ὔ駗໘㙚䠭챑磊왌砜ჟ轕闙乃ꚅᨔﲦꙜ茵᮶濠錡쮉蠡ĉ岪ᩇ柚✹⼊᮷姬︡诚㷎菓뤦霙崙駣腥Ὣ˱媩梎↿ᬜɼ硱餱嵑ꖼᤩጟ飫툰魕曋ꓒ笟画떊긋퉠哥웋ꛅŕ⻜톟墲ꁱ쌩뉱띹ꦦ穕苂걋⿻熥ﰥ־霿꽑겷ᗲ圀遷ꎈ뢞詵᧗䫒뿑릘ꖷꂶ鮂㪌黺툭ꆵ샂僲࢏벗쩊薏䁭쑄㲨ග〯쳏ꂢ鱡軖왘ⷨᤶ퓽㮖ཚ㠡煪ḯʗꯞ陋껆ﳷ芄胄ꝝ핕幂ꄥ밽惧桮抵栋Ĵ쨌爭휏㋒鎭ဳ置䟅乼톡₄ᨽ㟍ꇚ꘳涸恸坞㎵踹䏙ة矅벊ꄄ湬ᣎୃ幌᳆爌韈ꂳ፡ﰆ࿹뱘⏫ኍᩔ⣘韬ꬍꁵ씒蝔눦㾯恜㎟踧聸㖯愄轭➮ӂ㉩웂섵Ｐ饂ꊕ쁰〳吭훠ꐪ欴్데꾎䯋鞯麄쬵Ꮟ쐄剺⤮楟鰂묊拲袆䕌捺㰃鼑⪖뵾᭬෨섷Э篷㘻》ំ嬺荂㞳ᢢ럽艮顑ﮗ傚௢뵥䥽䥄˭㞲⦮鿬Ęﯷໜ஌颴巵曝渙᳖쩼ϒ餰㰢쮒庭뗭訕锫䷇ꁲ皉බ⨟ꭕⲌ외츫劌舆֞辕涃謸歸⬍᝾꜌핯鼔ޤມꏾ䔚砥唞ꡀ앵覇숹彴톒ﶸȽ떧风섔ᔢŨᙿ䳣鲙ꖔ喬ⵟꭘ䰙뇨傦唜ᒸ屰䷂젘籁ꩉ璥뾜供픧磢蟀閝址䇴黝໧䛪滟型휁鍜鹪䐹줥亾ꀃ鹁ꉧ甠㍣鄎츤퍜䭁๊곀麯顰덬㼵巴ඣ姢︈Ɨ䅐쁦因쾁鬳ᒥ烱꾂湒㐶獲綁′↓Ꮪ㼬Ⅿᛱ✬კ邙娳慳ߕ漋뤎꿢첕ᰲ煖㐭쀆都鲖堨쀰䏅丆㽁硱㛆騔됆鷴㒲㔱づἊരꑷ湠꺇࣍⍢埋苗㺅笮仡Ċ㉆㍅ᖳ샴촆ત悒꾘ࡓ烷灋楟Ⱍ㯜鉚䢰㕝ꭽ曵栘㊬鋿叞뜦瓰ﳺ옎㲆ﱫ䙆댽騮征觜磣犺롛鷟婓띲ฐ并挩⬛꫏烵⭬闡坞뙹Ѐ毡ꇋ￫䲨ﯪᶙ늉뫇ĩ虈貎솭䪥㭜봐Ⲳ輁ꯈ闉跴鹊꿺틅莧龛扯魊ǭ薌湮㔢菖릪㒚矉㩦㢼晑퍋﯎ﰀᅠﶭ냊ꃕᵗ徾썮뱜鲕婿䗋睭鏀娭쎀ꢘ䓿➼㽶䖾镝漣糄ਜ਼嗍Ĳ오봫㘎힃ᇧ㌀從퟊孪쾲ᗴ᯻⨌膼潕蹦䖅Ὦ둸⅊춳ҽ儘饁曖ዲ䁀塴溺䪅祖΍똆ﭓຑ곛慵祇ឥ㿋㴗湅ꎼ陖瀐᧾쁜⃒坟Ꜳ쵁垇肱榈䷪满ȹ쯓聺몀縐槑陲앩渆뮮쌒ꛨŠ齃좷봼鮧떳㌲觲矏͂ᆣ팤㖛쿖㑪雤ガ힒ㅁ炓ߴ烅⢊뾠驤擮鶓汢夷洽첺䍀俥떔胺ꁊࣉź᤬ᄈ뉘鶚ড়밎받ጅ媞퀲㑕哯텯條蹔ױ飚┮閰焖҂螗፵䘡컍姆ၗꔀ⠲積䭥᩿⿍㰄愄㧋ྛẨڹ谁坪쨵嫘⫪탐郰퓯⩛⁊嚰ᕼⶃ쪺䧏湃纾鐴率ĥ聜紳窌쳶瓲듙㠮죱丢୉쵩䏕ე᭲泆헵빀⃿熽碙Ɥ㦶ც률ｐ멐黏䙣꜊編顒ว◪鋌シꞐ䣄謁驸놰핕ोᢓ⼬ࡊ处㜠ᮞ蝽ｦӉ䌉债섖婯Ĩꆮ憵⻘㯱躖ｹﾱ砅ꎿ鎜唘馔쾰ሪൢ⒦쎒晣쌭ෙ꩏㏓墤䚬驣Ը䮣䛼蚕억捅᪗ꫲ胑묅㮪ↈ遉烆檥ꁔ毗ﻜ䆆梙﹪夤哻㘥敧㊢ỻ诀ĕᰨ䙍⨔퇼瓐硈脞챕깥ᳮ㦞싕䤗戙웝崣䥼瞹痄⢜႖憋᫺㒆ꅘ橊ﻑਖ夲໡맠ꛠ흱噶﯃ੰ揚ᰉ广鰹⪋橅浖߹겍猖띮悏ᔉꂏ睫幸ਫ਼웢ꆀᬛ徦罱឴Ĳ쥌䕕ﻸ锲ቌ煏糾擠♀蒳菉㜯୞䦜蝁힬ᇩ쯨浤Δ⼁吿笥쾦沙訟䵵牣뎒ᅪƊꋇ脺헦걭좍后였쐚솆渀諄収粠ⶕ䕡橤ﯛ瘽촖꼭䑿꿹娝屇ᣮ眳ꤍǞ뗵㇎ዛŝ宎雠匶⥏ᓹ⠡⚋屮洏蕀ॿ底⊒ㅦ᫸鱨ᦊ뱀埈뼍菻오傮㈋㶻月ﯵ计塹ꁫ餈鈊暉혚ﲠ祝묄槉ꓹ岣䝬᷉ᐾ餪淮鑿㼝꿄Ⳡ凱♫ė㓛풓쌜홙⡎ӽ쇱ǁ堮串泼ᐻ옚졣ᛧ볳䣿佧橯韶鍡ᑐⲈ۲⚬쏯㐰烫䢃쑑㉭묗䚗摄痛석雰ᚰ奿뫟⾱탯鐪㔤莰歂㽯빑Ꝉ꘬꼬㿾呾Ğ禯ँ⍡矐쮉ꌑ䓹顰돌ኂ컢䩻窃饦繣؄麦㜌냜ౡ贺阕ǂ㾩۳枎ꝸ溣ﲰ넶煸䡎䊚Ὥ굮칭쿮횸᠐їⳳᔲ̒뷎뗻ꩮ淋鰽⺬糍ϖ嬉ꗄ굒ㅭ鍘ķ꒢㜫쿻땉揜정惺ၱ씰콱ꔄגּ癆렞戍ￃ∢渹썫嶲䨡넥꥟碔䈄셖ꌨ㉾⨻䢕㷈榈⹂鼰쭣譮횮븣წ副칬⋜셡咅Ḕȑꍩ韽彸嚌룷쟀Ჸ娆쓯濧㎇㦉꛹Ċ㉃뷄㽴䡽둿ꖒ벌ꄖ㯡ꐝ槟ꄐ皮謚⭽遮砾ᄸ᳋힊盂龃擯ॅ邤ቔיּ긦彐볔㘱⥜슩㸭쓢⤎㛈嫗⩣੹랿겙뭶䬊浢ῥ廝쐜쩲阌ﾸ䄓㠘磱䷒ꀭȹ킶鑁쫎ꃟ⭄②寪籓싔斣﨩텗猄肟珪뱝歴﹯俒堲枺眦桀鷏窐㦯࢚ꏈ䮡鍅잁䩥守熧⊂铈㙍䠛츚㭹霨娣搷᪄辚锞〧夻㳡䠡乞ጤඡ攘湟賐㋅泐ť俼⚴쵞措衉鈦䁴作评鱻爊쑦喕튅㦍퓳妺䰥ಢ꛹揔૟ꤍ䯃☖꽴㝳콀ⲧ鸒者⯣篓⏥囗㈡钇兞壖ꙿ呼卛㍬怑䄬ꯥ퀭⛧葸಻囡웋굍☔좴﫜㸛䙌⣊瓂Ѧ爨剥香龅퟉五뺅멮ﳮ⸾쓜뚻谶헛칚菇墲糣饪鬸디银◅铨䕩䧽ࣄꀬ먙׎酣ᙳ匉䌾滔멍圠듬⒑쵈솄摯꼟鯖ꞥ∗欍៉ꔤꨋ孪脐ˑ败↯媿币킎蓺壺㮰欦ꯨ簱Ĭ왍ࢠᰢ뜯뚤젝⭷㐬툾쮃閣鉵恬核꛴ヱ軟箪訔勵畊㵏፹戔䳀Ⓢ᭒톁덮㩴傌投⾺㓟궆怇뿺㑧㡎穓❴뵶č͟仼횪ދᘛᑢ녘ਆ濐ꨧ㥽綪闹๧뉧Ꮛ把䵗䁊ꒀ濙⒋崬뽫ﲊ틌拲弯᏷㽟갊廘韠瀙ꫲ䀦ෝ䁥㿆㌐쪤煶ݎ㯐辫ʦ⢔嵯蕉쁘ﵱ컬㊷恝⒧〰릑ă㌿妘ᢉ볖볐韃悻ᬸᖘ淀ඔ顥欹倻ଅல馇⺆ԓ烖Ｐ㺼ᡴꏜ抶⓷鏿퉇뵫᫇麚꾝媁붚鬾넝㔨囂瀂ﶭ蠙㦤౮ﯦ坻ꈫ퉅衆煺찘齎䯗푅⣯盰靆쯿фϰ㏐博ꖇꞀ驒뜷Ჭş岴医㶝疎漧硝㜬璼蛅訂ꡋ⩣ᓬᘮ狽鲓ퟣআﱆሼ㈤ᵯ뤵惕㌍飱熄䪋礼紅㞉宱잉㫛쿸꺮钷脬ఢ䞱떴훒遀䳁贐蟋劭䠾剝늨ˮฑ鸴ċ㤙ℵ੐첋ꧤ㿁ᆷ蕵詧ǥ粃쳿툁销峵檤粵泠뉽孑鵆컛ⶲ䨇御锈༚縖帤밲뾼❋啿ㇺ邕益劇찷ퟬ냗␓㲇턫梵疢呖첹葖‱㩱᰿ౝম㫌㲹⅀옕龀㰪ૻ灹Ĩꍓੵ嗫⁼ￌ串࠵싉⨸幍囩܇ꦁ꘧똩얶ᡌȺẊ⻿ⶠ㰟舄ᛊᘥ鐗诼샤媸꠷⇃綥曅̚ꖔ厖鸥荣틩君ꆥ䷃朷빱᪎ᇍ㎡䄏헫ዜ闏˾욷Ƀ굧짡匯刺묠첔馇푫Ⓐꛧ뗦꾝꬏鄱礯፶밪粉ꞈ엷凌禼䋓닮携ᒓ㭀᫺韤僝衕㘊㖼体撓ཟⵠᾛ䑐걙땴ᠬ툡ﳟ㛿䑲眠㏔렊퇊܄嫰㗮቉წ梁੼콻疭ꥮ膂浔⛂퀢坳䆣穜竒፣鲣࿚䢱짏ጋ၊∧퀕벃᱐嘯鈦괔点㓾왛䯽煆걙ᇬڄ켣ﻑ鬇鈧̓菭꪿䝩巂ᦖ肶睹軐썾몼Ⲻ뭈㮨눗✬ꯣ쳰跸ď煎ӫ◁ಟ遒┤僨⽨΅森玼ᣎ庎㞪胟⻰ᘂ뇄뛜匐漻蝹划ꦤⓘ䭹ꩂį늩ଡ଼ⲑᓥ㔿맒㠱コ钬ꚡ伐覙귱ᚢ쟖Յ⯠匫恤쩸懅຿⯃聻╭禁ᴨΌ릛蚱䈯蕞؁䚃䥓ꂻ͗ꠁ㏙垐锚쾎習噷䦤緱斥ꇚ⡯Ꝁ낯꺤᧎ꎭꂘ၍洭눢셟쵩봤햰ꦦ챔揈풞蕞쏢망Ͽᶃ㾷ꌄ쐬আ㈍娗碹䢛Ἠ羝䕞앮훤비翇Ɨ害뱠눶酉耔溃氇⠧꫙蓮蕓푢鵐ⱞ澈ǿ楴⭥䖅쇳糹䉄䌵ⶨ玟燮줻⌿촟㵴谝ꃑ뙩閕듞揎ᱮ䵕捽蔯ᴳ暏몘ꛃ忕ꌖ砥돬ԣ荞工聿鎏ꍂꤚ쑭Ğ睊芫㟜ꏽ䑚ߩ걮㦞䯒δസ婎恲ྼლ피廳힔岼㇥ﴹ쨧占옧꺨뷗ퟐ橨뜶쯘銘頿唴笴⛵⁺묉ኹ䒇嗣㠨⹯ׯ뻘参ਓԗ킿ᯏຶ憖䩃拁惦湳ᣠ▤멒Ĕ俛執럽覲ጿ측踪ᰦ迤岏ྺ饇疧縦ꇿ⸱ૌ姂그ᰇ젰ꅱ鬾̊恫鴸羕뇹㜯顨틝쇭⺳낷쨰瑯萻ᾀ茘挑늶기킖ܺ빁沤虐듇๪춱⮔蓑㽔̬ểങರĩ裗놳䝋◠Ꞇ䘂ルஉⳫ쉂釙䷞ᾨƂⴄ޴箑锉쵨笗ꧾ궯㲵봚⦇疑∮ἢ䑲ナ㬶ꍩඪ抖㥱퓵⫊輼㓧䑣뺦ឋ휉叨ோ嚮嫯뺖቏㱞㭣膦ᅦ⃮攡ė摽읞섬ꛯ瓁氵霜䬯በ핆葵ᔅㅪ腻쟰䍒粷쁘❻䃝Ǆ藱ֻ巵㪃櫿㠳큮觶팀䦑ᢑ쁍遃驪콅ꛯ᜹붒౓⌁꽠鈴쿒읡ゟ℮霫斶㴛ꪼ鉧赽櫷탈竿͠"
;const 伾:I=99;const 佲:C=5;const 亦:C=8;const 侬:C=13;const 僡:[C;佲]=[2,3,4,8,13];const 儫:[M;侬]=[3,1,3,2,1,2,2,1,3,2,1,2,2];#[derive(Clone,Copy)]enum Type{僻,Guard,儅,傾,傿,}#[derive(Clone,Copy)]enum
亥{儓,儔,儕,儑,儒,}impl 亥{fn of(世:M)->亥{match 世{0..=6=>亥::儓,7..=8=>亥::儔,9..=12=>亥::儕,13..=20=>亥::儑,_=>亥::儒,}}}#[derive(Clone,Copy)]enum 伔{僞,儣,儤,儆,}#[derive(Clone,Copy,PartialEq)]enum
乚{了,倕,倖,倫,倬,倈,倉,倭,}impl 乚{const 侤:[乚;亦]=[乚::了,乚::倕,乚::倖,乚::倫,乚::倬,乚::倈,乚::倉,乚::倭,];}struct 伿{俾:Type,丆:I,为:亥,下:伔,仐:I,偘:I,偿:I,傩:I,d:I,俔:I,偾:O,侅:I,倻:I,}impl
伿{fn 俙(&self)->[M;侬]{let 一=self;let u=M::from;[一.俾 as M,u(一.丆.clamp(1,2)-1),一.为 as M,一.下 as M,u(一.仐.min(1)),u(一.偘.min(2)),u(一.偿.min(2)),u(一.傩.min(1)),u(一.d.min(4)),u(一.俔.min(3)),M::from(一.偾),u(一.侅.min(2)),u(一.倻.min(2)),]}fn
code(&self,亨:C)->M{元(&self.俙(),亨)}}fn 元(俙:&[M;侬],亨:C)->M{俙.iter().zip(儫).take(僡[亨.min(佲-1)]).fold(0,|仺,(&v,亄)|仺<<亄|v)}struct 但{丂:C,d:[I;Q],乭:G,guard:G,为:亥,kill:O,下:F,}impl
但{fn new(l:&J)->但{let 丂=l.丠();let 丮=1-丂;let 临=l.map().valid;let mut d=[伾;Q];let mut 义=l.丐(丮);let mut 亡=义;for 丁 in 亡.iter(){d[丁]=0;}let mut 两=0u8;while!亡.is_empty()&&两<伾-1{两+=1;亡=(亡.丏()&临).丗(义);for
丁 in 亡.iter(){d[丁]=两;}义|=亡;}let 乭=乭(l,丂);let mut 代=G::U;for 丁 in 乭.iter(){if d[丁]<=3{代.set(丁);}}代|=代.丏();let 兲=伟(l,丂,true,T[丮])<=l.下(丂)||{let 丛=伟(l,丮,true,T[丂]);丛!=仁&&丛<=l.下(丮)+l.丢(丮)};但{丂,d,乭,guard:代,为:亥::of(l.乬()),kill:兲,下:l.下(丂),}}fn
东(&self,丁:C)->I{self.d.get(丁).copied().unwrap_or(伾)}fn 入(&self)->伔{if self.kill{伔::僞}else if self.下<40{伔::儣}else if self.下<80{伔::儤}else{伔::儆}}fn key(&self,l:&J,丁:C)->P<伿>{let
一=self;let 丆=l.亯(丁)?;if!l.丈(一.丂).test(丁){return None;}let 丮=1-一.丂;let 七=G::丁(丁);let 么=七.丏();let 侍=l.丈(0)|l.丈(1);let 丫=(么&l.map().valid).丗(l.丐(0)).丗(l.丐(1)).丗(侍);let
亼=(七|么)|(七|么).丏();let 丙=|乖:G|I::try_from(乖.count()).unwrap_or(I::MAX);let d=一.东(丁);let 侅=丙(丫);let 俾=if d<=1{Type::僻}else if 一.guard.test(丁){Type::Guard}else
if d<=3{Type::儅}else if 侅>0{Type::傾}else{Type::傿};let 俔=一.乭.iter().map(|j|l.map().东(丁,j)).min().map_or(伾,|m|m.min(伾));let 儾=(么&l.丐(丮))&l.乐(一.丂,丆.clamp(1,3));Some(伿{俾,丆,为:一.为,下:一.入(),仐:I::from(!儾.is_empty()),偘:丙(l.丈(丮)&亼),偿:丙(l.丈(一.丂)&亼).saturating_sub(1),傩:丙(l.乣()&l.丐(丮)&亼),d,俔,偾:一.乭.test(丁),侅,倻:丙(么
&l.丈(一.丂)),})}fn kind(&self,l:&J,上:C,丟:C)->乚{let 一=self;if 上==丟{return 乚::倭;}if l.丐(1-一.丂).test(丟){return 乚::了;}let 业=l.丐(一.丂).test(丟);let(兌,仼)=(一.东(上),一.东(丟));match(仼.cmp(&兌),业){(std::cmp::Ordering::Less,false)=>乚::倕,(std::cmp::Ordering::Less,true)=>乚::倖,(std::cmp::Ordering::Equal,false)=>乚::倫,(std
::cmp::Ordering::Equal,true)=>乚::倬,(std::cmp::Ordering::Greater,false)=>乚::倈,(std::cmp::Ordering::Greater,true)=>乚::倉,}}}fn 乭(l:&J,丂:C)->G{let 丄=l.丄(丂);let
以=G::丁(T[丂]);let mut 万=G::U;for v in 丄.iter(){if v==T[丂]{continue;}let 乱=丄.丗(G::丁(v));let mut 乡=以&乱;loop{let 丅=(乡|乡.丏())&乱;if 丅==乡{break;}乡=丅;}if 乡.count()+1<丄.count(){万.set(v);}}万}#[derive(Default)]struct
什{偯:[K<(M,[I;亦])>;佲],}impl 什{fn 兓()->&'static 什{static 儖:OnceLock<什> =OnceLock::new();儖.get_or_init(||什::decode(儗).unwrap_or_default())}fn 伱(&self,书:&伿)->[I;亦]{for
亨 in(0..佲).rev(){let 仺=书.code(亨);let 丆=&self.偯[亨];if let Ok(丳)=丆.binary_search_by_key(&仺,|&(c,_)|c){return 丆[丳].1;}}[0;亦]}fn from_bytes(乥:&[I])->Result<什,乙>{let
mut 丳=0;let mut 傣=什::default();for(亨,丆)in 傣.偯.iter_mut().enumerate(){let 丙=傯(乥,&mut 丳)?;let mut 仺=0u32;for i in 0..丙{let 仢=傯(乥,&mut 丳)?;if i>0&&仢==0{return
Err(format!("level {}: keys are not in increasing order",亨));}仺=仺.checked_add(仢).ok_or_else(||format!("level {}: a key's code overflows",亨))?;let 乗=丳+亦-1;let
伦:&[I]=乥.get(丳..乗).ok_or("the table ends inside an entry")?;let 伵:M=伦.iter().map(|&q|M::from(q)).sum();let 亇=255u32.checked_sub(伵).ok_or_else(||format!("level {}: probabilities over 255",亨))?;let
mut p=[0u8;亦];p[..亦-1].copy_from_slice(伦);p[亦-1]=亇 as I;丳=乗;丆.push((仺,p));}}if 丳!=乥.len(){return Err(format!("{} bytes after the table",乥.len()-丳));}Ok(傣)}fn
decode(侑:&str)->Result<什,乙>{if 侑.is_empty(){return Ok(什::default());}let 乥=凃(侑).ok_or("the table is not packed text")?;什::from_bytes(&乥)}}fn 傯(乥:&[I],丳:&mut
usize)->Result<M,乙>{let mut 丩=0u32;for 傒 in(0..35).step_by(7){let 偅=*乥.get(*丳).ok_or("the table ends inside a number")?;*丳+=1;let 伋=M::from(偅&0x7f);if
傒==28&&伋>0xf{return Err("a number of the table is too large".to_string());}丩|=伋<<傒;if 偅&0x80==0{return Ok(丩);}}Err("a number of the table is too long".to_string())}fn
伱(书:&伿)->[I;亦]{什::兓().伱(书)}fn 兹(p:&[I;亦])->K<乚>{if p.iter().all(|&q|q==0){return K::new();}let mut 丸=乚::侤;丸.sort_by(|a,b|{p[*b as C].cmp(&p[*a as C]).then((*a
as C).cmp(&(*b as C)))});丸.to_vec()}const 傼:亅=45.0;const 僈:亅=800.0;const 倯:亅=1.0;const 儍:亅=0.6;fn 严(乄:乩,世:M)->乫{if let Some(亙)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
乫{乧:None,乌:None,乞:None,亙,};}let 俺=if 世==1{倯.min(10.0)}else{倯};let 伳=(俼()*俺).min(100.0);let 冂=if 世==1{僈}else{傼};let 九=Duration::from_secs_f64(冂/1000.0*伳);let
乞=倎.map(|_|{let 伳=俼().min(100.0);乄+Duration::from_secs_f64(僂/1000.0*伳)});乫{乧:Some(乄+九.mul_f64(儍)),乌:Some(乄+九),乞,亙:Z::MAX,}}const 僷:C=9;const 僳:仛=仛{且:3,丢:1,下:1,丶:2,乭:2,佤:2,丰:6,伒:1,丵:0,伝:false,丸:僸,};const
僼:M=0;const 僸:亞=亞::侭{仐:8};const 倥:I=I::MAX;const 倣:F=2;const 値:F=18;const 倢:F=0;const 僶:C=32;const 傷:O=false;const 傸:Settings1=Settings1{俳:3,俲:1,俌:4,仦:2,俖:2,侂:2,俹:true,reserve:true,俗:true,乽:仞::井,fill:true,乯:个{侘:倥,传:倣,侊:値,佸:倢,..个::价},};fn
乍()->亐{if 傷{return 亐::new(亳::佪(傸),K::new(),个::价);}冋(僶)}fn 冋(亄:C)->亐{let 亻=亳::佄{丱:僷,亄,丽:僳,侖:僼,};let 丹=儉.iter().map(|&乯|个{侘:倥,传:倣,侊:値,佸:倢,..乯}).collect();亐::new(亻,丹,个::价)}fn
main(){let 冬=io::stdin();let 冭=io::stdout();if let Err(仾)=冒(&严,乍(),冬.lock(),冭.lock()){eprintln!("plan: {}",仾);std::process::exit(1);}}const 儚:&str="CG_TIME_SCALE";fn
俼()->亅{再(std::env::var(儚).ok().as_deref())}fn 再(丩:P<&str>)->亅{丩.and_then(|亭|亭.trim().parse::<亅>().ok()).filter(|伳|伳.is_finite()&&*伳>0.0).unwrap_or(1.0)}const
倅:C=127;const 侥:C=64;const 傺:M=63172;fn 兯(c:M)->O{(0x100..0xFFF0).contains(&c)&&!(0xD800..0xE000).contains(&c)&&c!=0x2028&&c!=0x2029&&c!=0xFEFF&&!(0xFDD0..0xFDF0).contains(&c)&&!(0x202A..=0x202E).contains(&c)&&!(0x2066..=0x2069).contains(&c)}fn
凃(侑:&str)->P<K<I>>{let mut 买=vec![M::MAX;0x10000];let mut 丅=0;for c in 0..0x10000u32{if 兯(c){买[c as C]=丅;丅+=1;}}let 佽:K<char> =侑.chars().collect();if 佽.is_empty()||!佽.len().is_multiple_of(侥){return
None;}let mut 乥=K::with_capacity(佽.len()/侥*倅);for block in 佽.chunks(侥){let mut 伆=[0u32;32];for&c in block{let 偖=*买.get(c as C)?;if 偖==M::MAX{return None;}let
mut 俇=Z::from(偖);for 仕 in 伆.iter_mut().rev(){let v=Z::from(*仕)*Z::from(傺)+俇;*仕=v as M;俇=v>>32;}if 俇!=0{return None;}}if 伆[0]>>24!=0{return None;}for(i,仕)in
伆.iter().enumerate(){let 偱=仕.to_be_bytes();乥.extend_from_slice(if i==0{&偱[1..]}else{&偱});}}let 佖=M::from_le_bytes([乥[0],乥[1],乥[2],乥[3]])as C;let 乗=佖.checked_add(4)?;if
乗>乥.len()||乥.len()-乗>=倅||乥[乗..].iter().any(|&b|b!=0){return None;}乥.truncate(乗);乥.drain(..4);Some(乥)}const _:()=assert!(僆==16);const 僆:C=16;const 僀:亅=0.4;const
僭:I=4;#[derive(Clone,Copy)]struct 仸{丵:G,偋:G,佘:G,}impl 仸{fn of(l:&J)->仸{let(a,位)=傂(l,T[0]);let(b,侁)=傂(l,T[1]);let 乵=a[T[1]];let 典=位[T[1]];let mut 乊=仸{丵:G::U,偋:G::U,佘:G::U,};for
丁 in l.map().valid.iter(){if 位[丁]==I::MAX||侁[丁]==I::MAX{continue;}if 位[丁].abs_diff(侁[丁])<=1{乊.偋.set(丁);}if 位[丁].abs_diff(侁[丁])<=僭{乊.佘.set(丁);}let 冇=佩::from(位[丁])+佩::from(侁[丁])==佩::from(典);if
乵>0.0&&冇&&丁!=T[0]&&丁!=T[1]&&a[丁]*b[丁]/乵>=僀{乊.丵.set(丁);}}乊}fn 伈(&self)->G{self.丵&self.佘}}fn 冯(l:&J,冪:G)->[I;Q]{let 临=l.map().valid;let mut 万=[I::MAX;Q];let
mut 乡=冪&临;let mut 亡=乡;let mut 两=0u8;while!亡.is_empty(){for 丁 in 亡.iter(){万[丁]=两;}let 丅=(亡.丏()&临).丗(乡);乡|=丅;亡=丅;两=两.saturating_add(1);}万}fn 傂(l:&J,上:C)->([亅;Q],[I;Q]){let
之=冯(l,G::丁(上));let mut 丸:K<C> =l.map().valid.iter().filter(|&c|之[c]!=I::MAX).collect();丸.sort_by_key(|&c|之[c]);let mut 丙=[0.0;Q];丙[上]=1.0;for 丁 in 丸{if
丁==上{continue;}let 乷=G::丁(丁).丏()&l.map().valid;丙[丁]=乷.iter().filter(|&n|之[n]!=I::MAX&&之[n]+1==之[丁]).map(|n|丙[n]).sum();}(丙,之)}const Y:C=12;const Q:C=Y*Y;const
产:C=4*Y;const 亿:Z=(1<<产)-1;const 佬:Z=1|1<<12|1<<24|1<<36;const 侨:Z=佬<<11;const 倧:Z=(1<<Y)-1;#[derive(Clone,Copy,Debug,PartialEq)]struct G([Z;3]);impl G{const
U:G=G([0;3]);const 侤:G=G([亿;3]);#[inline]fn 丁(丁:C)->G{let mut 乖=G::U;乖.set(丁);乖}#[inline]fn test(self,丁:C)->O{(self.0[丁/产]>>(丁%产))&1!=0}#[inline]fn set(&mut
self,丁:C){self.0[丁/产]|=1<<(丁%产);}#[inline]fn clear(&mut self,丁:C){self.0[丁/产]&=!(1<<(丁%产));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
佦(self)->[Z;3]{self.0}#[inline]fn count(self)->M{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丗(self,乓:G)->G{G([self.0[0]&!乓.0[0],self.0[1]&!乓.0[1],self.0[2]&!乓.0[2],])}#[inline]fn
iter(self)->侦{侦{伆:self.0,丳:0,}}#[inline]fn 侚(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&倧)<<(产-Y)),(b>>Y)|((c&倧)<<(产-Y)),c>>Y,])}#[inline]fn 侙(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亿,((b<<Y)&亿)|(a>>(产-Y)),((c<<Y)&亿)|(b>>(产-Y)),])}#[inline]fn
傓(self)->G{let[a,b,c]=self.0;G([(a>>1)&!侨&亿,(b>>1)&!侨&亿,(c>>1)&!侨&亿,])}#[inline]fn 傔(self)->G{let[a,b,c]=self.0;G([(a<<1)&!佬&亿,(b<<1)&!佬&亿,(c<<1)&!佬&亿,])}#[inline]fn
丏(self)->G{let 一=self;一.侚()|一.侙()|一.傓()|一.傔()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,乓:G)->G{G([self.0[0]&乓.0[0],self.0[1]&乓.0[1],self.0[2]&乓.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,乓:G)->G{G([self.0[0]|乓.0[0],self.0[1]|乓.0[1],self.0[2]|乓.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,乓:G){*self=*self|乓;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侤.丗(self)}}struct 侦{伆:[Z;3],丳:C,}impl Iterator for 侦{type Item=C;#[inline]fn
next(&mut self)->P<C>{let 一=self;while 一.丳<3{let 仕=一.伆[一.丳];if 仕!=0{一.伆[一.丳]=仕&(仕-1);return Some(一.丳*产+仕.trailing_zeros()as C);}一.丳+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{于{丌:M,x:I,y:I},乆{丆:I,x:I,y:I},仌{x:I,y:I},亝{x:I,y:I},}impl L{fn 冼(&self)->乙{match*self{L::于{丌,x,y}=>format!("MOVE {} {x} {y}",丌),L::乆{丆,x,y}=>format!("TRAIN {} {x} {y}",丆),L::仌{x,y}=>format!("BUILD MINE {x} {y}"),L::亝{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
儰(三:&[L])->乙{if 三.is_empty(){return "WAIT".to_string();}let mut line=乙::with_capacity(16*三.len());for(与,丕)in 三.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&丕.冼());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亾{倸,减,儬,#[default]侧,}impl 亾{fn 亷(self)->I{match self{亾::倸=>1,_=>3,}}fn 佺(self)->O{matches!(self,亾::儬|亾::侧)}fn 伜(self)->O{self==亾::侧}}const 亲:I=I::MAX;struct
从{乻:G,valid:G,乃:G,东:Box<[I]>,丏:[[[I;4];Q];2],}thread_local!{static 僟:RefCell<P<Arc<从>>> =const{RefCell::new(None)};}impl 从{fn 冦(乻:G,乃:G)->Arc<从>{僟.with(|亇|{let
mut 亇=亇.borrow_mut();if let Some(丬)=亇.as_ref(){if 丬.乻==乻&&丬.乃==乃{return Arc::clone(丬);}}let 丬=Arc::new(从::build(乻,乃));*亇=Some(Arc::clone(&丬));丬})}fn build(乻:G,乃:G)->从{let
临=!乻;const 儎:[[(佨,佨);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丏=[[[亲;4];Q];2];for(丂,乔)in 儎.iter().enumerate(){for 丁 in
临.iter(){let(x,y)=((丁%Y)as 佨,(丁/Y)as 佨);let mut 丙=0;for&(兎,兏)in 乔{let(偹,偺)=(x+兎,y+兏);if(0..Y as 佨).contains(&偹)&&(0..Y as 佨).contains(&偺){let 丅=偺 as C*Y+偹
as C;if 临.test(丅){丏[丂][丁][丙]=丅 as I;丙+=1;}}}}}let mut 东=vec![亲;Q*Q].into_boxed_slice();let mut 俫=[0u8;Q];for 上 in 临.iter(){let 些=&mut 东[上*Q..(上+1)*Q];些[上]=0;俫[0]=上
as I;let(mut 伦,mut 俸)=(0,1);while 伦<俸{let 丁=俫[伦]as C;伦+=1;for&丅 in&丏[0][丁]{if 丅!=亲&&些[丅 as C]==亲{些[丅 as C]=些[丁]+1;俫[俸]=丅;俸+=1;}}}}从{乻,valid:临,乃,东,丏,}}#[inline]fn
东(&self,上:C,丟:C)->I{self.东[上*Q+丟]}}const 僫:M=100;const 亰:F=15;const 儡:[F;3]=[1,4,20];const T:[C;2]=[0,Q-1];#[derive(Clone,Copy,PartialEq)]enum 亏{亴(C),倏,}enum
了{侶,侲,侳,}#[derive(Clone)]struct J{map:Arc<从>,乇:亾,业:[G;2],丄:[G;2],乁:G,亃:G,丈:[G;2],丆:[G;3],仪:G,亮:[M;Q],下:[F;2],丢:[F;2],丠:C,乬:M,伮:M,丣:P<亏>,伇:O,}fn 偤(mut 亁:G,业:G)->G{loop{let
丅=(亁|亁.丏())&业;if 丅==亁{return 亁;}亁=丅;}}fn 亩(业:G,以:C)->G{if!业.test(以){return G::U;}let mut 亁=G::丁(以);loop{let 丅=(亁|亁.丏())&业;if 丅==亁{return 亁;}亁=丅;}}impl
J{fn 丠(&self)->C{self.丠}fn 乬(&self)->M{self.乬}fn 丣(&self)->P<亏>{self.丣}fn 乇(&self)->亾{self.乇}fn 伭(&self,丂:C)->F{let 一=self;let 丈=一.丈[丂];一.下[丂]+10*F::from((丈&一.丆[0]).count())+20*F::from((丈&一.丆[1]).count())+30*F::from((丈&一.丆[2]).count())}fn
下(&self,丂:C)->F{self.下[丂]}fn 丢(&self,丂:C)->F{self.丢[丂]}fn 俿(&self,丂:C)->C{self.丈[丂].count()as C}fn map(&self)->&从{&self.map}fn 丐(&self,丂:C)->G{self.业[丂]}fn
丄(&self,丂:C)->G{self.丄[丂]}fn 丈(&self,丂:C)->G{self.丈[丂]}fn 介(&self,丁:C)->P<(M,I)>{let 一=self;(一.丈[0]|一.丈[1]).test(丁).then(||(一.亮[丁],一.俢(丁)))}fn 互(&self)->G{self.丈[self.丠]&self.仪}fn
乂(&self)->G{self.乁}fn 乣(&self)->G{self.亃}fn 亯(&self,丁:C)->P<I>{(丁<Q&&(self.丈[0]|self.丈[1]).test(丁)).then(||self.俢(丁))}fn 亮(&self,丁:C)->P<M>{(丁<Q&&(self.丈[0]|self.丈[1]).test(丁)).then(||self.亮[丁])}fn
冫(&mut self,丂:C){let 一=self;if 一.伇{debug_assert_eq!(一.丄,[亩(一.业[0],T[0]),亩(一.业[1],T[1])]);}else{一.丄=[亩(一.业[0],T[0]),亩(一.业[1],T[1])];一.伇=true;}let 兇=一.丈[丂].丗(一.丄[丂]);一.俭(兇);一.偏();一.下[丂]+=一.丢[丂];if
一.下[丂]<0{一.下[丂]=0;一.俭(一.丈[丂]);}一.仪|=一.丈[丂];}fn 偏(&mut self){let 一=self;for 丂 in 0..2{let 丄=一.丄[丂];let 丈=丄&一.丈[丂];let mut 丢=F::from(丄.count())+4*F::from((丄&一.乁).count());for(丆,凄)in
儡.iter().enumerate(){丢-=凄*F::from((丈&一.丆[丆]).count());}一.丢[丂]=丢;}}fn 伣(&mut self){let 一=self;if 一.丣.is_some(){return;}if 一.丠==1{if 一.乬>=僫{一.丣=Some(match
一.伭(0).cmp(&一.伭(1)){std::cmp::Ordering::Greater=>亏::亴(0),std::cmp::Ordering::Less=>亏::亴(1),std::cmp::Ordering::Equal=>亏::倏,});return;}一.乬+=1;}一.丠=1-一.丠;一.冫(一.丠);}fn
侓(&mut self,三:&[L]){let 一=self;if 一.丣.is_some(){return;}let 俚=一.伮;for&丕 in 三{if 一.免(丕,俚){continue;}一.乏(丕);if 一.丣.is_some(){return;}}一.伣();}fn 免(&self,丕:L,俚:M)->O{let
一=self;match 丕{L::于{丌,x,y}=>{一.伞(x,y).is_none()||丌>=俚||一.停(一.丠,丌).is_none()}L::乆{丆,x,y}=>{一.伞(x,y).is_none()||丆==0||丆>一.乇.亷()}L::仌{x,y}|L::亝{x,y}=>{一.伞(x,y).is_none()}}}#[inline]fn
伞(&self,x:I,y:I)->P<C>{let(x,y)=(C::from(x),C::from(y));(x<Y&&y<Y&&!self.map.乻.test(y*Y+x)).then_some(y*Y+x)}fn 停(&self,丂:C,丌:M)->P<C>{self.丈[丂].iter().find(|&丁|self.亮[丁]==丌)}#[inline]fn
俢(&self,丁:C)->I{if self.丆[0].test(丁){1}else if self.丆[1].test(丁){2}else{3}}fn 儷(&self,争:C)->G{let 乣=self.亃&self.业[争];let 丄=self.丄[争];乣|(丄&(丄&乣).丏())}fn
乐(&self,丂:C,丆:I)->G{let 一=self;debug_assert!((1..=3).contains(&丆),"unit level {} is not 1 to 3",丆);let 争=1-丂;let 丈=一.丈[0]|一.丈[1];let 兔=一.份();let 凁=match
丆{1 if 一.乇==亾::倸=>一.丆[1]|一.丆[2],1=>丈,2=>一.丆[1]|一.丆[2],_=>G::U,};let 乐=兔|一.业[争].丗(凁);if 丆<3{乐.丗(一.儷(争))}else{乐}}fn 仫(&self,丂:C)->G{self.丄[丂]|self.丄[丂].丏()}fn
份(&self)->G{let 一=self;let 丈=一.丈[0]|一.丈[1];let 公=G::丁(T[0])|G::丁(T[1]);一.map.valid.丗(丈|一.乁|一.亃|公)}fn 乏(&mut self,丕:L)->O{let 一=self;if 一.丣.is_some(){return
false;}let 丂=一.丠;let 仹=match 丕{L::乆{丆,x,y}=>一.依(丂,丆,x,y),L::于{丌,x,y}=>一.冁(丂,丌,x,y),L::仌{x,y}=>一.build(丂,true,x,y),L::亝{x,y}=>一.build(丂,false,x,y),};let
Some(仹)=仹 else{return false;};match 仹{了::侶=>{}了::侲 if 一.伇=>{一.丄[丂]=偤(一.丄[丂],一.业[丂]);}了::侳 if 一.伇=>{一.丄[丂]=偤(一.丄[丂],一.业[丂]);一.丄[1-丂]=亩(一.业[1-丂],T[1-丂]);}了::侲=>一.丄[丂]=亩(一.业[丂],T[丂]),了::侳=>{一.丄=[亩(一.业[0],T[0]),亩(一.业[1],T[1])];}}debug_assert!(!一.伇||一.丄==[亩(一.业[0],T[0]),亩(一.业[1],T[1])]);一.偏();if!一.业[0].test
(T[0]){一.丣=Some(亏::亴(1));}else if!一.业[1].test(T[1]){一.丣=Some(亏::亴(0));}true}fn 俭(&mut self,且:G){let 一=self;一.丈[0]=一.丈[0].丗(且);一.丈[1]=一.丈[1].丗(且);for 丆
in&mut 一.丆{*丆=丆.丗(且);}一.仪=一.仪.丗(且);}fn 傤(&mut self,丂:C,丁:C)->了{let 一=self;一.俭(G::丁(丁));一.乁.clear(丁);一.亃.clear(丁);let 争=1-丂;if 一.业[争].test(丁){一.业[争].clear(丁);一.业[丂].set(丁);了::侳}else
if 一.业[丂].test(丁){了::侶}else{一.业[丂].set(丁);了::侲}}fn 依(&mut self,丂:C,丆:I,x:I,y:I)->P<了>{let 一=self;if 丆==0||丆>一.乇.亷(){return None;}let 丁=一.伞(x,y)?;let 专=10*F::from(丆);if
一.下[丂]<专||!一.仫(丂).test(丁)||!一.乐(丂,丆).test(丁){return None;}let 仹=一.傤(丂,丁);一.傃(丂,丆,丁,一.伮);一.伮=一.伮.saturating_add(1);一.下[丂]-=专;Some(仹)}fn 傃(&mut self,丂:C,丆:I,丁:C,丌:M){self.丈[丂].set(丁);self.丆[C::from(丆)-1].set(丁);self.亮[丁]=丌;}fn
冁(&mut self,丂:C,丌:M,x:I,y:I)->P<了>{let 一=self;let 丒=一.伞(x,y)?;let 上=一.停(丂,丌)?;if!一.仪.test(上){return None;}let 丆=一.俢(上);let 丯=一.乐(丂,丆);let 乷=一.map.丏[丂][上];if
乷.contains(&(丒 as I))&&!丯.test(丒){return None;}let mut 不=一.map.东(上,丒);let mut 住=None;for&丅 in 乷.iter().take_while(|&&丅|丅!=亲){let 丅=C::from(丅);let 东=一.map.东(丅,丒);if
东<不&&丯.test(丅){不=东;住=Some(丅);}}let 住=住?;let 仹=一.傤(丂,住);一.丈[丂].clear(上);一.丆[C::from(丆)-1].clear(上);一.仪.clear(上);一.傃(丂,丆,住,丌);Some(仹)}fn build(&mut self,丂:C,乁:O,x:I,y:I)->P<了>{let
一=self;let 便=if 乁{一.乇.佺()}else{一.乇.伜()};let 丁=一.伞(x,y)?;let 冈=一.map.乃.test(丁);if!便||一.下[丂]<亰||冈!=乁||!一.业[丂].test(丁)||!一.仫(丂).test(丁)||!一.份().test(丁){return
None;}if 乁{一.乁.set(丁);}else{一.亃.set(丁);}一.下[丂]-=亰;Some(了::侶)}}const 僦:F=1000000000;const 侰:F=0;const 僮:F=1;const 儛:F=2;impl J{fn read(伨:&str,世:&str,乇:亾,乬:M,)->Result<J,乙>{let
mut 乃=K::new();let mut 乒=伨.lines();let 丙=俑(&mut 乒,"mine spot count")?;for _ in 0..丙{let[x,y]=仗(伯(&mut 乒,"a mine spot")?,"a mine spot")?;乃.push((侀(x)?,侀(y)?));}偙(乒,"the initial input")?;let
mut 乒=世.lines();let mut 俜=[0i64;4];for(丩,乶)in 俜.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[俩]=仗(伯(&mut 乒,乶)?,乶)?;if 俩.abs()>僦{return
Err(format!("{}: {} is out of range",乶,俩));}*丩=俩;}let mut 乻=G::U;let mut 且=[G::U;2];let mut 丄=[G::U;2];for y in 0..Y{let 些=伯(&mut 乒,"a map row")?.trim();if
些.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,些));}for(x,你)in 些.bytes().enumerate(){let 丁=y*Y+x;match 你{b'#'=>乻.set(丁),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
亊=C::from(你==b'X'||你==b'x');且[亊].set(丁);if 你.is_ascii_uppercase(){丄[亊].set(丁);}}_=>{return Err(format!("map row {y} has {:?} at {x}",你 as char))}}}}let
儺=俑(&mut 乒,"building count")?;let mut 亟=K::new();for _ in 0..儺{let 仗=仗(伯(&mut 乒,"a building")?,"a building")?;亟.push(仗);}let 俿=俑(&mut 乒,"unit count")?;let
mut 丈=K::new();for _ in 0..俿{let 仗=仗(伯(&mut 乒,"a unit")?,"a unit")?;丈.push(仗);}偙(乒,"the turn's input")?;let 傀=亟.iter().find(|偄|偄[0]==0&&偄[1]==侰).ok_or("the bot's own HQ is not among the buildings")?;let
偨=俈(傀[2],傀[3],乻)?;let 丂=T.iter().position(|&以|以==偨).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",偨))?;let mut 傕=G::U;for(x,y)in 乃{if
乻.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}傕.set(y*Y+x);}let 侽=|亊:C|if 亊==0{丂}else{1-丂};let mut l=J{map:从::冦(乻,傕),乇,业:[G::U;2],丄:[G::U;2],乁:G::U,亃:G::U,丈:[G::U;2],丆:[G::U;3],仪:G::U,亮:[0;Q],下:[0;2],丢:[0;2],丠:丂,乬,伮:1,丣:None,伇:false,};for
亊 in 0..2{let 人=侽(亊);l.业[人]=且[亊];l.丄[人]=丄[亊];l.下[人]=俜[2*亊];l.丢[人]=俜[2*亊+1];}let mut 侍=G::U;for[人,乸,x,y]in 亟{let 亊=傊(人)?;let 丁=俈(x,y,乻)?;if!且[亊].test(丁){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 侍.test(丁){return Err(format!("two buildings at ({x}, {y})"));}侍.set(丁);match 乸{侰
if 丁==T[侽(亊)]=>{}侰=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}僮=>l.乁.set(丁),儛=>l.亃.set(丁),_=>return Err(format!("unknown building type {}",乸)),}}let
mut 义=G::U;let mut 侌=0u32;for[人,丌,丆,x,y]in 丈{let 亊=傊(人)?;let 丁=俈(x,y,乻)?;if!且[亊].test(丁){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
义.test(丁){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丆){return Err(format!("unit level {} is not 1, 2 or 3",丆));}let 丌=M::try_from(丌).map_err(|_|format!("unit id {} is out of range",丌))?;if
义.iter().any(|乓|l.亮[乓]==丌){return Err(format!("two units have id {}",丌));}义.set(丁);l.丈[侽(亊)].set(丁);l.丆[丆 as C-1].set(丁);l.亮[丁]=丌;侌=侌.max(丌);}if!义.is_empty(){l.伮=侌.checked_add(1).ok_or_else(||format!("unit id {} is out of range",侌))?;}l.仪=l.丈[丂];l.伇=l.丄==[亩(l.业[0],T[0]),亩(l.业[1],T[1])];Ok(l)}}fn
伯<'a>(乒:&mut Lines<'a>,乶:&str)->Result<&'a str,乙>{乒.next().ok_or_else(||format!("the text ends before {}",乶))}fn 偙(mut 乒:Lines<'_>,乶:&str)->Result<(),乙>{match
乒.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乶)),None=>Ok(()),}}fn 仗<const N:C>(line:&str,乶:&str)->Result<[F;N],乙>{let
mut 佦=line.split_whitespace();let mut 傮=[0i64;N];for 丩 in&mut 傮{let 伺=佦.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乶))?;*丩=伺.parse().map_err(|_|format!("{}: {:?} is not an integer",乶,伺))?;}if
佦.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乶));}Ok(傮)}fn 俑(乒:&mut Lines<'_>,乶:&str)->Result<C,乙>{let[丙]=仗(伯(乒,乶)?,乶)?;C::try_from(丙).ok().filter(|&丙|丙<=Q).ok_or_else(||format!("{}: {} is out of range",乶,丙))}fn
侀(丩:F)->Result<C,乙>{C::try_from(丩).ok().filter(|&丩|丩<Y).ok_or_else(||format!("coordinate {} is off the map",丩))}fn 俈(x:F,y:F,乻:G)->Result<C,乙>{let 丁=侀(y)?*Y+侀(x)?;if
乻.test(丁){return Err(format!("({x}, {y}) is a void cell"));}Ok(丁)}fn 傊(人:F)->Result<C,乙>{match 人{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",人)),}}#[derive(Clone,Copy,PartialEq)]enum
仞{井,倄,}#[derive(Clone,Copy)]struct Settings1{俳:I,俲:I,俌:F,仦:F,俖:C,侂:I,俹:O,reserve:O,俗:O,乽:仞,fill:O,乯:个,}impl Settings1{const DEFAULT:Settings1=Settings1{俳:3,俲:1,俌:4,仦:2,俖:2,侂:2,俹:true,reserve:true,俗:true,乽:仞::井,fill:true,乯:个::价,};}const
侼:F=1000;const 侻:F=10;fn 倁(丁:C)->(I,I){((丁%Y)as I,(丁/Y)as I)}fn 俪(l:&mut J,丕:L,万:&mut Vec<L>)->O{let 仂=l.乏(丕);if 仂{万.push(丕);}仂}fn 俷(l:&mut J,上:C,丟:C,万:&mut
Vec<L>)->O{let Some((丌,_))=l.介(上)else{return false;};let(x,y)=倁(丟);俪(l,L::于{丌,x,y},万)}fn 况(l:&J,乊:&仸,乎:&Settings1,严:C,)->K<C>{let 七=l.丠();let 並=丼(l,l.丐(七));let
丛=丼(l,l.丐(1-七));let 亸=l.map().valid.丗(l.丐(0)|l.丐(1));let 傈=|c:C|並[c]!=I::MAX&&丛[c]!=I::MAX;let mut 丵:K<C> =乊.伈().丗(l.丐(七)).iter().filter(|&c|傈(c)).collect();丵.sort_by_key(|&c|(並[c],c));let
mut 乑:K<C> =亸.iter().filter(|&c|傈(c)&&並[c].abs_diff(丛[c])<=乎.俲).collect();乑.sort_by_key(|&c|(並[c].abs_diff(丛[c]),並[c],c));let mut 万:K<C> =K::new();for
丁 in 丵.into_iter().chain(乑){if 万.len()>=严{break;}let 仿=万.iter().all(|&t|l.map().东(t,丁)>=乎.俳);let 兀=乊.伈().test(丁);if!万.contains(&丁)&&(兀||仿){万.push(丁);}}万}fn
六(专:&[K<F>])->K<C>{let n=专.len();let 偬=F::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 傱=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 仔=0usize;let mut 余=vec![偬;n+1];let mut 倀=vec![false;n+1];loop{倀[仔]=true;let 偪=p[仔];let mut 仢=偬;let mut 佔=0usize;for j in 1..=n{if!倀[j]{let
主=专[偪-1][j-1]-u[偪]-v[j];if 主<余[j]{余[j]=主;傱[j]=仔;}if 余[j]<仢{仢=余[j];佔=j;}}}for j in 0..=n{if 倀[j]{u[p[j]]+=仢;v[j]-=仢;}else{余[j]-=仢;}}仔=佔;if p[仔]==0{break;}}loop{let
佔=傱[仔];p[仔]=p[佔];仔=佔;if 仔==0{break;}}}let mut 万=vec![0usize;n];for j in 1..=n{if p[j]>0{万[p[j]-1]=j-1;}}万}#[derive(Clone,Copy)]enum 仴{佇(C),侺,}fn 亍(l:&J)->G{let
七=l.丠();(l.仫(七)&l.乐(七,1)).丗(l.丐(七))}fn 冐(l:&J,乯:&个)->C{侞(l,乯,乪::井).iter().filter(|c|matches!(c,L::乆{..})).count()}fn 丨(丁:C,丫:G)->M{(G::丁(丁).丏()&丫).count()}fn
偠(l:&J,乘:G,丒:C,佖:I)->K<[F;Q]>{let 临=l.map().valid;let mut 不=[F::MIN;Q];不[丒]=0;let mut 乿=vec![不];for _ in 0..佖{let mut 丅=不;for 丁 in 临.iter(){for n in(G::丁(丁).丏()&临).iter(){if
不[n]>F::MIN{let 丩=不[n]+F::from(乘.test(n));if 丩>丅[丁]{丅[丁]=丩;}}}}不=丅;乿.push(不);}乿}fn 傁(l:&J,乿:&[[F;Q]],乘:G,上:C,k:C,)->G{let 临=l.map().valid;let mut 万=G::U;let
mut 丁=上;let mut k=k.min(乿.len()-1);while k>0&&乿[k][丁]>0{let Some(丅)=(G::丁(丁).丏()&临).iter().filter(|&n|乿[k-1][n]>F::MIN).max_by_key(|&n|{(F::from(乘.test(n))+乿[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乘.test(丅){万.set(丅);}丁=丅;k-=1;}万}fn 世(l:&J,乎:&Settings1)->K<L>{let 七=l.丠();let 乊=仸::of(l);let mut 丑=l.clone();let mut 万=K::new();let 丛=丼(l,l.丐(1-七));let
丈:K<C> =l.互().iter().collect();let 偢=(l.下(七)/侻).max(0)as C;let 亽=match 乎.乽{仞::井=>冐(l,&乎.乯).min(偢),仞::倄=>偢,};let mut 佈:K<仴> =丈.iter().map(|&c|仴::佇(c)).collect();佈.extend(std::iter::repeat_n(仴::侺,亽));let
亠=况(l,&乊,乎,佈.len()+乎.俖);let 冾=亍(l);let 丵=乊.伈();let n=佈.len().max(亠.len());let mut 专=vec![vec![0i64;n];n];for(些,&丒)in 亠.iter().enumerate(){for(佌,仰)in 专[些].iter_mut().enumerate(){let
Some(&儯)=佈.get(佌)else{*仰=侼;continue;};let 佻=match 儯{仴::佇(丁)=>{let d=l.map().东(丁,丒);if d==I::MAX{None}else{Some(F::from(d)-1)}}仴::侺=>冾.iter().map(|c|l.map().东(c,丒)).filter(|&d|d!=I::MAX).min().map(F::from),};*仰=match
佻{None=>侼,Some(佻)=>{let 丮=F::from(丛[丒])-1;let 仦=乎.仦*(佻-丮).max(0);let 俄=if 丵.test(丒){乎.俌}else{0};(佻+仦-俄).max(0)}};}}let 儱=六(&专);let mut 俛:K<(C,C)> =K::new();let
mut 俽:K<(F,C)> =K::new();for(些,&丒)in 亠.iter().enumerate(){let 佌=儱[些];if 专[些][佌]>=侼{continue;}match 佈.get(佌){Some(仴::佇(丁))=>俛.push((*丁,丒)),Some(仴::侺)=>俽.push((专[些][佌],丒)),None=>{}}}let
mut 丫=l.map().valid.丗(l.丐(0)|l.丐(1));let mut 偷:K<C> =K::new();let mut 侗=G::U;let 亹=|丁:C|{let 丆=l.亯(丁).unwrap_or(1);let 佡=丫|l.丐(1-七);(G::丁(丁).丏()&佡&l.乐(七,丆)).count()};俛.sort_by_key(|&(丁,丒)|{let
兜=if 乎.俗{亹(丁)}else{0};(兜,l.map().东(丁,丒),丁)});for&(上,丒)in&俛{let Some(丆)=丑.亯(上)else{continue;};let 丯=丑.乐(七,丆);let d=丑.map().东(上,丒);let 乘=(丫|丑.丐(1-七)).丗(侗);let
佖=d.saturating_add(乎.侂);let 乿=偠(&丑,乘,丒,佖.saturating_sub(1));let 伛=乿[乿.len()-1];let 丅=(G::丁(上).丏()&丯).iter().filter(|&c|伛[c]>F::MIN).max_by_key(|&c|{let
兂=d.saturating_sub(丑.map().东(c,丒));let 佡=丫.test(c)||丑.丐(1-七).test(c);(佡&&乎.俹,F::from(乘.test(c))+伛[c],兂,丨(c,丫),std::cmp::Reverse(c),)}).filter(|&c|{丫.test(c)||丑.丐(1-七).test(c)||丑.map().东(c,丒)<d});if
let Some(丟)=丅{if 俷(&mut 丑,上,丟,&mut 万){丫.clear(丟);偷.push(上);if 乎.reserve{侗|=傁(&丑,&乿,乘,丟,乿.len()-1);}}}}let 並=丼(l,l.丐(七));let mut 乀=G::U;for c in 丫.iter(){if
並[c]<=丛[c]{乀.set(c);}}for&上 in&丈{if 偷.contains(&上)||!丑.互().test(上){continue;}let Some(丆)=丑.亯(上)else{continue;};let 丯=丑.乐(七,丆);let 佡=(G::丁(上).丏()&丫&丯).iter().max_by_key(|&c|(丨(c,丫),丛[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丟)=佡{if 俷(&mut 丑,上,丟,&mut 万){丫.clear(丟);}continue;}let 丞=[乀&丫,丫].into_iter().find_map(|且|{且.iter().min_by_key(|&c|(丑.map().东(上,c),c))});if let
Some(丞)=丞{let d=丑.map().东(上,丞);let 丅=(G::丁(上).丏()&丯).iter().filter(|&c|丑.map().东(c,丞)<d).min_by_key(|&c|c);if let Some(丟)=丅{俷(&mut 丑,上,丟,&mut 万);}}}俽.sort();let
mut 俤=0usize;for&(_,丒)in&俽{if 丑.下(七)<侻{break;}let 且=亍(&丑);let Some(么)=且.iter().map(|c|丑.map().东(c,丒)).min()else{break;};let 乘=(丫|丑.丐(1-七)).丗(侗);let 乿=偠(&丑,乘,丒,么.saturating_add(乎.侂));let
伛=乿[乿.len()-1];let 丁=且.iter().filter(|&c|丑.map().东(c,丒)==么).max_by_key(|&c|(伛[c],丨(c,丫),std::cmp::Reverse(c)));let Some(丁)=丁 else{break;};let(x,y)=倁(丁);if
俪(&mut 丑,L::乆{丆:1,x,y},&mut 万){丫.clear(丁);俤+=1;if 乎.reserve{侗|=傁(&丑,&乿,乘,丁,乿.len()-1);}}}if 乎.fill&&乎.乽==仞::井{while 俤<亽&&丑.下(七)>=侻{let 丁=(亍(&丑)&丫).iter().max_by_key(|&c|{(丨(c,丫),std::cmp::Reverse(丛[c]),std::cmp::Reverse(c),)});let
Some(丁)=丁 else{break;};let(x,y)=倁(丁);if!俪(&mut 丑,L::乆{丆:1,x,y},&mut 万){break;}丫.clear(丁);俤+=1;}}万}const 僗:F=15;const 僰:F=6;const 倴:F=10;const 儇:I=3;#[derive(Clone,Copy)]struct
个{open:F,使:F,偒:I,俞:F,侊:F,偳:F,侘:I,偑:F,偐:F,佸:F,伥:F,伶:F,乹:F,伩:F,仳:F,传:F,}impl 个{const 价:个=个{open:3,使:2,偒:12,俞:4,侊:僗,偳:僰,侘:儇,偑:2,偐:2,佸:0,伥:0,伶:0,乹:0,伩:0,仳:0,传:0,};const
fn 凍(self,伥:F,伶:F,乹:F)->个{个{伥,伶,乹,..self}}const fn 凌(self,伩:F,仳:F)->个{个{伩,仳,..self}}}fn 佧(丁:C)->(I,I){((丁%Y)as I,(丁/Y)as I)}fn 丼(l:&J,上:G)->[I;Q]{let mut
之=[I::MAX;Q];let 临=l.map().valid;let mut 乑=上&临;let mut 义=乑;let mut d=0u8;while!乑.is_empty(){for 丁 in 乑.iter(){之[丁]=d;}let 丅=(乑.丏()&临).丗(义);义|=丅;乑=丅;d=d.saturating_add(1);}之}fn
兣(l:&J)->I{let 之=丼(l,l.丐(0));l.丐(1).iter().map(|丁|之[丁]).min().unwrap_or(I::MAX)}struct 仠<'a>{乯:&'a 个,七:C,theirs:[I;Q],伷:[I;Q],free:G,乀:G,ours:G,乃:G,丈:G,}impl
仠<'_>{fn new<'a>(l:&J,乯:&'a 个)->仠<'a>{let 七=l.丠();let 乁=丼(l,l.丐(七));let 丛=丼(l,l.丐(1-七));let 伷=丼(l,G::丁(T[1-七]));let 丫=l.map().valid.丗(l.丐(0)|l.丐(1));let
mut 乀=G::U;for 丁 in 丫.iter(){if 乁[丁]<=丛[丁]{乀.set(丁);}}仠{乯,七,theirs:丛,伷,free:丫,乀,ours:l.丐(七),乃:l.map().乃,丈:l.丈(七),}}fn value(&self,丁:C)->F{let 一=self;let
p=一.乯;let 丨=F::from((G::丁(丁).丏()&一.free).count());let 仐=p.偒;let 使=F::from(仐)-F::from(一.theirs[丁].min(仐));let 以=if p.俞>0{F::from(一.伷[丁].min(30))/p.俞}else{0};let
业=F::from((G::丁(丁).丏()&一.ours).count());let line=if 业<=1{p.伶}else{0};let 乹=if 一.乃.test(丁){p.乹}else{0};let 冱=if 丨==0{p.伩}else{0};let 仳=p.仳*(22-F::from(一.伷[丁].min(22)));let
传=p.传*F::from((G::丁(丁).丏()&一.丈).count());p.open*丨+p.使*使-以+p.伥*业-line+乹-冱+仳-传}fn 不(&self,且:G)->P<C>{且.iter().max_by_key(|&丁|(self.value(丁),std::cmp::Reverse(丁)))}fn
俎(&mut self,丁:C){let 一=self;一.free.clear(丁);一.乀.clear(丁);一.ours.set(丁);一.丈.set(丁);}}fn 佝(l:&mut J,丕:L,万:&mut Vec<L>)->O{let 仂=l.乏(丕);if 仂{万.push(丕);}仂}fn
傚(l:&mut J,上:C,丟:C,万:&mut Vec<L>)->O{let Some((丌,_))=l.介(上)else{return false;};let(x,y)=佧(丟);佝(l,L::于{丌,x,y},万)}fn 凂(h:F,k:F)->F{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-倴}fn
冞(l:&J,乯:&个)->K<L>{侞(l,乯,乪::井)}#[derive(Clone,Copy,PartialEq)]enum 乪{井,仍{乂:I,丈:I},}fn 侞(l:&J,乯:&个,乽:乪)->K<L>{let mut 丑=l.clone();let mut 世=仠::new(l,乯);let
mut 万=K::new();他(&mut 丑,&mut 世,&mut 万);match 乽{乪::井=>冗(&mut 丑,&mut 世,&mut 万),乪::仍{乂,丈}=>{党(&mut 丑,&mut 世,乂,丈,&mut 万)}}万}fn 他(l:&mut J,世:&mut 仠<'_>,万:&mut
Vec<L>){let 七=世.七;let mut 仓:K<C> =K::new();loop{let 丯=l.乐(七,1);let mut 伌:P<(M,C)> =None;for 上 in l.互().iter(){if 仓.contains(&上){continue;}let 亹=(G::丁(上).丏()&世.free&丯).count();if
亹==0{仓.push(上);}else if 伌.is_none_or(|(n,_)|亹<n){伌=Some((亹,上));}}let Some((_,上))=伌 else{break;};世.丈.clear(上);match 世.不(G::丁(上).丏()&世.free&丯){Some(丟)if
傚(l,上,丟,万)=>世.俎(丟),_=>{世.丈.set(上);仓.push(上);}}}for 上 in 仓{if!l.互().test(上){continue;}let 丒=[世.乀,世.free].into_iter().find_map(|且|{且.iter().min_by_key(|&丁|(l.map().东(上,丁),丁))});if
let Some(丒)=丒{傚(l,上,丒,万);}}}fn 冗(l:&mut J,世:&mut 仠<'_>,万:&mut Vec<L>){let 七=世.七;let p=世.乯;let h=(p.侊-F::from(l.乬())).max(p.偳);let 冺=(F::from(兣(l))-p.偑).max(0)/p.偐.max(1)+p.佸;loop{let
下=l.下(七);let 丈=l.俿(七)as F;let 伐=F::from(世.乀.count());let 丁=世.不(l.仫(七)&世.free&l.乐(七,1));let 傫=凂(h,(伐/(丈+1)).min(冺));let 偉=丁.is_some()&&下>=倴&&傫>0;let 側=4*h-亰;let
乹=偵(l,世);let 儽=乹.is_some()&&下>=亰&&側>0;if 儽&&(!偉||側>=傫){let(x,y)=佧(乹.unwrap_or_default());if 佝(l,L::仌{x,y},万){continue;}break;}let Some(丁)=丁.filter(|_|偉)else{break;};let(x,y)=佧(丁);if!佝(l,L::乆{丆:1,x,y},万){break;}世.俎(丁);}}fn
偵(l:&J,世:&仠<'_>)->P<C>{let 七=世.七;(l.map().乃&l.丄(七)&l.份()).iter().filter(|&乹|世.theirs[乹]>世.乯.侘).min_by_key(|&乹|(l.map().东(T[七],乹),乹))}fn 党(l:&mut J,世:&mut
仠<'_>,乂:I,丈:I,万:&mut Vec<L>,){let 七=世.七;for _ in 0..乂{let Some(乹)=偵(l,世)else{break;};let(x,y)=佧(乹);if!佝(l,L::仌{x,y},万){break;}}for _ in 0..丈{let Some(丁)=世.不(l.仫(七)&世.free&l.乐(七,1))else{break;};let(x,y)=佧(丁);if!佝(l,L::乆{丆:1,x,y},万){break;}世.俎(丁);}}fn
乡(l:&J)->O{if l.丣().is_some(){return false;}let 七=l.丠();let 下=l.下(七);let 丰=伕::new(l,七,true);l.丐(1-七).iter().any(|丁|丰.专[丁]<=下)}#[derive(Clone,Copy)]struct
久{乤:I,乽:乪,}const 倂:I=I::MAX;const 傽:[乪;8]=[乪::井,乪::仍{乂:0,丈:0},乪::仍{乂:0,丈:1},乪::仍{乂:0,丈:2},乪::仍{乂:0,丈:倂,},乪::仍{乂:1,丈:0},乪::仍{乂:1,丈:1},乪::仍{乂:1,丈:倂,},];struct
侪<'a>{丹:&'a[个],仩:个,rest:久,仒:P<乩>,乎:Settings2,}impl 侪<'_>{fn 乯(&self,两:久)->&个{&self.丹[C::from(两.乤)%self.丹.len()]}fn 三(&self,l:&J,两:久)->K<L>{侞(l,self.乯(两),两.乽)}}fn
儭(l:&J,三:&[L],仩:&个)->(J,O){let mut 丅=l.clone();丅.侓(三);if 丅.丣().is_some()||乡(&丅){return(丅,true);}let 乢=冞(&丅,仩);丅.侓(&乢);let 付=丅.丣().is_some()||乡(&丅)||丅.乬()>個;(丅,付)}fn
冖(l:&J,丘:&[久],任:&侪<'_>)->K<Z>{let 七=l.丠();let mut 何=vec![佟(l,七)];let mut 事=l.clone();for 世 in 0..個 as C{let 两=丘.get(世).copied().unwrap_or(任.rest);let 三=任.三(&事,两);let(丅,付)=儭(&事,&三,&任.仩);if
付{break;}事=丅;何.push(佟(&事,七));}何}fn 佟(l:&J,七:C)->Z{let mut hash:Z=0xcbf29ce484222325;let mut 俥=|伺:Z|hash=(hash^伺).wrapping_mul(0x0100000001b3);for(冴,乖)in[l.丐(七),l.丈(七),l.乂()].into_iter().enumerate(){for
丁 in 乖.iter(){俥(((冴 as Z)<<8)|丁 as Z);}俥(0xff);}俥(l.下(七)as Z);hash}enum 亳{佄{丱:C,亄:C,丽:仛,侖:M,},佪(Settings1),}struct 亐{亻:亳,丹:K<个>,仩:个,丘:K<久>,侄:K<Z>,伍:C,串:M,付:O,}impl
亐{fn new(亻:亳,丹:K<个>,仩:个)->亐{亐{亻,丹,仩,丘:K::new(),侄:K::new(),伍:0,串:0,付:false,}}fn 冓(&mut self,l:&J)->O{let 一=self;let 冨=matches!(一.亻,亳::佄{..}|亳::佪(_));let
冕=!冨&&一.串>0&&一.伍>=一.侄.len();if!一.付&&(冕||乡(l)){一.付=true;}!一.付}fn end(&mut self){self.付=true;}fn 世(&mut self,l:&J,仒:P<乩>)->K<L>{let 一=self;let 任=侪{丹:&一.丹,仩:一.仩,rest:久{乤:0,乽:乪::井,},仒,乎:Settings2::DEFAULT,};if
let 亳::佪(乎)=一.亻{一.串+=1;一.伍+=1;return 世(l,&乎);}let 七=l.丠();let 冉=一.侄.get(一.伍)==Some(&佟(l,七));let 兙=matches!(一.亻,亳::佄{侖,..}if 侖>0&&l.乬()>=侖);if!冉||兙{let
亘=一.串==0;一.丘=match 一.亻{亳::佄{丱,亄,丽,..}=>{let 丽=&丽;let 乱=儀;let mut 乔=丘(l,&一.丹,乱,丱,亄,仒,丽,);乔.resize(丱,乱);乔}亳::佪(_)=>K::new(),};一.侄=match 一.亻{亳::佄{..}=>冩(l,&一.丹,&一.丘),_=>冖(l,&一.丘,&任),};一.伍=0;一.串+=1;}let
两=一.丘.get(一.伍).copied().unwrap_or(任.rest);一.伍+=1;任.三(l,两)}}fn 冩(l:&J,丹:&[个],丘:&[久])->K<Z>{let 七=l.丠();let mut 何=K::with_capacity(丘.len());let mut 事=l.clone();for&两
in 丘{if 事.丣().is_some(){break;}何.push(佟(&事,七));事=侾(&事,&三(&事,丹,两));}何}const 個:M=20;const 儝:F=3;#[derive(Clone,Copy)]struct Settings2{丱:F,}impl Settings2{const
DEFAULT:Settings2=Settings2{丱:儝};}fn 乔(l:&J,上:G)->[I;Q]{let 临=l.map().valid;let mut 之=[I::MAX;Q];let mut 乑=上&临;let mut 义=乑;let mut d=0u8;while!乑.is_empty(){for
丁 in 乑.iter(){之[丁]=d;}let 丅=(乑.丏()&临).丗(义);义|=丅;乑=丅;d=d.saturating_add(1);}之}fn 准(l:&J,丂:C)->F{let 並=乔(l,l.丄(丂));let 丛=乔(l,l.丄(1-丂));l.map().valid.iter().map(|丁|match
並[丁].cmp(&丛[丁]){_ if 並[丁]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 儉:[个;27]=丹();const
儀:久=久{乤:16,乽:乪::井,};const fn 丹()->[个;27]{let 伥=[0,2,4];let 伶=[0,3,6];let 仳=[0,1,2];let mut 万=[个::价;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
伩=if g>0||t>0{4}else{0};万[i]=个::价.凍(伥[g],伶[t],0).凌(伩,仳[w]);i+=1;}万}#[derive(Default)]struct 侷{且:F,丢:F,下:F,乭:F,丶:F,佤:F,丰:F,伒:F,}#[derive(Clone,Copy)]struct
仛{且:F,丢:F,下:F,丶:F,乭:F,佤:F,丰:F,伒:F,丵:F,伝:O,丸:亞,}#[derive(Clone,Copy,PartialEq)]enum 亞{倝,倍(侫),侭{仐:F},仵(仵),倊,}fn 冻(l:&J,丂:C)->[I;Q]{let 並=丼(l,G::丁(T[丂]));let
丛=丼(l,G::丁(T[1-丂]));let mut 仅=G::U;let mut 乀=G::U;for c in l.map().valid.iter(){if 並[c]==I::MAX||丛[c]==I::MAX{continue;}if 並[c]==丛[c]{仅.set(c);}if 並[c]<=丛[c]{乀.set(c);}}let
d=丼(l,仅);let mut 万=[I::MAX;Q];for c in 乀.iter(){万[c]=d[c];}万}fn 儶(l:&J,丂:C,仅:&[I;Q])->F{let d=丼(l,l.丄(丂));(0..Q).filter(|&c|仅[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&仅[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 仵{僺,}impl
仵{fn 伸(self,a:F,b:F,d:F)->F{let 亼=(a-b).abs()/2;match self{仵::僺=>a.min(d/2),}}}fn 兝(l:&J,丂:C,俘:仵)->[F;Q]{let 並=丼(l,G::丁(T[丂]));let 丛=丼(l,G::丁(T[1-丂]));let
d=F::from(並[T[1-丂]]);let mut 万=[0;Q];for c in 0..Q{if 並[c]!=I::MAX&&丛[c]!=I::MAX{万[c]=俘.伸(F::from(並[c]),F::from(丛[c]),d);}}万}#[derive(Clone,Copy,PartialEq)]enum
侫{儐(F),}impl 侫{fn 伸(self,亼:F)->F{match self{侫::儐(交)=>(交-亼).max(0).pow(2),}}}type 佱=[F;4];fn 冣(l:&J,丂:C)->[F;Q]{struct 伽{丄:G,云:[佩;Q],乳:[佩;Q],size:[F;Q],丶:[F;Q],仲:佩,}fn
休(d:&mut 伽,v:C){d.仲+=1;d.云[v]=d.仲;d.乳[v]=d.仲;d.size[v]=1;for w in(G::丁(v).丏()&d.丄).iter(){if d.云[w]==0{休(d,w);d.size[v]+=d.size[w];d.乳[v]=d.乳[v].min(d.乳[w]);if
d.乳[w]>=d.云[v]{d.丶[v]+=d.size[w];}}else{d.乳[v]=d.乳[v].min(d.云[w]);}}}let mut d=伽{丄:l.丄(丂),云:[0;Q],乳:[0;Q],size:[0;Q],丶:[0;Q],仲:0,};let 乴=T[丂];if d.丄.test(乴){休(&mut
d,乴);}d.丶[乴]=0;d.丶}fn 冥(l:&J,丂:C)->侷{let 丄=l.丄(丂);let 丶=冣(l,丂);let 伷=丼(l,G::丁(T[1-丂]));let mut 万=侷{且:F::from(丄.count()),丢:l.丢(丂),下:l.下(丂),丰:F::MAX,伒:准(l,丂)/2,..侷::default()};for
丁 in 丄.iter(){if 丶[丁]>0{万.乭+=1;万.丶=万.丶.max(丶[丁]);}if 丁!=T[丂]&&(G::丁(丁).丏()&丄).count()<=1{万.佤+=1;}万.丰=万.丰.min(F::from(伷[丁]));}万}fn 凅(l:&J,丂:C,丽:&仛,佘:G)->F{let
伄=F::from((l.丐(丂)&佘).count());let s=冥(l,丂);let o=丽;o.且*s.且+o.丢*s.丢+o.下*s.下-o.丶*s.丶-o.乭*s.乭-o.佤*s.佤-o.丰*s.丰.min(30)+o.伒*s.伒+o.丵*伄}fn 伈(l:&J)->G{仸::of(l).伈()}fn
兾(l:&J)->G{let 並=丼(l,G::丁(T[0]));let 丛=丼(l,G::丁(T[1]));let mut 为=G::U;for c in l.map().valid.iter(){if 並[c]!=I::MAX&&丛[c]!=I::MAX&&並[c].abs_diff(丛[c])<=2{为.set(c);}}为}fn
伏(l:&J)->[F;Q]{let a=丼(l,G::丁(T[0]));let b=丼(l,G::丁(T[1]));let mut 万=[F::MAX;Q];for c in 0..Q{if a[c]!=I::MAX&&b[c]!=I::MAX{万[c]=F::from(a[c].abs_diff(b[c]))/2;}}万}#[derive(Clone,Copy)]struct
会{丵:G,为:G,乤:[F;Q],伏:[F;Q],乑:G,front:[I;Q],}impl 会{fn of(l:&J,丽:&仛)->会{let 偰=matches!(丽.丸,亞::倝);let mut 乤=[0;Q];if let 亞::倍(先)=丽.丸{for(w,亼)in 乤.iter_mut().zip(伏(l)){*w=if
亼==F::MAX{0}else{先.伸(亼)};}}if let 亞::仵(俘)=丽.丸{乤=兝(l,l.丠(),俘);}let(冝,乑)=match 丽.丸{亞::侭{..}=>{let r=伏(l);let mut f=G::U;for(c,&亼)in r.iter().enumerate(){if
亼==0{f.set(c);}}(r,f)}_=>([F::MAX;Q],G::U),};会{乤,伏:冝,乑,front:if 丽.丸==亞::倊{冻(l,l.丠())}else{[I::MAX;Q]},丵:if 丽.丵!=0||偰{仸::of(l).伈()}else{G::U},为:if 偰{兾(l)}else{G::U},}}}fn
伪(l:&J,丂:C,丽:&仛,乊:会)->佱{let 伵=凅(l,丂,丽,乊.丵);match 丽.丸{亞::倍(_)|亞::仵(_)=>{let 丄=l.丄(丂);let 凉:F=丄.iter().map(|c|乊.乤[c]).sum();[F::from(丄.count()),凉,伵,0]}亞::侭{仐}=>{let
丄=l.丄(丂);let d=丼(l,丄);let 仿:F=乊.乑.iter().map(|c|F::from(d[c]).min(仐)).sum();[F::from(丄.count()),-仿,伵,0]}亞::倊=>[F::from(l.丄(丂).count()),儶(l,丂,&乊.front),伵,0,],亞::倝=>{let
丐=l.丐(丂);let 丵=F::from((丐&乊.丵).count());let 为=F::from((丐&乊.为).count());match 丽.丸{_=>[F::from(l.丄(丂).count()),丵,为,伵],}}}}fn 侾(l:&J,三:&[L])->J{let mut 丅=l.clone();丅.侓(三);if
丅.丣().is_none(){丅.侓(&[]);}丅}const 倃:I=I::MAX;fn 三(l:&J,丹:&[个],两:久)->K<L>{if 两.乤==倃{let 乎=Settings1{乯:丹.first().copied().unwrap_or(个::价),乽:if 两.乽==乪::井{仞::井}else{仞::倄},..Settings1::DEFAULT};return
世(l,&乎);}侞(l,&丹[C::from(两.乤)%丹.len()],两.乽,)}fn 傄(l:&J,丹:&[个],丘:&[久],乱:久,丱:C,)->J{let mut 事=l.clone();for 世 in 0..丱{if 事.丣().is_some(){break;}let 两=丘.get(世).copied().unwrap_or(乱);事=侾(&事,&三(&事,丹,两));}事}fn
冮(l:&J,丹:&[个],伝:O)->K<(久,K<L>)>{let mut 万:K<(久,K<L>)> =K::new();if 伝{for 乽 in[乪::井,乪::仍{乂:0,丈:I::MAX,},]{let 两=久{乤:倃,乽,};let c=三(l,丹,两);if!万.iter().any(|(_,乓)|*乓==c){万.push((两,c));}}}for
乤 in 0..丹.len().min(C::from(I::MAX)){for&乽 in&傽{let 两=久{乤:乤 as I,乽,};let c=三(l,丹,两);if!万.iter().any(|(_,乓)|*乓==c){万.push((两,c));}}}万}fn 儴(l:&J,丹:&[个],乱:久,丱:C,亄:C,仒:P<乩>,丽:&仛,)->K<久>{let
七=l.丠();let 作=||仒.is_some_and(|t|乩::now()>=t);let 乊=会::of(l,丽);let 仈=|丘:&[久]|伪(&傄(l,丹,丘,乱,丱),七,丽,乊);let mut 不:(佱,K<久>)=(仈(&[]),K::new());let mut 乑:K<(J,K<久>)>
=vec![(l.clone(),K::new())];for 亨 in 0..丱{let mut 仑:K<(佱,J,K<久>)> =K::new();let mut 义:HashMap<Z,C> =HashMap::new();for(偸,丘)in&乑{if 作(){break;}for(两,c)in
冮(偸,丹,丽.伝){let 丅=侾(偸,&c);let mut 俋=丘.clone();俋.push(两);let v=伪(&丅,七,丽,乊);let 伪=佟(&丅,七);match 义.get(&伪){Some(&丳)if 仑[丳].0>=v=>{}Some(&丳)=>仑[丳]=(v,丅,俋),None=>{义.insert(伪,仑.len());仑.push((v,丅,俋));}}}}if
仑.is_empty(){break;}仑.sort_by(|a,b|b.0 .cmp(&a.0));仑.truncate(亄);let 亇=亨+1==丱||作();for(i,(_,_,丘))in 仑.iter().take(if 亇{亄}else{1}).enumerate(){if i>0&&作(){break;}let
v=仈(丘);if v>不.0{不=(v,丘.clone());}}if 亇{break;}乑=仑.into_iter().map(|(_,s,p)|(s,p)).collect();}不.1}fn 丘(l:&J,丹:&[个],乱:久,丱:C,兽:C,仒:P<乩>,丽:&仛,)->K<久>{let 七=l.丠();let
乊=会::of(l,丽);let 仈=|丘:&[久]|伪(&傄(l,丹,丘,乱,丱),七,丽,乊);let mut 不:(佱,K<久>)=(仈(&[]),K::new());let mut 亄=2;loop{let 也=儴(l,丹,乱,丱,亄,仒,丽);let v=仈(&也);if v>不.0{不=(v,也);}let
作=仒.is_some_and(|t|乩::now()>=t);if 作||(仒.is_none()&&亄>=兽){break;}亄*=2;}不.1}const 僖:C=4+12;fn 丱(俟:impl BufRead,mut 侐:impl Write,mut 俀:impl FnMut(&str,&str,M,乩)->乙,)->Result<(),乙>{let
mut 乒=倞{傉:俟};let Some(亘)=乒.line()?else{return Ok(());};let mut 伨=format!("{}\n",亘);乒.take(丙(&亘,"mine spot count")?,&mut 伨)?;let mut 世=0;loop{let Some(亘)=乒.line()?else{return
Ok(());};let 乄=乩::now();世+=1;let mut 亭=format!("{}\n",亘);乒.take(僖-1,&mut 亭)?;for 乶 in["building count","unit count"]{let line=乒.傋()?;let n=丙(&line,乶)?;亭.push_str(&line);亭.push('\n');乒.take(n,&mut
亭)?;}let 俀=俀(&伨,&亭,世,乄);writeln!(侐,"{}",俀).and_then(|()|侐.flush()).map_err(|仾|format!("turn {}: cannot write the answer: {}",世,仾))?;}}struct 倞<R>{傉:R,}impl<R:BufRead>倞<R>{fn
line(&mut self)->Result<P<乙>,乙>{let mut line=乙::new();match self.傉.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(仾)=>Err(format!("cannot read input: {}",仾)),}}fn
傋(&mut self)->Result<乙,乙>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:C,亭:&mut String)->Result<(),乙>{for
_ in 0..n{亭.push_str(&self.傋()?);亭.push('\n');}Ok(())}}fn 丙(line:&str,乶:&str)->Result<C,乙>{match line.trim().parse::<C>(){Ok(n)if n<=僥=>Ok(n),_=>Err(format!("bad {} {line:?}",乶)),}}const
僥:C=144;const 儘:F=3;const 僝:F=2;const 僪:C=16;fn 冑(l:&J,万:&mut Vec<L>){let 丂=l.丠();let mut 亻=倦{s:l.clone(),丂,丮:1-丂,丒:T[1-丂],侉:T[丂],世:l.乬(),万,};亻.run();}struct
倦<'a>{s:J,丂:C,丮:C,丒:C,侉:C,世:M,万:&'a mut Vec<L>,}fn 侣(丁:C)->(I,I){((丁%Y)as I,(丁/Y)as I)}fn 俦(丌:M,丁:C)->L{let(x,y)=侣(丁);L::于{丌,x,y}}fn 傧(丆:I,丁:C)->L{let(x,y)=侣(丁);L::乆{丆,x,y}}impl
倦<'_>{fn run(&mut self){let 一=self;if 一.佗(){return;}一.他();if 一.佗(){return;}一.亟();一.亽();}fn 之(&self,上:C,丟:C)->F{F::from(self.s.map().东(上,丟))}fn 乏(&mut self,丕:L)->O{let
乾=self.s.乏(丕);if 乾{self.万.push(丕);}乾}fn 伹(&self,举:&J)->O{举.丣()==Some(亏::亴(self.丂))}fn 佗(&mut self)->O{let 一=self;if 一.s.丣().is_some(){return false;}for
丁 in 一.s.互().iter(){if 一.之(丁,一.丒)!=1{continue;}let Some((丌,_))=一.s.介(丁)else{continue;};let 丕=俦(丌,一.丒);let mut 举=一.s.clone();if 举.乏(丕)&&一.伹(&举){return 一.乏(丕);}}let
Some(両)=一.儿()else{return false;};let mut 举=一.s.clone();if!両.iter().all(|&丕|举.乏(丕))||!一.伹(&举){return false;}for 丕 in 両{一.乏(丕);}true}fn 儿(&self)->P<K<L>>{let
一=self;const 們:F=F::MAX;let 享=一.s.乇().亷();let 丯:K<G> =(1..=享).map(|丆|一.s.乐(一.丂,丆)).collect();let 佒=|丁:C|{丯.iter().position(|乖|乖.test(丁)).map(|与|(与 as I+1,10*(与
as F+1)))};let 丄=一.s.丄(一.丂);let mut 专=[們;Q];let mut 乜=[C::MAX;Q];let mut 乾=[false;Q];for 丁 in 丄.iter(){专[丁]=0;}loop{let mut 丅:P<C> =None;for 丁 in 0..Q{if!乾[丁]&&专[丁]<們&&丅.is_none_or(|n|专[丁]<专[n]){丅=Some(丁);}}let
Some(丁)=丅 else{break;};乾[丁]=true;if 丁==一.丒{break;}for&乷 in&一.s.map().丏[0][丁]{if 乷==亲{break;}let 乷=C::from(乷);if 乾[乷]||丄.test(乷){continue;}let Some((_,仭))=佒(乷)else{continue;};if
专[丁]+仭<专[乷]{专[乷]=专[丁]+仭;乜[乷]=丁;}}}if 专[一.丒]>一.s.下(一.丂){return None;}let mut 両=K::new();let mut 丁=一.丒;while!丄.test(丁){let(丆,_)=佒(丁)?;両.push(傧(丆,丁));丁=乜[丁];}両.reverse();Some(両)}fn
乘(&self,乕:&J)->F{let(七,仱)=(self.丂,self.丮);let 乜=&self.s;let 丙=|乖:G|F::from(乖.count());let 业=丙(乕.丐(七))-丙(乜.丐(七));let 今=丙(乜.丐(仱))-丙(乕.丐(仱));let 俣=丙(乜.丄(仱))-丙(乕.丄(仱));let
例=乜.伭(仱)-乕.伭(仱);let 乂=丙(乜.乂()&乜.丐(仱))-丙(乕.乂()&乕.丐(仱));12*业+20*今+8*(俣-今).max(0)+5*例+25*乂}fn 傖(&self,丁:C)->F{let 一=self;let 乹=一.s.map().乃.test(丁);if 乹&&一.s.乇().佺()&&!一.s.丐(一.丂).test(丁){20}else{0}}fn
做(&self,严:F)->O{let 一=self;一.s.丈(一.丮).iter().any(|丁|一.之(丁,一.侉)<=严)}fn 他(&mut self){let 一=self;let mut 丈:K<(F,C)> =一.s.互().iter().map(|丁|(一.之(丁,一.丒),丁)).collect();丈.sort_unstable();for(令,丁)in
丈{if 一.s.丣().is_some(){return;}let Some((丌,丆))=一.s.介(丁)else{continue;};let 丯=一.s.乐(一.丂,丆);let 乷=一.s.map().丏[一.丂][丁];let mut 不:P<(F,C)> =None;for&丅 in 乷.iter().take_while(|&&丅|丅!=亲){let
丅=C::from(丅);if!丯.test(丅){continue;}let mut 举=一.s.clone();if!举.乏(俦(丌,丅)){continue;}let 丩=if 一.伹(&举){100000}else{一.乘(&举)+10*(令-一.之(丅,一.丒))+一.傖(丅)};if 不.is_none_or(|(交,_)|丩>交){不=Some((丩,丅));}}if
let Some((丩,丅))=不{if 丩>0{一.乏(俦(丌,丅));}}}}fn 亟(&mut self){let 一=self;if 一.s.丣().is_some(){return;}if 一.做(儘){一.冽();}else{一.乂();}}fn 冽(&mut self){let 一=self;if!一.s.乇().伜()||一.s.下(一.丂)<亰{return;}let
乷=一.s.map().丏[一.丂][一.侉];let 且:K<C> =乷.iter().take_while(|&&丁|丁!=亲).map(|&丁|C::from(丁)).collect();let 乁=一.s.丐(一.丂);if 且.iter().any(|&丁|乁.test(丁)&&一.s.乣().test(丁)){return;}for
丁 in 且{let(x,y)=侣(丁);if 一.乏(L::亝{x,y}){return;}}}fn 乂(&mut self){let 一=self;if!一.s.乇().佺(){return;}while 一.s.下(一.丂)>=亰{let 丫=一.s.map().乃&一.s.丐(一.丂)&一.s.丄(一.丂)&一.s.份();let
Some(丁)=丫.iter().min_by_key(|&丁|(一.之(丁,一.侉),丁))else{return;};let(x,y)=侣(丁);if!一.乏(L::仌{x,y}){return;}}}fn 亽(&mut self){let 一=self;let 傌=if 一.世<=3{0}else{5};for
_ in 0..僪{if 一.s.丣().is_some(){return;}let 下=一.s.下(一.丂);if 下<10{return;}let 兮=一.做(僝);let 么=一.s.仫(一.丂);let mut 不:P<(F,L)> =None;let mut 俴:P<(F,L)> =None;let
mut 偂=G::U;for 丆 in 1..=一.s.乇().亷(){let 丯=一.s.乐(一.丂,丆);let 亹=(么&丯).丗(偂);偂=丯;let 仭=10*F::from(丆);if 仭>下{continue;}for 丁 in 亹.iter(){let 丕=傧(丆,丁);let mut
举=一.s.clone();if!举.乏(丕){continue;}let 乮=举.下(一.丂);let 丢=举.丢(一.丂);let 例=一.s.伭(一.丮)-举.伭(一.丮);let 儮=if 例>0&&兮{乮+丢>=0}else{丢>=0&&(乮>=傌||例>0)};if!儮{continue;}let
乘=一.乘(&举);let 丩=乘+一.傖(丁)-6*一.之(丁,一.丒)-10*(F::from(丆)-1);if 乘>0{if 不.is_none_or(|(交,_)|丩>交){不=Some((丩,丕));}}else if 丢>=3&&乮>=傌+10&&丆==1{if 俴.is_none_or(|(交,_)|丩>交){俴=Some((丩,丕));}}}}let
Some((_,丕))=不.or(俴)else{return;};if!一.乏(丕){return;}}}}fn 兦(l:&J,万:&mut Vec<L>){万.clear();冑(l,万);}const 仁:F=F::MAX;const 僩:C=8;struct 伕{专:[F;Q],乜:[I;Q],}fn
伟(l:&J,丂:C,他:O,丒:C)->F{俐(l,丂,伉(l,丂,他),丒)}fn 俐(l:&J,丂:C,伉:G,丒:C)->F{let mut 专=仁;偞(l,丂,伉,|两,new|{if new.test(丒){专=10*两 as F;}专!=仁});专}fn 写(l:&J,丂:C,伉:G,九:F)->G{let
mut 乡=G::U;偞(l,丂,伉,|两,new|{if 10*两 as F>九{return true;}乡|=new;false});乡.丗(l.丄(丂))}fn 伉(l:&J,丂:C,他:O)->G{if 他&&丂==l.丠(){l.互()}else{G::U}}fn 偞(l:&J,丂:C,伉:G,mut
休:impl FnMut(C,G)->O,){let 享=l.乇().亷();let mut 佒=[G::U;3];let mut 义=G::U;for 丆 in 1..=享{let 乖=l.乐(丂,丆);佒[C::from(丆)-1]=乖.丗(义);义|=乖;}let mut 丨=[l.丄(丂),G::U,G::U,G::U];for
丁 in(伉&l.丈(丂)).iter(){if let Some(丆)=l.亯(丁).filter(|&丆|丆<=享){丨[0]|=G::丁(丁).丏()&l.乐(丂,丆);}}let mut 乾=G::U;for 两 in 0..{let new=丨[两%4].丗(乾);丨[两%4]=G::U;if
休(两,new){return;}乾|=new;let 丅=new.丏().丗(乾);for(伛,且)in 佒.iter().enumerate(){丨[(两+伛+1)%4]|=丅&*且;}if 丨.iter().all(|乖|乖.丗(乾).is_empty()){break;}}}impl 伕{fn
new(l:&J,丂:C,他:O)->伕{let 享=C::from(l.乇().亷());let mut 丯=[G::U;3];for(与,乖)in 丯.iter_mut().enumerate().take(享){*乖=l.乐(丂,与 as I+1);}let 丄=l.丄(丂);let 丏=&l.map().丏[0];let
mut 专=[仁;Q];let mut 乜=[亲;Q];let mut 丨=[0u8;Q];let mut 丙=0;let mut 傑=G::U;for 丁 in 丄.iter(){专[丁]=0;丨[丙]=丁 as I;丙+=1;}if 他&&丂==l.丠(){for 上 in l.互().iter(){let
Some(丆)=l.亯(上)else{continue;};let 乔=G::丁(上).丏()&丯[C::from(丆)-1];for 丁 in 乔.iter(){if 专[丁]!=0{专[丁]=0;丨[丙]=丁 as I;丙+=1;}}}}while 丙>0{let mut 不=0;for 与 in
1..丙{let(a,b)=(C::from(丨[与]),C::from(丨[不]));if(专[a],a)<(专[b],b){不=与;}}let 丁=C::from(丨[不]);丙-=1;丨[不]=丨[丙];傑.set(丁);for&丅 in&丏[丁]{if 丅==亲{break;}let 丅=C::from(丅);if
傑.test(丅)||丄.test(丅){continue;}let Some(丆)=丯[..享].iter().position(|乖|乖.test(丅))else{continue;};let 傦=专[丁]+10*(丆 as F+1);if 傦<专[丅]{if 专[丅]==仁{丨[丙]=丅 as
I;丙+=1;}专[丅]=傦;乜[丅]=丁 as I;}}}伕{专,乜}}fn path(&self,丒:C)->K<C>{let mut path=K::new();if 丒>=Q||self.专[丒]==仁{return path;}let mut 丁=丒;while self.乜[丁]!=亲{path.push(丁);丁=C::from(self.乜[丁]);}path.reverse();path}}struct
佭{三:K<L>,专:F,}fn 兼(l:&J,丂:C,丁:C)->P<I>{(1..=l.乇().亷()).find(|&丆|l.乐(丂,丆).test(丁))}fn 农(丑:&mut J,七:C,丞:C,九:F,三:&mut Vec<L>,京:&mut i64,)->O{loop{if 丑.丣()==Some(亏::亴(七)){return
true;}if 丑.丐(七).test(丞)&&丑.丄(七).test(丞){return true;}let 丰=伕::new(丑,七,false);let 专=丰.专[丞];if 专==仁||*京+专>九{return false;}let Some(&丁)=丰.path(丞).first()else{return
false;};let Some(丆)=兼(丑,七,丁)else{return false;};let 丕=L::乆{丆,x:(丁%Y)as I,y:(丁/Y)as I,};if!丑.乏(丕){return false;}*京+=10*F::from(丆);三.push(丕);}}fn 倿(l:&J,七:C,亠:&[C],九:F)->P<佭>{let
mut 丑=l.clone();let mut 三=K::new();let mut 京=0;for&丞 in 亠{if!农(&mut 丑,七,丞,九,&mut 三,&mut 京){return None;}}(京<=九).then_some(佭{三,专:京,})}fn 冿(l:&J,七:C,丒:C)->F{let
临=l.map().valid;let 丐=l.丐(七)&临;let 偍=|mut 主:G|loop{let 丅=主|(主.丏()&丐);if 丅==主{return 主;}主=丅;};let mut 主=偍(l.丄(七));let mut 乔=0;while!主.test(丒){let new=主.丏().丗(主)&临;if
new.is_empty(){return F::MAX/20;}主=偍(主|new);乔+=1;}乔}fn 俉(l:&J,丒:C,九:F)->P<佭>{if 九<0||l.丣().is_some()||丒>=Q||l.map().乻.test(丒){return None;}let 七=l.丠();if
l.丐(七).test(丒)&&l.丄(七).test(丒){return Some(佭{三:K::new(),专:0,});}if 10*冿(l,七,丒)>九{return None;}let mut 不=倿(l,七,&[丒],九);let 亘=伕::new(l,七,false);let mut 乣:K<C>
=(l.乣()&l.丐(1-七)).iter().filter(|&亃|亃!=丒&&亘.专[亃]!=仁).collect();乣.sort_by_key(|&亃|(l.map().东(亃,丒),亃));乣.truncate(僩);for 亃 in 乣{if let Some(両)=倿(l,七,&[亃,丒],九){if
不.as_ref().is_none_or(|交|両.专<交.专){不=Some(両);}}}不}const 僧:C=20000;struct 佯{三:K<L>,专:F,且:F,丈:F,亟:F,丢:F,丧:F,}struct 侱{且:F,丈:F,亟:F,}fn 偧(l:&J,丂:C)->侱{let 丄=l.丄(丂);let
丈=l.丈(丂)&丄;let 丩=丈.iter().filter_map(|丁|l.亯(丁)).map(|丆|10*F::from(丆)).sum();侱{且:F::from(丄.count()),丈:丩,亟:F::from(((l.乂()|l.乣())&丄).count()),}}struct 倪{七:C,伻:G,乜:侱,丢:F,伬:C,九:F,估:C,path:K<L>,也:K<佯>,index:HashMap<佀,C,BuildHasherDefault<DefaultHasher>>,}type
佀=[Z;3];fn 凋(mut 书:佀,丁:C)->佀{书[丁/64]|=1<<(丁%64);书}impl 倪{fn 偕(&mut self,丑:&J,亇:P<C>,今:佀,专:F,)->O{let 一=self;let 七=一.七;let 亹=match 亇{None=>丑.仫(七),Some(丁)=>G::丁(丁).丏(),}&一.伻;let
亹=亹.丗(丑.丐(七));let mut 丯=[G::U;3];for 丆 in 1..=丑.乇().亷(){丯[C::from(丆)-1]=丑.乐(七,丆);}for 丁 in 亹.iter(){let 享=C::from(丑.乇().亷());let Some(与)=丯[..享].iter().position(|乖|乖.test(丁))else{continue;};let
丆=与 as I+1;let 仭=10*F::from(丆);if 专+仭>一.九{continue;}if 一.估>=僧{return false;}一.估+=1;let mut 丅=丑.clone();let 丕=L::乆{丆,x:(丁%Y)as I,y:(丁/Y)as I,};if!丅.乏(丕){continue;}一.path.push(丕);let
伴=凋(今,丁);一.冚(&丅,伴,专+仭);let 兊=一.path.len()<一.伬&&丅.丣().is_none();let 兤=!兊||一.偕(&丅,Some(丁),伴,专+仭);一.path.pop();if!兤{return false;}}true}fn 冚(&mut self,乕:&J,伴:佀,专:F){let
一=self;let 佛=1-一.七;let 予=偧(乕,佛);let 且=一.乜.且-予.且;let 丈=一.乜.丈-予.丈;if 且<=0&&丈<=0{return;}let 亟=一.乜.亟-予.亟;let 丢=乕.丢(一.七)-一.丢;let 凎=乕.丣()==Some(亏::亴(一.七));let
丧=if 凎{F::MAX/2}else{3*且+丈+10*亟+2*丢-专};if let Some(&丳)=一.index.get(&伴){if 专>=一.也[丳].专{return;}}let 丶=佯{三:一.path.clone(),专,且,丈,亟,丢,丧,};match 一.index.get(&伴){Some(&丳)=>一.也[丳]=丶,None=>{一.index.insert(伴,一.也.len());一.也.push(丶);}}}}fn
偓(l:&J,伬:C,九:F,严:C)->K<佯>{丿(l,伬,九,严).0}fn 丿(l:&J,伬:C,九:F,严:C)->(K<佯>,C){if l.丣().is_some()||伬==0||严==0{return(K::new(),0);}let 七=l.丠();let 佛=1-七;let mut
俯=倪{七,伻:l.丐(佛)|l.丐(佛).丏(),乜:偧(l,佛),丢:l.丢(七),伬,九,估:0,path:K::new(),也:K::new(),index:HashMap::default(),};俯.偕(l,None,[0;3],0);let 估=俯.估;let mut 也=俯.也;也.sort_by(|a,b|b.丧.cmp(&a.丧).then(a.专.cmp(&b.专)));也.truncate(严);(也,估)}const
伙:F=1000000;const 僨:F=200;fn 俰(l:&J,丂:C)->[F;5]{let 丄=l.丄(丂);let 丈:F=(l.丈(丂)&丄).iter().filter_map(|丁|l.亯(丁)).map(|丆|10*F::from(丆)).sum();[l.下(丂),3*l.丢(丂),4*F::from(丄.count()),丈,10*F::from((l.乣()&丄).count()),]}fn
乀(l:&J,丂:C)->F{俰(l,丂).iter().sum()}fn 偲(l:&J,丂:C,亚:G)->F{兢(l,丂,俐(l,丂,亚,T[1-丂]),)}fn 兢(l:&J,丂:C,专:F)->F{if 专==仁{return F::MIN;}l.下(丂)+l.丢(丂)-专}fn 侔(仆:F)->F{仆.clamp(-僨,0)}fn
兘(l:&J,丂:C,亚:[G;2],)->(F,[F;2]){let 乓=1-丂;match l.丣(){Some(亏::亴(凊))=>{return(if 凊==丂{伙}else{-伙},[0,0])}Some(亏::倏)=>return(0,[0,0]),None=>{}}let mut 丧=乀(l,丂)-乀(l,乓);if
丂==l.丠(){let mut 丅=l.clone();丅.伣();if 丅.丣().is_none()&&伟(&丅,乓,true,T[丂])<=丅.下(乓){丧-=伙/2;}}let 仧=[偲(l,丂,亚[0]),偲(l,乓,亚[1]),];(丧+侔(仧[0])-侔(仧[1]),仧)}
