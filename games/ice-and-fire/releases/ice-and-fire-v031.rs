// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伐=u16;type 中=u32;type Z=u64;type 伏=i32;type C=i64;type 亪=f64;type O=bool;type 乑=String;type 乸=Instant;type K<T>=Vec<T>;type
U<T>=Option<T>;use std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
倜:O=true;fn 偻(l:&J,一:F,伌:C,丨:G,丗:C)->O{let 丂=l.丛();let 义=1-丂;let 俗=倜&&丗<=l.上(义)+2*l.丟(义).max(0);let 们=G::一(一)|G::一(一).与();if!俗&&(们&丨).is_empty(){return
false;}let(x,y)=乢(一);let mut 乖=l.clone();if!乖.丿(L::乹{x,y}){return false;}if 例(&乖,丂)<伌{return true;}俗&&传(&乖,义,false,Q[丂])>丗}const 偎:O=false;const 倵:I=2;const
侰:I=1;const 侚:I=2;const 众:F=0;const 佈:F=2;const 值:O=false;const 倾:O=false;const 倽:O=false;const 倻:O=false;const 倣:F=4;const 侘:F=5;#[derive(Clone)]enum
丫{人(L),京(K<L>),}impl 丫{fn 万(&self)->&[L]{match self{丫::人(丐)=>std::slice::from_ref(丐),丫::京(万)=>万,}}}fn 乢(一:F)->(I,I){((一%T)as I,(一/T)as I)}fn 俑(l:&J,丆:&mut
Vec<丫>){丆.clear();if l.丝().is_some(){return;}let 丂=l.丛();let 乼=l.临().亇();let mut 丢=[G::P;4];for 丄 in 1..=乼{丢[F::from(丄)]=l.乂(丂,丄);}for 三 in l.乷().iter(){let
Some((世,丄))=l.亩(三)else{continue;};let 乓=G::一(三).与()&丢[F::from(丄)];for 丧 in 乓.iter(){let(x,y)=乢(丧);丆.push(丫::人(L::乲{世,x,y}));}}let 上=l.上(丂);let 丅=l.丅(丂);let
亥=l.亳(丂);let 乿=match 倵{0=>亥,1=>{let 仴=l.map().valid.丕(丅).与()&丅;亥.丕(丅)|仴}_=>亥.丕(丅),};for 丄 in 1..=乼{let 下=10*C::from(丄);if 上<下{break;}let mut 丏=乿&丢[F::from(丄)];for
傃 in 1..丄{丏=丏.丕(丢[F::from(傃)]);}for 一 in 丏.iter(){let(x,y)=乢(一);丆.push(丫::人(L::乎{丄,x,y}));}}if 上>=亀{let 丣=l.不(丂)&亥&l.亿();if l.临().会(){for 一 in(丣&l.map().丱).iter(){let(x,y)=乢(一);丆.push(丫::人(L::亗{x,y}));}}if
l.临().仑(){let 仴=l.map().valid.丕(l.不(丂)).与();let 丏=(丣&仴).丕(l.map().丱);let 丨=丨(l,1-丂);let 伌=C::from((丨&l.丅(丂)).count());let 丗=传(l,1-丂,false,Q[丂]);let 並=if
偎{let(互,伨)=丼(l,丂);互.丕(伨)}else{G::P};for 一 in 丏.iter().filter(|&一|{let 们=G::一(一)|G::一(一).与();!(们&並).is_empty()||偻(l,一,伌,丨,丗)}){let(x,y)=乢(一);丆.push(丫::人(L::乹{x,y}));}}}let
偺=match 侰{1=>亀,2=>10,_=>C::MAX,};if 上>=偺{let 丨=丨(l,1-丂);let 仅=l.map().valid.丕(l.不(丂)|l.不(1-丂));for 三 in(l.乷()&丨).iter(){let Some((世,丄))=l.亩(三)else{continue;};let
乓=G::一(三).与()&丢[F::from(丄)]&仅;let(伦,伧)=乢(三);let 偹=if 侰==1{if l.map().丱.test(三){continue;}L::乹{x:伦,y:伧}}else{L::乎{丄:1,x:伦,y:伧,}};for 丧 in 乓.iter(){let(x,y)=乢(丧);丆.push(丫::京(vec![L::乲{世,x,y},偹]));}}}if
侚!=0&&上>=亀&&l.临().仑(){偱(l,&丢,丆);}if 众!=0&&上>=20{傟(l,丢[1],上,丆);}if 倻{偨(l,丢[1],上,丆);}for 丳 in 俇(l,侘,上,倣){if 丳.万.len()>1{丆.push(丫::京(丳.万));}}if let Some(丗)=佚(l,Q[1-丂],上){if
丗.万.len()>1{丆.push(丫::京(丗.万));}}}fn 偱(l:&J,丢:&[G;4],丆:&mut Vec<丫>){let 丂=l.丛();let 义=1-丂;let 丨=丨(l,义);let 丠=l.不(丂);let 丘=l.不(义);let 仴=丘.与();let 丱=l.map().丱;for
三 in(l.乷()&丨&l.丅(丂)).iter(){if 丱.test(三){continue;}let Some((世,丄))=l.亩(三)else{continue;};let 偖=l.map().丙(三,Q[义]);let 乓=G::一(三).与()&丢[F::from(丄)];let(伦,伧)=乢(三);for
丧 in 乓.iter(){let 体=match 侚{1=>丠.test(丧)&&l.map().丙(丧,Q[义])>偖,2=>丠.test(丧)&&仴.test(丧),_=>丠.test(丧)||(丘.test(丧)&&丧!=Q[义]),};if!体{continue;}let(x,y)=乢(丧);丆.push(丫::京(vec![L::乲{世,x,y},L::乹{x:伦,y:伧},]));}}}fn
傟(l:&J,佳:G,上:C,丆:&mut Vec<丫>){let 丂=l.丛();let 丹=l.map();let 仅=丹.valid.丕(l.不(丂)|l.不(1-丂));let 並=仅&佳;let 乿:K<F> =(l.丅(丂).与()&並).iter().collect();if 乿.is_empty(){return;}let
傊=|丌:F|{乿.iter().copied().min_by_key(|&一|(丹.丙(一,丌),一))};let mut 佛:K<(F,F)> =K::new();if 值{for 两 in 傠(l,&乿){if let Some(举)=傊(两){佛.push((举,两));}}}else{let
丌=Q[1-丂];let mut 來=乿.clone();來.sort_by_key(|&一|(丹.丙(一,丌),一));來.truncate(佈);佛.extend(來.into_iter().map(|举|(举,丌)));}let 仟=F::try_from(上/10).unwrap_or(0).min(众);for(举,丌)in
佛{let mut 丗=vec![举];let mut 亖=G::一(举);while 丗.len()<仟{let 乭=丗[丗.len()-1];if 乭==丌{break;}let 七=(G::一(乭).与()&並).丕(亖).iter().min_by_key(|&一|(丹.丙(一,丌),一));let
Some(七)=七 else{break;};丗.push(七);亖.set(七);let 万=丗.iter().map(|&一|{let(x,y)=乢(一);L::乎{丄:1,x,y}}).collect();丆.push(丫::京(万));}if 倽{傀(l,&丗,丆);}}}fn 傀(l:&J,丗:&[F],丆:&mut
Vec<丫>){let 丂=l.丛();let 举=丗[0];let 乮=l.乷()&l.丈(丂)&G::一(举).与();for 三 in 乮.iter(){let Some((世,丄))=l.亩(三)else{continue;};if!l.乂(丂,丄).test(举){continue;}let(x,y)=乢(举);let
mut 万=vec![L::乲{世,x,y}];for&一 in&丗[1..]{let(x,y)=乢(一);万.push(L::乎{丄:1,x,y});丆.push(丫::京(万.clone()));}}}fn 傠(l:&J,乿:&[F])->K<F>{let 丂=l.丛();let 丹=l.map();let(乀,丬)=佫(l);let
丠:K<F> =l.丈(丂).iter().collect();let 丘:K<F> =l.丈(1-丂).iter().collect();let 伥=|两:F|{乿.iter().map(|&一|丹.丙(一,两)).min().unwrap_or(I::MAX)};let 偢=|两:F|{let 丨=伥(两);丠.iter().any(|&僂|{let
d=丹.丙(僂,两);d<=丨&&丘.iter().all(|&义|d<丹.丙(义,两))})};let 佶=|两:F|{倾&&(l.不(1-丂).test(两)||丘.iter().any(|&义|丹.丙(义,两)<伥(两)))};let 並=|丏:G|->K<F>{let mut 乶:K<F> =丏.丕(l.不(丂)).iter().filter(|&两|伥(两)!=I::MAX&&!偢(两)&&!佶(两)).collect();乶.sort_by_key(|&两|(伥(两),两));乶};let
mut 乶=並(丬);if 乶.is_empty(){乶=並(乀);}乶.truncate(佈);乶}fn 偤(l:&J)->K<F>{let 丂=l.丛();let 丹=l.map();let(乀,丬)=佫(l);let 丅=l.丅(丂);let 丠=l.不(丂);let 丨=丨(l,1-丂);let
丘:K<F> =l.丈(1-丂).iter().collect();let 俏=|一:F|{丅.iter().map(|a|丹.丙(a,一)).min().unwrap_or(I::MAX)};let mut 丏:K<F> =(乀|丬).iter().filter(|&一|{if 丠.test(一){丅.test(一)&&丨.test(一)}else{!l.不(1-丂).test(一)&&丘.iter().any(|&e|丹.丙(e,一)<俏(一))}}).collect();丏.sort_by_key(|&一|(I::from(!丬.test(一)),俏(一),一));丏.truncate(
2*佈);丏}fn 偨(l:&J,佳:G,上:C,丆:&mut Vec<丫>){let 丂=l.丛();let 丹=l.map();let 仅=丹.valid.丕(l.不(丂)|l.不(1-丂));let 並=仅&佳;let 丅=l.丅(丂);let 丣=丅&l.亿();let 傂=if 众==0{3}else{众};for
一 in 偤(l){if!l.不(丂).test(一){let 举=(丅.与()&並).iter().min_by_key(|&c|(丹.丙(c,一),c));if let Some(举)=举{let mut 丗=vec![举];let mut 亖=G::一(举);while 丗[丗.len()-1]!=一&&丗.len()<傂{let
乭=丗[丗.len()-1];let 七=(G::一(乭).与()&並).丕(亖).iter().min_by_key(|&c|(丹.丙(c,一),c));let Some(七)=七 else{break;};丗.push(七);亖.set(七);}let 下=10*C::try_from(丗.len()).unwrap_or(C::MAX);if
丗.len()>=2&&丗[丗.len()-1]==一&&下<=上{let 万=丗.iter().map(|&c|{let(x,y)=乢(c);L::乎{丄:1,x,y}}).collect();丆.push(丫::京(万));}}}if 上>=亀&&l.临().仑(){let 丱=(G::一(一)|G::一(一).与())&丣;for
乡 in 丱.丕(丹.丱).iter(){let(x,y)=乢(乡);丆.push(丫::人(L::乹{x,y}));}}}}const 倰:I=2;const 偅:C=5;fn 乮(l:&J,丁:F)->G{佺(l,丁,倰)}fn 佺(l:&J,丁:F,三:I)->G{let mut 乮=G::P;if
三>3{return 乮;}for 一 in(l.丈(丁)&l.丅(丁)).iter(){if l.亍(一).is_some_and(|丄|丄>=三){乮|=G::一(一);}}乮}fn 丨(l:&J,乐:F)->G{let 乇=l.上(乐)+l.丟(乐);傘(l,乐,乮(l,乐),乇)}fn 任(l:&J,丁:F)->G{丨(l,1-丁)&l.丅(丁)}fn
例(l:&J,丁:F)->C{C::from(任(l,丁).count())}const 侦:C=4;fn 傇(丏:C)->C{丏/侦}fn 丯(l:&J,丁:F,任:G)->C{let 丅=l.丅(丁);let 丯=(l.丯()&丅).丕(任);let 丏=C::from(丅.count());let
佖=if l.乩()<=倏{丏/倎+倍}else{傇(丏)};佖*C::from(丯.count())}const 倏:中=15;const 倍:C=6;const 倎:C=侦;const 偁:C=2;fn 倃(l:&J,丁:F)->C{let 丅=l.丅(丁);let[a,b,c]=丅.似();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^丁 as Z).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 东=(hash as F)&(侶-1);偏.with(|佘|{let mut 佘=佘.borrow_mut();let
仉=&mut 佘[东];if 仉.0==丅&&仉.1==丁 as I+1{return 仉.2;}let 丮=僇(丅,Q[丁]);*仉=(丅,丁 as I+1,丮);丮})}const 侶:F=1<<12;thread_local!{static 偏:std::cell::RefCell<K<(G,I,C)>>
=std::cell::RefCell::new(vec![(G::P,0,0);侶]);}fn 僇(丅:G,井:F)->C{struct 仭{丅:G,乧:[I;M],乒:[I;M],size:[I;M],亶:I,亣:C,仟:C,}fn 今(d:&mut 仭,v:F)->I{d.亶+=1;let 乧=d.亶;d.乧[v]=乧;let
mut 乒=乧;let mut 侂=1u8;let mut 丳=0u8;for w in(G::一(v).与()&d.丅).iter(){if d.乧[w]==0{let 俺=今(d,w);侂+=俺;乒=乒.min(d.乒[w]);if d.乒[w]>=乧{丳+=俺;}}else{乒=乒.min(d.乧[w]);}}d.乒[v]=乒;d.size[v]=侂;if
丳>0&&d.乧[v]>1{d.亣+=1;d.仟=d.仟.max(C::from(丳));}侂}if!丅.test(井){return 0;}let mut d=仭{丅,乧:[0;M],乒:[0;M],size:[0;M],亶:0,亣:0,仟:0,};今(&mut d,井);d.亣+d.仟}fn 傢(l:&J,丁:F)->C{偁*(倃(l,1-丁)-倃(l,丁))}fn
俈(l:&J,丁:F)->C{if l.上(丁)+l.丟(丁)>=0{return 0;}(l.丈(丁)&l.丅(丁)).iter().filter_map(|一|l.亍(一)).map(|丄|10*C::from(丄)).sum()}const 倢:C=15;const 偋:[C;2]=[3,1];const
偉:[C;2]=[4,20];fn 傴(l:&J,丁:F)->[C;2]{let mut 且=[0;2];for 一 in(l.丈(丁)&l.丅(丁)).iter(){match l.亍(一){Some(2)=>且[0]+=1,Some(3)=>且[1]+=1,_=>{}}}且}fn 俹(l:&J,丁:F)->C{let[俾,傹]=傴(l,丁);let
侻=|丄:F|(3-偋[丄])*偉[丄];-倢*俾+侻(0)*俾+侻(1)*傹}const 倛:C=0;const 倞:C=2;const 倝:C=100;fn 俵(伲:C)->C{if 伲<0{return 0;}倛+倞*伲.min(倝)/2}const 侥:I=2;const 価:C=2;const
侠:C=4;const 借:I=3;fn 亡(l:&J,丁:F)->C{if 価==0{return 0;}let 丶=l.map().valid;let mut 丈=G::P;for 一 in(l.丈(丁)&l.丅(丁)).iter(){if l.亍(一).is_some_and(|丄|丄<=借){丈|=G::一(一);}}let
mut 仌=丶.丕(l.不(丁));let mut 佣=G::P;let mut 乓=0;for 丙 in 1..=侠{仌=(仌|仌.与())&丶;let 伩=(丈&仌).丕(佣);乓+=(丙-1)*C::from(伩.count());佣|=伩;}乓+=侠*C::from(丈.丕(佣).count());価*乓}const
侵:C=8;fn 丼(l:&J,丁:F)->(G,G){let 丅=l.丅(丁);let 互=丅&l.不(1-丁).与();let 乌=l.乌()&丅;(互,互&(乌|乌.与()))}fn 倁(l:&J,丁:F)->C{if 侵==0{return 0;}侵*C::from(丼(l,丁).1 .count())}const
佄:C=0;const 依:I=1;const 侞:C=4;const 佅:C=0;const 侜:C=0;const 倓:中=14;const 倒:C=2;const 侴:C=0;const 偍:中=14;fn 介(l:&J,丁:F)->C{if 侴==0||l.乩()>偍{return 0;}let
丠=主(l,l.丅(丁));let 丘=主(l,l.丅(1-丁));let mut 仄=0;for c in l.map().valid.iter(){match 丠[c].cmp(&丘[c]){std::cmp::Ordering::Less=>仄+=1,std::cmp::Ordering::Greater=>仄-=1,std::cmp::Ordering::Equal=>{}}}侴*仄}struct
侟{valid:G,乀:G,丬:G,}thread_local!{static 倔:std::cell::RefCell<U<侟>> =const{std::cell::RefCell::new(None)};}fn 佫(l:&J)->(G,G){倔.with(|一|{let mut 丷=一.borrow_mut();let
丶=l.map().valid;if 丷.as_ref().is_none_or(|z|z.valid!=丶){let 令=令(l);let mut 乀=G::P;for c in 丶.iter(){if 令[c]<=倒{乀.set(c);}}*丷=Some(侟{valid:丶,乀,丬:仂(l),});}丷.as_ref().map_or((G::P,G::P),|z|(z.乀,z.丬))})}fn
伨(l:&J,丁:F)->G{let 丅=l.丅(丁);let 乌=l.乌()&丅;(乌|乌.与())&丅}fn 两(l:&J,丁:F)->C{if(佄==0&&侞==0&&佅==0&&侜==0)||l.乩()>倓{return 0;}let(乀,丬)=佫(l);let mut 亷=0;if 佄!=0{let
丠=主(l,l.丅(丁));let 丘=主(l,l.丅(1-丁));let mut 仄=0;for c in 乀.iter(){let(a,b)=(丠[c],丘[c]);if a!=I::MAX&&a.saturating_add(依)<=b{仄+=1;}else if b!=I::MAX&&b.saturating_add(依)<=a{仄-=1;}}亷+=佄*仄;}let
且=|乆:G|C::from(乆.count());let(乊,丘)=(l.丅(丁),l.丅(1-丁));亷+=侞*(且(乊&乀)-且(丘&乀));if 佅!=0{亷+=佅*(且(伨(l,丁)&乀)-且(伨(l,1-丁)&乀));}亷+=侜*(且(乊&丬)-且(丘&丬));亷}fn 佥(l:&J,丁:F)->C{let
乮=[佺(l,丁,侥),佺(l,1-丁,侥),];let(个,仜)=偮(l,丁,乮);let 个=个+俵(仜[0])-俵(仜[1]);if l.丝().is_some(){return 个;}let 丠=任(l,丁);let 丘=任(l,1-丁);let 伌=C::from(丘.count())-C::from(丠.count());个-2*俈(l,丁)+2*俈(l,1-丁)+傢(l,丁)+俹(l,丁)-俹(l,1-丁)-亡(l,丁)+亡(l,1-丁)+偅*伌/4+丯(l,丁,丠)-丯(l,1-丁,丘)+倁(l,丁)-倁(l,1-丁)+两(l,丁)+介(l,丁)}const
倬:F=8;const 倸:F=3;const 値:F=32;const 倹:Z=300;const 個:亪=0.75;const 倘:U<中> =Some(7);const 倖:O=false;const 倕:C=10;const 倗:中=6;struct 享{亨:U<乸>,仁:U<乸>,伀:Z,}#[derive(Default)]struct
侲{万:K<L>,你:F,亟:Z,严:F,之:F,係:F,亠:F,亓:O,}const 偌:O=true;#[derive(Clone)]struct 乻{万:K<L>,佤:C,}struct 仍{万:K<L>,end:J,first:U<Z>,両:U<K<乻>>,}type 仮=HashMap<Z,C,BuildHasherDefault<DefaultHasher>>;struct
佋{丂:F,limit:享,亟:Z,份:仮,仈:U<K<丫>>,侇:O,}const 佉:C=0;const 侕:O=true;const 倇:O=false;const 侔:U<I> =None;fn 做(l:&J,严:&[乻],丆:&mut Vec<丫>){if!侕&&侔.is_none(){return;}let
伈=严.iter().map(|乯|乯.万.as_slice());偛(l,伈,(侕,倇,侔),丆,);}fn 偛<'a>(l:&J,伈:impl Iterator<Item=&'a[L]>,(乌,与,丄):(O,O,U<I>),丆:&mut Vec<丫>,){let 丂=l.丛();let 乊=l.丅(丂);let
mut 仹=G::P;for 乯 in 伈{for 丐 in 乯{if let L::乎{x,y,..}|L::乲{x,y,..}=*丐{let 一=F::from(y)*T+F::from(x);if 乊.test(一){仹|=G::一(一);}}}}if 仹.is_empty(){return;}let
丣=l.亿()&乊;let 上=l.上(丂);let 俫=|丐:L,丆:&mut Vec<丫>|{if!丆.iter().any(|a|a.万()==[丐]){丆.push(丫::人(丐));}};if 乌&&上>=亀&&l.临().仑(){let mut 丏=仹;if 与{丏|=仹.与();}for
一 in(丏&丣).丕(l.map().丱).iter(){let(x,y)=乢(一);俫(L::乹{x,y},丆);}}if let Some(丄)=丄.filter(|&丄|丄<=l.临().亇()){if 上>=10*C::from(丄){for 一 in(仹&丣).iter(){let(x,y)=乢(一);俫(L::乎{丄,x,y},丆);}}}}fn
何(l:&mut J,万:&[L])->O{万.iter().all(|&丐|l.丿(丐))}fn 了(万:&[L])->Z{let mut hash:Z=0xcbf29ce484222325;for 丐 in 万{let(伭,a,b,c)=match*丐{L::乲{世,x,y}=>(1,Z::from(世),x,y),L::乎{丄,x,y}=>(2,Z::from(丄),x,y),L::亗{x,y}=>(3,0,x,y),L::乹{x,y}=>(4,0,x,y),};let
仫=伭|a<<8|Z::from(b)<<40|Z::from(c)<<48;hash=(hash^仫).wrapping_mul(0x0100000001b3);}hash}fn 侸(乨:&J,乯:&[L])->J{let mut 七=乨.clone();七.仕();for&丐 in 乯{七.丿(丐);}七.仕();七}fn
估(l:&J)->U<K<L>>{let 丂=l.丛();let 丌=Q[1-丂];let 仪=|丵:&J|丵.丝()==Some(书::亅(丂));for 一 in l.乷().iter(){if l.map().丙(一,丌)!=1{continue;}let Some((世,_))=l.亩(一)else{continue;};let(x,y)=乢(丌);let
丐=L::乲{世,x,y};let mut 丵=l.clone();if 丵.丿(丐)&&仪(&丵){return Some(vec![丐]);}}let 丗=佚(l,丌,l.上(丂))?;let mut 丵=l.clone();(何(&mut 丵,&丗.万)&&仪(&丵)).then_some(丗.万)}fn
俸(l:&J,丁:F,乔:C)->O{let 下=佡(l,丁,l.丈(丁),Q[1-丁]);下!=些&&l.上(丁)+乔*l.丟(丁)>=下}fn 傳(l:&J)->O{let 丂=l.丛();if 佉==0||l.丝().is_some(){return false;}let mut 七=l.clone();七.仕();let
例=七.丝().is_none()&&传(&七,1-丂,true,Q[丂])<=七.上(1-丂);!例&&俸(l,丂,佉)}impl 佋{fn 侽(&self,乨:&J)->O{self.侇&&乨.丝()!=Some(书::亅(self.丂))&&!俸(乨,self.丂,佉)}fn 二(&self)->O{self.亟>=self.limit.伀||self.limit.亨.is_some_and(|t|乸::now()>=t)}fn
个(&mut self,乨:&J,严:&[乻],佗:C)->C{self.亟+=1;self.使(乨,严,佗)}fn 使(&self,乨:&J,严:&[乻],佗:C)->C{if 乨.丝()==Some(书::亅(self.丂)){return 优;}if 严.is_empty(){return 佥(乨,self.丂);}let
mut 伽=C::MAX;for 乯 in 严{伽=伽.min(佥(&侸(乨,&乯.万),self.丂));if 伽<=佗{break;}}伽}#[allow(clippy::too_many_arguments)]fn 傐(&mut self,乵:&J,仔:C,乳:&[丫],乙:&[(C,F)],乞:&mut[O],严:&[乻],他:&mut
仮,佰:O,)->(U<(F,J,C)>,O){let mut 丒=None;let mut 伝=仔;for&(伳,东)in 乙{if 乞[东]{continue;}if 佰&&伳!=C::MAX&&伳<=伝-仔{return(丒,true);}if self.二(){return(丒,false);}乞[东]=true;let
万=乳[东].万();let mut 七=乵.clone();if!何(&mut 七,万)||self.侽(&七){他.insert(了(万),C::MIN/4);continue;}let 个=self.个(&七,严,伝);他.insert(了(万),个-仔);if 个>伝{伝=个;丒=Some((东,七,个));}}(丒,false)}fn
佟(&mut self,井:&J,严:&[乻],仳:&[Z],偞:O)->仍{let q=self;let mut 乬=None;let mut 乵=井.clone();let mut 万=K::new();let mut 仔=q.个(&乵,严,C::MIN);let mut 他=仮::default();let
mut 乳=K::new();let mut 乙=K::new();while 乵.丝().is_none()&&!q.二(){if 万.is_empty()&&偞{match&q.仈{Some(仈)=>乳.clone_from(仈),None=>{俑(&乵,&mut 乳);q.仈=Some(乳.clone());}}}else{俑(&乵,&mut
乳);}做(&乵,严,&mut 乳);乙.clear();for(东,亜)in 乳.iter().enumerate(){let k=了(亜.万());let 伳=他.get(&k).or(q.份.get(&k)).copied();乙.push((伳.unwrap_or(C::MAX),东));}乙.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乞=vec![false;乳.len()];if 万.is_empty(){for(东,亜)in 乳.iter().enumerate(){乞[东]=仳.contains(&了(亜.万()));}}let mut 丒=None;for
佰 in[true,false]{let(乃,傪)=q.傐(&乵,仔,&乳,&乙,&mut 乞,严,&mut 他,佰,);丒=乃;if 丒.is_some()||!傪{break;}}let Some((东,七,个))=丒 else{break;};if 万.is_empty(){乬=Some(了(乳[东].万()));}万.extend_from_slice(乳[东].万());乵=七;仔=个;}for(k,乄)in
他{let 俔=q.份.entry(k).or_insert(乄);*俔=(*俔).max(乄);}仍{万,end:乵,first:乬,両:None,}}fn 伈(&self,乨:&J)->K<K<L>>{let mut 七=乨.clone();七.仕();let mut 丆=K::new();if
七.丝().is_some(){return 丆;}let 予=七.丛();let 上=七.上(予);if let Some(丗)=佚(&七,Q[self.丂],上){丆.push(丗.万);}for 丳 in 俇(&七,侘,上,倸){丆.push(丳.万);}let mut 俓=K::new();偸(&七,&mut
俓);丆.push(俓);let mut 丘=佋{丂:予,limit:享{伀:倹,..self.limit},亟:0,份:仮::default(),仈:None,侇:false,};丆.push(丘.佟(&七,&[],&[],true).万);丆.retain(|万|!万.is_empty());丆}fn
侹(&self,乨:&J)->K<乻>{self.伈(乨).into_iter().map(|万|{let 佤=佥(&侸(乨,&万),self.丂);乻{万,佤}}).collect()}fn 亠(&mut self,井:&J,亜:&丫,严:&[乻])->U<仍>{let mut 乖=井.clone();if!何(&mut
乖,亜.万())||self.侽(&乖){return None;}self.亟+=1;let 乽=self.佟(&乖,严,&[],false);let mut 万=亜.万().to_vec();万.extend_from_slice(&乽.万);Some(仍{万,end:乽.end,first:Some(了(亜.万())),両:None,})}fn
俌(&self,丞:&仍,严:&[乻])->C{let 个=self.使(&丞.end,严,C::MIN);match&丞.両{Some(両)if!両.is_empty()=>个.min(self.使(&丞.end,両,C::MIN)),_=>个,}}}fn 傚(乃:&[乻],严:&mut Vec<乻>)->O{let
乁:K<_> =严.iter().map(|r|r.万.clone()).collect();for 乯 in 乃{if 严.iter().any(|r|r.万==乯.万){continue;}严.push(乯.clone());}严.sort_by_key(|乯|乯.佤);严.truncate(倬);严.iter().map(|r|&r.万).ne(&乁)}fn
傸(l:&J,乘:享,亓:&mut 亚)->侲{let 丂=l.丛();let mut 久=侲::default();if l.丝().is_some(){return 久;}if let Some(万)=估(l){久.万=万;return 久;}let 乜=|傤:亪|match(乘.亨,乘.仁){(Some(亨),Some(仁))=>Some(亨+(仁-亨).mul_f64(傤)),_=>None,};if
倘.is_some_and(|丑|l.乩()>=丑){亓.end();}if 亓.傔(l){let 万=亓.丑(l,乜(0.4));let 偝=万.iter().any(|丐|{matches!(丐,L::乎{..}|L::亗{..}|L::乹{..})});let 亡=倖&&!偝&&l.乩()>=倗&&l.上(丂)>=倕;if!亡{久.亓=true;久.万=万;return
久;}亓.end();}let mut 乱=佋{丂,limit:享{亨:乜(個),..乘},亟:0,份:仮::default(),仈:None,侇:傳(l),};let mut 严=K::new();let mut 之:K<仍> =K::new();let mut 仳=K::new();let mut
伣=false;let mut 亠:K<丫> =K::new();while 久.你<値&&(之.is_empty()||!乱.二()){let 丞=if 伣{let Some(亜)=亠.pop()else{break;};match 乱.亠(l,&亜,&严){Some(丞)=>丞,None=>continue,}}else{乱.佟(l,&严,&仳,true)};久.你+=1;let
俱=之.iter().any(|p|p.万==丞.万);let 乬=丞.first;let 俷=乱.二();let mut 丞=丞;let mut 俟=false;if!俷&&!俱{let 乃=乱.侹(&丞.end);俟=傚(&乃,&mut 严);丞.両=Some(乃);}if!俱{之.push(丞);}if
俷{break;}if!俟&&!伣{match 乬{Some(乬)=>仳.push(乬),None=>{伣=true;亠=乱.仈.clone().unwrap_or_default();亠.retain(|a|!仳.contains(&了(a.万())));let 乄=|a:&丫|{乱.份.get(&了(a.万())).copied().unwrap_or(C::MIN)};亠.sort_by_key(乄);}}}}久.亠=F::from(伣);久.亟=乱.亟;久.严=严.len();久.之=之.len();let
亰=||乘.仁.is_some_and(|t|乸::now()>=t);let mut 以:K<U<C>> =vec![None;之.len()];for 东 in(0..之.len()).rev(){if 东+1<之.len()&&亰(){break;}以[东]=Some(乱.俌(&之[东],&严));}let
代=|以:&[U<C>]|{let mut 丒:U<(C,F)> =None;for(东,个)in 以.iter().enumerate().rev(){if let Some(个)=*个{if 丒.is_none_or(|(习,_)|个>习){丒=Some((个,东));}}}丒};let mut
丒=代(&以);乱.limit.亨=乘.仁;while let Some((_,东))=丒{if!偌||之[东].両.is_some()||亰(){break;}let 乃=乱.侹(&之[东].end);之[东].両=Some(乃);久.係+=1;以[东]=Some(乱.俌(&之[东],&严));丒=代(&以);}if
let Some((_,东))=丒{let mut 专=l.clone();for&丐 in&之[东].万{if 专.丿(丐){久.万.push(丐);}}}久}fn 傓(乘:&dyn Fn(乸,中)->享,mut 亓:亚,佯:impl BufRead,伴:impl Write,)->Result<(),乑>{乔(佯,伴,|仙,乾,丑,举|match
J::read(仙,乾,亐::佀,丑){Ok(l)=>{let 久=傸(&l,乘(举,丑),&mut 亓);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",丑,久.你,久.亟,久.严,久.係,久.亠,举.elapsed());偔(&久.万)}Err(什)=>{eprintln!("plan: cannot read the input: {}",什);"WAIT".to_string()}},)}const
倈:亪=45.0;const 們:亪=800.0;const 偂:亪=0.6;fn 乘(举:乸,丑:中)->享{if let Some(伀)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
享{亨:None,仁:None,伀,};}let 伹=傺().min(100.0);let 傉=if 丑==1{們}else{倈};let 乇=Duration::from_secs_f64(傉/1000.0*伹);享{亨:Some(举+乇.mul_f64(偂)),仁:Some(举+乇),伀:Z::MAX,}}const
倳:F=9;const 倱:亭=亭{丏:3,丟:1,上:1,丳:2,亣:2,伍:2,丨:6,介:1,丬:0,伜:false,乙:倴,};const 倷:中=0;const 倴:亃=亃::佂(伕::侣(7));const 侫:I=I::MAX;const 侩:C=2;const 侪:C=18;const
侨:C=0;const 倲:F=32;const 倅:O=false;const 倆:Settings=Settings{侄:3,侃:1,佝:4,亰:2,佦:2,伢:2,侊:true,reserve:true,佧:true,九:亮::乫,fill:true,乏:Y{伸:侫,仓:侩,伫:侪,伙:侨,..Y::亻},};fn
亓()->亚{if 倅{return 亚::new(亄::休(倆),K::new(),Y::亻);}傎(倲)}fn 傎(于:F)->亚{let 事=亄::仰{乔:倳,于,丰:倱,伶:倷,};let 丩=偀.iter().map(|&乏|Y{伸:侫,仓:侩,伫:侪,伙:侨,..乏}).collect();亚::new(事,丩,Y::亻)}fn
main(){let 傯=io::stdin();let 傰=io::stdout();if let Err(什)=傓(&乘,亓(),傯.lock(),傰.lock()){eprintln!("plan: {}",什);std::process::exit(1);}}const 偆:&str="CG_TIME_SCALE";fn
傺()->亪{傏(std::env::var(偆).ok().as_deref())}fn 傏(丮:U<&str>)->亪{丮.and_then(|乾|乾.trim().parse::<亪>().ok()).filter(|伹|伹.is_finite()&&*伹>0.0).unwrap_or(1.0)}const
_:()=assert!(倐==16);const 倐:F=16;const 倊:亪=0.4;const 倭:I=4;#[derive(Clone,Copy)]struct 亼{丬:G,俁:G,企:G,}impl 亼{fn of(l:&J)->亼{let(a,件)=俧(l,Q[0]);let(b,伡)=俧(l,Q[1]);let
亷=a[Q[1]];let 傁=件[Q[1]];let mut 丷=亼{丬:G::P,俁:G::P,企:G::P,};for 一 in l.map().valid.iter(){if 件[一]==I::MAX||伡[一]==I::MAX{continue;}if 件[一].abs_diff(伡[一])<=1{丷.俁.set(一);}if
件[一].abs_diff(伡[一])<=倭{丷.企.set(一);}let 傋=伐::from(件[一])+伐::from(伡[一])==伐::from(傁);if 亷>0.0&&傋&&一!=Q[0]&&一!=Q[1]&&a[一]*b[一]/亷>=倊{丷.丬.set(一);}}丷}fn 仂(&self)->G{self.丬&self.企}}fn
傲(l:&J,傭:G)->[I;M]{let 丶=l.map().valid;let mut 丆=[I::MAX;M];let mut 五=傭&丶;let mut 伯=五;let mut 业=0u8;while!伯.is_empty(){for 一 in 伯.iter(){丆[一]=业;}let 七=(伯.与()&丶).丕(五);五|=七;伯=七;业=业.saturating_add(1);}丆}fn
俧(l:&J,三:F)->([亪;M],[I;M]){let 么=傲(l,G::一(三));let mut 乙:K<F> =l.map().valid.iter().filter(|&c|么[c]!=I::MAX).collect();乙.sort_by_key(|&c|么[c]);let mut 且=[0.0;M];且[三]=1.0;for
一 in 乙{if 一==三{continue;}let 乗=G::一(一).与()&l.map().valid;且[一]=乗.iter().filter(|&n|么[n]!=I::MAX&&么[n]+1==么[一]).map(|n|且[n]).sum();}(且,么)}const T:F=12;const
M:F=T*T;const 乺:F=4*T;const 云:Z=(1<<乺)-1;const 伒:Z=1|1<<12|1<<24|1<<36;const 佁:Z=伒<<11;const 侮:Z=(1<<T)-1;#[derive(Clone,Copy,PartialEq)]struct G([Z;3]);impl
G{const P:G=G([0;3]);const 倄:G=G([云;3]);#[inline]fn 一(一:F)->G{let mut 乆=G::P;乆.set(一);乆}#[inline]fn test(self,一:F)->O{(self.0[一/乺]>>(一%乺))&1!=0}#[inline]fn
set(&mut self,一:F){self.0[一/乺]|=1<<(一%乺);}#[inline]fn clear(&mut self,一:F){self.0[一/乺]&=!(1<<(一%乺));}#[inline]fn is_empty(self)->O{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
似(self)->[Z;3]{self.0}#[inline]fn count(self)->中{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丕(self,为:G)->G{G([self.0[0]&!为.0[0],self.0[1]&!为.0[1],self.0[2]&!为.0[2],])}#[inline]fn
iter(self)->伿{伿{併:self.0,乜:0,}}#[inline]fn 傩(self)->G{let[a,b,c]=self.0;G([(a>>T)|((b&侮)<<(乺-T)),(b>>T)|((c&侮)<<(乺-T)),c>>T,])}#[inline]fn 傦(self)->G{let[a,b,c]=self.0;G([(a<<T)&云,((b<<T)&云)|(a>>(乺-T)),((c<<T)&云)|(b>>(乺-T)),])}#[inline]fn
傧(self)->G{let[a,b,c]=self.0;G([(a>>1)&!佁&云,(b>>1)&!佁&云,(c>>1)&!佁&云,])}#[inline]fn 储(self)->G{let[a,b,c]=self.0;G([(a<<1)&!伒&云,(b<<1)&!伒&云,(c<<1)&!伒&云,])}#[inline]fn
与(self)->G{let q=self;q.傩()|q.傦()|q.傧()|q.储()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,为:G)->G{G([self.0[0]&为.0[0],self.0[1]&为.0[1],self.0[2]&为.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,为:G)->G{G([self.0[0]|为.0[0],self.0[1]|为.0[1],self.0[2]|为.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,为:G){*self=*self|为;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::倄.丕(self)}}struct 伿{併:[Z;3],乜:F,}impl Iterator for 伿{type Item=F;#[inline]fn
next(&mut self)->U<F>{let q=self;while q.乜<3{let 伱=q.併[q.乜];if 伱!=0{q.併[q.乜]=伱&(伱-1);return Some(q.乜*乺+伱.trailing_zeros()as F);}q.乜+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乲{世:中,x:I,y:I},乎{丄:I,x:I,y:I},亗{x:I,y:I},乹{x:I,y:I},}impl L{fn 傽(&self)->乑{match*self{L::乲{世,x,y}=>format!("MOVE {} {x} {y}",世),L::乎{丄,x,y}=>format!("TRAIN {} {x} {y}",丄),L::亗{x,y}=>format!("BUILD MINE {x} {y}"),L::乹{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
偔(万:&[L])->乑{if 万.is_empty(){return "WAIT".to_string();}let mut line=乑::with_capacity(16*万.len());for(东,丐)in 万.iter().enumerate(){if 东>0{line.push(';');}line.push_str(&丐.傽());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
亐{侷,僎,偐,#[default]佀,}impl 亐{fn 亇(self)->I{match self{亐::侷=>1,_=>3,}}fn 会(self)->O{matches!(self,亐::偐|亐::佀)}fn 仑(self)->O{self==亐::佀}}const 亂:I=I::MAX;struct
亙{乛:G,valid:G,丱:G,丙:Box<[I]>,与:[[[I;4];M];2],}thread_local!{static 倡:RefCell<U<Arc<亙>>> =const{RefCell::new(None)};}impl 亙{fn 傥(乛:G,丱:G)->Arc<亙>{倡.with(|乭|{let
mut 乭=乭.borrow_mut();if let Some(丹)=乭.as_ref(){if 丹.乛==乛&&丹.丱==丱{return Arc::clone(丹);}}let 丹=Arc::new(亙::build(乛,丱));*乭=Some(Arc::clone(&丹));丹})}fn build(乛:G,丱:G)->亙{let
丶=!乛;const 偃:[[(伏,伏);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 与=[[[亂;4];M];2];for(丁,乓)in 偃.iter().enumerate(){for 一 in
丶.iter(){let(x,y)=((一%T)as 伏,(一/T)as 伏);let mut 且=0;for&(偫,偬)in 乓{let(俢,俣)=(x+偫,y+偬);if(0..T as 伏).contains(&俢)&&(0..T as 伏).contains(&俣){let 七=俣 as F*T+俢
as F;if 丶.test(七){与[丁][一][且]=七 as I;且+=1;}}}}}let mut 丙=vec![亂;M*M].into_boxed_slice();let mut 佾=[0u8;M];for 三 in 丶.iter(){let 买=&mut 丙[三*M..(三+1)*M];买[三]=0;佾[0]=三
as I;let(mut 佬,mut 侉)=(0,1);while 佬<侉{let 一=佾[佬]as F;佬+=1;for&七 in&与[0][一]{if 七!=亂&&买[七 as F]==亂{买[七 as F]=买[一]+1;佾[侉]=七;侉+=1;}}}}亙{乛,valid:丶,丱,丙,与,}}#[inline]fn
丙(&self,三:F,丧:F)->I{self.丙[三*M+丧]}}const 倫:中=100;const 亀:C=15;const 偊:[C;3]=[1,4,20];const Q:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 书{亅(F),侙,}enum
亁{佊,侧,侬,}#[derive(Clone)]struct J{map:Arc<亙>,临:亐,両:[G;2],丅:[G;2],乊:G,乡:G,丈:[G;2],丄:[G;3],亲:G,仩:[中;M],上:[C;2],丟:[C;2],丛:F,乩:中,仠:中,丝:U<书>,}fn 仼(両:G,付:F)->G{if!両.test(付){return
G::P;}let mut 伇=G::一(付);loop{let 七=(伇|伇.与())&両;if 七==伇{return 伇;}伇=七;}}impl J{fn 丛(&self)->F{self.丛}fn 乩(&self)->中{self.乩}fn 丝(&self)->U<书>{self.丝}fn 临(&self)->亐{self.临}fn
仞(&self,丁:F)->C{let q=self;let 丈=q.丈[丁];q.上[丁]+10*C::from((丈&q.丄[0]).count())+20*C::from((丈&q.丄[1]).count())+30*C::from((丈&q.丄[2]).count())}fn 上(&self,丁:F)->C{self.上[丁]}fn
丟(&self,丁:F)->C{self.丟[丁]}fn 侍(&self,丁:F)->F{self.丈[丁].count()as F}fn map(&self)->&亙{&self.map}fn 不(&self,丁:F)->G{self.両[丁]}fn 丅(&self,丁:F)->G{self.丅[丁]}fn
丈(&self,丁:F)->G{self.丈[丁]}fn 亩(&self,一:F)->U<(中,I)>{let q=self;(q.丈[0]|q.丈[1]).test(一).then(||(q.仩[一],q.佴(一)))}fn 乷(&self)->G{self.丈[self.丛]&self.亲}fn
丯(&self)->G{self.乊}fn 乌(&self)->G{self.乡}fn 亍(&self,一:F)->U<I>{(一<M&&(self.丈[0]|self.丈[1]).test(一)).then(||self.佴(一))}fn 傮(&mut self,丁:F){let q=self;q.丅=[仼(q.両[0],Q[0]),仼(q.両[1],Q[1])];let
健=q.丈[丁].丕(q.丅[丁]);q.侀(健);q.促();q.上[丁]+=q.丟[丁];if q.上[丁]<0{q.上[丁]=0;q.侀(q.丈[丁]);}q.亲|=q.丈[丁];}fn 促(&mut self){let q=self;for 丁 in 0..2{let 丅=q.丅[丁];let
丈=丅&q.丈[丁];let mut 丟=C::from(丅.count())+4*C::from((丅&q.乊).count());for(丄,僄)in 偊.iter().enumerate(){丟-=僄*C::from((丈&q.丄[丄]).count());}q.丟[丁]=丟;}}fn 仕(&mut
self){let q=self;if q.丝.is_some(){return;}if q.丛==1{if q.乩>=倫{q.丝=Some(match q.仞(0).cmp(&q.仞(1)){std::cmp::Ordering::Greater=>书::亅(0),std::cmp::Ordering::Less=>书::亅(1),std::cmp::Ordering::Equal=>书::侙,});return;}q.乩+=1;}q.丛=1-q.丛;q.傮(q.丛);}fn
伵(&mut self,万:&[L]){let q=self;if q.丝.is_some(){return;}let 佩=q.仠;for&丐 in 万{if q.偪(丐,佩){continue;}q.丿(丐);if q.丝.is_some(){return;}}q.仕();}fn 偪(&self,丐:L,佩:中)->O{let
q=self;match 丐{L::乲{世,x,y}=>{q.仒(x,y).is_none()||世>=佩||q.俍(q.丛,世).is_none()}L::乎{丄,x,y}=>{q.仒(x,y).is_none()||丄==0||丄>q.临.亇()}L::亗{x,y}|L::乹{x,y}=>{q.仒(x,y).is_none()}}}#[inline]fn
仒(&self,x:I,y:I)->U<F>{let(x,y)=(F::from(x),F::from(y));(x<T&&y<T&&!self.map.乛.test(y*T+x)).then_some(y*T+x)}fn 俍(&self,丁:F,世:中)->U<F>{self.丈[丁].iter().find(|&一|self.仩[一]==世)}#[inline]fn
佴(&self,一:F)->I{if self.丄[0].test(一){1}else if self.丄[1].test(一){2}else{3}}fn 偙(&self,予:F)->G{let 乌=self.乡&self.両[予];let 丅=self.丅[予];乌|(丅&(丅&乌).与())}fn
乂(&self,丁:F,丄:I)->G{let q=self;debug_assert!((1..=3).contains(&丄),"unit level {} is not 1 to 3",丄);let 予=1-丁;let 丈=q.丈[0]|q.丈[1];let 偭=q.亿();let 僁=match
丄{1 if q.临==亐::侷=>q.丄[1]|q.丄[2],1=>丈,2=>q.丄[1]|q.丄[2],_=>G::P,};let 乂=偭|q.両[予].丕(僁);if 丄<3{乂.丕(q.偙(予))}else{乂}}fn 亳(&self,丁:F)->G{self.丅[丁]|self.丅[丁].与()}fn
亿(&self)->G{let q=self;let 丈=q.丈[0]|q.丈[1];let 偽=G::一(Q[0])|G::一(Q[1]);q.map.valid.丕(丈|q.乊|q.乡|偽)}fn 丿(&mut self,丐:L)->O{let q=self;if q.丝.is_some(){return
false;}let 丁=q.丛;let 亽=match 丐{L::乎{丄,x,y}=>q.傿(丁,丄,x,y),L::乲{世,x,y}=>q.傈(丁,世,x,y),L::亗{x,y}=>q.build(丁,true,x,y),L::乹{x,y}=>q.build(丁,false,x,y),};let
Some(亽)=亽 else{return false;};match 亽{亁::佊=>{}亁::侧=>q.丅[丁]=仼(q.両[丁],Q[丁]),亁::侬=>{q.丅=[仼(q.両[0],Q[0]),仼(q.両[1],Q[1])];}}q.促();if!q.両[0].test(Q[0]){q.丝=Some(书::亅(1));}else
if!q.両[1].test(Q[1]){q.丝=Some(书::亅(0));}true}fn 侀(&mut self,丏:G){let q=self;q.丈[0]=q.丈[0].丕(丏);q.丈[1]=q.丈[1].丕(丏);for 丄 in&mut q.丄{*丄=丄.丕(丏);}q.亲=q.亲.丕(丏);}fn
俻(&mut self,丁:F,一:F)->亁{let q=self;q.侀(G::一(一));q.乊.clear(一);q.乡.clear(一);let 予=1-丁;if q.両[予].test(一){q.両[予].clear(一);q.両[丁].set(一);亁::侬}else if q.両[丁].test(一){亁::佊}else{q.両[丁].set(一);亁::侧}}fn
傿(&mut self,丁:F,丄:I,x:I,y:I)->U<亁>{let q=self;if 丄==0||丄>q.临.亇(){return None;}let 一=q.仒(x,y)?;let 下=10*C::from(丄);if q.上[丁]<下||!q.亳(丁).test(一)||!q.乂(丁,丄).test(一){return
None;}let 亽=q.俻(丁,一);q.俨(丁,丄,一,q.仠);q.仠=q.仠.saturating_add(1);q.上[丁]-=下;Some(亽)}fn 俨(&mut self,丁:F,丄:I,一:F,世:中){self.丈[丁].set(一);self.丄[F::from(丄)-1].set(一);self.仩[一]=世;}fn
傈(&mut self,丁:F,世:中,x:I,y:I)->U<亁>{let q=self;let 丌=q.仒(x,y)?;let 三=q.俍(丁,世)?;if!q.亲.test(三){return None;}let 丄=q.佴(三);let 丢=q.乂(丁,丄);let 乗=q.map.与[丁][三];if
乗.contains(&(丌 as I))&&!丢.test(丌){return None;}let mut 丒=q.map.丙(三,丌);let mut 仸=None;for&七 in 乗.iter().take_while(|&&七|七!=亂){let 七=F::from(七);let 丙=q.map.丙(七,丌);if
丙<丒&&丢.test(七){丒=丙;仸=Some(七);}}let 仸=仸?;let 亽=q.俻(丁,仸);q.丈[丁].clear(三);q.丄[F::from(丄)-1].clear(三);q.亲.clear(三);q.俨(丁,丄,仸,世);Some(亽)}fn build(&mut self,丁:F,乊:O,x:I,y:I)->U<亁>{let
q=self;let 体=if 乊{q.临.会()}else{q.临.仑()};let 一=q.仒(x,y)?;let 傌=q.map.丱.test(一);if!体||q.上[丁]<亀||傌!=乊||!q.両[丁].test(一)||!q.亳(丁).test(一)||!q.亿().test(一){return
None;}if 乊{q.乊.set(一);}else{q.乡.set(一);}q.上[丁]-=亀;Some(亁::佊)}}const 倦:C=1000000000;const 但:C=0;const 倮:C=1;const 假:C=2;impl J{fn read(仙:&str,丑:&str,临:亐,乩:中,)->Result<J,乑>{let
mut 丱=K::new();let mut 丸=仙.lines();let 且=佢(&mut 丸,"mine spot count")?;for _ in 0..且{let[x,y]=亦(仡(&mut 丸,"a mine spot")?,"a mine spot")?;丱.push((伟(x)?,伟(y)?));}俊(丸,"the initial input")?;let
mut 丸=丑.lines();let mut 佭=[0i64;4];for(丮,乕)in 佭.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[佼]=亦(仡(&mut 丸,乕)?,乕)?;if 佼.abs()>倦{return
Err(format!("{}: {} is out of range",乕,佼));}*丮=佼;}let mut 乛=G::P;let mut 丏=[G::P;2];let mut 丅=[G::P;2];for y in 0..T{let 买=仡(&mut 丸,"a map row")?.trim();if
买.len()!=T{return Err(format!("map row {y} is not {} characters: {:?}",T,买));}for(x,伊)in 买.bytes().enumerate(){let 一=y*T+x;match 伊{b'#'=>乛.set(一),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乣=F::from(伊==b'X'||伊==b'x');丏[乣].set(一);if 伊.is_ascii_uppercase(){丅[乣].set(一);}}_=>{return Err(format!("map row {y} has {:?} at {x}",伊 as char))}}}}let
停=佢(&mut 丸,"building count")?;let mut 乴=K::new();for _ in 0..停{let 亦=亦(仡(&mut 丸,"a building")?,"a building")?;乴.push(亦);}let 侍=佢(&mut 丸,"unit count")?;let
mut 丈=K::new();for _ in 0..侍{let 亦=亦(仡(&mut 丸,"a unit")?,"a unit")?;丈.push(亦);}俊(丸,"the turn's input")?;let 俥=乴.iter().find(|侾|侾[0]==0&&侾[1]==但).ok_or("the bot's own HQ is not among the buildings")?;let
俖=余(俥[2],俥[3],乛)?;let 丁=Q.iter().position(|&付|付==俖).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",俖))?;let mut 俳=G::P;for(x,y)in 丱{if
乛.test(y*T+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}俳.set(y*T+x);}let 佑=|乣:F|if 乣==0{丁}else{1-丁};let mut l=J{map:亙::傥(乛,俳),临,両:[G::P;2],丅:[G::P;2],乊:G::P,乡:G::P,丈:[G::P;2],丄:[G::P;3],亲:G::P,仩:[0;M],上:[0;2],丟:[0;2],丛:丁,乩,仠:1,丝:None,};for
乣 in 0..2{let 亊=佑(乣);l.両[亊]=丏[乣];l.丅[亊]=丅[乣];l.上[亊]=佭[2*乣];l.丟[亊]=佭[2*乣+1];}let mut 俤=G::P;for[亊,伭,x,y]in 乴{let 乣=修(亊)?;let 一=余(x,y,乛)?;if!丏[乣].test(一){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 俤.test(一){return Err(format!("two buildings at ({x}, {y})"));}俤.set(一);match 伭{但
if 一==Q[佑(乣)]=>{}但=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}倮=>l.乊.set(一),假=>l.乡.set(一),_=>return Err(format!("unknown building type {}",伭)),}}let
mut 乤=G::P;let mut 伮=0u32;for[亊,世,丄,x,y]in 丈{let 乣=修(亊)?;let 一=余(x,y,乛)?;if!丏[乣].test(一){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乤.test(一){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丄){return Err(format!("unit level {} is not 1, 2 or 3",丄));}let 世=中::try_from(世).map_err(|_|format!("unit id {} is out of range",世))?;if
乤.iter().any(|为|l.仩[为]==世){return Err(format!("two units have id {}",世));}乤.set(一);l.丈[佑(乣)].set(一);l.丄[丄 as F-1].set(一);l.仩[一]=世;伮=伮.max(世);}if!乤.is_empty(){l.仠=伮.checked_add(1).ok_or_else(||format!("unit id {} is out of range",伮))?;}l.亲=l.丈[丁];Ok(l)}}fn
仡<'a>(丸:&mut Lines<'a>,乕:&str)->Result<&'a str,乑>{丸.next().ok_or_else(||format!("the text ends before {}",乕))}fn 俊(mut 丸:Lines<'_>,乕:&str)->Result<(),乑>{match
丸.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乕)),None=>Ok(()),}}fn 亦<const N:F>(line:&str,乕:&str)->Result<[C;N],乑>{let
mut 似=line.split_whitespace();let mut 倀=[0i64;N];for 丮 in&mut 倀{let 仫=似.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乕))?;*丮=仫.parse().map_err(|_|format!("{}: {:?} is not an integer",乕,仫))?;}if
似.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乕));}Ok(倀)}fn 佢(丸:&mut Lines<'_>,乕:&str)->Result<F,乑>{let[且]=亦(仡(丸,乕)?,乕)?;F::try_from(且).ok().filter(|&且|且<=M).ok_or_else(||format!("{}: {} is out of range",乕,且))}fn
伟(丮:C)->Result<F,乑>{F::try_from(丮).ok().filter(|&丮|丮<T).ok_or_else(||format!("coordinate {} is off the map",丮))}fn 余(x:C,y:C,乛:G)->Result<F,乑>{let 一=伟(y)?*T+伟(x)?;if
乛.test(一){return Err(format!("({x}, {y}) is a void cell"));}Ok(一)}fn 修(亊:C)->Result<F,乑>{match 亊{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亊)),}}#[derive(Clone,Copy,PartialEq)]enum
亮{乫,侓,}#[derive(Clone,Copy)]struct Settings{侄:I,侃:I,佝:C,亰:C,佦:F,伢:I,侊:O,reserve:O,佧:O,九:亮,fill:O,乏:Y,}impl Settings{const DEFAULT:Settings=Settings{侄:3,侃:1,佝:4,亰:2,佦:2,伢:2,侊:true,reserve:true,佧:true,九:亮::乫,fill:true,乏:Y::亻,};}const
住:C=1000;const 低:C=10;fn 侐(一:F)->(I,I){((一%T)as I,(一/T)as I)}fn 佽(l:&mut J,丐:L,丆:&mut Vec<L>)->O{let 仱=l.丿(丐);if 仱{丆.push(丐);}仱}fn 业(l:&mut J,三:F,丧:F,丆:&mut
Vec<L>)->O{let Some((世,_))=l.亩(三)else{return false;};let(x,y)=侐(丧);佽(l,L::乲{世,x,y},丆)}fn 傷(l:&J,丷:&亼,丽:&Settings,乘:F,)->K<F>{let 丂=l.丛();let 丠=主(l,l.不(丂));let
丘=主(l,l.不(1-丂));let 仅=l.map().valid.丕(l.不(0)|l.不(1));let 俬=|c:F|丠[c]!=I::MAX&&丘[c]!=I::MAX;let mut 丬:K<F> =丷.仂().丕(l.不(丂)).iter().filter(|&c|俬(c)).collect();丬.sort_by_key(|&c|(丠[c],c));let
mut 丼:K<F> =仅.iter().filter(|&c|俬(c)&&丠[c].abs_diff(丘[c])<=丽.侃).collect();丼.sort_by_key(|&c|(丠[c].abs_diff(丘[c]),丠[c],c));let mut 丆:K<F> =K::new();for
一 in 丬.into_iter().chain(丼){if 丆.len()>=乘{break;}let 偲=丆.iter().all(|&t|l.map().丙(t,一)>=丽.侄);let 偡=丷.仂().test(一);if!丆.contains(&一)&&(偡||偲){丆.push(一);}}丆}fn
偾(下:&[K<C>])->K<F>{let n=下.len();let 俙=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 倂=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亢=0usize;let mut 伂=vec![俙;n+1];let mut 侎=vec![false;n+1];loop{侎[亢]=true;let 俘=p[亢];let mut 价=俙;let mut 仾=0usize;for j in 1..=n{if!侎[j]{let
乵=下[俘-1][j-1]-u[俘]-v[j];if 乵<伂[j]{伂[j]=乵;倂[j]=亢;}if 伂[j]<价{价=伂[j];仾=j;}}}for j in 0..=n{if 侎[j]{u[p[j]]+=价;v[j]-=价;}else{伂[j]-=价;}}亢=仾;if p[亢]==0{break;}}loop{let
仾=倂[亢];p[亢]=p[仾];亢=仾;if 亢==0{break;}}}let mut 丆=vec![0usize;n];for j in 1..=n{if p[j]>0{丆[p[j]-1]=j-1;}}丆}#[derive(Clone,Copy)]enum 亹{佐(F),位,}fn 乿(l:&J)->G{let
丂=l.丛();(l.亳(丂)&l.乂(丂,1)).丕(l.不(丂))}fn 傑(l:&J,乏:&Y)->F{伻(l,乏,乍::乫).iter().filter(|c|matches!(c,L::乎{..})).count()}fn 並(一:F,丣:G)->中{(G::一(一).与()&丣).count()}fn
俐(l:&J,乄:G,丌:F,佲:I)->K<[C;M]>{let 丶=l.map().valid;let mut 丒=[C::MIN;M];丒[丌]=0;let mut 也=vec![丒];for _ in 0..佲{let mut 七=丒;for 一 in 丶.iter(){for n in(G::一(一).与()&丶).iter(){if
丒[n]>C::MIN{let 丮=丒[n]+C::from(乄.test(n));if 丮>七[一]{七[一]=丮;}}}}丒=七;也.push(丒);}也}fn 俦(l:&J,也:&[[C;M]],乄:G,三:F,k:F,)->G{let 丶=l.map().valid;let mut 丆=G::P;let
mut 一=三;let mut k=k.min(也.len()-1);while k>0&&也[k][一]>0{let Some(七)=(G::一(一).与()&丶).iter().filter(|&n|也[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(乄.test(n))+也[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乄.test(七){丆.set(七);}一=七;k-=1;}丆}fn 丑(l:&J,丽:&Settings)->K<L>{let 丂=l.丛();let 丷=亼::of(l);let mut 专=l.clone();let mut 丆=K::new();let 丘=主(l,l.不(1-丂));let
丈:K<F> =l.乷().iter().collect();let 俒=(l.上(丂)/低).max(0)as F;let 伺=match 丽.九{亮::乫=>傑(l,&丽.乏).min(俒),亮::侓=>俒,};let mut 仲:K<亹> =丈.iter().map(|&c|亹::佐(c)).collect();仲.extend(std::iter::repeat_n(亹::位,伺));let
乶=傷(l,&丷,丽,仲.len()+丽.佦);let 僀=乿(l);let 丬=丷.仂();let n=仲.len().max(乶.len());let mut 下=vec![vec![0i64;n];n];for(买,&丌)in 乶.iter().enumerate(){for(仵,仉)in 下[买].iter_mut().enumerate(){let
Some(&偓)=仲.get(仵)else{*仉=住;continue;};let 伛=match 偓{亹::佐(一)=>{let d=l.map().丙(一,丌);if d==I::MAX{None}else{Some(C::from(d)-1)}}亹::位=>僀.iter().map(|c|l.map().丙(c,丌)).filter(|&d|d!=I::MAX).min().map(C::from),};*仉=match
伛{None=>住,Some(伛)=>{let 义=C::from(丘[丌])-1;let 亰=丽.亰*(伛-义).max(0);let 佖=if 丬.test(丌){丽.佝}else{0};(伛+亰-佖).max(0)}};}}let 偕=偾(&下);let mut 佪:K<(F,F)> =K::new();let
mut 侌:K<(C,F)> =K::new();for(买,&丌)in 乶.iter().enumerate(){let 仵=偕[买];if 下[买][仵]>=住{continue;}match 仲.get(仵){Some(亹::佐(一))=>佪.push((*一,丌)),Some(亹::位)=>侌.push((下[买][仵],丌)),None=>{}}}let
mut 丣=l.map().valid.丕(l.不(0)|l.不(1));let mut 俠:K<F> =K::new();let mut 伷=G::P;let 争=|一:F|{let 丄=l.亍(一).unwrap_or(1);let 伋=丣|l.不(1-丂);(G::一(一).与()&伋&l.乂(丂,丄)).count()};佪.sort_by_key(|&(一,丌)|{let
偳=if 丽.佧{争(一)}else{0};(偳,l.map().丙(一,丌),一)});for&(三,丌)in&佪{let Some(丄)=专.亍(三)else{continue;};let 丢=专.乂(丂,丄);let d=专.map().丙(三,丌);let 乄=(丣|专.不(1-丂)).丕(伷);let
佲=d.saturating_add(丽.伢);let 也=俐(&专,乄,丌,佲.saturating_sub(1));let 仐=也[也.len()-1];let 七=(G::一(三).与()&丢).iter().filter(|&c|仐[c]>C::MIN).max_by_key(|&c|{let
偣=d.saturating_sub(专.map().丙(c,丌));let 伋=丣.test(c)||专.不(1-丂).test(c);(伋&&丽.侊,C::from(乄.test(c))+仐[c],偣,並(c,丣),std::cmp::Reverse(c),)}).filter(|&c|{丣.test(c)||专.不(1-丂).test(c)||专.map().丙(c,丌)<d});if
let Some(丧)=七{if 业(&mut 专,三,丧,&mut 丆){丣.clear(丧);俠.push(三);if 丽.reserve{伷|=俦(&专,&也,乄,丧,也.len()-1);}}}}let 丠=主(l,l.不(丂));let mut 乐=G::P;for c in 丣.iter(){if
丠[c]<=丘[c]{乐.set(c);}}for&三 in&丈{if 俠.contains(&三)||!专.乷().test(三){continue;}let Some(丄)=专.亍(三)else{continue;};let 丢=专.乂(丂,丄);let 伋=(G::一(三).与()&丣&丢).iter().max_by_key(|&c|(並(c,丣),丘[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丧)=伋{if 业(&mut 专,三,丧,&mut 丆){丣.clear(丧);}continue;}let 两=[乐&丣,丣].into_iter().find_map(|丏|{丏.iter().min_by_key(|&c|(专.map().丙(三,c),c))});if let
Some(两)=两{let d=专.map().丙(三,两);let 七=(G::一(三).与()&丢).iter().filter(|&c|专.map().丙(c,两)<d).min_by_key(|&c|c);if let Some(丧)=七{业(&mut 专,三,丧,&mut 丆);}}}侌.sort();let
mut 佷=0usize;for&(_,丌)in&侌{if 专.上(丂)<低{break;}let 丏=乿(&专);let Some(亥)=丏.iter().map(|c|专.map().丙(c,丌)).min()else{break;};let 乄=(丣|专.不(1-丂)).丕(伷);let 也=俐(&专,乄,丌,亥.saturating_add(丽.伢));let
仐=也[也.len()-1];let 一=丏.iter().filter(|&c|专.map().丙(c,丌)==亥).max_by_key(|&c|(仐[c],並(c,丣),std::cmp::Reverse(c)));let Some(一)=一 else{break;};let(x,y)=侐(一);if
佽(&mut 专,L::乎{丄:1,x,y},&mut 丆){丣.clear(一);佷+=1;if 丽.reserve{伷|=俦(&专,&也,乄,一,也.len()-1);}}}if 丽.fill&&丽.九==亮::乫{while 佷<伺&&专.上(丂)>=低{let 一=(乿(&专)&丣).iter().max_by_key(|&c|{(並(c,丣),std::cmp::Reverse(丘[c]),std::cmp::Reverse(c),)});let
Some(一)=一 else{break;};let(x,y)=侐(一);if!佽(&mut 专,L::乎{丄:1,x,y},&mut 丆){break;}丣.clear(一);佷+=1;}}丆}const 倚:C=15;const 倯:C=6;const 侳:C=10;const 倿:I=3;#[derive(Clone,Copy)]struct
Y{open:C,伞:C,俆:I,佮:C,伫:C,俜:C,伸:I,俅:C,俄:C,伙:C,仗:C,仧:C,乚:C,仚:C,亸:C,仓:C,}impl Y{const 亻:Y=Y{open:3,伞:2,俆:12,佮:4,伫:倚,俜:倯,伸:倿,俅:2,俄:2,伙:0,仗:0,仧:0,乚:0,仚:0,亸:0,仓:0,};const
fn 僌(self,仗:C,仧:C,乚:C)->Y{Y{仗,仧,乚,..self}}const fn 僋(self,仚:C,亸:C)->Y{Y{仚,亸,..self}}}fn 伎(一:F)->(I,I){((一%T)as I,(一/T)as I)}fn 主(l:&J,三:G)->[I;M]{let mut
么=[I::MAX;M];let 丶=l.map().valid;let mut 丼=三&丶;let mut 乤=丼;let mut d=0u8;while!丼.is_empty(){for 一 in 丼.iter(){么[一]=d;}let 七=(丼.与()&丶).丕(乤);乤|=七;丼=七;d=d.saturating_add(1);}么}fn
偶(l:&J)->I{let 么=主(l,l.不(0));l.不(1).iter().map(|一|么[一]).min().unwrap_or(I::MAX)}struct 亯<'a>{乏:&'a Y,丂:F,theirs:[I;M],仨:[I;M],free:G,乐:G,ours:G,丱:G,丈:G,}impl
亯<'_>{fn new<'a>(l:&J,乏:&'a Y)->亯<'a>{let 丂=l.丛();let 乊=主(l,l.不(丂));let 丘=主(l,l.不(1-丂));let 仨=主(l,G::一(Q[1-丂]));let 丣=l.map().valid.丕(l.不(0)|l.不(1));let
mut 乐=G::P;for 一 in 丣.iter(){if 乊[一]<=丘[一]{乐.set(一);}}亯{乏,丂,theirs:丘,仨,free:丣,乐,ours:l.不(丂),丱:l.map().丱,丈:l.丈(丂),}}fn value(&self,一:F)->C{let q=self;let
p=q.乏;let 並=C::from((G::一(一).与()&q.free).count());let 俀=p.俆;let 伞=C::from(俀)-C::from(q.theirs[一].min(俀));let 付=if p.佮>0{C::from(q.仨[一].min(30))/p.佮}else{0};let
両=C::from((G::一(一).与()&q.ours).count());let line=if 両<=1{p.仧}else{0};let 乚=if q.丱.test(一){p.乚}else{0};let 債=if 並==0{p.仚}else{0};let 亸=p.亸*(22-C::from(q.仨[一].min(22)));let
仓=p.仓*C::from((G::一(一).与()&q.丈).count());p.open*並+p.伞*伞-付+p.仗*両-line+乚-債+亸-仓}fn 丒(&self,丏:G)->U<F>{丏.iter().max_by_key(|&一|(self.value(一),std::cmp::Reverse(一)))}fn
佞(&mut self,一:F){let q=self;q.free.clear(一);q.乐.clear(一);q.ours.set(一);q.丈.set(一);}}fn 伅(l:&mut J,丐:L,丆:&mut Vec<L>)->O{let 仱=l.丿(丐);if 仱{丆.push(丐);}仱}fn
俶(l:&mut J,三:F,丧:F,丆:&mut Vec<L>)->O{let Some((世,_))=l.亩(三)else{return false;};let(x,y)=伎(丧);伅(l,L::乲{世,x,y},丆)}fn 僃(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-侳}fn
傞(l:&J,乏:&Y)->K<L>{伻(l,乏,乍::乫)}#[derive(Clone,Copy,PartialEq)]enum 乍{乫,亘{丯:I,丈:I},}fn 伻(l:&J,乏:&Y,九:乍)->K<L>{let mut 专=l.clone();let mut 丑=亯::new(l,乏);let
mut 丆=K::new();交(&mut 专,&mut 丑,&mut 丆);match 九{乍::乫=>傗(&mut 专,&mut 丑,&mut 丆),乍::亘{丯,丈}=>{偰(&mut 专,&mut 丑,丯,丈,&mut 丆)}}丆}fn 交(l:&mut J,丑:&mut 亯<'_>,丆:&mut
Vec<L>){let 丂=丑.丂;let mut 亡:K<F> =K::new();loop{let 丢=l.乂(丂,1);let mut 代:U<(中,F)> =None;for 三 in l.乷().iter(){if 亡.contains(&三){continue;}let 争=(G::一(三).与()&丑.free&丢).count();if
争==0{亡.push(三);}else if 代.is_none_or(|(n,_)|争<n){代=Some((争,三));}}let Some((_,三))=代 else{break;};丑.丈.clear(三);match 丑.丒(G::一(三).与()&丑.free&丢){Some(丧)if
俶(l,三,丧,丆)=>丑.佞(丧),_=>{丑.丈.set(三);亡.push(三);}}}for 三 in 亡{if!l.乷().test(三){continue;}let 丌=[丑.乐,丑.free].into_iter().find_map(|丏|{丏.iter().min_by_key(|&一|(l.map().丙(三,一),一))});if
let Some(丌)=丌{俶(l,三,丌,丆);}}}fn 傗(l:&mut J,丑:&mut 亯<'_>,丆:&mut Vec<L>){let 丂=丑.丂;let p=丑.乏;let h=(p.伫-C::from(l.乩())).max(p.俜);let 傻=(C::from(偶(l))-p.俅).max(0)/p.俄.max(1)+p.伙;loop{let
上=l.上(丂);let 丈=l.侍(丂)as C;let 傝=C::from(丑.乐.count());let 一=丑.丒(l.亳(丂)&丑.free&l.乂(丂,1));let 俿=僃(h,(傝/(丈+1)).min(傻));let 便=一.is_some()&&上>=侳&&俿>0;let 保=4*h-亀;let
乚=俞(l,丑);let 偟=乚.is_some()&&上>=亀&&保>0;if 偟&&(!便||保>=俿){let(x,y)=伎(乚.unwrap_or_default());if 伅(l,L::亗{x,y},丆){continue;}break;}let Some(一)=一.filter(|_|便)else{break;};let(x,y)=伎(一);if!伅(l,L::乎{丄:1,x,y},丆){break;}丑.佞(一);}}fn
俞(l:&J,丑:&亯<'_>)->U<F>{let 丂=丑.丂;(l.map().丱&l.丅(丂)&l.亿()).iter().filter(|&乚|丑.theirs[乚]>丑.乏.伸).min_by_key(|&乚|(l.map().丙(Q[丂],乚),乚))}fn 偰(l:&mut J,丑:&mut
亯<'_>,丯:I,丈:I,丆:&mut Vec<L>,){let 丂=丑.丂;for _ in 0..丯{let Some(乚)=俞(l,丑)else{break;};let(x,y)=伎(乚);if!伅(l,L::亗{x,y},丆){break;}}for _ in 0..丈{let Some(一)=丑.丒(l.亳(丂)&丑.free&l.乂(丂,1))else{break;};let(x,y)=伎(一);if!伅(l,L::乎{丄:1,x,y},丆){break;}丑.佞(一);}}fn
五(l:&J)->O{if l.丝().is_some(){return false;}let 丂=l.丛();let 上=l.上(丂);let 丨=从::new(l,丂,true);l.不(1-丂).iter().any(|一|丨.下[一]<=上)}#[derive(Clone,Copy)]struct
串{乪:I,九:乍,}const 侑:I=I::MAX;const 倉:[乍;8]=[乍::乫,乍::亘{丯:0,丈:0},乍::亘{丯:0,丈:1},乍::亘{丯:0,丈:2},乍::亘{丯:0,丈:侑,},乍::亘{丯:1,丈:0},乍::亘{丯:1,丈:1},乍::亘{丯:1,丈:侑,},];struct
佃<'a>{丩:&'a[Y],亱:Y,rest:串,亞:U<乸>,丽:Settings1,}impl 佃<'_>{fn 乏(&self,业:串)->&Y{&self.丩[F::from(业.乪)%self.丩.len()]}fn 万(&self,l:&J,业:串)->K<L>{伻(l,self.乏(业),业.九)}}fn
偑(l:&J,万:&[L],亱:&Y)->(J,O){let mut 七=l.clone();七.伵(万);if 七.丝().is_some()||五(&七){return(七,true);}let 乯=傞(&七,亱);七.伵(&乯);let 产=七.丝().is_some()||五(&七)||七.乩()>侗;(七,产)}fn
傖(l:&J,丞:&[串],亾:&佃<'_>)->K<Z>{let 丂=l.丛();let mut 仿=vec![伉(l,丂)];let mut 乥=l.clone();for 丑 in 0..侗 as F{let 业=丞.get(丑).copied().unwrap_or(亾.rest);let 万=亾.万(&乥,业);let(七,产)=偑(&乥,&万,&亾.亱);if
产{break;}乥=七;仿.push(伉(&乥,丂));}仿}fn 伉(l:&J,丂:F)->Z{let mut hash:Z=0xcbf29ce484222325;let mut 佸=|仫:Z|hash=(hash^仫).wrapping_mul(0x0100000001b3);for(傶,乆)in[l.不(丂),l.丈(丂),l.丯()].into_iter().enumerate(){for
一 in 乆.iter(){佸(((傶 as Z)<<8)|一 as Z);}佸(0xff);}佸(l.上(丂)as Z);hash}enum 亄{仰{乔:F,于:F,丰:亭,伶:中,},休(Settings),}struct 亚{事:亄,丩:K<Y>,亱:Y,丞:K<串>,伤:K<Z>,仆:F,之:中,产:O,}impl
亚{fn new(事:亄,丩:K<Y>,亱:Y)->亚{亚{事,丩,亱,丞:K::new(),伤:K::new(),仆:0,之:0,产:false,}}fn 傔(&mut self,l:&J)->O{let q=self;let 傫=matches!(q.事,亄::仰{..}|亄::休(_));let
傕=!傫&&q.之>0&&q.仆>=q.伤.len();if!q.产&&(傕||五(l)){q.产=true;}!q.产}fn end(&mut self){self.产=true;}fn 丑(&mut self,l:&J,亞:U<乸>)->K<L>{let q=self;let 亾=佃{丩:&q.丩,亱:q.亱,rest:串{乪:0,九:乍::乫,},亞,丽:Settings1::DEFAULT,};if
let 亄::休(丽)=q.事{q.之+=1;q.仆+=1;return 丑(l,&丽);}let 丂=l.丛();let 傍=q.伤.get(q.仆)==Some(&伉(l,丂));let 偯=matches!(q.事,亄::仰{伶,..}if 伶>0&&l.乩()>=伶);if!傍||偯{let
乬=q.之==0;q.丞=match q.事{亄::仰{乔,于,丰,..}=>{let 丰=&丰;let 乽=债;let mut 乓=丞(l,&q.丩,乽,乔,于,亞,丰,);乓.resize(乔,乽);乓}亄::休(_)=>K::new(),};q.伤=match q.事{亄::仰{..}=>催(l,&q.丩,&q.丞),_=>傖(l,&q.丞,&亾),};q.仆=0;q.之+=1;}let
业=q.丞.get(q.仆).copied().unwrap_or(亾.rest);q.仆+=1;亾.万(l,业)}}fn 催(l:&J,丩:&[Y],丞:&[串])->K<Z>{let 丂=l.丛();let mut 仿=K::with_capacity(丞.len());let mut 乥=l.clone();for&业
in 丞{if 乥.丝().is_some(){break;}仿.push(伉(&乥,丂));乥=佒(&乥,&万(&乥,丩,业));}仿}const 侗:中=20;const 偈:C=3;#[derive(Clone,Copy)]struct Settings1{乔:C,}impl Settings1{const
DEFAULT:Settings1=Settings1{乔:偈};}fn 乓(l:&J,三:G)->[I;M]{let 丶=l.map().valid;let mut 么=[I::MAX;M];let mut 丼=三&丶;let mut 乤=丼;let mut d=0u8;while!丼.is_empty(){for
一 in 丼.iter(){么[一]=d;}let 七=(丼.与()&丶).丕(乤);乤|=七;丼=七;d=d.saturating_add(1);}么}fn 僆(l:&J,丁:F)->C{let 丠=乓(l,l.丅(丁));let 丘=乓(l,l.丅(1-丁));l.map().valid.iter().map(|一|match
丠[一].cmp(&丘[一]){_ if 丠[一]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 偀:[Y;27]=丩();const
债:串=串{乪:16,九:乍::乫,};const fn 丩()->[Y;27]{let 仗=[0,2,4];let 仧=[0,3,6];let 亸=[0,1,2];let mut 丆=[Y::亻;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
仚=if g>0||t>0{4}else{0};丆[i]=Y::亻.僌(仗[g],仧[t],0).僋(仚,亸[w]);i+=1;}丆}#[derive(Default)]struct 佌{丏:C,丟:C,上:C,亣:C,丳:C,伍:C,丨:C,介:C,}#[derive(Clone,Copy)]struct
亭{丏:C,丟:C,上:C,丳:C,亣:C,伍:C,丨:C,介:C,丬:C,伜:O,乙:亃,}#[derive(Clone,Copy,PartialEq)]enum 亃{侢,佂(伕),亏(亏),侖,}fn 傼(l:&J,丁:F)->[I;M]{let 丠=主(l,G::一(Q[丁]));let 丘=主(l,G::一(Q[1-丁]));let
mut 互=G::P;let mut 乐=G::P;for c in l.map().valid.iter(){if 丠[c]==I::MAX||丘[c]==I::MAX{continue;}if 丠[c]==丘[c]{互.set(c);}if 丠[c]<=丘[c]{乐.set(c);}}let d=主(l,互);let
mut 丆=[I::MAX;M];for c in 乐.iter(){丆[c]=d[c];}丆}fn 偘(l:&J,丁:F,互:&[I;M])->C{let d=主(l,l.丅(丁));(0..M).filter(|&c|互[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&互[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 亏{倌{习:C},侯{习:C,仌:C},倶,}impl
亏{fn 侏(self,a:C,b:C,d:C)->C{let 仇=(a-b).abs()/2;match self{亏::倌{习}=>(习-仇-(a+b-d)/2).max(0),亏::侯{习,仌}if a<=仌=>(习-仇).max(0),亏::侯{..}=>0,亏::倶=>a.min(d/2),}}}fn
側(l:&J,丁:F,佨:亏)->[C;M]{let 丠=主(l,G::一(Q[丁]));let 丘=主(l,G::一(Q[1-丁]));let d=C::from(丠[Q[1-丁]]);let mut 丆=[0;M];for c in 0..M{if 丠[c]!=I::MAX&&丘[c]!=I::MAX{丆[c]=佨.侏(C::from(丠[c]),C::from(丘[c]),d);}}丆}#[derive(Clone,Copy,PartialEq)]enum
伕{侣(C),}impl 伕{fn 侏(self,仇:C)->C{match self{伕::侣(习)=>(习-仇).max(0),}}}type 伖=[C;4];fn 傡(l:&J,丁:F)->[C;M]{struct 仭{丅:G,乧:[伐;M],乒:[伐;M],size:[C;M],丳:[C;M],亶:伐,}fn
今(d:&mut 仭,v:F){d.亶+=1;d.乧[v]=d.亶;d.乒[v]=d.亶;d.size[v]=1;for w in(G::一(v).与()&d.丅).iter(){if d.乧[w]==0{今(d,w);d.size[v]+=d.size[w];d.乒[v]=d.乒[v].min(d.乒[w]);if
d.乒[w]>=d.乧[v]{d.丳[v]+=d.size[w];}}else{d.乒[v]=d.乒[v].min(d.乧[w]);}}}let mut d=仭{丅:l.丅(丁),乧:[0;M],乒:[0;M],size:[0;M],丳:[0;M],亶:0,};let 井=Q[丁];if d.丅.test(井){今(&mut
d,井);}d.丳[井]=0;d.丳}fn 傣(l:&J,丁:F)->佌{let 丅=l.丅(丁);let 丳=傡(l,丁);let 仨=主(l,G::一(Q[1-丁]));let mut 丆=佌{丏:C::from(丅.count()),丟:l.丟(丁),上:l.上(丁),丨:C::MAX,介:僆(l,丁)/2,..佌::default()};for
一 in 丅.iter(){if 丳[一]>0{丆.亣+=1;丆.丳=丆.丳.max(丳[一]);}if 一!=Q[丁]&&(G::一(一).与()&丅).count()<=1{丆.伍+=1;}丆.丨=丆.丨.min(C::from(仨[一]));}丆}fn 僅(l:&J,丁:F,丰:&亭,企:G)->C{let
偼=C::from((l.不(丁)&企).count());let s=傣(l,丁);let o=丰;o.丏*s.丏+o.丟*s.丟+o.上*s.上-o.丳*s.丳-o.亣*s.亣-o.伍*s.伍-o.丨*s.丨.min(30)+o.介*s.介+o.丬*偼}fn 仂(l:&J)->G{亼::of(l).仂()}fn
傆(l:&J)->G{let 丠=主(l,G::一(Q[0]));let 丘=主(l,G::一(Q[1]));let mut 乀=G::P;for c in l.map().valid.iter(){if 丠[c]!=I::MAX&&丘[c]!=I::MAX&&丠[c].abs_diff(丘[c])<=2{乀.set(c);}}乀}fn
令(l:&J)->[C;M]{let a=主(l,G::一(Q[0]));let b=主(l,G::一(Q[1]));let mut 丆=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丆[c]=C::from(a[c].abs_diff(b[c]))/2;}}丆}#[derive(Clone,Copy)]struct
仏{丬:G,乀:G,乪:[C;M],令:[C;M],丼:G,front:[I;M],}impl 仏{fn of(l:&J,丰:&亭)->仏{let 俚=matches!(丰.乙,亃::侢);let mut 乪=[0;M];if let 亃::佂(偦)=丰.乙{for(w,仇)in 乪.iter_mut().zip(令(l)){*w=if
仇==C::MAX{0}else{偦.侏(仇)};}}if let 亃::亏(佨)=丰.乙{乪=側(l,l.丛(),佨);}let(傜,丼)=match 丰.乙{_=>([C::MAX;M],G::P),};仏{乪,令:傜,丼,front:if 丰.乙==亃::侖{傼(l,l.丛())}else{[I::MAX;M]},丬:if
丰.丬!=0||俚{亼::of(l).仂()}else{G::P},乀:if 俚{傆(l)}else{G::P},}}}fn 仛(l:&J,丁:F,丰:&亭,丷:仏)->伖{let 侈=僅(l,丁,丰,丷.丬);match 丰.乙{亃::佂(_)|亃::亏(_)=>{let 丅=l.丅(丁);let
僈:C=丅.iter().map(|c|丷.乪[c]).sum();[C::from(丅.count()),僈,侈,0]}亃::侖=>[C::from(l.丅(丁).count()),偘(l,丁,&丷.front),侈,0,],亃::侢=>{let 不=l.不(丁);let 丬=C::from((不&丷.丬).count());let
乀=C::from((不&丷.乀).count());match 丰.乙{_=>[C::from(l.丅(丁).count()),丬,乀,侈],}}}}fn 佒(l:&J,万:&[L])->J{let mut 七=l.clone();七.伵(万);if 七.丝().is_none(){七.伵(&[]);}七}const
侒:I=I::MAX;fn 万(l:&J,丩:&[Y],业:串)->K<L>{if 业.乪==侒{let 丽=Settings{乏:丩.first().copied().unwrap_or(Y::亻),九:if 业.九==乍::乫{亮::乫}else{亮::侓},..Settings::DEFAULT};return
丑(l,&丽);}伻(l,&丩[F::from(业.乪)%丩.len()],业.九,)}fn 俩(l:&J,丩:&[Y],丞:&[串],乽:串,乔:F,)->J{let mut 乥=l.clone();for 丑 in 0..乔{if 乥.丝().is_some(){break;}let 业=丞.get(丑).copied().unwrap_or(乽);乥=佒(&乥,&万(&乥,丩,业));}乥}fn
傱(l:&J,丩:&[Y],伜:O)->K<(串,K<L>)>{let mut 丆:K<(串,K<L>)> =K::new();if 伜{for 九 in[乍::乫,乍::亘{丯:0,丈:I::MAX,},]{let 业=串{乪:侒,九,};let c=万(l,丩,业);if!丆.iter().any(|(_,为)|*为==c){丆.push((业,c));}}}for
乪 in 0..丩.len().min(F::from(I::MAX)){for&九 in&倉{let 业=串{乪:乪 as I,九,};let c=万(l,丩,业);if!丆.iter().any(|(_,为)|*为==c){丆.push((业,c));}}}丆}fn 偗(l:&J,丩:&[Y],乽:串,乔:F,于:F,亞:U<乸>,丰:&亭,)->K<串>{let
丂=l.丛();let 伄=||亞.is_some_and(|t|乸::now()>=t);let 丷=仏::of(l,丰);let 伆=|丞:&[串]|仛(&俩(l,丩,丞,乽,乔),丂,丰,丷);let mut 丒:(伖,K<串>)=(伆(&[]),K::new());let mut 丼:K<(J,K<串>)>
=vec![(l.clone(),K::new())];for 偩 in 0..乔{let mut 亝:K<(伖,J,K<串>)> =K::new();let mut 乤:HashMap<Z,F> =HashMap::new();for(信,丞)in&丼{if 伄(){break;}for(业,c)in
傱(信,丩,丰.伜){let 七=佒(信,&c);let mut 作=丞.clone();作.push(业);let v=仛(&七,丂,丰,丷);let 仛=伉(&七,丂);match 乤.get(&仛){Some(&乜)if 亝[乜].0>=v=>{}Some(&乜)=>亝[乜]=(v,七,作),None=>{乤.insert(仛,亝.len());亝.push((v,七,作));}}}}if
亝.is_empty(){break;}亝.sort_by(|a,b|b.0 .cmp(&a.0));亝.truncate(于);let 乭=偩+1==乔||伄();for(i,(_,_,丞))in 亝.iter().take(if 乭{于}else{1}).enumerate(){if i>0&&伄(){break;}let
v=伆(丞);if v>丒.0{丒=(v,丞.clone());}}if 乭{break;}丼=亝.into_iter().map(|(_,s,p)|(s,p)).collect();}丒.1}fn 丞(l:&J,丩:&[Y],乽:串,乔:F,傅:F,亞:U<乸>,丰:&亭,)->K<串>{let 丂=l.丛();let
丷=仏::of(l,丰);let 伆=|丞:&[串]|仛(&俩(l,丩,丞,乽,乔),丂,丰,丷);let mut 丒:(伖,K<串>)=(伆(&[]),K::new());let mut 于=2;loop{let 乃=偗(l,丩,乽,乔,于,亞,丰);let v=伆(&乃);if v>丒.0{丒=(v,乃);}let
伄=亞.is_some_and(|t|乸::now()>=t);if 伄||(亞.is_none()&&于>=傅){break;}于*=2;}丒.1}const 候:F=4+12;fn 乔(佯:impl BufRead,mut 伴:impl Write,mut 佔:impl FnMut(&str,&str,中,乸)->乑,)->Result<(),乑>{let
mut 丸=侤{俭:佯};let Some(乬)=丸.line()?else{return Ok(());};let mut 仙=format!("{}\n",乬);丸.take(且(&乬,"mine spot count")?,&mut 仙)?;let mut 丑=0;loop{let Some(乬)=丸.line()?else{return
Ok(());};let 举=乸::now();丑+=1;let mut 乾=format!("{}\n",乬);丸.take(候-1,&mut 乾)?;for 乕 in["building count","unit count"]{let line=丸.俯()?;let n=且(&line,乕)?;乾.push_str(&line);乾.push('\n');丸.take(n,&mut
乾)?;}let 佔=佔(&仙,&乾,丑,举);writeln!(伴,"{}",佔).and_then(|()|伴.flush()).map_err(|什|format!("turn {}: cannot write the answer: {}",丑,什))?;}}struct 侤<R>{俭:R,}impl<R:BufRead>侤<R>{fn
line(&mut self)->Result<U<乑>,乑>{let mut line=乑::new();match self.俭.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(什)=>Err(format!("cannot read input: {}",什)),}}fn
俯(&mut self)->Result<乑,乑>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乾:&mut String)->Result<(),乑>{for
_ in 0..n{乾.push_str(&self.俯()?);乾.push('\n');}Ok(())}}fn 且(line:&str,乕:&str)->Result<F,乑>{match line.trim().parse::<F>(){Ok(n)if n<=倥=>Ok(n),_=>Err(format!("bad {} {line:?}",乕)),}}const
倥:F=144;const 偄:C=3;const 倠:C=2;const 倪:F=16;fn 傒(l:&J,丆:&mut Vec<L>){let 丁=l.丛();let mut 事=侭{s:l.clone(),丁,义:1-丁,丌:Q[1-丁],伪:Q[丁],丑:l.乩(),丆,};事.run();}struct
侭<'a>{s:J,丁:F,义:F,丌:F,伪:F,丑:中,丆:&'a mut Vec<L>,}fn 伾(一:F)->(I,I){((一%T)as I,(一/T)as I)}fn 佹(世:中,一:F)->L{let(x,y)=伾(一);L::乲{世,x,y}}fn 俽(丄:I,一:F)->L{let(x,y)=伾(一);L::乎{丄,x,y}}impl
侭<'_>{fn run(&mut self){let q=self;if q.估(){return;}q.交();if q.估(){return;}q.乴();q.伺();}fn 么(&self,三:F,丧:F)->C{C::from(self.s.map().丙(三,丧))}fn 丿(&mut self,丐:L)->O{let
乞=self.s.丿(丐);if 乞{self.丆.push(丐);}乞}fn 仪(&self,丵:&J)->O{丵.丝()==Some(书::亅(self.丁))}fn 估(&mut self)->O{let q=self;if q.s.丝().is_some(){return false;}for
一 in q.s.乷().iter(){if q.么(一,q.丌)!=1{continue;}let Some((世,_))=q.s.亩(一)else{continue;};let 丐=佹(世,q.丌);let mut 丵=q.s.clone();if 丵.丿(丐)&&q.仪(&丵){return q.丿(丐);}}let
Some(丗)=q.偠()else{return false;};let mut 丵=q.s.clone();if!丗.iter().all(|&丐|丵.丿(丐))||!q.仪(&丵){return false;}for 丐 in 丗{q.丿(丐);}true}fn 偠(&self)->U<K<L>>{let
q=self;const 供:C=C::MAX;let 乼=q.s.临().亇();let 丢:K<G> =(1..=乼).map(|丄|q.s.乂(q.丁,丄)).collect();let 仺=|一:F|{丢.iter().position(|乆|乆.test(一)).map(|东|(东 as I+1,10*(东
as C+1)))};let 丅=q.s.丅(q.丁);let mut 下=[供;M];let mut 乁=[F::MAX;M];let mut 乞=[false;M];for 一 in 丅.iter(){下[一]=0;}loop{let mut 七:U<F> =None;for 一 in 0..M{if!乞[一]&&下[一]<供&&七.is_none_or(|n|下[一]<下[n]){七=Some(一);}}let
Some(一)=七 else{break;};乞[一]=true;if 一==q.丌{break;}for&乗 in&q.s.map().与[0][一]{if 乗==亂{break;}let 乗=F::from(乗);if 乞[乗]||丅.test(乗){continue;}let Some((_,亴))=仺(乗)else{continue;};if
下[一]+亴<下[乗]{下[乗]=下[一]+亴;乁[乗]=一;}}}if 下[q.丌]>q.s.上(q.丁){return None;}let mut 丗=K::new();let mut 一=q.丌;while!丅.test(一){let(丄,_)=仺(一)?;丗.push(俽(丄,一));一=乁[一];}丗.reverse();Some(丗)}fn
乄(&self,乖:&J)->C{let(丂,亵)=(self.丁,self.义);let 乁=&self.s;let 且=|乆:G|C::from(乆.count());let 両=且(乖.不(丂))-且(乁.不(丂));let 亖=且(乁.不(亵))-且(乖.不(亵));let 佶=且(乁.丅(亵))-且(乖.丅(亵));let
伬=乁.仞(亵)-乖.仞(亵);let 丯=且(乁.丯()&乁.不(亵))-且(乖.丯()&乖.不(亵));12*両+20*亖+8*(佶-亖).max(0)+5*伬+25*丯}fn 俴(&self,一:F)->C{let q=self;let 乚=q.s.map().丱.test(一);if 乚&&q.s.临().会()&&!q.s.不(q.丁).test(一){20}else{0}}fn
俋(&self,乘:C)->O{let q=self;q.s.丈(q.义).iter().any(|一|q.么(一,q.伪)<=乘)}fn 交(&mut self){let q=self;let mut 丈:K<(C,F)> =q.s.乷().iter().map(|一|(q.么(一,q.丌),一)).collect();丈.sort_unstable();for(伩,一)in
丈{if q.s.丝().is_some(){return;}let Some((世,丄))=q.s.亩(一)else{continue;};let 丢=q.s.乂(q.丁,丄);let 乗=q.s.map().与[q.丁][一];let mut 丒:U<(C,F)> =None;for&七 in 乗.iter().take_while(|&&七|七!=亂){let
七=F::from(七);if!丢.test(七){continue;}let mut 丵=q.s.clone();if!丵.丿(佹(世,七)){continue;}let 丮=if q.仪(&丵){100000}else{q.乄(&丵)+10*(伩-q.么(七,q.丌))+q.俴(七)};if 丒.is_none_or(|(习,_)|丮>习){丒=Some((丮,七));}}if
let Some((丮,七))=丒{if 丮>0{q.丿(佹(世,七));}}}}fn 乴(&mut self){let q=self;if q.s.丝().is_some(){return;}if q.俋(偄){q.傾();}else{q.丯();}}fn 傾(&mut self){let q=self;if!q.s.临().仑()||q.s.上(q.丁)<亀{return;}let
乗=q.s.map().与[q.丁][q.伪];let 丏:K<F> =乗.iter().take_while(|&&一|一!=亂).map(|&一|F::from(一)).collect();let 乊=q.s.不(q.丁);if 丏.iter().any(|&一|乊.test(一)&&q.s.乌().test(一)){return;}for
一 in 丏{let(x,y)=伾(一);if q.丿(L::乹{x,y}){return;}}}fn 丯(&mut self){let q=self;if!q.s.临().会(){return;}while q.s.上(q.丁)>=亀{let 丣=q.s.map().丱&q.s.不(q.丁)&q.s.丅(q.丁)&q.s.亿();let
Some(一)=丣.iter().min_by_key(|&一|(q.么(一,q.伪),一))else{return;};let(x,y)=伾(一);if!q.丿(L::亗{x,y}){return;}}}fn 伺(&mut self){let q=self;let 俰=if q.丑<=3{0}else{5};for
_ in 0..倪{if q.s.丝().is_some(){return;}let 上=q.s.上(q.丁);if 上<10{return;}let 偿=q.俋(倠);let 亥=q.s.亳(q.丁);let mut 丒:U<(C,L)> =None;let mut 侅:U<(C,L)> =None;let
mut 侼=G::P;for 丄 in 1..=q.s.临().亇(){let 丢=q.s.乂(q.丁,丄);let 争=(亥&丢).丕(侼);侼=丢;let 亴=10*C::from(丄);if 亴>上{continue;}for 一 in 争.iter(){let 丐=俽(丄,一);let mut
丵=q.s.clone();if!丵.丿(丐){continue;}let 佱=丵.上(q.丁);let 丟=丵.丟(q.丁);let 伬=q.s.仞(q.义)-丵.仞(q.义);let 偒=if 伬>0&&偿{佱+丟>=0}else{丟>=0&&(佱>=俰||伬>0)};if!偒{continue;}let
乄=q.乄(&丵);let 丮=乄+q.俴(一)-6*q.么(一,q.丌)-10*(C::from(丄)-1);if 乄>0{if 丒.is_none_or(|(习,_)|丮>习){丒=Some((丮,丐));}}else if 丟>=3&&佱>=俰+10&&丄==1{if 侅.is_none_or(|(习,_)|丮>习){侅=Some((丮,丐));}}}}let
Some((_,丐))=丒.or(侅)else{return;};if!q.丿(丐){return;}}}}fn 偸(l:&J,丆:&mut Vec<L>){丆.clear();傒(l,丆);}const 些:C=C::MAX;const 倩:F=8;struct 从{下:[C;M],乁:[I;M],}fn
传(l:&J,丁:F,交:O,丌:F)->C{佡(l,丁,仃(l,丁,交),丌)}fn 佡(l:&J,丁:F,仃:G,丌:F)->C{let mut 下=些;俎(l,丁,仃,|业,new|{if new.test(丌){下=10*业 as C;}下!=些});下}fn 傘(l:&J,丁:F,仃:G,乇:C)->G{let
mut 五=G::P;俎(l,丁,仃,|业,new|{if 10*业 as C>乇{return true;}五|=new;false});五.丕(l.丅(丁))}fn 仃(l:&J,丁:F,交:O)->G{if 交&&丁==l.丛(){l.乷()}else{G::P}}fn 俎(l:&J,丁:F,仃:G,mut
今:impl FnMut(F,G)->O,){let 乼=l.临().亇();let mut 仺=[G::P;3];let mut 乤=G::P;for 丄 in 1..=乼{let 乆=l.乂(丁,丄);仺[F::from(丄)-1]=乆.丕(乤);乤|=乆;}let mut 並=[l.丅(丁),G::P,G::P,G::P];for
一 in(仃&l.丈(丁)).iter(){if let Some(丄)=l.亍(一).filter(|&丄|丄<=乼){並[0]|=G::一(一).与()&l.乂(丁,丄);}}let mut 乞=G::P;for 业 in 0..{let new=並[业%4].丕(乞);並[业%4]=G::P;if
今(业,new){return;}乞|=new;let 七=new.与().丕(乞);for(仐,丏)in 仺.iter().enumerate(){並[(业+仐+1)%4]|=七&*丏;}if 並.iter().all(|乆|乆.丕(乞).is_empty()){break;}}}impl 从{fn
new(l:&J,丁:F,交:O)->从{let 乼=F::from(l.临().亇());let mut 丢=[G::P;3];for(东,乆)in 丢.iter_mut().enumerate().take(乼){*乆=l.乂(丁,东 as I+1);}let 丅=l.丅(丁);let 与=&l.map().与[0];let
mut 下=[些;M];let mut 乁=[亂;M];let mut 並=[0u8;M];let mut 且=0;let mut 俲=G::P;for 一 in 丅.iter(){下[一]=0;並[且]=一 as I;且+=1;}if 交&&丁==l.丛(){for 三 in l.乷().iter(){let
Some(丄)=l.亍(三)else{continue;};let 乓=G::一(三).与()&丢[F::from(丄)-1];for 一 in 乓.iter(){if 下[一]!=0{下[一]=0;並[且]=一 as I;且+=1;}}}}while 且>0{let mut 丒=0;for 东 in
1..且{let(a,b)=(F::from(並[东]),F::from(並[丒]));if(下[a],a)<(下[b],b){丒=东;}}let 一=F::from(並[丒]);且-=1;並[丒]=並[且];俲.set(一);for&七 in&与[一]{if 七==亂{break;}let 七=F::from(七);if
俲.test(七)||丅.test(七){continue;}let Some(丄)=丢[..乼].iter().position(|乆|乆.test(七))else{continue;};let 俼=下[一]+10*(丄 as C+1);if 俼<下[七]{if 下[七]==些{並[且]=七 as
I;且+=1;}下[七]=俼;乁[七]=一 as I;}}}从{下,乁}}fn path(&self,丌:F)->K<F>{let mut path=K::new();if 丌>=M||self.下[丌]==些{return path;}let mut 一=丌;while self.乁[一]!=亂{path.push(一);一=F::from(self.乁[一]);}path.reverse();path}}struct
伓{万:K<L>,下:C,}fn 傄(l:&J,丁:F,一:F)->U<I>{(1..=l.临().亇()).find(|&丄|l.乂(丁,丄).test(一))}fn 傛(专:&mut J,丂:F,两:F,乇:C,万:&mut Vec<L>,二:&mut i64,)->O{loop{if 专.丝()==Some(书::亅(丂)){return
true;}if 专.不(丂).test(两)&&专.丅(丂).test(两){return true;}let 丨=从::new(专,丂,false);let 下=丨.下[两];if 下==些||*二+下>乇{return false;}let Some(&一)=丨.path(两).first()else{return
false;};let Some(丄)=傄(专,丂,一)else{return false;};let 丐=L::乎{丄,x:(一%T)as I,y:(一/T)as I,};if!专.丿(丐){return false;}*二+=10*C::from(丄);万.push(丐);}}fn 侺(l:&J,丂:F,乶:&[F],乇:C)->U<伓>{let
mut 专=l.clone();let mut 万=K::new();let mut 二=0;for&两 in 乶{if!傛(&mut 专,丂,两,乇,&mut 万,&mut 二){return None;}}(二<=乇).then_some(伓{万,下:二,})}fn 佚(l:&J,丌:F,乇:C)->U<伓>{if
乇<0||l.丝().is_some()||丌>=M||l.map().乛.test(丌){return None;}let 丂=l.丛();if l.不(丂).test(丌)&&l.丅(丂).test(丌){return Some(伓{万:K::new(),下:0,});}let mut 丒=侺(l,丂,&[丌],乇);let
乬=从::new(l,丂,false);let mut 乌:K<F> =(l.乌()&l.不(1-丂)).iter().filter(|&乡|乡!=丌&&乬.下[乡]!=些).collect();乌.sort_by_key(|&乡|(l.map().丙(乡,丌),乡));乌.truncate(倩);for
乡 in 乌{if let Some(丗)=侺(l,丂,&[乡,丌],乇){if 丒.as_ref().is_none_or(|习|丗.下<习.下){丒=Some(丗);}}}丒}const 倧:F=20000;struct 伔{万:K<L>,下:C,丏:C,丈:C,乴:C,丟:C,个:C,}struct
佇{丏:C,丈:C,乴:C,}fn 俕(l:&J,丁:F)->佇{let 丅=l.丅(丁);let 丈=l.丈(丁)&丅;let 丮=丈.iter().filter_map(|一|l.亍(一)).map(|丄|10*C::from(丄)).sum();佇{丏:C::from(丅.count()),丈:丮,乴:C::from(((l.丯()|l.乌())&丅).count()),}}struct
侱{丂:F,们:G,乁:佇,丟:C,仝:F,乇:C,仢:F,path:K<L>,乃:K<伔>,index:HashMap<仯,F,BuildHasherDefault<DefaultHasher>>,}type 仯=[Z;3];fn 僊(mut 了:仯,一:F)->仯{了[一/64]|=1<<(一%64);了}impl
侱{fn 俉(&mut self,专:&J,乭:U<F>,亖:仯,下:C,)->O{let q=self;let 丂=q.丂;let 争=match 乭{None=>专.亳(丂),Some(一)=>G::一(一).与(),}&q.们;let 争=争.丕(专.不(丂));let mut 丢=[G::P;3];for
丄 in 1..=专.临().亇(){丢[F::from(丄)-1]=专.乂(丂,丄);}for 一 in 争.iter(){let 乼=F::from(专.临().亇());let Some(东)=丢[..乼].iter().position(|乆|乆.test(一))else{continue;};let
丄=东 as I+1;let 亴=10*C::from(丄);if 下+亴>q.乇{continue;}if q.仢>=倧{return false;}q.仢+=1;let mut 七=专.clone();let 丐=L::乎{丄,x:(一%T)as I,y:(一/T)as I,};if!七.丿(丐){continue;}q.path.push(丐);let
仦=僊(亖,一);q.備(&七,仦,下+亴);let 偧=q.path.len()<q.仝&&七.丝().is_none();let 偷=!偧||q.俉(&七,Some(一),仦,下+亴);q.path.pop();if!偷{return false;}}true}fn 備(&mut self,乖:&J,仦:仯,下:C){let
q=self;let 伃=1-q.丂;let 佻=俕(乖,伃);let 丏=q.乁.丏-佻.丏;let 丈=q.乁.丈-佻.丈;if 丏<=0&&丈<=0{return;}let 乴=q.乁.乴-佻.乴;let 丟=乖.丟(q.丂)-q.丟;let 働=乖.丝()==Some(书::亅(q.丂));let
个=if 働{C::MAX/2}else{3*丏+丈+10*乴+2*丟-下};if let Some(&乜)=q.index.get(&仦){if 下>=q.乃[乜].下{return;}}let 丳=伔{万:q.path.clone(),下,丏,丈,乴,丟,个,};match q.index.get(&仦){Some(&乜)=>q.乃[乜]=丳,None=>{q.index.insert(仦,q.乃.len());q.乃.push(丳);}}}}fn
俇(l:&J,仝:F,乇:C,乘:F)->K<伔>{乱(l,仝,乇,乘).0}fn 乱(l:&J,仝:F,乇:C,乘:F)->(K<伔>,F){if l.丝().is_some()||仝==0||乘==0{return(K::new(),0);}let 丂=l.丛();let 伃=1-丂;let mut
侁=侱{丂,们:l.不(伃)|l.不(伃).与(),乁:俕(l,伃),丟:l.丟(丂),仝,乇,仢:0,path:K::new(),乃:K::new(),index:HashMap::default(),};侁.俉(l,None,[0;3],0);let 仢=侁.仢;let mut 乃=侁.乃;乃.sort_by(|a,b|b.个.cmp(&a.个).then(a.下.cmp(&b.下)));乃.truncate(乘);(乃,仢)}const
优:C=1000000;const 倨:C=200;fn 乐(l:&J,丁:F)->C{let 丅=l.丅(丁);let 丈:C=(l.丈(丁)&丅).iter().filter_map(|一|l.亍(一)).map(|丄|10*C::from(丄)).sum();l.上(丁)+3*l.丟(丁)+4*C::from(丅.count())+丈+10*C::from((l.乌()&丅).count())}fn
俛(l:&J,丁:F,乮:G)->C{偵(l,丁,佡(l,丁,乮,Q[1-丁]),)}fn 偵(l:&J,丁:F,下:C)->C{if 下==些{return C::MIN;}l.上(丁)+l.丟(丁)-下}fn 俪(伲:C)->C{伲.clamp(-倨,0)}fn 偮(l:&J,丁:F,乮:[G;2],)->(C,[C;2]){let
为=1-丁;match l.丝(){Some(书::亅(僉))=>{return(if 僉==丁{优}else{-优},[0,0])}Some(书::侙)=>return(0,[0,0]),None=>{}}let mut 个=乐(l,丁)-乐(l,为);if 丁==l.丛(){let mut 七=l.clone();七.仕();if
七.丝().is_none()&&传(&七,为,true,Q[丁])<=七.上(为){个-=优/2;}}let 仜=[俛(l,丁,乮[0]),俛(l,为,乮[1]),];(个+俪(仜[0])-俪(仜[1]),仜)}
