// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type C=usize;type I=u8;type 佔=u16;type M=u32;type 丯=u64;type 体=i32;type F=i64;type 介=f64;type P=bool;type 乓=String;type 互=Instant;type K<T>=Vec<T>;type
Q<T>=Option<T>;use std::cell::RefCell;use std::collections::{BTreeMap,BTreeSet,HashMap};use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::{Arc,OnceLock};use std::time::{Duration,Instant};const
傸:P=true;fn 兂(l:&J,一:C,低:F,中:G,丟:F)->P{let 七=l.丞();let 个=1-七;let 偋=傸&&丟<=l.下(个)+2*l.丠(个).max(0);let 伣=G::一(一)|G::一(一).不();if!偋&&(伣&中).is_empty(){return
false;}let(x,y)=乞(一);let mut 乪=l.clone();if!乪.义(L::予{x,y}){return false;}if 位(&乪,七)<低{return true;}偋&&伈(&乪,个,false,T[七])>丟}const 億:P=false;const 僖:I=2;const
倌:I=1;const 俴:I=2;const 作:C=0;const 侚:C=2;const 僞:P=false;const 僠:P=false;const 僟:P=false;const 僝:P=false;const 僁:C=4;const 俱:C=5;#[derive(Clone)]enum
丩{仍(L),亟(K<L>),}impl 丩{fn 三(&self)->&[L]{match self{丩::仍(专)=>std::slice::from_ref(专),丩::亟(三)=>三,}}}fn 乞(一:C)->(I,I){((一%Y)as I,(一/Y)as I)}fn 偃(l:&J,万:&mut
Vec<丩>){万.clear();if l.並().is_some(){return;}let 七=l.丞();let 亚=l.乁().交();let mut 丬=[G::U;4];for 丆 in 1..=亚{丬[C::from(丆)]=l.乊(七,丆);}for 上 in l.乿().iter(){let
Some((丌,丆))=l.亸(上)else{continue;};let 乎=G::一(上).不()&丬[C::from(丆)];for 丝 in 乎.iter(){let(x,y)=乞(丝);万.push(丩::仍(L::乼{丌,x,y}));}}let 下=l.下(七);let 丄=l.丄(七);let
乂=l.仜(七);let 乻=match 僖{0=>乂,1=>{let 伳=l.map().valid.世(丄).不()&丄;乂.世(丄)|伳}_=>乂.世(丄),};for 丆 in 1..=亚{let 丑=10*F::from(丆);if 下<丑{break;}let mut 丒=乻&丬[C::from(丆)];for
兑 in 1..丆{丒=丒.世(丬[C::from(兑)]);}for 一 in 丒.iter(){let(x,y)=乞(一);万.push(丩::仍(L::乀{丆,x,y}));}}if 下>=亞{let 丧=l.与(七)&乂&l.仫();if l.乁().佡(){for 一 in(丧&l.map().举).iter(){let(x,y)=乞(一);万.push(丩::仍(L::亹{x,y}));}}if
l.乁().伅(){let 伳=l.map().valid.世(l.与(七)).不();let 丒=(丧&伳).世(l.map().举);let 中=中(l,1-七);let 低=F::from((中&l.丄(七)).count());let 丟=伈(l,1-七,false,T[七]);let 严=if
億{let(亳,佮)=之(l,七);亳.世(佮)}else{G::U};for 一 in 丒.iter().filter(|&一|{let 伣=G::一(一)|G::一(一).不();!(伣&严).is_empty()||兂(l,一,低,中,丟)}){let(x,y)=乞(一);万.push(丩::仍(L::予{x,y}));}}}let
允=match 倌{1=>亞,2=>10,_=>F::MAX,};if 下>=允{let 中=中(l,1-七);let 亥=l.map().valid.世(l.与(七)|l.与(1-七));for 上 in(l.乿()&中).iter(){let Some((丌,丆))=l.亸(上)else{continue;};let
乎=G::一(上).不()&丬[C::from(丆)]&亥;let(仯,仰)=乞(上);let 仓=if 倌==1{if l.map().举.test(上){continue;}L::予{x:仯,y:仰}}else{L::乀{丆:1,x:仯,y:仰,}};for 丝 in 乎.iter(){let(x,y)=乞(丝);万.push(丩::亟(vec![L::乼{丌,x,y},仓]));}}}if
俴!=0&&下>=亞&&l.乁().伅(){儵(l,&丬,万);}if 作!=0&&下>=20{兵(l,丬[1],下,万);}if 僝{儧(l,丬[1],下,万);}for 丷 in 倵(l,俱,下,僁){if 丷.三.len()>1{万.push(丩::亟(丷.三));}}if let Some(丟)=侰(l,T[1-七],下){if
丟.三.len()>1{万.push(丩::亟(丟.三));}}}fn 儵(l:&J,丬:&[G;4],万:&mut Vec<丩>){let 七=l.丞();let 个=1-七;let 中=中(l,个);let 两=l.与(七);let 丘=l.与(个);let 伳=丘.不();let 举=l.map().举;for
上 in(l.乿()&中&l.丄(七)).iter(){if 举.test(上){continue;}let Some((丌,丆))=l.亸(上)else{continue;};let 儎=l.map().丙(上,T[个]);let 乎=G::一(上).不()&丬[C::from(丆)];let(仯,仰)=乞(上);for
丝 in 乎.iter(){let 侧=match 俴{1=>两.test(丝)&&l.map().丙(丝,T[个])>儎,2=>两.test(丝)&&伳.test(丝),_=>两.test(丝)||(丘.test(丝)&&丝!=T[个]),};if!侧{continue;}let(x,y)=乞(丝);万.push(丩::亟(vec![L::乼{丌,x,y},L::予{x:仯,y:仰},]));}}}fn
兵(l:&J,俈:G,下:F,万:&mut Vec<丩>){let 七=l.丞();let 丨=l.map();let 亥=丨.valid.世(l.与(七)|l.与(1-七));let 严=亥&俈;let 乻:K<C> =(l.丄(七).不()&严).iter().collect();if 乻.is_empty(){return;}let
俏=|丐:C|{乻.iter().copied().min_by_key(|&一|(丨.丙(一,丐),一))};let mut 侱:K<(C,C)> =K::new();if 僞{for 业 in 其(l,&乻){if let Some(乃)=俏(业){侱.push((乃,业));}}}else{let
丐=T[1-七];let mut 保=乻.clone();保.sort_by_key(|&一|(丨.丙(一,丐),一));保.truncate(侚);侱.extend(保.into_iter().map(|乃|(乃,丐)));}let 亵=C::try_from(下/10).unwrap_or(0).min(作);for(乃,丐)in
侱{let mut 丟=vec![乃];let mut 亷=G::一(乃);while 丟.len()<亵{let 乷=丟[丟.len()-1];if 乷==丐{break;}let 丅=(G::一(乷).不()&严).世(亷).iter().min_by_key(|&一|(丨.丙(一,丐),一));let
Some(丅)=丅 else{break;};丟.push(丅);亷.set(丅);let 三=丟.iter().map(|&一|{let(x,y)=乞(一);L::乀{丆:1,x,y}}).collect();万.push(丩::亟(三));}if 僟{免(l,&丟,万);}}}fn 免(l:&J,丟:&[C],万:&mut
Vec<丩>){let 七=l.丞();let 乃=丟[0];let 亄=l.乿()&l.丈(七)&G::一(乃).不();for 上 in 亄.iter(){let Some((丌,丆))=l.亸(上)else{continue;};if!l.乊(七,丆).test(乃){continue;}let(x,y)=乞(乃);let
mut 三=vec![L::乼{丌,x,y}];for&一 in&丟[1..]{let(x,y)=乞(一);三.push(L::乀{丆:1,x,y});万.push(丩::亟(三.clone()));}}}fn 其(l:&J,乻:&[C])->K<C>{let 七=l.丞();let 丨=l.map();let(丵,串)=佭(l);let
两:K<C> =l.丈(七).iter().collect();let 丘:K<C> =l.丈(1-七).iter().collect();let 佬=|业:C|{乻.iter().map(|&一|丨.丙(一,业)).min().unwrap_or(I::MAX)};let 儞=|业:C|{let 中=佬(业);两.iter().any(|&仉|{let
d=丨.丙(仉,业);d<=中&&丘.iter().all(|&个|d<丨.丙(个,业))})};let 俊=|业:C|{僠&&(l.与(1-七).test(业)||丘.iter().any(|&个|丨.丙(个,业)<佬(业)))};let 严=|丒:G|->K<C>{let mut 事:K<C> =丒.世(l.与(七)).iter().filter(|&业|佬(业)!=I::MAX&&!儞(业)&&!俊(业)).collect();事.sort_by_key(|&业|(佬(业),业));事};let
mut 事=严(串);if 事.is_empty(){事=严(丵);}事.truncate(侚);事}fn 儡(l:&J)->K<C>{let 七=l.丞();let 丨=l.map();let(丵,串)=佭(l);let 丄=l.丄(七);let 两=l.与(七);let 中=中(l,1-七);let
丘:K<C> =l.丈(1-七).iter().collect();let 偁=|一:C|{丄.iter().map(|a|丨.丙(a,一)).min().unwrap_or(I::MAX)};let mut 丒:K<C> =(丵|串).iter().filter(|&一|{if 两.test(一){丄.test(一)&&中.test(一)}else{!l.与(1-七).test(一)&&丘.iter().any(|&e|丨.丙(e,一)<偁(一))}}).collect();丒.sort_by_key(|&一|(I::from(!串.test(一)),偁(一),一));丒.truncate(
2*侚);丒}fn 儧(l:&J,俈:G,下:F,万:&mut Vec<丩>){let 七=l.丞();let 丨=l.map();let 亥=丨.valid.世(l.与(七)|l.与(1-七));let 严=亥&俈;let 丄=l.丄(七);let 丧=丄&l.仫();let 児=if 作==0{3}else{作};for
一 in 儡(l){if!l.与(七).test(一){let 乃=(丄.不()&严).iter().min_by_key(|&c|(丨.丙(c,一),c));if let Some(乃)=乃{let mut 丟=vec![乃];let mut 亷=G::一(乃);while 丟[丟.len()-1]!=一&&丟.len()<児{let
乷=丟[丟.len()-1];let 丅=(G::一(乷).不()&严).世(亷).iter().min_by_key(|&c|(丨.丙(c,一),c));let Some(丅)=丅 else{break;};丟.push(丅);亷.set(丅);}let 丑=10*F::try_from(丟.len()).unwrap_or(F::MAX);if
丟.len()>=2&&丟[丟.len()-1]==一&&丑<=下{let 三=丟.iter().map(|&c|{let(x,y)=乞(c);L::乀{丆:1,x,y}}).collect();万.push(丩::亟(三));}}}if 下>=亞&&l.乁().伅(){let 举=(G::一(一)|G::一(一).不())&丧;for
乵 in 举.世(丨.举).iter(){let(x,y)=乞(乵);万.push(丩::仍(L::予{x,y}));}}}}const 僎:I=2;const 僵:F=5;fn 亄(l:&J,丂:C)->G{俎(l,丂,僎)}fn 俎(l:&J,丂:C,上:I)->G{let mut 亄=G::U;if
上>3{return 亄;}for 一 in(l.丈(丂)&l.丄(丂)).iter(){if l.亝(一).is_some_and(|丆|丆>=上){亄|=G::一(一);}}亄}fn 中(l:&J,主:C)->G{let 乗=l.下(主)+l.丠(主);兮(l,主,亄(l,主),乗)}fn 亲(l:&J,丂:C)->G{中(l,1-丂)&l.丄(丂)}fn
位(l:&J,丂:C)->F{F::from(亲(l,丂).count())}const 倃:F=4;fn 兕(丒:F)->F{丒/倃}fn 丽(l:&J,丂:C,亲:G)->F{let 丄=l.丄(丂);let 丽=(l.丽()&丄).世(亲);let 丒=F::from(丄.count());let
侫=if l.乤()<=傥{丒/傤+傣}else{兕(丒)};侫*F::from(丽.count())}const 傥:M=15;const 傣:F=6;const 傤:F=倃;const 僧:F=2;const 佝:I=1;const 個:I=3;const 僨:I=6;fn 克(仙:I,乂:I,仭:I,d:I)->I{match
仙{0=>4,1 if d<=乂=>4,1=>0,_ if d<=乂=>4,_ if d>=仭=>0,_=>(4*M::from(仭-d)/M::from(仭-乂))as I,}}fn 傔(l:&J,丂:C)->F{let 丄=l.丄(丂);let 个=if 佝==0{G::U}else{l.丄(1-丂)};let[a,b,c]=丄.佑();let[e,f,g]=个.佑();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^(e^f.rotate_left(13)^g.rotate_left(37)).wrapping_mul(0x2545f4914f6cdd1d)^丂 as 丯).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
丣=(hash as C)&(候-1);儅.with(|侭|{let mut 侭=侭.borrow_mut();let 仟=&mut 侭[丣];if 仟.0==丄&&仟.1==个&&仟.2==丂 as I+1{return 仟.3;}let 丫=if 佝==0{冡(丄,T[丂])}else if 佝==1{let
丰=l.map().valid;let mut 乂=个&丰;for _ in 0..個{乂=(乂|乂.不())&丰;}冠(丄,T[丂],乂)}else{let d=丸(l,个);let mut 乛=[0u8;O];for(w,&乎)in 乛.iter_mut().zip(d.iter()){*w=克(佝,個,僨,乎);}傕(丄,T[丂],&乛)};*仟=(丄,个,丂
as I+1,丫);丫})}const 候:C=1<<12;thread_local!{static 儅:std::cell::RefCell<K<(G,G,I,F)>> =std::cell::RefCell::new(vec![(G::U,G::U,0,0);候]);}fn 兩(丄:G)->G{let
n=丄.侀();let s=丄.侁();let e=丄.偶();let w=丄.偷();let 兘=e.侀();let 兙=w.侀();let 具=e.侁();let 冊=w.侁();let 兊=[n&兘&e,e&具&s,s&冊&w,w&兙&n];let 今=|a:G,b:G|(a|b).世(a&b);let
倛=|[a,b,c,d]:[G;4]|{let(偰,倩)=(今(a,b),a&b);let(偱,倫)=(今(c,d),c&d);let t=偰&偱;let 倪=今(倩,倫);(今(偰,偱),今(倪,t),(倩&倫)|(t&倪))};let(兄,假,充)=倛([n,e,s,w]);let(儬,倹,儭)=倛(兊);let
倣=儬.世(兄);let 傗=今(假,倹);let 儢=今(傗,倣);let 儏=倹.世(假)|倣.世(傗);let 儣=今(今(充,儭),儏);丄&(儢|儣)}fn 冠(丄:G,乧:C,乂:G)->F{if!丄.test(乧){return 0;}let 亐=丄.count();let mut 习=0i64;let
mut 亵=0i64;for v in(兩(丄)&乂).iter(){if v==乧{continue;}let 乢=丄.世(G::一(v));let mut 乴=G::一(乧);loop{let 丅=(乴|乴.不())&乢;if 丅==乴{break;}乴=丅;}let 丷=F::from(亐-1-乴.count());if
丷>0{习+=4;亵=亵.max(4*丷);}}(习+亵)/4}fn 冡(丄:G,乧:C)->F{傕(丄,乧,&[4;O])}fn 傕(丄:G,乧:C,乛:&[I;O])->F{struct 伤<'a>{乛:&'a[I;O],丄:G,乾:[I;O],书:[I;O],size:[I;O],仡:I,习:F,亵:F,}fn
仼(d:&mut 伤<'_>,v:C)->I{d.仡+=1;let 乾=d.仡;d.乾[v]=乾;let mut 书=乾;let mut 俙=1u8;let mut 丷=0u8;for w in(G::一(v).不()&d.丄).iter(){if d.乾[w]==0{let 傃=仼(d,w);俙+=傃;书=书.min(d.书[w]);if
d.书[w]>=乾{丷+=傃;}}else{书=书.min(d.乾[w]);}}d.书[v]=书;d.size[v]=俙;if 丷>0&&d.乾[v]>1{let w=F::from(d.乛[v]);d.习+=w;d.亵=d.亵.max(w*F::from(丷));}俙}if!丄.test(乧){return
0;}let mut d=伤{乛,丄,乾:[0;O],书:[0;O],size:[0;O],仡:0,习:0,亵:0,};仼(&mut d,乧);(d.习+d.亵)/4}fn 兹(l:&J,丂:C)->F{僧*(傔(l,1-丂)-傔(l,丂))}fn 倶(l:&J,丂:C)->F{if l.下(丂)+l.丠(丂)>=0{return
0;}(l.丈(丂)&l.丄(丂)).iter().filter_map(|一|l.亝(一)).map(|丆|10*F::from(丆)).sum()}const 僀:F=15;const 僾:[F;2]=[3,1];const 僼:[F;2]=[4,20];fn 冈(l:&J,丂:C)->[F;2]{let
mut 丗=[0;2];for 一 in(l.丈(丂)&l.丄(丂)).iter(){match l.亝(一){Some(2)=>丗[0]+=1,Some(3)=>丗[1]+=1,_=>{}}}丗}fn 傂(l:&J,丂:C)->F{let[傋,冒]=冈(l,丂);let 値=|丆:C|(3-僾[丆])*僼[丆];-僀*傋+値(0)*傋+値(1)*冒}const
傷:F=0;const 傺:F=2;const 傹:F=100;fn 偺(亴:F)->F{if 亴<0{return 0;}傷+傺*亴.min(傹)/2}const 倂:I=2;const 俿:F=2;const 俾:F=4;const 傻:I=3;fn 仂(l:&J,丂:C)->F{if 俿==0{return
0;}let 丰=l.map().valid;let mut 丈=G::U;for 一 in(l.丈(丂)&l.丄(丂)).iter(){if l.亝(一).is_some_and(|丆|丆<=傻){丈|=G::一(一);}}let mut 侇=丰.世(l.与(丂));let mut 仑=G::U;let
mut 乎=0;for 丙 in 1..=俾{侇=(侇|侇.不())&丰;let 仔=(丈&侇).世(仑);乎+=(丙-1)*F::from(仔.count());仑|=仔;}乎+=俾*F::from(丈.世(仑).count());俿*乎}const 倘:F=8;fn 之(l:&J,丂:C)->(G,G){let
丄=l.丄(丂);let 亳=丄&l.与(1-丂).不();let 乚=l.乚()&丄;(亳,亳&(乚|乚.不()))}fn 傑(l:&J,丂:C)->F{if 倘==0{return 0;}倘*F::from(之(l,丂).1 .count())}const 侔:F=0;const 俻:I=1;const
俼:F=4;const 侕:F=0;const 俺:F=0;const 傮:M=14;const 傭:F=2;const 倗:F=0;const 儃:M=14;fn 份(l:&J,丂:C)->F{if 倗==0||l.乤()>儃{return 0;}let 两=丸(l,l.丄(丂));let 丘=丸(l,l.丄(1-丂));let
mut 价=0;for c in l.map().valid.iter(){match 两[c].cmp(&丘[c]){std::cmp::Ordering::Less=>价+=1,std::cmp::Ordering::Greater=>价-=1,std::cmp::Ordering::Equal=>{}}}倗*价}struct
俽{valid:G,丵:G,串:G,}thread_local!{static 傯:std::cell::RefCell<Q<俽>> =const{std::cell::RefCell::new(None)};}fn 佭(l:&J)->(G,G){傯.with(|一|{let mut 久=一.borrow_mut();let
丰=l.map().valid;if 久.as_ref().is_none_or(|z|z.valid!=丰){let 仺=仺(l);let mut 丵=G::U;for c in 丰.iter(){if 仺[c]<=傭{丵.set(c);}}*久=Some(俽{valid:丰,丵,串:仵(l),});}久.as_ref().map_or((G::U,G::U),|z|(z.丵,z.串))})}fn
佮(l:&J,丂:C)->G{let 丄=l.丄(丂);let 乚=l.乚()&丄;(乚|乚.不())&丄}fn 业(l:&J,丂:C)->F{if(侔==0&&俼==0&&侕==0&&俺==0)||l.乤()>傮{return 0;}let(丵,串)=佭(l);let mut 亐=0;if 侔!=0{let
两=丸(l,l.丄(丂));let 丘=丸(l,l.丄(1-丂));let mut 价=0;for c in 丵.iter(){let(a,b)=(两[c],丘[c]);if a!=I::MAX&&a.saturating_add(俻)<=b{价+=1;}else if b!=I::MAX&&b.saturating_add(俻)<=a{价-=1;}}亐+=侔*价;}let
丗=|乏:G|F::from(乏.count());let(丼,丘)=(l.丄(丂),l.丄(1-丂));亐+=俼*(丗(丼&丵)-丗(丘&丵));if 侕!=0{亐+=侕*(丗(佮(l,丂)&丵)-丗(佮(l,1-丂)&丵));}亐+=俺*(丗(丼&串)-丗(丘&串));亐}const 俷:F=4;const
俶:F=0;const 傪:F=4;const 傫:M=12;fn 儹()->[G;2]{static 傩:std::sync::OnceLock<[G;2]> =std::sync::OnceLock::new();*傩.get_or_init(儺)}fn 儺()->[G;2]{let mut 万=[G::U;2];for
c in 0..O{let(x,y)=((c%Y)as F,(c/Y)as F);if x-y>=俷{万[0].set(c);}else if y-x>=俷{万[1].set(c);}}万}fn 傓(l:&J,丂:C)->F{let[a,b]=儹();let 丄=l.丄(丂);F::from((丄&a).count().min((丄&b).count())).min(傪)}fn
儸(l:&J,丂:C)->F{if 俶==0||l.乤()>傫{return 0;}俶*(傓(l,丂)-傓(l,1-丂))}const 們:C=17;const 儆:[F;們]=[82,110,124,86,81,97,123,92,96,107,126,121,109,88,100,100,100,];fn
于(侂:F,传:F)->F{if 传==100{侂}else{侂*传/100}}fn 儰(l:&J,丂:C)->F{儱(l,丂,&儆)}#[inline(always)]fn 儱(l:&J,丂:C,w:&[F;們])->F{let 亄=[俎(l,丂,倂),俎(l,1-丂,倂),];let(丳,付)=儲(l,丂,亄);let
偻=偺(付[0])-偺(付[1]);if l.並().is_some(){return 丳+偻;}let mut 丳=丳+于(偻,w[6]);if w[..6].iter().any(|&x|x!=100){let 两=俘(l,丂);let 丘=俘(l,1-丂);for i in 0..5{let 佅=两[i]-丘[i];丳+=于(佅,w[i])-佅;}let
偧=佻(付[0])-佻(付[1]);丳+=于(偧,w[5])-偧;}let 两=亲(l,丂);let 丘=亲(l,1-丂);let 低=F::from(丘.count())-F::from(两.count());丳+于(-2*倶(l,丂)+2*倶(l,1-丂),w[7])+于(兹(l,丂),w[8])+于(傂(l,丂)-傂(l,1-丂),w[9],)+于(-仂(l,丂)+仂(l,1-丂),w[10])+于(僵*低/4,w[11])+于(丽(l,丂,两)-丽(l,1-丂,丘),w[12],)+于(傑(l,丂)-傑(l,1-丂),w[13])+于(业(l,丂),w[14])+于(份(l,丂),w[
15])+于(儸(l,丂),w[16])}struct 伭{侅:伮,丈:K<(M,C,[I;五])>,}impl 伭{fn new(l:&J)->伭{let 侅=伮::new(l);let 丈=l.乿().iter().filter_map(|一|{let(丌,_)=l.亸(一)?;let 乱=侅.key(l,一)?;Some((丌,一,伙(&乱)))}).collect();伭{侅,丈}}fn
偐(&self,l:&J,乮:&丩)->Q<(乔,[I;五])>{let 丩::仍(L::乼{丌,x,y})=乮 else{return None;};let&(_,上,p)=self.丈.iter().find(|u|u.0==*丌)?;let 丝=C::from(*y)*Y+C::from(*x);Some((self.侅.kind(l,上,丝),p))}fn
伙(&self,l:&J,乮:&丩)->I{self.偐(l,乮).map_or(0,|(他,p)|p[他 as C])}fn filter(&self,l:&J,k:C,乕:&mut Vec<丩>){乕.retain(|乮|match self.偐(l,乮){Some((他,p))=>{let 乇=兏(&p);乇.is_empty()||乇.iter().take(k).any(|&亏|亏==他)}None=>true,});}}const
僊:C=8;const 僚:C=3;const 僂:C=32;const 僛:丯=300;const 傢:介=0.75;const 傴:Q<M> =Some(7);const 傲:P=false;const 傱:F=10;const 傳:M=6;const 侟:Q<C> =None;const 侠:P=false;const
僺:M=7;struct 亻{仈:Q<互>,仱:Q<互>,伾:丯,}#[derive(Default)]struct 価{三:K<L>,侶:C,仁:丯,両:C,乒:C,倯:C,亙:C,亍:P,偿:P,仝:C,}const 儂:P=true;#[derive(Clone)]struct 亖{三:K<L>,侹:F,}struct
伀{三:K<L>,end:J,first:Q<丯>,丛:Q<K<亖>>,}type 伦=HashMap<丯,F,BuildHasherDefault<DefaultHasher>>;struct 依{七:C,亀:P,limit:亻,仁:丯,伺:伦,任:Q<K<丩>>,俟:P,乥:Q<仿>,}const
供:F=0;const 俬:P=true;const 傜:P=false;const 俫:Q<I> =None;fn 儔(l:&J,両:&[亖],万:&mut Vec<丩>){if!俬&&俫.is_none(){return;}let 佉=両.iter().map(|亅|亅.三.as_slice());儕(l,佉,(俬,傜,俫),万,);}fn
儕<'a>(l:&J,佉:impl Iterator<Item=&'a[L]>,(乚,不,丆):(P,P,Q<I>),万:&mut Vec<丩>,){let 七=l.丞();let 丼=l.丄(七);let mut 伸=G::U;for 亅 in 佉{for 专 in 亅{if let L::乀{x,y,..}|L::乼{x,y,..}=*专{let
一=C::from(y)*Y+C::from(x);if 丼.test(一){伸|=G::一(一);}}}}if 伸.is_empty(){return;}let 丧=l.仫()&丼;let 下=l.下(七);let 偩=|专:L,万:&mut Vec<丩>|{if!万.iter().any(|a|a.三()==[专]){万.push(丩::仍(专));}};if
乚&&下>=亞&&l.乁().伅(){let mut 丒=伸;if 不{丒|=伸.不();}for 一 in(丒&丧).世(l.map().举).iter(){let(x,y)=乞(一);偩(L::予{x,y},万);}}if let Some(丆)=丆.filter(|&丆|丆<=l.乁().交()){if
下>=10*F::from(丆){for 一 in(伸&丧).iter(){let(x,y)=乞(一);偩(L::乀{丆,x,y},万);}}}}fn 侩(l:&mut J,三:&[L])->P{三.iter().all(|&专|l.义(专))}fn 乱(三:&[L])->丯{let mut hash:丯=0xcbf29ce484222325;for
专 in 三{let(他,a,b,c)=match*专{L::乼{丌,x,y}=>(1,丯::from(丌),x,y),L::乀{丆,x,y}=>(2,丯::from(丆),x,y),L::亹{x,y}=>(3,0,x,y),L::予{x,y}=>(4,0,x,y),};let 伢=他|a<<8|丯::from(b)<<40|丯::from(c)<<48;hash=(hash^伢).wrapping_mul(0x0100000001b3);}hash}fn
倞(乐:&J,亅:&[L])->J{let mut 丅=乐.clone();丅.伋();for&专 in 亅{丅.义(专);}丅.伋();丅}fn 佳(l:&J)->Q<K<L>>{let 七=l.丞();let 丐=T[1-七];let 伡=|为:&J|为.並()==Some(乽::亢(七));for
一 in l.乿().iter(){if l.map().丙(一,丐)!=1{continue;}let Some((丌,_))=l.亸(一)else{continue;};let(x,y)=乞(丐);let 专=L::乼{丌,x,y};let mut 为=l.clone();if 为.义(专)&&伡(&为){return
Some(vec![专]);}}let 丟=侰(l,丐,l.下(七))?;let mut 为=l.clone();(侩(&mut 为,&丟.三)&&伡(&为)).then_some(丟.三)}fn 傁(l:&J,丂:C,丮:F)->P{let 丑=侷(l,丂,l.丈(丂),T[1-丂]);丑!=亰&&l.下(丂)+丮*l.丠(丂)>=丑}fn
冇(l:&J)->P{let 七=l.丞();if 供==0||l.並().is_some(){return false;}let mut 丅=l.clone();丅.伋();let 位=丅.並().is_none()&&伈(&丅,1-七,true,T[七])<=丅.下(1-七);!位&&傁(l,七,供)}impl
依{fn 倦(&self,乐:&J)->P{self.俟&&乐.並()!=Some(乽::亢(self.七))&&!傁(乐,self.七,供)}fn 亪(&self)->P{self.仁>=self.limit.伾||self.limit.仈.is_some_and(|t|互::now()>=t)}fn
丳(&mut self,乐:&J,両:&[亖],侬:F)->F{self.仁+=1;self.俔(乐,両,侬)}fn 俔(&self,乐:&J,両:&[亖],侬:F)->F{if 乐.並()==Some(乽::亢(self.七)){return 佟;}if 両.is_empty(){return self.value(乐);}let
mut 侈=F::MAX;for 亅 in 両{侈=侈.min(self.value(&倞(乐,&亅.三)));if 侈<=侬{break;}}侈}fn value(&self,l:&J)->F{let 伲=儰(l,self.七);match&self.乥{Some(乥)if l.並().is_none()=>伲+乥.侂(l,self.七),_=>伲,}}fn
乺(&self,乧:&J,丶:&J)->Q<伭>{let 兛=(侟.is_some()||侠)&&!self.亀&&乧.乤()>=僺;兛.then(||伭::new(丶))}fn 倜(&self,乕:&mut Vec<丩>){if let Some(乥)=&self.乥{for 佹 in&乥.仝{乕.push(丩::亟(佹.clone()));}}}#[allow(clippy::too_many_arguments)]fn
兤(&mut self,丶:&J,伊:F,乕:&[丩],乇:&[(F,C)],买:&mut[P],両:&[亖],伌:&mut 伦,俇:P,)->(Q<(C,J,F)>,P){let mut 丕=None;let mut 佣=伊;for&(併,丣)in 乇{if 买[丣]{continue;}if 俇&&併!=F::MAX&&併<=佣-伊{return(丕,true);}if
self.亪(){return(丕,false);}买[丣]=true;let 三=乕[丣].三();let mut 丅=丶.clone();if!侩(&mut 丅,三)||self.倦(&丅){伌.insert(乱(三),F::MIN/4);continue;}let 丳=self.丳(&丅,両,佣);伌.insert(乱(三),丳-伊);if
丳>佣{佣=丳;丕=Some((丣,丅,丳));}}(丕,false)}fn 侵(&mut self,乧:&J,両:&[亖],伱:&[丯],儘:P)->伀{let 丁=self;let mut 亃=None;let mut 丶=乧.clone();let mut 三=K::new();let mut
伊=丁.丳(&丶,両,F::MIN);let mut 伌=伦::default();let mut 乕=K::new();let mut 乇=K::new();while 丶.並().is_none()&&!丁.亪(){if 三.is_empty()&&儘{match&丁.任{Some(任)=>乕.clone_from(任),None=>{偃(&丶,&mut
乕);if let Some(乥)=丁.乥.as_ref().filter(|g|g.偞){乕.push(丩::亟(乥.东.clone()));}丁.倜(&mut 乕);丁.任=Some(乕.clone());}}}else{偃(&丶,&mut 乕);丁.倜(&mut 乕);}儔(&丶,両,&mut
乕);let 乺=丁.乺(乧,&丶);if let(Some(乺),Some(k))=(&乺,侟){乺.filter(&丶,k,&mut 乕);}乇.clear();for(丣,乮)in 乕.iter().enumerate(){let k=乱(乮.三());let 併=伌.get(&k).or(丁.伺.get(&k)).copied();乇.push((併.unwrap_or(F::MAX),丣));}match
乺.as_ref().filter(|_|侠){Some(乺)=>{let 偨:K<I> =乕.iter().map(|a|乺.伙(&丶,a)).collect();乇.sort_by(|a,b|{(b.0 .cmp(&a.0)).then(偨[b.1].cmp(&偨[a.1])).then(a.1
.cmp(&b.1))});}None=>乇.sort_by(|a,b|b.0 .cmp(&a.0).then(a.1 .cmp(&b.1))),}let mut 买=vec![false;乕.len()];if 三.is_empty(){for(丣,乮)in 乕.iter().enumerate(){买[丣]=伱.contains(&乱(乮.三()));}}let
mut 丕=None;for 俇 in[true,false]{let(乘,兽)=丁.兤(&丶,伊,&乕,&乇,&mut 买,両,&mut 伌,俇,);丕=乘;if 丕.is_some()||!兽{break;}}let Some((丣,丅,丳))=丕 else{break;};if 三.is_empty(){亃=Some(乱(乕[丣].三()));}三.extend_from_slice(乕[丣].三());丶=丅;伊=丳;}for(k,乑)in
伌{let 偈=丁.伺.entry(k).or_insert(乑);*偈=(*偈).max(乑);}伀{三,end:丶,first:亃,丛:None,}}fn 佉(&self,乐:&J)->K<K<L>>{let mut 丅=乐.clone();丅.伋();let mut 万=K::new();if
丅.並().is_some(){return 万;}let 亀=丅.丞();let 下=丅.下(亀);if let Some(丟)=侰(&丅,T[self.七],下){万.push(丟.三);}for 丷 in 倵(&丅,俱,下,僚){万.push(丷.三);}let mut 偅=K::new();兀(&丅,&mut
偅);万.push(偅);let mut 丘=依{七:亀,亀:true,limit:亻{伾:僛,..self.limit},仁:0,伺:伦::default(),任:None,俟:false,乥:None,};万.push(丘.侵(&丅,&[],&[],true).三);万.retain(|三|!三.is_empty());万}fn
借(&self,乐:&J)->K<亖>{self.佉(乐).into_iter().map(|三|{let 侹=self.value(&倞(乐,&三));亖{三,侹}}).collect()}fn 亙(&mut self,乧:&J,乮:&丩,両:&[亖])->Q<伀>{let mut 乪=乧.clone();if!侩(&mut
乪,乮.三())||self.倦(&乪){return None;}self.仁+=1;let 乢=self.侵(&乪,両,&[],false);let mut 三=乮.三().to_vec();三.extend_from_slice(&乢.三);Some(伀{三,end:乢.end,first:Some(乱(乮.三())),丛:None,})}fn
倾(&self,东:&伀,両:&[亖])->F{let 丳=self.俔(&东.end,両,F::MIN);match&东.丛{Some(丛)if!丛.is_empty()=>丳.min(self.俔(&东.end,丛,F::MIN)),_=>丳,}}}fn 兰(乘:&[亖],両:&mut Vec<亖>)->P{let
乖:K<_> =両.iter().map(|r|r.三.clone()).collect();for 亅 in 乘{if 両.iter().any(|r|r.三==亅.三){continue;}両.push(亅.clone());}両.sort_by_key(|亅|亅.侹);両.truncate(僊);両.iter().map(|r|&r.三).ne(&乖)}fn
冑(l:&J,九:亻,亍:&mut 亮)->価{傆(l,九,亍,None)}fn 傆(l:&J,九:亻,亍:&mut 亮,乥:Q<仿>,)->価{let 七=l.丞();let mut 乄=価::default();if l.並().is_some(){return 乄;}if let Some(三)=佳(l){乄.三=三;return
乄;}let 丱=|养:介|match(九.仈,九.仱){(Some(仈),Some(仱))=>Some(仈+(仱-仈).mul_f64(养)),_=>None,};if 傴.is_some_and(|且|l.乤()>=且){亍.end();}if 亍.全(l){let 三=亍.且(l,丱(0.4));let
儗=三.iter().any(|专|{matches!(专,L::乀{..}|L::亹{..}|L::予{..})});let 仂=傲&&!儗&&l.乤()>=傳&&l.下(七)>=傱;if!仂{乄.亍=true;乄.三=三;return 乄;}亍.end();}let mut 乨=依{七,亀:false,limit:亻{仈:丱(傢),..九},仁:0,伺:伦::default(),任:None,俟:冇(l),乥,};let
mut 両=K::new();let mut 乒:K<伀> =K::new();let mut 伱=K::new();let mut 佩=false;let mut 亙:K<丩> =K::new();let mut 乃=乨.乥.as_ref().filter(|g|g.start).map(|g|丩::亟(g.东.clone()));while
乄.侶<僂&&(乒.is_empty()||!乨.亪()){let 冂=match 乃.take(){Some(乮)=>乨.亙(l,&乮,&両),None=>None,};let 东=if let Some(东)=冂{东}else if 佩{let Some(乮)=亙.pop()else{break;};match
乨.亙(l,&乮,&両){Some(东)=>东,None=>continue,}}else{乨.侵(l,&両,&伱,true)};乄.侶+=1;let 偳=乒.iter().any(|p|p.三==东.三);let 亃=东.first;let 偾=乨.亪();let mut 东=东;let mut 偘=false;if!偾&&!偳{let
乘=乨.借(&东.end);偘=兰(&乘,&mut 両);东.丛=Some(乘);}if!偳{乒.push(东);}if 偾{break;}if!偘&&!佩{match 亃{Some(亃)=>伱.push(亃),None=>{佩=true;亙=乨.任.clone().unwrap_or_default();亙.retain(|a|!伱.contains(&乱(a.三())));let
乺=乨.乺(l,l);if let(Some(乺),Some(k))=(&乺,侟){乺.filter(l,k,&mut 亙);}let 乑=|a:&丩|{乨.伺.get(&乱(a.三())).copied().unwrap_or(F::MIN)};match 乺.as_ref().filter(|_|侠){Some(乺)=>{亙.sort_by_key(|a|(乑(a),乺.伙(l,a)));}None=>亙.sort_by_key(乑),}}}}}乄.亙=C::from(佩);乄.仁=乨.仁;乄.両=両.len();乄.乒=乒.len();let
仗=||九.仱.is_some_and(|t|互::now()>=t);let mut 伛:K<Q<F>> =vec![None;乒.len()];for 丣 in(0..乒.len()).rev(){if 丣+1<乒.len()&&仗(){break;}伛[丣]=Some(乨.倾(&乒[丣],&両));}let
优=|伛:&[Q<F>]|{let mut 丕:Q<(F,C)> =None;for(丣,丳)in 伛.iter().enumerate().rev(){if let Some(丳)=*丳{if 丕.is_none_or(|(亏,_)|丳>亏){丕=Some((丳,丣));}}}丕};let mut
丕=优(&伛);乨.limit.仈=九.仱;while let Some((_,丣))=丕{if!儂||乒[丣].丛.is_some()||仗(){break;}let 乘=乨.借(&乒[丣].end);乒[丣].丛=Some(乘);乄.倯+=1;伛[丣]=Some(乨.倾(&乒[丣],&両));丕=优(&伛);}if
let Some((_,丣))=丕{乄.偿=乨.乥.as_ref().is_some_and(|g|!g.东.is_empty()&&乒[丣].三.starts_with(&g.东));乄.仝=乨.乥.as_ref().map_or(0,|g|{g.仝.iter().filter(|p|p.iter().all(|c|乒[丣].三.contains(c))).count()});let
mut 丏=l.clone();for&专 in&乒[丣].三{if 丏.义(专){乄.三.push(专);}}}乄}fn 內(九:&dyn Fn(互,M)->亻,mut 亍:亮,俆:impl BufRead,佷:impl Write,)->Result<(),乓>{let mut 亶=伪::default();丮(俆,佷,|伏,些,且,乃|match
J::read(伏,些,京::侍,且){Ok(l)=>{let 二=乥(&l,&mut 亶);let 乄=match 二{Some(乥)=>{傆(&l,九(乃,且),&mut 亍,Some(乥))}None=>冑(&l,九(乃,且),&mut 亍),};eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",且,乄.侶,乄.仁,乄.両,乄.倯,乄.亙,I::from(乄.偿),乄.仝,乃.elapsed());儌(&乄.三)}Err(们)=>{eprintln!("plan: cannot read the input: {}"
,们);"WAIT".to_string()}},)}const 僑:Q<F> =None;const 儁:P=false;#[derive(Clone,Copy,PartialEq)]enum 伩{侏,亓,}#[derive(Clone,Copy)]struct 亽{一:C,kind:伩,仲:P,value:F,}fn
乳(l:&J,丂:C)->K<亽>{党(l,丂,儁)}fn 党(l:&J,丂:C,冋:P)->K<亽>{let(丵,串)=佭(l);let(丼,丘)=(l.丄(丂),l.丄(1-丂));let(伷,仪)=(丸(l,丼),丸(l,丘));let 伲=佁(l,&伷,&伷,&仪);let mut 万=K::new();for
一 in(丵|串).iter(){let 侺=丸(l,G::一(一));let 仲=丼.test(一);let 丫=if 冋{佁(l,&伷,&侺,&仪)-佁(l,&仪,&侺,&伷)}else if 仲{let 傖=丸(l,儻(丼,丂,一));伲-佁(l,&傖,&傖,&仪)}else{佁(l,&伷,&侺,&仪)-伲};万.push(亽{一,kind:if
串.test(一){伩::侏}else{伩::亓},仲,value:丫,});}万.sort_by_key(|o|(-o.value,o.kind!=伩::侏,o.一));万}fn 佁(l:&J,a:&[I;O],g:&[I;O],b:&[I;O])->F{let mut n=0;for c in l.map().valid.iter(){let
d=a[c].min(g[c]);if d!=I::MAX&&d<b[c]{n+=1;}}n}fn 仑(乳:&[亽])->K<亽>{乳.iter().filter(|o|o.value>=僑.unwrap_or(1)).copied().collect()}fn 儻(丄:G,丂:C,一:C)->G{let
乢=丄.世(G::一(一));let mut 乙=G::一(T[丂])&乢;loop{let 丅=乙|(乙.不()&乢);if 丅==乙{return 乙;}乙=丅;}}const 倒:F=10;const 像:F=100;#[derive(Clone,Copy)]enum 余{伯(M),乀(C),}#[derive(Clone,Copy)]struct
亇{丮:M,俄:余,}#[derive(Clone,Copy)]struct 伫{一:C,ours:Q<亇>,佶:Q<亇>,theirs:Q<亇>,亴:F,}fn 六(l:&J,丂:C,乳:&[亽])->K<伫>{let mut 万=K::new();for 丹 in 乳.iter().filter(|o|!o.仲){let
一=丹.一;let(兡,佶)=倠(l,丂,一);let 两=丕(兡,佶);let(冐,冏)=倠(l,1-丂,一);let 丘=丕(冐,冏);let 丮=|a:Q<亇>|a.map_or(像,|a|F::from(a.丮));万.push(伫{一,ours:两,佶,theirs:丘,亴:丮(丘)-丮(两),});}万}fn
丕(仉:Q<亇>,佐:Q<亇>)->Q<亇>{match(仉,佐){(Some(u),Some(t))=>Some(if t.丮<u.丮{t}else{u}),(u,t)=>u.or(t),}}fn 倠(l:&J,主:C,一:C)->(Q<亇>,Q<亇>){let 丨=l.map();let mut
仉:Q<亇> =None;for 上 in l.丈(主).iter(){let d=丨.丙(上,一);let Some(丌)=l.亜(上)else{continue;};if d!=I::MAX&&仉.is_none_or(|u|M::from(d)<u.丮){仉=Some(亇{丮:M::from(d),俄:余::伯(丌),});}}let
下=l.下(主)+if 主==l.丞(){0}else{l.丠(主)};let 來=match(下,l.丠(主)){(下,_)if 下>=倒=>Some(0),(_,丠)if 丠<=0=>None,(下,丠)=>Some((倒-下+丠-1)/丠),};let mut 佐:Q<亇> =None;if let
Some(來)=來{let 乻=l.丄(主).不()&丨.valid&l.乊(主,1);for 上 in 乻.iter(){let d=丨.丙(上,一);if d!=I::MAX&&佐.is_none_or(|t|1+來 as M+M::from(d)<t.丮){佐=Some(亇{丮:1+來 as M+M::from(d),俄:余::乀(上),});}}}(仉,佐)}const
倓:F=10;#[derive(Default)]struct 伪{偝:BTreeMap<M,C>,}#[derive(Default)]struct 仏{仮:K<(M,C)>,享:K<(C,C)>,back:K<M>,}fn 伆(l:&J,丂:C,乳:&[亽],仹:&[伫],両:&mut 伪,)->仏{倡(l,丂,乳,仹,両,l.丈(丂))}fn
倡(l:&J,丂:C,乳:&[亽],仹:&[伫],両:&mut 伪,侼:G,)->仏{let 丨=l.map();let 丈:BTreeMap<M,C> =(l.丈(丂)&侼).iter().filter_map(|c|l.亜(c).map(|丌|(丌,c))).collect();let mut 严:K<(亽,伫)>
=K::new();for o in 仑(乳).into_iter().filter(|o|!o.仲){if let Some(r)=仹.iter().find(|r|r.一==o.一){if r.ours.is_some()&&r.亴>=0{严.push((o,*r));}}}严.sort_by_key(|(o,r)|{(-o.value,o.kind!=伩::侏,r.ours.map_or(M::MAX,|a|a.丮),o.一,)});let
乆=|仉:M,业:C|M::from(丨.丙(丈[&仉],业));let 偍=|亣:&BTreeMap<C,M>,佈:&BTreeSet<M>,丌:M|{佈.contains(&丌)||亣.values().any(|&u|u==丌)};let mut 亣:BTreeMap<C,M> =BTreeMap::new();for(&丌,&业)in&両.偝{if
丈.contains_key(&丌)&&严.iter().any(|(o,_)|o.一==业)&&!亣.contains_key(&业){亣.insert(业,丌);}}let mut 佈:BTreeSet<M> =BTreeSet::new();for(o,_)in&严{let Some(&偏)=亣.get(&o.一)else{continue;};let
俏=丈.keys().copied().filter(|&丌|!偍(&亣,&佈,丌)).map(|丌|(乆(丌,o.一),丌)).filter(|&(d,_)|d!=M::from(I::MAX)).min();if let Some((d,丌))=俏{if d<乆(偏,o.一){佈.insert(偏);亣.insert(o.一,丌);}}}let
mut 享=K::new();for(o,r)in&严{if 亣.contains_key(&o.一){continue;}let 丧=丈.keys().copied().filter(|&丌|!偍(&亣,&佈,丌)).map(|丌|(乆(丌,o.一),丌)).filter(|&(d,_)|d!=M::from(I::MAX)).min();let
傍=丧.map(|(d,_)|F::from(d));let 侃=r.佶.and_then(|t|match t.俄{余::乀(一)if l.下(丂)-倓*享.len()as F>=倓&&!享.iter().any(|&(c,_)|c==一)=>{Some((一,F::from(t.丮)))}_=>None,});let
傉=侃.map(|(_,t)|t);let 傎=match(傍,傉){(Some(u),Some(t))=>t<u,(None,Some(_))=>true,_=>false,};let 儝=if 傎{傉}else{傍};let Some(丮)=儝 else{continue;};if r.theirs.map_or(100,|a|F::from(a.丮))<丮{continue;}match(傎,侃,丧){(true,Some((一,_)),_)=>享.push((一,o.一)),(false,_,Some((_,丌)))=>{亣.insert(o.一,丌);}_=>{}}}let
mut 万=仏{享,..仏::default()};for(o,_)in&严{if let Some(&丌)=亣.get(&o.一){万.仮.push((丌,o.一));}}万.back=l.丈(丂).iter().filter_map(|c|l.亜(c)).filter(|丌|!万.仮.iter().any(|&(u,_)|u==*丌)).collect();万.back.sort_unstable();両.偝=万.仮.iter().copied().collect();万}const
倔:F=10;fn 倭(l:&J,丂:C,丌:M)->Q<C>{l.丈(丂).iter().find(|&c|l.亜(c)==Some(丌))}fn 偽(l:&J,丂:C,上:C,业:C)->K<C>{let 丨=l.map();let 仔=丨.丙(上,业);if 仔==I::MAX||仔==0{return
K::new();}let mut 万:K<C> =(G::一(上).不()&丨.valid).iter().filter(|&n|丨.丙(n,业)<仔).collect();万.sort_by_key(|&n|(l.与(丂).test(n),n));万}fn 亱(丏:&mut J,万:&mut Vec<L>,专:L)->P{let
偟=丏.义(专);if 偟{万.push(专);}偟}#[derive(Clone,Copy)]struct 伂{俖:P,guard:I,倽:P,偲:P,}fn 仝(l:&J,亶:&仏,亁:伂)->K<K<L>>{let mut 万=K::new();for&(丌,业)in&亶.仮{let 兟=仏{仮:vec![(丌,业)],..仏::default()};let
佹=傀(l,&兟,伂{俖:false,..亁},);if!佹.is_empty(){万.push(佹);}}万}fn 丢(丏:&J,丌:M,上:C,丝:C,亲:P,亁:伂,)->Q<K<L>>{let 丂=丏.丞();let mut 为=丏.clone();let mut 三=K::new();let(x,y)=乞(丝);if!亱(&mut
为,&mut 三,L::乼{丌,x,y}){return None;}if 亲&&亁.guard!=0{let(仯,仰)=乞(上);let 仓=if 亁.guard==1{L::予{x:仯,y:仰}}else{L::乀{丆:1,x:仯,y:仰,}};if!亱(&mut 为,&mut 三,仓){return
None;}}if 亁.偲&&位(&为,丂)>位(丏,丂){return None;}Some(三)}fn 傀(l:&J,亶:&仏,亁:伂)->K<L>{let 丂=l.丞();let 丨=l.map();let mut 丏=l.clone();let mut 万=K::new();let 中=中(l,1-丂);let
mut 伒:K<(M,C)> =K::new();for&(丌,业)in&亶.仮{let Some(上)=倭(&丏,丂,丌)else{continue;};if!丏.乿().test(上){continue;}let 亲=中.test(上);for 丝 in 偽(&丏,丂,上,业){if let Some(三)=丢(&丏,丌,上,丝,亲,亁){for
专 in&三{丏.义(*专);}if 亲&&亁.guard==0{伒.push((丌,上));}万.extend(三);break;}}}for&(丌,业)in&亶.仮{if 丏.下(丂)<倔{break;}let Some(丱)=倭(&丏,丂,丌)else{continue;};let 儑=伒.iter().find(|&&(u,_)|u==丌).map(|&(_,c)|c);let
丒=match 儑{Some(一)if 亁.倽=>vec![一],_=>偽(&丏,丂,丱,业),};for 丝 in 丒{let(x,y)=乞(丝);if 亱(&mut 丏,&mut 万,L::乀{丆:1,x,y}){break;}}}if 亁.俖{for&(_,业)in&亶.享{if 丏.下(丂)<倔{break;}let
乻=丏.丄(丂).不()&丨.valid;let mut 丒:K<C> =乻.iter().filter(|&c|丨.丙(c,业)!=I::MAX).collect();丒.sort_by_key(|&c|(丨.丙(c,业),c));for 丝 in 丒{let(x,y)=乞(丝);if 亱(&mut
丏,&mut 万,L::乀{丆:1,x,y}){break;}}}}万}const 倕:I=0;const 僸:I=1;const 倄:F=0;const 僒:P=false;const 傰:I=0;const 傧:P=false;const 僤:P=false;const 傚:P=false;const
催:M=7;const 僻:M=14;const 僫:P=倕!=0||倄!=0;struct 仿{东:K<L>,偞:P,start:P,仝:K<K<L>>,乳:K<(C,F,F)>,会:F,}impl 仿{fn 侂(&self,l:&J,丂:C)->F{if self.会==0{return 0;}let(丼,丘)=(l.丄(丂),l.丄(1-丂));let
mut 亐=0;for&(一,丫,亥)in&self.乳{亐+=if 丼.test(一){丫}else if 丘.test(一){-丫}else{亥};}self.会*亐/4}}fn 乥(l:&J,両:&mut 伪)->Q<仿>{let 佂=l.乤();if!僫||!(催..=僻).contains(&佂)||l.並().is_some(){return
None;}元(l,両,僦)}#[derive(Clone,Copy)]struct 佞{仙:I,亁:伂,侪:P,会:F,俓:P,}const 僦:佞=佞{仙:倕,亁:伂{俖:僸==2,guard:傰,倽:傧,偲:僤,},侪:傚,会:倄,俓:僒,};fn 元(l:&J,両:&mut 伪,么:佞)->Q<仿>{let
佞{仙,亁,侪,会,俓,}=么;let 丂=l.丞();let 乳=乳(l,丂);let 仹=六(l,丂,&乳);let 亶=if 侪{let 侼=l.丈(丂).世(中(l,1-丂));倡(l,丂,&乳,&仹,両,侼)}else{伆(l,丂,&乳,&仹,両)};let 东=if 仙!=0{傀(l,&亶,亁)}else{K::new()};let
仑=仑(&乳);let 冎=仑.iter().map(|o|{let 亴=仹.iter().find(|r|r.一==o.一).map_or(0,|r|r.亴.clamp(-1,1));let 亥=if 俓{o.value*亴}else{0};(o.一,o.value,亥)}).collect();let
兣=if 仙&4!=0{仝(l,&亶,亁)}else{K::new()};Some(仿{仝:兣,偞:!东.is_empty()&&仙&1!=0,start:!东.is_empty()&&仙&2!=0,东,乳:冎,会,})}const 僳:&str="׳椮浦粜顯漏煵波螲〴ꁜ却⬖ﾊ瀄颡뷻쓓퟊㧉鵃ꨠ渠诡懕豐퀊틞ⴂ⦃맷꽩灹칁쿜埓㞄뾢ῂए蕰穱茄⻯逎黷䒌䝷㐺亮φ䨋ᢞꦏ㠬Տᜅ᷊Ⴒ꺬뱌蔆悸ቝ蘱ꅺ쏉묇踗榴뾵碴ꀊ˯᮴莈繄探ꉷ襩鋊㍄탊赍䮟ᳺ囔ᬡ狮죇ꥻ镵葛⤈齢⸫ءؚ矹쭐쨒걕钣有㔑畳짉谶漰᷄纼뜺剷Ĩ翾쳉鍞ꋤ`柫볘뇣駟氯襟弸僸洂౓먨৛罆쯙㒫銷驯Ღ꽖據䋉ྏ护뷨䌞鎡篻ꦆ轛ડ蹮ኌ팱眄얜ƻ㖪ࣩ樖讞魞컙旻ꯚ찧鳄㯹ť캹剪詨댯画帙᪒爠盭늓ਲ켃ࢪ琳ೡ懊吾᭹睆줾䡋獙㴉봂➜삜푎郆થ괬⿬ṓ悊ឩ堤湹潰੬뻹틫⭸耣ધᔉﴱ垭䯁걙ꊒ边氤Ƭử㵒觷쉴䏉宎患䭟숾嗏橣Ⱃ鼼廚鮒鍶ⷭ求㻅穻빃咤㸱皪䑲ﾃ穔︓컝됶㜰ぅ妼梩鮯ਚ梣닏謏척漥⇽阔ﬞ廧䋐旟昛ಉ⬉㤀픲㒒薑间砰闈䛌Ĳ올媯ᩩ泈朡㕵饚ו䜛熋谷➭쌳떾쉻暻䒭ꅵࣳ᡺顗䎞᳃习뺰⡻樸❲笞ẟ㩴⛐䱿뽆喞ﰘ袃涸泮Ḇ㕴គ＂₶㑮툀혣힓ᤀ繨ᑧ벳娘ⲧ렎⃽䦖˺ಆ殜涝齫ף茒ꚙ䙫ꤖ諓此≕봤∲㣾㹥팤ྟ펀ᜱ荬犅䊠鲎셎㯢똺㣫쵵ⷍ绕뙁ඍ㾞톴骨Ӛ널㞼쫄貶㐍ᘇ㼮퓌㨚婠䦇⩣ǜ႑譇瘺䟖煦䠷۠ђⲤꬣ镯挎䒎좿ᙼ㷹꣊⮐槢뉷ຣ힥䶏ꄈ睍Ꮷሮ䎽㩭굗彦橞䆨❡ᡵ澭䭼ꦅ宒鐸옖回雸岟礔錯Ż⺾⢸꾻䪖턉᧟떞醌ꓘ힓ⴔᒒ鞙睼쥟ꐼࣾ쾄Āħ؁몱▏ꄧ䑎㳨獼뷮ꎀ찿ﰈ厓਌珜ⲵ啫뻫灕湘ⲓ㜓曩ẋ燛絣瑭㣽鳮〮羌鏥∶˞琓੆걄⋰㲉ᮩ햀涣瑏渐骩᳦谊뽬⾅浇થ⴨Ą釔틯ꁶ캫㐜㇓Ό灕妚ꎆꁦ袈粹旯䆿鎮矛뙰첦䫳ꆍ薿馻Ⴤ徚ꁍ窳ꮌ轞ɕ⠸ꗤ䷸ꇈ밙ꌭᚈ꿑헦碗୞쟨ₛ㣚援篴⃢㞠⿐崜쨼△쮿♌ઇ填ċ깵莑퉜娴凌쉉ᡆ紐惲Ꮷ⛆筄䆓ﺳ磐菀ऀ簉ܝ豻꧿쀇瓢ᙄ╸禗䊅ࠣ싈벑㪒ﺜ쾵흛岂ͰⷫԊ㚘扙嗝⦦蹰뎄ꖤꃱɖ泡誳冡韬嶁礀㙐ﾷ졗ꤟퟚĊ⨈쑭៽廆喆旮燔ⲳί枖㪸캔啟㔶ⲉಧ밋쿌펍搒캋洋轺膹궯⛏턥褈俚㺱┷݋䠧꘢旾쎜鼾팅뒎䫦怫풴씰杼眸礤ᥐ뗌꛶͞뿸㋚孃衫ｭĲ鼀䳖䩎캢ꊩ耭堠裶᠖榉灾뿼楎ⶦ⟭௯쵍悊邷鮤⊝炭卺ີ噕뷯뛧뷴뇔ᆼ艆줠䞓푱嚛䱴禿⼮燨天䡘ꀀ뉙슯왦哎꘏✚寷枘圫꿇┫籍㒆Ĕ妰㤻뵝霂Ṻ뇔숌ų颃ᦣ돶꘣ᗲ㨰쎕䕉ⓥ⩕霂ᦕꊡ齾Ꮆ઄ɺ觢쮉缯因蒾㩳㵟䲤悵쒀⏥ओ泀ⶲ돦瘩葑點獇蕑걱嚆뇌찿땀滐鼁ӀƢૹᡂ⫈稍理䍆狑힊쩳䗁쒏焿ⷐඉ혖㕟ﲌ枠极養䝕믩臘熎籩齚눳氁ꞩ링쟽珨㝐ף湑ꆜĒ뎃덟该覕呋竹囎ꕎ㈕倭毒嚁䏠ﴹ鑺퀘᳟䟓渺䪁懩Ċ侂췴펡誂萪ⶅڷ贇枠ﾔ帺阷勯銫릅붸ᝢɳ䎛腓ƃ⇉혵倚鉄龚┨曞肛ᬊ洏횷䀷璟뺽픻躔탡騂諯㎂犍ឩ륃㚃낉皕엽멽呐嶦딏掠伎罹芸˦ᙎ咙ぱ䟹ਈ苕迫ⲷ爭ﭽ뙿롏ﱽ᜽ⵉ晚ޤ偤艹꼈숹⵾൓ྠഡ杁㩰㩓ꀘ얝愋⓮휦炘Თ㕑溞Ʋ埓骣㬋࡚༙ꡯ彵곘ఔ၎誌뤻鰩톂ꦢŦ螠哠喐붗趓鎚贰ᚒ꠿扣桃䔿׋쌢뜢녢떯⾤乞ꓺ廙橬視秳亠㶲金띅绒ᑊѰ푿鹮牱䶠⥌Ɵ뫲塰瞽ප譓곽龄㈚ꍈ洱ꠊ嵂妩擿㕊钺摠霦᥅Ċ揗荁㑕ὒ婧勉뇁⤡퀁὘絥闅譡ቜᒴ耊隘顑솞溣䈧喐鐡具怉ㆍ䄻↉氍᳕巏㿀锵⾺畝ﺓ㌘ﻀ䀧政塱鵭뱲瓬퉩꫄庙གᄃᅃ挂饼ᤵ䜦㾎힂ᵠ໛Ĕ俦孇囬酵赋ᦳ줰⠴ꯐ錂祜歍᭍튓혀뚠⑋乷ᵱ圤屌媇ꇯ頊䳞♝耥朁❨じఘ⬮ʑ᎕ꬦ交ಚῧ鑾氭䯷鿭퀤념蕿䧾긏嬗ᢜ獚ሏ摒休ᑫĞ睒ⶲ䉮䁬䗹ᥒ셐ᅉ㨓驽ﻊ鯞砉铏埱ꟃ㥋钳돮㰪҃ໞ흿運㈩⥴鑸嬴拽㒽髀麶⢄왬䄫孻㫷ഀ蚵揟邏ᣋఅ訤樌遹袎询榙걒啄퀚㧰襄⹟ɮڦ嵍슾ȹݰ晣毁Ŗ貱跭ᤁ臝㚋術紧득㥽੾軌๩빽깜ᤈ竾䑻橣౗帉欞팵趽삸歳灙ㇵ䞜磖長熱츊鹫઱↞懇䪥◺䒗㳆ꀫ㸢뫩⸶똤粦ࢉ進뛘裡힭Ǟ껝걒箁줊뉽恹陊丬患㠇痖⇊탔誉욯⹨밙뵡勘Ƕ㏝쬥씪爍ﱮ뮫䔎ꩢ抾묡薡涅氂䅄栙⫊㵮蜾㠃牙埯ꢤ⁆띏伴館犽혌꒕㰠봽栺횊觽숐䗭뿝縅䄑Ċ鏆䢮᛿凚懅ᔆ㺮捆嶴༂쌎䉕ʹᓛ勅ᵝἙ噰䖛셷ᜅ㺐汸番⩜⚘젎叉䗹腓ꞇ퇝늿軽ꭢ궰➋헢⤳戸햖峛鼫乑ㄜ妐韍纏ﲪ涊世뒼Ọ靭顭ྍ㚨탩ㇵ돇ƃ늭ㅝ稷Ⱝ┹獚経쥩ꀥ헟鈚ᶪ푬ᡦ䡨㊕㼤೺鲞ꢵ뙈Ἃ熿첁⪌ޢ杢魖兂짼迊欔ﻩ蟢洴䞈簔皽槧彲澣擝镧欭뽅鼄䕈뜷᷺洇ﳱ愘䂛볕ሦ㿷蟶鹓䰽Ĕ묳眷Ř䫦䁈㹼Ⓑ牉梩娮ↄ뒊囶⇷꽶뫉韭睥읷썊謣澻쳩ச㤂늢ǀ裯巴돏胫囎돰캎㉥氫籬ꡲ澑唙蓮뼏얳ꮞԾ➍뭞귭襗⨁⽜亼黮睾锗ꘗᗝ憎䚃䙕㤶Ğ硛Ұᴋ됲쥫矵鸬䫂糮骶⻬䡣ᢧ驄ꨝ䴊ࠒ爋ᜎ侢悴辧添घ惙査턪칿圐筢ऴ㤟䔬륿ꛀ婾銷诰렡奥睛폏宣鏰맭ꬠ渽欈ㆴ帞ባϷ寰郷䄯픞牭ᙑ붨أ딩擾夡噻㢷뱜漍緫㴪⌡並䈷덆ꑥ苂獳춆ힽ≞适딉咛挘鶳뵨﬈゜ᕯ㯮넰⼗৅䷱Ꮒ稰裴키賬栬옧ǥ外䓴㸂ꆐ毭ﲺ磐⪶몞ㇾᰊꉘ஍釋礻ର還境ĩ坵ᵬഅ▋绳篓ꃱퟆߟퟳ넹駕챭騛규嫤ㅮ듨뢠剽媡㞃넍뱅䧛㙟洄޸㗴᜵헭귎㸀폂蝜⇧减뭙빻뢐꒽쀦厅뛑쬑鲌푇㖅࢐㈼듨흍릷⵴졺劆Ĩ製ජൽ陾킛娭ﾸꁎ玮뜌癙缃童໘输溵⋴Ⴌ갟ﱖ㯇烘飨枬⌿᚟䀅䒡蚽䂶㲂高₾㹧瀣ࣧ荋䐩둙觅鹛鸭霒㺍땭齍៓孔뎥ˉ긡ꊁ䷌ꨮ擋䥁ǞퟘĽ쵞捶䳁墨Ď矠૏䑣᠕畋鬋槻䳔紮秠鬆閊ᯋ䣿ꝟ散雟鑀㪚琱⚰鍌⢲楋謯屡䩙됟ᢊၠ躴嫛鏆삟★୑罭⤍˙ꗩ巔ץꛘ忶ꪧ騅⺠㆛ᘧ∐凋Ğ睒蘀呗㭚ꑍ飡渋눾둈坛㔨褸ⴙ轖긺赲봇킏⮀⠡極舓鐃⺗⣺蹱欮팁쎀꣘뚟尊䖈⯀౉哬컐贆䎖톏࿽澈财⬾羊㪷钼蜝繼↰꒨ꪨ尘┎較懻㱎Ğ뇚ⷄ栈쯖冬鋰蔈橵♙䟧ƀ煄뵏ﻊ䎑㪞숾鸥垸ㅃ㙭᷽ﲭ늑谂ꖎ鈌㭷ᗬ蕻훢털ꉼ쪕㍂줾왐瑍篇笍厙〧⮀䳋༞픨悙ꎙ㳒큠䴞厤뭒㓤콭붴߂躺爬␨酫ⓟ䒘꧘錋흇垸輻퇙筂鰞ᶿ⎍味豜Ⲫ궤᭓黫ྂၝ༷鮅ⱹ懞奈䏪衶惤⩤␍㣪쀇쩔湃攄崣ꊮ䙊麏䝠携ꦋ蕋䆳鯒憾譆襚잯땧룗䰵Ċ侚騳⯰謝ॼ駕ꮦ폽儽䓢퇟Ἢ疖평늞躶廈㇅㾽㥭⏦൥楝짊峝껅꿙飃ꩣ⪨랆澰⍞骐䮓믔척띎ᙎ稢؇ഃ촥㔄ﳰ䧇格➰ບ撩༱͕Ῥമ劘㷲パ囫᭖▾㱦ෑ튛馎㑢종︰韙퀏㌢븣䇾聆襆觀뚓짖㪳釤ꦱჱ稙䍣黇錓䜡橂럽讁碲ʮ㏷좌ꃟ첓崶燝쳅ꭣ꟦㻧収햁ꧦ됚鄇咷૘ʁ鲹ो鐍周慧灢簍㱣餡쾚⚫갲銖̍媞ⴹ뫴㫤뀡絠්則ᘼ峈䪨쇁ᔨ줮紶띎著餢朚ዽ띚삫샲雥纭䨋涫䄵ᅼ뽌吋㝍袭ꄳᘷ⸟嶿䓏⒢ꕤꇘ쬽栛祯Ĩ齝綱鸕엁昪勅폦娳톣竰誾ꊫ걨왱쁎픾ꇆ톀秹௔䛲Ϗ澧姝᤼僳椐帀ീ蝛൯㝗✨㑼卟涍쑞杴싹㡶톿杈翽ƴп㟆ꖿ튤밻墔䍵♘ꟺ㊘玳觏ﺫ쥂湔亼Ƥꇐᇚ驳씜ﭞ曝ഩ嬳薚ﻓ樇犎앐廽᪛⑌簪띶嶡渄ḯ⬗錹샤풜⿸㚂묒䳄ⲗ䌅㊦黩儅ॼ掻ℝ蜑㽒Б혆ܷ蓩᫕碭⾾≡䶘ᖴ첊ҷ䞌몞妼휆ꕲű㉣뼋汞苛Ĕ俩穨贞옟佩ꌁ쳍鰿㲴럖賶ᖲ쬁壇ꦧ숍鴲┇鎷⌼뉫囆︃졈㋈ⲷ唬ꘇ鋾䫗ꢛ㺇疯⡇梖澷稅욟㉳৵織뛖ホꖾ骎䲺蘭灖斏목⫌ऍ앩昁Ƙ觙畻΋雿᫗ꔃ嶐⒥￧襱摒쭳䝞뼝瘙ᘅᰠ﨧蝜큌蠏緶⤬봭䐡䱃෥삫뎖햄븴㧇迡忪뉺앬⚰煃ഐᷝ휰膦뀭厌珈ߧ켃Ώ᧯ᾐ븻ğ畆蛜ณ༳宫ᆒ鳟粜⟔ご蜵ⴏ̨鰊姷挊荰囆廋ㆁ퇼﹍ᓭ쭷泎潯都⿌忨욎儉쓓᝖泳盘봧瓟帔嘟⹲╷뿤䌶顝餃藉㼸깤⸅⇣䦜Ⰳ땒ӕ惚磖฽ސ៝䆰넣施땉尋欯弛䵅懠垩쨫鴂㱒ᦥ誜䏆緭磫㤆ᩭ䕈羡粿虓麬뗅ꓰ㧺ർ彌৚ꄊ啛γ兞붘碒̗녡칪퀚竡닖゚齛౓ꦉ솊坻᜙遍㼣銂◉ἝĲ올嫔쯫䶏އ醩俄ₔ녰骨䀑鼠퀲뜸⽵㋦쌽泄䁷걀慡鉌列讂쒏ᏹ䄲驼憷철﮵⺢硖ꖁ됊笥廚쪹씡拵諶澈楋庯襾玏硢ꨥ׶쵏딝훨涝Ỳ㮨挼瀌뛓全縘Ĕ兾헆굊⢴籁Ͱ筣瓢笗꼎呣➤橦훣ᇗ噫닣䄤楢뱻ⱘ휕헙魻䯗璿㔆颙ᓢ䱳⯗쉒籥᾽텨끆얄䱪혰㬴ꗨ⋉崒컂໾汨鞟㬤챑䨉读䈻輂ᵝ몏˛뎔膤꿓ᒷ죦㴑⩆墊儙鴐몎滵ꯃ㹈茏찙终褋僵◓隅鼿뢚䰼攒쭙옳騤趤⊯ኲ哣꾐嬺樀ᔖ뎧魯꠶䒽耫簒㿂洁㹴䥢酄﫳啭騽爝隯큮ࢦ⡠Ċ넍ធ樳릕术綢︩ザ뚋ꋥ猺谙말ഐ漫ᇰⱓﯙ芛晰赢Ą熡Ⱳப▝ｱ萨跕༰쭹휍趦ᇩ䲇㬎핗ｯ뿎鵋껢쵹ᵍ뎸襮㬠尨뺅ﵪ톼र액䒇燜ퟕ鈘쑳ăᚽ잽冋爿ᆟ蠔申眆뤏㝏삽◙諺殳傗噊鵃뎆ೋፓ묦뼃虧홠剱ƌ餳깒ࡡ鷀Ǉ↹몗鯴グ૽⌜鍙㶔廾ᤨ਀菗㹤㲿孆龹⋁㢗碭㸻곢䩡賙࡯᝝”귅ῧ㣾竵罫孍땴⭜툾명⨧ﷸ郂ゆ㱔ũ昼㱥䘒甋꘿绻布曮鼀쎤뜍낒ᄿꉪ鼳ł鱟䄢ꀉ兕ꍻቺ仁耱⧤㱾훩㳮悆⍙瓼뺼꓁ᯘ᭪濾頺掜ㅬőĴ㬺쩢䍗ᒄ㬔娙䶗寜퇦רႭ䧓띾䠈≳ⱕᥜ흍骕▥ἵ㜸䕹懠巾ᐔⶤ扊쬏௏ᜑஒ䌰눁⾞䢗彵훝塛װ怂鱍鋊鼦损ᖇ몛찥踁쪆틮䣿솆佁⼩뀍⌦Ġ栜錮쀅挣Ⰾꙛ秭竳蝓돈嬎ศⱈゝ맯뤂㤈‘㫔ᐖ畨㏐䡍拰ퟴ⢒䬗ದጸ浃ྫഁ弧䕡幘ꡐ㡡廴䉊ᷱ亱㻊艊᫶ܶ뼌췵ꂦ橴濭Ⱉ버㉅빗돟࿃嬨Ԝ赀⹫䰶뾘㮞菽伸ⵀᾷ祝㻧ｻ籼鿯쪩瘾ٹ秹嬳ᇼⶎ쎓ꋽꎕ㌩캿㈵㵂Ͻ胅￠嫤♲텶驫는꺗矷뫮橤ᎏ䂢椛䯾傿偽Ỿ嫄釟ᥲ甩謥땀₶痍葂♾쵈蛌편̀藣拫蹀톛召ⅶ㶏䜎䩥ೲߝ妚㓵渠Ḋ릓꜏౐뛲悴㽤ꗤꦝ퇨멐ꙇ홊揵ܴ跬㪝舓䃪⥰鸹廪细儀﹦朩䋙︊숞溤꽥뾂仉㫖㽮ᒀ룏뒉毀額ㅩ☓햋ฤĪ咮댆䊗⩁ꄶ쵪䋫㴺閜◆왫滳罣㼴ɤ竘髐檜갖⊃꒱ꆤႠ㪻皘슀팰難ầ㕌睉쏧㱍ჷ姶୯姀窷ຽ耭ᘽ⌠鑦쀢霪倁ၙ췞츃㞑㶠醯Ĩ䮼㵓⿍ޤ᳴ሗ槅︸攸ꏽ⫧庌᩿⧲怹竿䤂㓨䙲꛿抢郶봣执걇왴Ꝛ궸似끘釡膅䁨ሶ⫺韑ᖹ淼즪㯻兛隤灵츂퀝ᜂ턣相炁팤뚴扦囫Ĩꢊ喞饈瑸䗛뜂೩苮斣㪇ᐜ縒냣쿳啲撶㕭郾활ᘔ출懚戗볻駂걶輕٘ۛ旟ශ꫼癔ఀ彜겂雿㩲납臂ꇥ칤ᜥ㐹睕剭︊慈ކ듒蒦Ү쒝쒀⨕ꗉⓂ談኱꛿䵴Ė㚜ʹ紁䐶槞㕁쫄諿㉣孿ꆦ궗夙肬斔ﬄ辀ᢇ瑫Ⓢ徳稗辭嚃朠馽⮧匭ꬎힾ㑩⥈퓘瘺ፊ녧䪍쯂쪡쩸萼Ჯ抇㟻ᇭ᭢ྭ笕靹엃서⮮㓔텚袁⓲ᒒ졗Ĩ黁텚콰旓䙏姵੒䵖⢒桳톼릮蚂鑖䑥薴ၾ씖묚䔋ᱹງ竽럤ꀲ㕀‾놻⽨ݡ岗铏ܻ耽맍ⷬ쀐叹䶻侚腺㉍섛葠ࡽ蔘㗑㾵侅紹끺〺೑哔얬೜ᩀߵⰽ뢶㱺㎐掗滣ꥺϳ誐鎸ﯔ垅ᾳ呑ڹ㙑⎎要易鈜赵轍䦠瀫춓魵袖﹪쑟篎ꑪ㛦鄲䗩쒞䵧孴䖰ꀣ蒐蘺饒閤᭫ᚦᢢ⫝̸蟕昶睫耧胇畠Ĳ욄⬓칱郸䵱᷁ꔷ䁢⢐๔ﲶ슝秓嫌⼆彁끾⊌轂눂下不ͺ뒆蟠풂牝릹隡뚀芖混鍠酃䎐℟㛅✡ﲡᾘ⪍뇮物瑴源ਹ彠Ꞽу㰗❘뱶鑫槣Ƈ몢ꖩﲤ쎡㦩៩᭤䋖䃝荾罆죔镏ⴡ梜铓␔萵ꦊᦔ͎豊㇙銴㿿脮髞⑦銣羸部匽橥瑴垉䓢齦撎랚辶ￂօꏡṊ㥪ⶻ⟷扈ᥭ뾾얐澢〔㍅ၶꭴ翊ť俾皇硷ꖄ갽⻅튶ꁢ䁶襇涳獭灵턙榑璼兀䡃宀福ﴺි⡘➑ᐆ鉞숦妌쾡뢝舘☚ￎ콋菪䯤嫹⼮刋甆譶鄵ތ㍘⦞怿寕쬋迍筬쉽虯ꉭꬷꃲꝃ去ꯏĲ옭⧧녳禪ꉲ斉нݪꎍ膘睞㥉勴귥ŧ쟃⻱琙䶴ⵣ䀲庈貾Ꭼ닾眐蛯㣿붺镈昚꺰攀剢頻ሴ킛벼崝郋憜쀭ྟ侪㚌퉪탆ꪍ蟈猲塞诘戥韔Ĕ倕䬎ⳉ㑙ꎊ糃퀠콠逍ᰕ륝햫䆩㔚茩徇೘懈Ꮠ߾퓛떠ׯ읹膘谲ɳ㲼⢰屢в榵Ｃཏၐ籴爠务뽈掏㏳蜶妸杴뀻標霌੉࿁좬銕ҙ價傯濨䴺鮗畏䭋蠅Ἣ릷썾㞭麆켔ᮖ㖉ͳ᏿㹩葐枃좢迉䭃옙赥य귟鎮㘛ꞑꠌ怢ؤ⯔셃揽ḛ鸃ﶀᕥᛎץッቃ擆ള㶬攗륃峕㈕➥垧왚硐Ċ⯹蜭伢껷郣䒘殕욻ї⋻꾴٩챊탪茟╞⤽腍㻤궸픢쒚妕囇늱廪睡䖑벧ڮ耉❶븭጗狧믟淳ᏹ๳돸⳿ꋑ홫厾慝歇柔噻閆燭씑⤘媧͟ﭢ羀ɳ袄♑㬒짦꣇旇䥤袷琍첧験⭝ԇ瘛콝禢焧踏펖ｖה㑗㵑惼ዹΧ먹顱䏬⍶ᣵ机뒺蘝줙塱ᒘ祩쿀穟➻ꕕ鞜䠧ᑅ뷪뻢ㅈ葑䭱띹鵞ꨞ筚㶤Ĳ쥌䘛䝸䦠喝Ꞻ縓徕䫀긎㪅⻝䔳亱▇ꃣ嘊⿺肉祘垷ↀ嬀귲髎͓牏閾탯ゼ೒拉灪✶绐磨姇ꭘ쌺껥耾ꚲ퐀ɀ練骘優弞쾃副셡訧❸묣ߤ塖倝︉꽌쯭맧㯕埍㽴﮻욅ꓩ䟴⹞俞긮咉腀쓭帤뇈薈趍봢쓪䯯랤뉨할텔壳ꩆ褴얷຀렺㤳⚣떫칝靕㴦騪嵉ㄬ䨋秡꾦䯣咷溾ḹ䕩彿컆쾡謭⃦Ğ瞂ﻄᬼ쉲阺᝱ሒ玊뀙틶똹㋬軥ɀꪅὑ嘿巃の皰윥㕸⧊廠呎ޚ㣨ﾧ㼧횰ᴍ璒᪇싒묣㛹玓쀝朐렣䬜灯䕄虍俢⒇狞䔗ퟄ馄輁䶍⛧Ĩ鼛틜㭳㋹ⴓ횺椮Ⰹ吰ꈇ뱼拓ថ≀㫰齁ﾈៀᥪℍ괍躜䈬㊻놳蜾⧆洸銬処㫛鉿襉㨗탟仆웄꣱⠰䛆ﵱ櫰녟돧슗ᑃ盦ၑℭ餣︕ⶺኩ⠶秲䌯抛Ĳ뱡䞂꥛·ﵨㄯ껲ㆬ覵刱轇㞋叡ꦮ薯㳦ᴂ伂칹㽛ඃᠫꅝᜊ璤餏ﰴᘗ඾뼊꓇㻏冫䷣穃鈏醅உ옽츝ꖹ궼饓酳㻔ᨭ笂ঢ଎짻ᣏᡸﭕﬗ껄ꭲ႗Ğ睒鹙㞊ꛎ쁒ⴣ狖簄飛✀茜籋篇оⷈ಍ꥢ͜剎ᯑ䯨ￕ貃捒雴웯㾘ⴶ児팦쵳놰ꑬ鲻岩ꎮ䬔쯜衏긠迿굉䠮郲顇㖬釆㛇ᩁ䠳먍𢡄ⱘ蝧䒏ȑ旂칞㡓䏤쪟耤葸㾗ﻐ褐ظᗂ挱ቒ愁㡾ﲽ憎늂徯䖛Ὥ㓽탏伄ꭗ涡ⅉᾴ藚᜙羙਩・紻土ḿ䓷菻䔻뀴嫍㠞܀ꯛ㟨蘐炉宨ᴃⴙ柗䈿Ĩꄈ鳝筆ᨇ痀㯳솧삯붜ᥠ牀硑揬燭莄↨뢰雍兦콊镧哬ཀꡗ쇆券먢뼒伭麵燧〙矩匯鼪䶅፮⯮䒥뢘࿰껜僴㺘腧ゃ샜첎䓻贏嗟柩遜ﯮ⾎봾菔ĳ鱄髣凃ᄧ䙫㫓ꆪᙉ慭憕ꒊ貮鿨穀叜隫猺ᐪ悂㙸᫵尝햊㒏⛫뾱涫颉林吃⺃ⰱ㺇魓뫪料밂史斍뙴ɗ爾၆Ἳ鏫炖᝷槽㾷晄ෙৱ崙ᙾ죜얪Ĕ冥넰␭즕씺搱ў㍱赬䲣峐咶༿釃㣮璯硽䔦冫ꊐ⏼繵ܒ㹨뜧披潷퇀뗊胒섞秋餡㧜씮๚蜫피ⱷ唗㼽ꌝ癋푘尿쵧␋唚녭〚ㅜⲻᔯ좢蓙俰稦Ĩ麿䣟ꍟꞎꂚꖂ懲ꂯ㒓씘뎂ʬ᥸뮥䄝벚ޣݖ褏͜蔥㗒偻뜋熴ꄬ抦㳔ᗨӽꬥ猳杕홴曁烿蠹ल驝ᆭ鷚慊卆Խ扣漰㯃ꔦꤸ戜廑褵惵ꯒ⢛꧲䳞벝튓Ĕ俥⛔ᇞ褛脈服啀菇떢憍瑍㙷褆왹軬쭢殄尹ꌴ⟧쿿ꄉ塜̚蒒㬗⪚猌ᢔ몗̚ꀬ⩚츜ﰧ콱틛땭ᔑ侶䏖悪犋迓僘⧿䊣慱鹟콖랞ਏۆ䲼儦逤﩮홪荁Ĩ麿鶁耷䭲阕邳튑䦫㽆痠䨵蜴⫪ᢍ춿悎丕뤦榏㑼抩ຍ骦䵤ⴠ새啉䭍諝ﴪ폧蝒٧ෘ鱰꿊᩸㓡弜㬿驤혞䚯폟的艩넟藵햚﯂甥ꀦꟉ醼뙯䟬뾆彀ݓ䝎괟唁췻䎦Ꝝ柑흎凮峁ጇ냶냨ᅲਐ⚳덣ﱐ１ᬓ饒뺘ഽ䄑貺⬐Ṉ瑈趈퇁ㅻꎜఎ뤇劳Ϋ兟岓굌뽛є泥䝥鶣季쬠ꠣо䦌䝖統ᑰވ殘ꁆٮ搁ĬꟄ屜므揋䥵똬甊飼폇볍徙థﺼ寚琴谗⠔쟹俢䰁涎あ倳̷ļ꠲阊鵳宓ꀫ㫪˳⦪Პ䅠ᵺ䤺竪䀙쾏ꊛ鞒뒘᫟Ģ慙瘨瞉珮顁뼴ヲ哬Ĳ올寥脽拉쫻魯㾭膕▍銰䆊䤕쌶﫪褵ꘟ甧鑩巟杯癙垶㧹↾ۏ濪惮㳍쥛ꕆꡅ픝ᔉᬤ忽៘⌛쵙풤ᴐ匥⭩ꟹ᤭쪫רꊸ띡慙蟶⒭羒猄௪䇥鞩꒜Ĳ옳稻ਉ＾ꄇᓴᷔ큕伭䰇殉耮㩟庳༟⡍ꛇ᭵渾錓ླྀ唃駴ﺓꊕ鸤갍Ȧ嗝읎僳䧦쭁맱曏ᏻꨙꍞ瀱覆䟁鯦䔕Ẋ쬰ヲ긮里⴫媽謖了把尒㿩壱ꔶġ閬ᜂꊄ展䄶偯蚰ↅ뇻Ⱏ贍彸萓笙ȳ毚伒诖녖悸烼⺴ﯭ쀶ໟ᩶윓옼팕臧略宆뿢뀨좢ὔ駗໘㙚䠭챑磊왌砜ჟ轕闙乃ꚅᨔﲦꙜ茵᮶濠錡쮉蠡ĉ岪ᩇ柚✹⼊᮷姬︡诚㷎菓뤦霙崙駣腥Ὣ˱媩梎↿ᬜɼ硱餱嵑ꖼᤩጟ飫툰魕曋ꓒ笟画떊긋퉠哥웋ꛅŕ⻜톟墲ꁱ쌩뉱띹ꦦ穕苂걋⿻熥ﰥ־霿꽑겷ᗲ圀遷ꎈ뢞詵᧗䫒뿑릘ꖷꂶ鮂㪌黺툭ꆵ샂僲࢏벗쩊薏䁭쑄㲨ග〯쳏ꂢ鱡軖왘ⷨᤶ퓽㮖ཚ㠡煪ḯʗꯞ陋껆ﳷ芄胄ꝝ핕幂ꄥ밽惧桮抵栋Ĵ쨌爭휏㋒鎭ဳ置䟅乼톡₄ᨽ㟍ꇚ꘳涸恸坞㎵踹䏙ة矅벊ꄄ湬ᣎୃ幌᳆爌韈ꂳ፡ﰆ࿹뱘⏫ኍᩔ⣘韬ꬍꁵ씒蝔눦㾯恜㎟踧聸㖯愄轭➮ӂ㉩웂섵Ｐ饂ꊕ쁰〳吭훠ꐪ欴్데꾎䯋鞯麄쬵Ꮟ쐄剺⤮楟鰂묊拲袆䕌捺㰃鼑⪖뵾᭬෨섷Э篷㘻》ំ嬺荂㞳ᢢ럽艮顑ﮗ傚௢뵥䥽䥄˭㞲⦮鿬Ęﯷໜ஌颴巵曝渙᳖쩼ϒ餰㰢쮒庭뗭訕锫䷇ꁲ皉බ⨟ꭕⲌ외츫劌舆֞辕涃謸歸⬍᝾꜌핯鼔ޤມꏾ䔚砥唞ꡀ앵覇숹彴톒ﶸȽ떧风섔ᔢŨᙿ䳣鲙ꖔ喬ⵟꭘ䰙뇨傦唜ᒸ屰䷂젘籁ꩉ璥뾜供픧磢蟀閝址䇴黝໧䛪滟型휁鍜鹪䐹줥亾ꀃ鹁ꉧ甠㍣鄎츤퍜䭁๊곀麯顰덬㼵巴ඣ姢︈Ɨ䅐쁦因쾁鬳ᒥ烱꾂湒㐶獲綁′↓Ꮪ㼬Ⅿᛱ✬კ邙娳慳ߕ漋뤎꿢첕ᰲ煖㐭쀆都鲖堨쀰䏅丆㽁硱㛆騔됆鷴㒲㔱づἊരꑷ湠꺇࣍⍢埋苗㺅笮仡Ċ㉆㍅ᖳ샴촆ત悒꾘ࡓ烷灋楟Ⱍ㯜鉚䢰㕝ꭽ曵栘㊬鋿叞뜦瓰ﳺ옎㲆ﱫ䙆댽騮征觜磣犺롛鷟婓띲ฐ并挩⬛꫏烵⭬闡坞뙹Ѐ毡ꇋ￫䲨ﯪᶙ늉뫇ĩ虈貎솭䪥㭜봐Ⲳ輁ꯈ闉跴鹊꿺틅莧龛扯魊ǭ薌湮㔢菖릪㒚矉㩦㢼晑퍋﯎ﰀᅠﶭ냊ꃕᵗ徾썮뱜鲕婿䗋睭鏀娭쎀ꢘ䓿➼㽶䖾镝漣糄ਜ਼嗍Ĳ오봫㘎힃ᇧ㌀從퟊孪쾲ᗴ᯻⨌膼潕蹦䖅Ὦ둸⅊춳ҽ儘饁曖ዲ䁀塴溺䪅祖΍똆ﭓຑ곛慵祇ឥ㿋㴗湅ꎼ陖瀐᧾쁜⃒坟Ꜳ쵁垇肱榈䷪满ȹ쯓聺몀縐槑陲앩渆뮮쌒ꛨŠ齃좷봼鮧떳㌲觲矏͂ᆣ팤㖛쿖㑪雤ガ힒ㅁ炓ߴ烅⢊뾠驤擮鶓汢夷洽첺䍀俥떔胺ꁊࣉź᤬ᄈ뉘鶚ড়밎받ጅ媞퀲㑕哯텯條蹔ױ飚┮閰焖҂螗፵䘡컍姆ၗꔀ⠲積䭥᩿⿍㰄愄㧋ྛẨڹ谁坪쨵嫘⫪탐郰퓯⩛⁊嚰ᕼⶃ쪺䧏湃纾鐴率ĥ聜紳窌쳶瓲듙㠮죱丢୉쵩䏕ე᭲泆헵빀⃿熽碙Ɥ㦶ც률ｐ멐黏䙣꜊編顒ว◪鋌シꞐ䣄謁驸놰핕ोᢓ⼬ࡊ处㜠ᮞ蝽ｦӉ䌉债섖婯Ĩꆮ憵⻘㯱躖ｹﾱ砅ꎿ鎜唘馔쾰ሪൢ⒦쎒晣쌭ෙ꩏㏓墤䚬驣Ը䮣䛼蚕억捅᪗ꫲ胑묅㮪ↈ遉烆檥ꁔ毗ﻜ䆆梙﹪夤哻㘥敧㊢ỻ诀ĕᰨ䙍⨔퇼瓐硈脞챕깥ᳮ㦞싕䤗戙웝崣䥼瞹痄⢜႖憋᫺㒆ꅘ橊ﻑਖ夲໡맠ꛠ흱噶﯃ੰ揚ᰉ广鰹⪋橅浖߹겍猖띮悏ᔉꂏ睫幸ਫ਼웢ꆀᬛ徦罱឴Ĳ쥌䕕ﻸ锲ቌ煏糾擠♀蒳菉㜯୞䦜蝁힬ᇩ쯨浤Δ⼁吿笥쾦沙訟䵵牣뎒ᅪƊꋇ脺헦걭좍后였쐚솆渀諄収粠ⶕ䕡橤ﯛ瘽촖꼭䑿꿹娝屇ᣮ眳ꤍǞ뗵㇎ዛŝ宎雠匶⥏ᓹ⠡⚋屮洏蕀ॿ底⊒ㅦ᫸鱨ᦊ뱀埈뼍菻오傮㈋㶻月ﯵ计塹ꁫ餈鈊暉혚ﲠ祝묄槉ꓹ岣䝬᷉ᐾ餪淮鑿㼝꿄Ⳡ凱♫ė㓛풓쌜홙⡎ӽ쇱ǁ堮串泼ᐻ옚졣ᛧ볳䣿佧橯韶鍡ᑐⲈ۲⚬쏯㐰烫䢃쑑㉭묗䚗摄痛석雰ᚰ奿뫟⾱탯鐪㔤莰歂㽯빑Ꝉ꘬꼬㿾呾Ğ禯ँ⍡矐쮉ꌑ䓹顰돌ኂ컢䩻窃饦繣؄麦㜌냜ౡ贺阕ǂ㾩۳枎ꝸ溣ﲰ넶煸䡎䊚Ὥ굮칭쿮횸᠐їⳳᔲ̒뷎뗻ꩮ淋鰽⺬糍ϖ嬉ꗄ굒ㅭ鍘ķ꒢㜫쿻땉揜정惺ၱ씰콱ꔄגּ癆렞戍ￃ∢渹썫嶲䨡넥꥟碔䈄셖ꌨ㉾⨻䢕㷈榈⹂鼰쭣譮횮븣წ副칬⋜셡咅Ḕȑꍩ韽彸嚌룷쟀Ჸ娆쓯濧㎇㦉꛹Ċ㉃뷄㽴䡽둿ꖒ벌ꄖ㯡ꐝ槟ꄐ皮謚⭽遮砾ᄸ᳋힊盂龃擯ॅ邤ቔיּ긦彐볔㘱⥜슩㸭쓢⤎㛈嫗⩣੹랿겙뭶䬊浢ῥ廝쐜쩲阌ﾸ䄓㠘磱䷒ꀭȹ킶鑁쫎ꃟ⭄②寪籓싔斣﨩텗猄肟珪뱝歴﹯俒堲枺眦桀鷏窐㦯࢚ꏈ䮡鍅잁䩥守熧⊂铈㙍䠛츚㭹霨娣搷᪄辚锞〧夻㳡䠡乞ጤඡ攘湟賐㋅泐ť俼⚴쵞措衉鈦䁴作评鱻爊쑦喕튅㦍퓳妺䰥ಢ꛹揔૟ꤍ䯃☖꽴㝳콀ⲧ鸒者⯣篓⏥囗㈡钇兞壖ꙿ呼卛㍬怑䄬ꯥ퀭⛧葸಻囡웋굍☔좴﫜㸛䙌⣊瓂Ѧ爨剥香龅퟉五뺅멮ﳮ⸾쓜뚻谶헛칚菇墲糣饪鬸디银◅铨䕩䧽ࣄꀬ먙׎酣ᙳ匉䌾滔멍圠듬⒑쵈솄摯꼟鯖ꞥ∗欍៉ꔤꨋ孪脐ˑ败↯媿币킎蓺壺㮰欦ꯨ簱Ĭ왍ࢠᰢ뜯뚤젝⭷㐬툾쮃閣鉵恬核꛴ヱ軟箪訔勵畊㵏፹戔䳀Ⓢ᭒톁덮㩴傌投⾺㓟궆怇뿺㑧㡎穓❴뵶č͟仼횪ދᘛᑢ녘ਆ濐ꨧ㥽綪闹๧뉧Ꮛ把䵗䁊ꒀ濙⒋崬뽫ﲊ틌拲弯᏷㽟갊廘韠瀙ꫲ䀦ෝ䁥㿆㌐쪤煶ݎ㯐辫ʦ⢔嵯蕉쁘ﵱ컬㊷恝⒧〰릑ă㌿妘ᢉ볖볐韃悻ᬸᖘ淀ඔ顥欹倻ଅல馇⺆ԓ烖Ｐ㺼ᡴꏜ抶⓷鏿퉇뵫᫇麚꾝媁붚鬾넝㔨囂瀂ﶭ蠙㦤౮ﯦ坻ꈫ퉅衆煺찘齎䯗푅⣯盰靆쯿фϰ㏐博ꖇꞀ驒뜷Ჭş岴医㶝疎漧硝㜬璼蛅訂ꡋ⩣ᓬᘮ狽鲓ퟣআﱆሼ㈤ᵯ뤵惕㌍飱熄䪋礼紅㞉宱잉㫛쿸꺮钷脬ఢ䞱떴훒遀䳁贐蟋劭䠾剝늨ˮฑ鸴ċ㤙ℵ੐첋ꧤ㿁ᆷ蕵詧ǥ粃쳿툁销峵檤粵泠뉽孑鵆컛ⶲ䨇御锈༚縖帤밲뾼❋啿ㇺ邕益劇찷ퟬ냗␓㲇턫梵疢呖첹葖‱㩱᰿ౝম㫌㲹⅀옕龀㰪ૻ灹Ĩꍓੵ嗫⁼ￌ串࠵싉⨸幍囩܇ꦁ꘧똩얶ᡌȺẊ⻿ⶠ㰟舄ᛊᘥ鐗诼샤媸꠷⇃綥曅̚ꖔ厖鸥荣틩君ꆥ䷃朷빱᪎ᇍ㎡䄏헫ዜ闏˾욷Ƀ굧짡匯刺묠첔馇푫Ⓐꛧ뗦꾝꬏鄱礯፶밪粉ꞈ엷凌禼䋓닮携ᒓ㭀᫺韤僝衕㘊㖼体撓ཟⵠᾛ䑐걙땴ᠬ툡ﳟ㛿䑲眠㏔렊퇊܄嫰㗮቉წ梁੼콻疭ꥮ膂浔⛂퀢坳䆣穜竒፣鲣࿚䢱짏ጋ၊∧퀕벃᱐嘯鈦괔点㓾왛䯽煆걙ᇬڄ켣ﻑ鬇鈧̓菭꪿䝩巂ᦖ肶睹軐썾몼Ⲻ뭈㮨눗✬ꯣ쳰跸ď煎ӫ◁ಟ遒┤僨⽨΅森玼ᣎ庎㞪胟⻰ᘂ뇄뛜匐漻蝹划ꦤⓘ䭹ꩂį늩ଡ଼ⲑᓥ㔿맒㠱コ钬ꚡ伐覙귱ᚢ쟖Յ⯠匫恤쩸懅຿⯃聻╭禁ᴨΌ릛蚱䈯蕞؁䚃䥓ꂻ͗ꠁ㏙垐锚쾎習噷䦤緱斥ꇚ⡯Ꝁ낯꺤᧎ꎭꂘ၍洭눢셟쵩봤햰ꦦ챔揈풞蕞쏢망Ͽᶃ㾷ꌄ쐬আ㈍娗碹䢛Ἠ羝䕞앮훤비翇Ɨ害뱠눶酉耔溃氇⠧꫙蓮蕓푢鵐ⱞ澈ǿ楴⭥䖅쇳糹䉄䌵ⶨ玟燮줻⌿촟㵴谝ꃑ뙩閕듞揎ᱮ䵕捽蔯ᴳ暏몘ꛃ忕ꌖ砥돬ԣ荞工聿鎏ꍂꤚ쑭Ğ睊芫㟜ꏽ䑚ߩ걮㦞䯒δസ婎恲ྼლ피廳힔岼㇥ﴹ쨧占옧꺨뷗ퟐ橨뜶쯘銘頿唴笴⛵⁺묉ኹ䒇嗣㠨⹯ׯ뻘参ਓԗ킿ᯏຶ憖䩃拁惦湳ᣠ▤멒Ĕ俛執럽覲ጿ측踪ᰦ迤岏ྺ饇疧縦ꇿ⸱ૌ姂그ᰇ젰ꅱ鬾̊恫鴸羕뇹㜯顨틝쇭⺳낷쨰瑯萻ᾀ茘挑늶기킖ܺ빁沤虐듇๪춱⮔蓑㽔̬ểങರĩ裗놳䝋◠Ꞇ䘂ルஉⳫ쉂釙䷞ᾨƂⴄ޴箑锉쵨笗ꧾ궯㲵봚⦇疑∮ἢ䑲ナ㬶ꍩඪ抖㥱퓵⫊輼㓧䑣뺦ឋ휉叨ோ嚮嫯뺖቏㱞㭣膦ᅦ⃮攡ė摽읞섬ꛯ瓁氵霜䬯በ핆葵ᔅㅪ腻쟰䍒粷쁘❻䃝Ǆ藱ֻ巵㪃櫿㠳큮觶팀䦑ᢑ쁍遃驪콅ꛯ᜹붒౓⌁꽠鈴쿒읡ゟ℮霫斶㴛ꪼ鉧赽櫷탈竿͠"
;const 伥:I=99;const 佛:C=5;const 五:C=8;const 侒:C=13;const 傿:[C;佛]=[2,3,4,8,13];const 儇:[M;侒]=[3,1,3,2,1,2,2,1,3,2,1,2,2];#[derive(Clone,Copy)]enum Type{僘,Guard,僡,傟,傠,}#[derive(Clone,Copy)]enum
亓{僯,僰,僱,僭,僮,}impl 亓{fn of(且:M)->亓{match 且{0..=6=>亓::僯,7..=8=>亓::僰,9..=12=>亓::僱,13..=20=>亓::僭,_=>亓::僮,}}}#[derive(Clone,Copy)]enum 仾{傽,僿,儀,僢,}#[derive(Clone,Copy,PartialEq)]enum
乔{乶,俸,俹,倎,倏,俭,修,倐,}impl 乔{const 侊:[乔;五]=[乔::乶,乔::俸,乔::俹,乔::倎,乔::倏,乔::俭,乔::修,乔::倐,];}struct 伧{俣:Type,丆:I,丵:亓,下:仾,亾:I,债:I,偡:I,傊:I,d:I,侻:I,偠:P,佫:I,倝:I,}impl
伧{fn 俀(&self)->[M;侒]{let 丁=self;let u=M::from;[丁.俣 as M,u(丁.丆.clamp(1,2)-1),丁.丵 as M,丁.下 as M,u(丁.亾.min(1)),u(丁.债.min(2)),u(丁.偡.min(2)),u(丁.傊.min(1)),u(丁.d.min(4)),u(丁.侻.min(3)),M::from(丁.偠),u(丁.佫.min(2)),u(丁.倝.min(2)),]}fn
code(&self,亗:C)->M{儠(&self.俀(),亗)}}fn 儠(俀:&[M;侒],亗:C)->M{俀.iter().zip(儇).take(傿[亗.min(佛-1)]).fold(0,|仨,(&v,云)|仨<<云|v)}struct 伮{丂:C,d:[I;O],习:G,guard:G,丵:亓,kill:P,下:F,}impl
伮{fn new(l:&J)->伮{let 丂=l.丞();let 个=1-丂;let 丰=l.map().valid;let mut d=[伥;O];let mut 乣=l.与(个);let mut 二=乣;for 一 in 二.iter(){d[一]=0;}let mut 丢=0u8;while!二.is_empty()&&丢<伥-1{丢+=1;二=(二.不()&丰).世(乣);for
一 in 二.iter(){d[一]=丢;}乣|=二;}let 习=习(l,丂);let mut 仓=G::U;for 一 in 习.iter(){if d[一]<=3{仓.set(一);}}仓|=仓.不();let 兌=伈(l,丂,true,T[个])<=l.下(丂)||{let 丘=伈(l,个,true,T[丂]);丘!=亰&&丘<=l.下(个)+l.丠(个)};伮{丂,d,习,guard:仓,丵:亓::of(l.乤()),kill:兌,下:l.下(丂),}}fn
丙(&self,一:C)->I{self.d.get(一).copied().unwrap_or(伥)}fn 儿(&self)->仾{if self.kill{仾::傽}else if self.下<40{仾::僿}else if self.下<80{仾::儀}else{仾::僢}}fn key(&self,l:&J,一:C)->Q<伧>{let
丁=self;let 丆=l.亝(一)?;if!l.丈(丁.丂).test(一){return None;}let 个=1-丁.丂;let 七=G::一(一);let 乂=七.不();let 佴=l.丈(0)|l.丈(1);let 丧=(乂&l.map().valid).世(l.与(0)).世(l.与(1)).世(佴);let
亩=(七|乂)|(七|乂).不();let 丗=|乏:G|I::try_from(乏.count()).unwrap_or(I::MAX);let d=丁.丙(一);let 佫=丗(丧);let 俣=if d<=1{Type::僘}else if 丁.guard.test(一){Type::Guard}else
if d<=3{Type::僡}else if 佫>0{Type::傟}else{Type::傠};let 侻=丁.习.iter().map(|j|l.map().丙(一,j)).min().map_or(伥,|m|m.min(伥));let 儚=(乂&l.与(个))&l.乊(丁.丂,丆.clamp(1,3));Some(伧{俣,丆,丵:丁.丵,下:丁.儿(),亾:I::from(!儚.is_empty()),债:丗(l.丈(个)&亩),偡:丗(l.丈(丁.丂)&亩).saturating_sub(1),傊:丗(l.乚()&l.与(个)&亩),d,侻,偠:丁.习.test(一),佫,倝:丗(乂
&l.丈(丁.丂)),})}fn kind(&self,l:&J,上:C,丝:C)->乔{let 丁=self;if 上==丝{return 乔::倐;}if l.与(1-丁.丂).test(丝){return 乔::乶;}let 丛=l.与(丁.丂).test(丝);let(儨,仪)=(丁.丙(上),丁.丙(丝));match(仪.cmp(&儨),丛){(std::cmp::Ordering::Less,false)=>乔::俸,(std::cmp::Ordering::Less,true)=>乔::俹,(std::cmp::Ordering::Equal,false)=>乔::倎,(std
::cmp::Ordering::Equal,true)=>乔::倏,(std::cmp::Ordering::Greater,false)=>乔::俭,(std::cmp::Ordering::Greater,true)=>乔::修,}}}fn 习(l:&J,丂:C)->G{let 丄=l.丄(丂);let
仕=G::一(T[丂]);let mut 万=G::U;for v in 丄.iter(){if v==T[丂]{continue;}let 乢=丄.世(G::一(v));let mut 乙=仕&乢;loop{let 丅=(乙|乙.不())&乢;if 丅==乙{break;}乙=丅;}if 乙.count()+1<丄.count(){万.set(v);}}万}#[derive(Default)]struct
亯{偑:[K<(M,[I;五])>;佛],}impl 亯{fn 儮()->&'static 亯{static 僲:OnceLock<亯> =OnceLock::new();僲.get_or_init(||亯::decode(僳).unwrap_or_default())}fn 伙(&self,乱:&伧)->[I;五]{for
亗 in(0..佛).rev(){let 仨=乱.code(亗);let 丆=&self.偑[亗];if let Ok(丱)=丆.binary_search_by_key(&仨,|&(c,_)|c){return 丆[丱].1;}}[0;五]}fn from_bytes(乜:&[I])->Result<亯,乓>{let
mut 丱=0;let mut 傄=亯::default();for(亗,丆)in 傄.偑.iter_mut().enumerate(){let 丗=傐(乜,&mut 丱)?;let mut 仨=0u32;for i in 0..丗{let 仒=傐(乜,&mut 丱)?;if i>0&&仒==0{return
Err(format!("level {}: keys are not in increasing order",亗));}仨=仨.checked_add(仒).ok_or_else(||format!("level {}: a key's code overflows",亗))?;let 乐=丱+五-1;let
伎:&[I]=乜.get(丱..乐).ok_or("the table ends inside an entry")?;let 伝:M=伎.iter().map(|&q|M::from(q)).sum();let 乷=255u32.checked_sub(伝).ok_or_else(||format!("level {}: probabilities over 255",亗))?;let
mut p=[0u8;五];p[..五-1].copy_from_slice(伎);p[五-1]=乷 as I;丱=乐;丆.push((仨,p));}}if 丱!=乜.len(){return Err(format!("{} bytes after the table",乜.len()-丱));}Ok(傄)}fn
decode(佸:&str)->Result<亯,乓>{if 佸.is_empty(){return Ok(亯::default());}let 乜=农(佸).ok_or("the table is not packed text")?;亯::from_bytes(&乜)}}fn 傐(乜:&[I],丱:&mut
usize)->Result<M,乓>{let mut 丫=0u32;for 偵 in(0..35).step_by(7){let 倨=*乜.get(*丱).ok_or("the table ends inside a number")?;*丱+=1;let 佅=M::from(倨&0x7f);if
偵==28&&佅>0xf{return Err("a number of the table is too large".to_string());}丫|=佅<<偵;if 倨&0x80==0{return Ok(丫);}}Err("a number of the table is too long".to_string())}fn
伙(乱:&伧)->[I;五]{亯::儮().伙(乱)}fn 兏(p:&[I;五])->K<乔>{if p.iter().all(|&q|q==0){return K::new();}let mut 乇=乔::侊;乇.sort_by(|a,b|{p[*b as C].cmp(&p[*a as C]).then((*a
as C).cmp(&(*b as C)))});乇.to_vec()}const 傝:介=45.0;const 储:介=800.0;const 僩:介=0.6;fn 九(乃:互,且:M)->亻{if let Some(伾)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亻{仈:None,仱:None,伾,};}let 使=冓().min(100.0);let 兗=if 且==1{储}else{傝};let 乗=Duration::from_secs_f64(兗/1000.0*使);亻{仈:Some(乃+乗.mul_f64(僩)),仱:Some(乃+乗),伾:丯::MAX,}}const
僔:C=9;const 僐:仌=仌{丒:3,丠:1,下:1,丷:2,习:2,住:2,中:6,份:1,串:0,伆:false,乇:僕,};const 僙:M=0;const 僕:争=争::侓{亾:8};const 倈:I=I::MAX;const 倆:F=2;const 倇:F=18;const 倅:F=0;const
僓:C=32;const 傘:P=false;const 備:Settings1=Settings1{俛:3,俚:1,侳:4,仗:2,侽:2,佨:2,信:true,reserve:true,侾:true,乯:从::亂,fill:true,乡:Z{佾:倈,伉:倆,佰:倇,你:倅,..Z::以},};fn
亍()->亮{if 傘{return 亮::new(亡::何(備),K::new(),Z::以);}兠(僓)}fn 兠(云:C)->亮{let 亨=亡::伬{丮:僔,云,丹:僐,佼:僙,};let 临=僥.iter().map(|&乡|Z{佾:倈,伉:倆,佰:倇,你:倅,..乡}).collect();亮::new(亨,临,Z::以)}fn
main(){let 冃=io::stdin();let 冄=io::stdout();if let Err(们)=內(&九,亍(),冃.lock(),冄.lock()){eprintln!("plan: {}",们);std::process::exit(1);}}const 僶:&str="CG_TIME_SCALE";fn
冓()->介{兢(std::env::var(僶).ok().as_deref())}fn 兢(丫:Q<&str>)->介{丫.and_then(|些|些.trim().parse::<介>().ok()).filter(|使|使.is_finite()&&*使>0.0).unwrap_or(1.0)}const
俪:C=127;const 例:C=64;const 傛:M=63172;fn 光(c:M)->P{(0x100..0xFFF0).contains(&c)&&!(0xD800..0xE000).contains(&c)&&c!=0x2028&&c!=0x2029&&c!=0xFEFF&&!(0xFDD0..0xFDF0).contains(&c)&&!(0x202A..=0x202E).contains(&c)&&!(0x2066..=0x2069).contains(&c)}fn
农(佸:&str)->Q<K<I>>{let mut 偪=vec![M::MAX;0x10000];let mut 丅=0;for c in 0..0x10000u32{if 光(c){偪[c as C]=丅;丅+=1;}}let 佤:K<char> =佸.chars().collect();if 佤.is_empty()||!佤.len().is_multiple_of(例){return
None;}let mut 乜=K::with_capacity(佤.len()/例*俪);for block in 佤.chunks(例){let mut 仳=[0u32;32];for&c in block{let 倸=*偪.get(c as C)?;if 倸==M::MAX{return None;}let
mut 侮=丯::from(倸);for 仄 in 仳.iter_mut().rev(){let v=丯::from(*仄)*丯::from(傛)+侮;*仄=v as M;侮=v>>32;}if 侮!=0{return None;}}if 仳[0]>>24!=0{return None;}for(i,仄)in
仳.iter().enumerate(){let 偓=仄.to_be_bytes();乜.extend_from_slice(if i==0{&偓[1..]}else{&偓});}}let 伽=M::from_le_bytes([乜[0],乜[1],乜[2],乜[3]])as C;let 乐=伽.checked_add(4)?;if
乐>乜.len()||乜.len()-乐>=俪||乜[乐..].iter().any(|&b|b!=0){return None;}乜.truncate(乐);乜.drain(..4);Some(乜)}const _:()=assert!(傦==16);const 傦:C=16;const 傡:介=0.4;const
僋:I=4;#[derive(Clone,Copy)]struct 仦{串:G,倮:G,伿:G,}impl 仦{fn of(l:&J)->仦{let(a,伵)=偤(l,T[0]);let(b,佧)=偤(l,T[1]);let 亐=a[T[1]];let 兎=伵[T[1]];let mut 久=仦{串:G::U,倮:G::U,伿:G::U,};for
一 in l.map().valid.iter(){if 伵[一]==I::MAX||佧[一]==I::MAX{continue;}if 伵[一].abs_diff(佧[一])<=1{久.倮.set(一);}if 伵[一].abs_diff(佧[一])<=僋{久.伿.set(一);}let 兜=佔::from(伵[一])+佔::from(佧[一])==佔::from(兎);if
亐>0.0&&兜&&一!=T[0]&&一!=T[1]&&a[一]*b[一]/亐>=傡{久.串.set(一);}}久}fn 仵(&self)->G{self.串&self.伿}}fn 円(l:&J,冀:G)->[I;O]{let 丰=l.map().valid;let mut 万=[I::MAX;O];let
mut 乙=冀&丰;let mut 二=乙;let mut 丢=0u8;while!二.is_empty(){for 一 in 二.iter(){万[一]=丢;}let 丅=(二.不()&丰).世(乙);乙|=丅;二=丅;丢=丢.saturating_add(1);}万}fn 偤(l:&J,上:C)->([介;O],[I;O]){let
乆=円(l,G::一(上));let mut 乇:K<C> =l.map().valid.iter().filter(|&c|乆[c]!=I::MAX).collect();乇.sort_by_key(|&c|乆[c]);let mut 丗=[0.0;O];丗[上]=1.0;for 一 in 乇{if
一==上{continue;}let 乫=G::一(一).不()&l.map().valid;丗[一]=乫.iter().filter(|&n|乆[n]!=I::MAX&&乆[n]+1==乆[一]).map(|n|丗[n]).sum();}(丗,乆)}const Y:C=12;const O:C=Y*Y;const
井:C=4*Y;const 亭:丯=(1<<井)-1;const 佖:丯=1|1<<12|1<<24|1<<36;const 侎:丯=佖<<11;const 倊:丯=(1<<Y)-1;#[derive(Clone,Copy,Debug,PartialEq)]struct G([丯;3]);impl G{const
U:G=G([0;3]);const 侊:G=G([亭;3]);#[inline]fn 一(一:C)->G{let mut 乏=G::U;乏.set(一);乏}#[inline]fn test(self,一:C)->P{(self.0[一/井]>>(一%井))&1!=0}#[inline]fn set(&mut
self,一:C){self.0[一/井]|=1<<(一%井);}#[inline]fn clear(&mut self,一:C){self.0[一/井]&=!(1<<(一%井));}#[inline]fn is_empty(self)->P{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
佑(self)->[丯;3]{self.0}#[inline]fn count(self)->M{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 世(self,乍:G)->G{G([self.0[0]&!乍.0[0],self.0[1]&!乍.0[1],self.0[2]&!乍.0[2],])}#[inline]fn
iter(self)->侌{侌{仳:self.0,丱:0,}}#[inline]fn 侁(self)->G{let[a,b,c]=self.0;G([(a>>Y)|((b&倊)<<(井-Y)),(b>>Y)|((c&倊)<<(井-Y)),c>>Y,])}#[inline]fn 侀(self)->G{let[a,b,c]=self.0;G([(a<<Y)&亭,((b<<Y)&亭)|(a>>(井-Y)),((c<<Y)&亭)|(b>>(井-Y)),])}#[inline]fn
偶(self)->G{let[a,b,c]=self.0;G([(a>>1)&!侎&亭,(b>>1)&!侎&亭,(c>>1)&!侎&亭,])}#[inline]fn 偷(self)->G{let[a,b,c]=self.0;G([(a<<1)&!佖&亭,(b<<1)&!佖&亭,(c<<1)&!佖&亭,])}#[inline]fn
不(self)->G{let 丁=self;丁.侁()|丁.侀()|丁.偶()|丁.偷()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,乍:G)->G{G([self.0[0]&乍.0[0],self.0[1]&乍.0[1],self.0[2]&乍.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,乍:G)->G{G([self.0[0]|乍.0[0],self.0[1]|乍.0[1],self.0[2]|乍.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,乍:G){*self=*self|乍;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侊.世(self)}}struct 侌{仳:[丯;3],丱:C,}impl Iterator for 侌{type Item=C;#[inline]fn
next(&mut self)->Q<C>{let 丁=self;while 丁.丱<3{let 仄=丁.仳[丁.丱];if 仄!=0{丁.仳[丁.丱]=仄&(仄-1);return Some(丁.丱*井+仄.trailing_zeros()as C);}丁.丱+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乼{丌:M,x:I,y:I},乀{丆:I,x:I,y:I},亹{x:I,y:I},予{x:I,y:I},}impl L{fn 冖(&self)->乓{match*self{L::乼{丌,x,y}=>format!("MOVE {} {x} {y}",丌),L::乀{丆,x,y}=>format!("TRAIN {} {x} {y}",丆),L::亹{x,y}=>format!("BUILD MINE {x} {y}"),L::予{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
儌(三:&[L])->乓{if 三.is_empty(){return "WAIT".to_string();}let mut line=乓::with_capacity(16*三.len());for(丣,专)in 三.iter().enumerate(){if 丣>0{line.push(';');}line.push_str(&专.冖());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
京{倚,冨,儈,#[default]侍,}impl 京{fn 交(self)->I{match self{京::倚=>1,_=>3,}}fn 佡(self)->P{matches!(self,京::儈|京::侍)}fn 伅(self)->P{self==京::侍}}const 亠:I=I::MAX;struct
亼{乭:G,valid:G,举:G,丙:Box<[I]>,不:[[[I;4];O];2],}thread_local!{static 傾:RefCell<Q<Arc<亼>>> =const{RefCell::new(None)};}impl 亼{fn 兼(乭:G,举:G)->Arc<亼>{傾.with(|乷|{let
mut 乷=乷.borrow_mut();if let Some(丨)=乷.as_ref(){if 丨.乭==乭&&丨.举==举{return Arc::clone(丨);}}let 丨=Arc::new(亼::build(乭,举));*乷=Some(Arc::clone(&丨));丨})}fn build(乭:G,举:G)->亼{let
丰=!乭;const 僪:[[(体,体);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 不=[[[亠;4];O];2];for(丂,乎)in 僪.iter().enumerate(){for 一 in
丰.iter(){let(x,y)=((一%Y)as 体,(一/Y)as 体);let mut 丗=0;for&(優,儫)in 乎{let(偛,停)=(x+優,y+儫);if(0..Y as 体).contains(&偛)&&(0..Y as 体).contains(&停){let 丅=停 as C*Y+偛
as C;if 丰.test(丅){不[丂][一][丗]=丅 as I;丗+=1;}}}}}let mut 丙=vec![亠;O*O].into_boxed_slice();let mut 俒=[0u8;O];for 上 in 丰.iter(){let 了=&mut 丙[上*O..(上+1)*O];了[上]=0;俒[0]=上
as I;let(mut 伎,mut 俠)=(0,1);while 伎<俠{let 一=俒[伎]as C;伎+=1;for&丅 in&不[0][一]{if 丅!=亠&&了[丅 as C]==亠{了[丅 as C]=了[一]+1;俒[俠]=丅;俠+=1;}}}}亼{乭,valid:丰,举,丙,不,}}#[inline]fn
丙(&self,上:C,丝:C)->I{self.丙[上*O+丝]}}const 僉:M=100;const 亞:F=15;const 僽:[F;3]=[1,4,20];const T:[C;2]=[0,O-1];#[derive(Clone,Copy,PartialEq)]enum 乽{亢(C),俳,}enum
乶{侜,侘,侙,}#[derive(Clone)]struct J{map:Arc<亼>,乁:京,丛:[G;2],丄:[G;2],丼:G,乵:G,丈:[G;2],丆:[G;3],仛:G,亜:[M;O],下:[F;2],丠:[F;2],丞:C,乤:M,伕:M,並:Q<乽>,仴:P,}fn 偆(mut 乴:G,丛:G)->G{loop{let
丅=(乴|乴.不())&丛;if 丅==乴{return 乴;}乴=丅;}}fn 亘(丛:G,仕:C)->G{if!丛.test(仕){return G::U;}let mut 乴=G::一(仕);loop{let 丅=(乴|乴.不())&丛;if 丅==乴{return 乴;}乴=丅;}}impl
J{fn 丞(&self)->C{self.丞}fn 乤(&self)->M{self.乤}fn 並(&self)->Q<乽>{self.並}fn 乁(&self)->京{self.乁}fn 伔(&self,丂:C)->F{let 丁=self;let 丈=丁.丈[丂];丁.下[丂]+10*F::from((丈&丁.丆[0]).count())+20*F::from((丈&丁.丆[1]).count())+30*F::from((丈&丁.丆[2]).count())}fn
下(&self,丂:C)->F{self.下[丂]}fn 丠(&self,丂:C)->F{self.丠[丂]}fn 俤(&self,丂:C)->C{self.丈[丂].count()as C}fn map(&self)->&亼{&self.map}fn 与(&self,丂:C)->G{self.丛[丂]}fn
丄(&self,丂:C)->G{self.丄[丂]}fn 丈(&self,丂:C)->G{self.丈[丂]}fn 亸(&self,一:C)->Q<(M,I)>{let 丁=self;(丁.丈[0]|丁.丈[1]).test(一).then(||(丁.亜[一],丁.俉(一)))}fn 乿(&self)->G{self.丈[self.丞]&self.仛}fn
丽(&self)->G{self.丼}fn 乚(&self)->G{self.乵}fn 亝(&self,一:C)->Q<I>{(一<O&&(self.丈[0]|self.丈[1]).test(一)).then(||self.俉(一))}fn 亜(&self,一:C)->Q<M>{(一<O&&(self.丈[0]|self.丈[1]).test(一)).then(||self.亜[一])}fn
冁(&mut self,丂:C){let 丁=self;if 丁.仴{debug_assert_eq!(丁.丄,[亘(丁.丛[0],T[0]),亘(丁.丛[1],T[1])]);}else{丁.丄=[亘(丁.丛[0],T[0]),亘(丁.丛[1],T[1])];丁.仴=true;}let 儤=丁.丈[丂].世(丁.丄[丂]);丁.俕(儤);丁.倱();丁.下[丂]+=丁.丠[丂];if
丁.下[丂]<0{丁.下[丂]=0;丁.俕(丁.丈[丂]);}丁.仛|=丁.丈[丂];}fn 倱(&mut self){let 丁=self;for 丂 in 0..2{let 丄=丁.丄[丂];let 丈=丄&丁.丈[丂];let mut 丠=F::from(丄.count())+4*F::from((丄&丁.丼).count());for(丆,冝)in
僽.iter().enumerate(){丠-=冝*F::from((丈&丁.丆[丆]).count());}丁.丠[丂]=丠;}}fn 伋(&mut self){let 丁=self;if 丁.並.is_some(){return;}if 丁.丞==1{if 丁.乤>=僉{丁.並=Some(match
丁.伔(0).cmp(&丁.伔(1)){std::cmp::Ordering::Greater=>乽::亢(0),std::cmp::Ordering::Less=>乽::亢(1),std::cmp::Ordering::Equal=>乽::俳,});return;}丁.乤+=1;}丁.丞=1-丁.丞;丁.冁(丁.丞);}fn
佺(&mut self,三:&[L]){let 丁=self;if 丁.並.is_some(){return;}let 俁=丁.伕;for&专 in 三{if 丁.儩(专,俁){continue;}丁.义(专);if 丁.並.is_some(){return;}}丁.伋();}fn 儩(&self,专:L,俁:M)->P{let
丁=self;match 专{L::乼{丌,x,y}=>{丁.伇(x,y).is_none()||丌>=俁||丁.倿(丁.丞,丌).is_none()}L::乀{丆,x,y}=>{丁.伇(x,y).is_none()||丆==0||丆>丁.乁.交()}L::亹{x,y}|L::予{x,y}=>{丁.伇(x,y).is_none()}}}#[inline]fn
伇(&self,x:I,y:I)->Q<C>{let(x,y)=(C::from(x),C::from(y));(x<Y&&y<Y&&!self.map.乭.test(y*Y+x)).then_some(y*Y+x)}fn 倿(&self,丂:C,丌:M)->Q<C>{self.丈[丂].iter().find(|&一|self.亜[一]==丌)}#[inline]fn
俉(&self,一:C)->I{if self.丆[0].test(一){1}else if self.丆[1].test(一){2}else{3}}fn 儓(&self,亀:C)->G{let 乚=self.乵&self.丛[亀];let 丄=self.丄[亀];乚|(丄&(丄&乚).不())}fn
乊(&self,丂:C,丆:I)->G{let 丁=self;debug_assert!((1..=3).contains(&丆),"unit level {} is not 1 to 3",丆);let 亀=1-丂;let 丈=丁.丈[0]|丁.丈[1];let 儯=丁.仫();let 冚=match
丆{1 if 丁.乁==京::倚=>丁.丆[1]|丁.丆[2],1=>丈,2=>丁.丆[1]|丁.丆[2],_=>G::U,};let 乊=儯|丁.丛[亀].世(冚);if 丆<3{乊.世(丁.儓(亀))}else{乊}}fn 仜(&self,丂:C)->G{self.丄[丂]|self.丄[丂].不()}fn
仫(&self)->G{let 丁=self;let 丈=丁.丈[0]|丁.丈[1];let 兆=G::一(T[0])|G::一(T[1]);丁.map.valid.世(丈|丁.丼|丁.乵|兆)}fn 义(&mut self,专:L)->P{let 丁=self;if 丁.並.is_some(){return
false;}let 丂=丁.丞;let 仧=match 专{L::乀{丆,x,y}=>丁.侃(丂,丆,x,y),L::乼{丌,x,y}=>丁.兖(丂,丌,x,y),L::亹{x,y}=>丁.build(丂,true,x,y),L::予{x,y}=>丁.build(丂,false,x,y),};let
Some(仧)=仧 else{return false;};match 仧{乶::侜=>{}乶::侘 if 丁.仴=>{丁.丄[丂]=偆(丁.丄[丂],丁.丛[丂]);}乶::侙 if 丁.仴=>{丁.丄[丂]=偆(丁.丄[丂],丁.丛[丂]);丁.丄[1-丂]=亘(丁.丛[1-丂],T[1-丂]);}乶::侘=>丁.丄[丂]=亘(丁.丛[丂],T[丂]),乶::侙=>{丁.丄=[亘(丁.丛[0],T[0]),亘(丁.丛[1],T[1])];}}debug_assert!(!丁.仴||丁.丄==[亘(丁.丛[0],T[0]),亘(丁.丛[1],T[1])]);丁.倱();if!丁.丛[0].test
(T[0]){丁.並=Some(乽::亢(1));}else if!丁.丛[1].test(T[1]){丁.並=Some(乽::亢(0));}true}fn 俕(&mut self,丒:G){let 丁=self;丁.丈[0]=丁.丈[0].世(丒);丁.丈[1]=丁.丈[1].世(丒);for 丆
in&mut 丁.丆{*丆=丆.世(丒);}丁.仛=丁.仛.世(丒);}fn 傅(&mut self,丂:C,一:C)->乶{let 丁=self;丁.俕(G::一(一));丁.丼.clear(一);丁.乵.clear(一);let 亀=1-丂;if 丁.丛[亀].test(一){丁.丛[亀].clear(一);丁.丛[丂].set(一);乶::侙}else
if 丁.丛[丂].test(一){乶::侜}else{丁.丛[丂].set(一);乶::侘}}fn 侃(&mut self,丂:C,丆:I,x:I,y:I)->Q<乶>{let 丁=self;if 丆==0||丆>丁.乁.交(){return None;}let 一=丁.伇(x,y)?;let 丑=10*F::from(丆);if
丁.下[丂]<丑||!丁.仜(丂).test(一)||!丁.乊(丂,丆).test(一){return None;}let 仧=丁.傅(丂,一);丁.健(丂,丆,一,丁.伕);丁.伕=丁.伕.saturating_add(1);丁.下[丂]-=丑;Some(仧)}fn 健(&mut self,丂:C,丆:I,一:C,丌:M){self.丈[丂].set(一);self.丆[C::from(丆)-1].set(一);self.亜[一]=丌;}fn
兖(&mut self,丂:C,丌:M,x:I,y:I)->Q<乶>{let 丁=self;let 丐=丁.伇(x,y)?;let 上=丁.倿(丂,丌)?;if!丁.仛.test(上){return None;}let 丆=丁.俉(上);let 丬=丁.乊(丂,丆);let 乫=丁.map.不[丂][上];if
乫.contains(&(丐 as I))&&!丬.test(丐){return None;}let mut 丕=丁.map.丙(上,丐);let mut 伶=None;for&丅 in 乫.iter().take_while(|&&丅|丅!=亠){let 丅=C::from(丅);let 丙=丁.map.丙(丅,丐);if
丙<丕&&丬.test(丅){丕=丙;伶=Some(丅);}}let 伶=伶?;let 仧=丁.傅(丂,伶);丁.丈[丂].clear(上);丁.丆[C::from(丆)-1].clear(上);丁.仛.clear(上);丁.健(丂,丆,伶,丌);Some(仧)}fn build(&mut self,丂:C,丼:P,x:I,y:I)->Q<乶>{let
丁=self;let 侧=if 丼{丁.乁.佡()}else{丁.乁.伅()};let 一=丁.伇(x,y)?;let 兝=丁.map.举.test(一);if!侧||丁.下[丂]<亞||兝!=丼||!丁.丛[丂].test(一)||!丁.仜(丂).test(一)||!丁.仫().test(一){return
None;}if 丼{丁.丼.set(一);}else{丁.乵.set(一);}丁.下[丂]-=亞;Some(乶::侜)}}const 僄:F=1000000000;const 侖:F=0;const 僌:F=1;const 僷:F=2;impl J{fn read(伏:&str,且:&str,乁:京,乤:M,)->Result<J,乓>{let
mut 举=K::new();let mut 乌=伏.lines();let 丗=侸(&mut 乌,"mine spot count")?;for _ in 0..丗{let[x,y]=仆(伖(&mut 乌,"a mine spot")?,"a mine spot")?;举.push((佦(x)?,佦(y)?));}倻(乌,"the initial input")?;let
mut 乌=且.lines();let mut 促=[0i64;4];for(丫,乩)in 促.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[俐]=仆(伖(&mut 乌,乩)?,乩)?;if 俐.abs()>僄{return
Err(format!("{}: {} is out of range",乩,俐));}*丫=俐;}let mut 乭=G::U;let mut 丒=[G::U;2];let mut 丄=[G::U;2];for y in 0..Y{let 了=伖(&mut 乌,"a map row")?.trim();if
了.len()!=Y{return Err(format!("map row {y} is not {} characters: {:?}",Y,了));}for(x,佋)in 了.bytes().enumerate(){let 一=y*Y+x;match 佋{b'#'=>乭.set(一),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乸=C::from(佋==b'X'||佋==b'x');丒[乸].set(一);if 佋.is_ascii_uppercase(){丄[乸].set(一);}}_=>{return Err(format!("map row {y} has {:?} at {x}",佋 as char))}}}}let
儖=侸(&mut 乌,"building count")?;let mut 亊=K::new();for _ in 0..儖{let 仆=仆(伖(&mut 乌,"a building")?,"a building")?;亊.push(仆);}let 俤=侸(&mut 乌,"unit count")?;let
mut 丈=K::new();for _ in 0..俤{let 仆=仆(伖(&mut 乌,"a unit")?,"a unit")?;丈.push(仆);}倻(乌,"the turn's input")?;let 偢=亊.iter().find(|倧|倧[0]==0&&倧[1]==侖).ok_or("the bot's own HQ is not among the buildings")?;let
偊=侯(偢[2],偢[3],乭)?;let 丂=T.iter().position(|&仕|仕==偊).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",偊))?;let mut 偸=G::U;for(x,y)in 举{if
乭.test(y*Y+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}偸.set(y*Y+x);}let 侥=|乸:C|if 乸==0{丂}else{1-丂};let mut l=J{map:亼::兼(乭,偸),乁,丛:[G::U;2],丄:[G::U;2],丼:G::U,乵:G::U,丈:[G::U;2],丆:[G::U;3],仛:G::U,亜:[0;O],下:[0;2],丠:[0;2],丞:丂,乤,伕:1,並:None,仴:false,};for
乸 in 0..2{let 产=侥(乸);l.丛[产]=丒[乸];l.丄[产]=丄[乸];l.下[产]=促[2*乸];l.丠[产]=促[2*乸+1];}let mut 佴=G::U;for[产,他,x,y]in 亊{let 乸=偭(产)?;let 一=侯(x,y,乭)?;if!丒[乸].test(一){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佴.test(一){return Err(format!("two buildings at ({x}, {y})"));}佴.set(一);match 他{侖
if 一==T[侥(乸)]=>{}侖=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}僌=>l.丼.set(一),僷=>l.乵.set(一),_=>return Err(format!("unknown building type {}",他)),}}let
mut 乣=G::U;let mut 佲=0u32;for[产,丌,丆,x,y]in 丈{let 乸=偭(产)?;let 一=侯(x,y,乭)?;if!丒[乸].test(一){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乣.test(一){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&丆){return Err(format!("unit level {} is not 1, 2 or 3",丆));}let 丌=M::try_from(丌).map_err(|_|format!("unit id {} is out of range",丌))?;if
乣.iter().any(|乍|l.亜[乍]==丌){return Err(format!("two units have id {}",丌));}乣.set(一);l.丈[侥(乸)].set(一);l.丆[丆 as C-1].set(一);l.亜[一]=丌;佲=佲.max(丌);}if!乣.is_empty(){l.伕=佲.checked_add(1).ok_or_else(||format!("unit id {} is out of range",佲))?;}l.仛=l.丈[丂];l.仴=l.丄==[亘(l.丛[0],T[0]),亘(l.丛[1],T[1])];Ok(l)}}fn
伖<'a>(乌:&mut Lines<'a>,乩:&str)->Result<&'a str,乓>{乌.next().ok_or_else(||format!("the text ends before {}",乩))}fn 倻(mut 乌:Lines<'_>,乩:&str)->Result<(),乓>{match
乌.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乩)),None=>Ok(()),}}fn 仆<const N:C>(line:&str,乩:&str)->Result<[F;N],乓>{let
mut 佑=line.split_whitespace();let mut 傏=[0i64;N];for 丫 in&mut 傏{let 伢=佑.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乩))?;*丫=伢.parse().map_err(|_|format!("{}: {:?} is not an integer",乩,伢))?;}if
佑.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乩));}Ok(傏)}fn 侸(乌:&mut Lines<'_>,乩:&str)->Result<C,乓>{let[丗]=仆(伖(乌,乩)?,乩)?;C::try_from(丗).ok().filter(|&丗|丗<=O).ok_or_else(||format!("{}: {} is out of range",乩,丗))}fn
佦(丫:F)->Result<C,乓>{C::try_from(丫).ok().filter(|&丫|丫<Y).ok_or_else(||format!("coordinate {} is off the map",丫))}fn 侯(x:F,y:F,乭:G)->Result<C,乓>{let 一=佦(y)?*Y+佦(x)?;if
乭.test(一){return Err(format!("({x}, {y}) is a void cell"));}Ok(一)}fn 偭(产:F)->Result<C,乓>{match 产{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",产)),}}#[derive(Clone,Copy,PartialEq)]enum
从{亂,俩,}#[derive(Clone,Copy)]struct Settings1{俛:I,俚:I,侳:F,仗:F,侽:C,佨:I,信:P,reserve:P,侾:P,乯:从,fill:P,乡:Z,}impl Settings1{const DEFAULT:Settings1=Settings1{俛:3,俚:1,侳:4,仗:2,侽:2,佨:2,信:true,reserve:true,侾:true,乯:从::亂,fill:true,乡:Z::以,};}const
侤:F=1000;const 侣:F=10;fn 俦(一:C)->(I,I){((一%Y)as I,(一/Y)as I)}fn 俑(l:&mut J,专:L,万:&mut Vec<L>)->P{let 亱=l.义(专);if 亱{万.push(专);}亱}fn 俞(l:&mut J,上:C,丝:C,万:&mut
Vec<L>)->P{let Some((丌,_))=l.亸(上)else{return false;};let(x,y)=俦(丝);俑(l,L::乼{丌,x,y},万)}fn 再(l:&J,久:&仦,么:&Settings1,九:C,)->K<C>{let 七=l.丞();let 两=丸(l,l.与(七));let
丘=丸(l,l.与(1-七));let 亥=l.map().valid.世(l.与(0)|l.与(1));let 偫=|c:C|两[c]!=I::MAX&&丘[c]!=I::MAX;let mut 串:K<C> =久.仵().世(l.与(七)).iter().filter(|&c|偫(c)).collect();串.sort_by_key(|&c|(两[c],c));let
mut 之:K<C> =亥.iter().filter(|&c|偫(c)&&两[c].abs_diff(丘[c])<=么.俚).collect();之.sort_by_key(|&c|(两[c].abs_diff(丘[c]),两[c],c));let mut 万:K<C> =K::new();for
一 in 串.into_iter().chain(之){if 万.len()>=九{break;}let 仭=万.iter().all(|&t|l.map().丙(t,一)>=么.俛);let 儜=久.仵().test(一);if!万.contains(&一)&&(儜||仭){万.push(一);}}万}fn
兇(丑:&[K<F>])->K<C>{let n=丑.len();let 偎=F::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 傒=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 仃=0usize;let mut 佀=vec![偎;n+1];let mut 俥=vec![false;n+1];loop{俥[仃]=true;let 偌=p[仃];let mut 仒=偎;let mut 伻=0usize;for j in 1..=n{if!俥[j]{let
丶=丑[偌-1][j-1]-u[偌]-v[j];if 丶<佀[j]{佀[j]=丶;傒[j]=仃;}if 佀[j]<仒{仒=佀[j];伻=j;}}}for j in 0..=n{if 俥[j]{u[p[j]]+=仒;v[j]-=仒;}else{佀[j]-=仒;}}仃=伻;if p[仃]==0{break;}}loop{let
伻=傒[仃];p[仃]=p[伻];仃=伻;if 仃==0{break;}}}let mut 万=vec![0usize;n];for j in 1..=n{if p[j]>0{万[p[j]-1]=j-1;}}万}#[derive(Clone,Copy)]enum 代{伯(C),侢,}fn 乻(l:&J)->G{let
七=l.丞();(l.仜(七)&l.乊(七,1)).世(l.与(七))}fn 入(l:&J,乡:&Z)->C{侄(l,乡,也::亂).iter().filter(|c|matches!(c,L::乀{..})).count()}fn 严(一:C,丧:G)->M{(G::一(一).不()&丧).count()}fn
偂(l:&J,乑:G,丐:C,伽:I)->K<[F;O]>{let 丰=l.map().valid;let mut 丕=[F::MIN;O];丕[丐]=0;let mut 乲=vec![丕];for _ in 0..伽{let mut 丅=丕;for 一 in 丰.iter(){for n in(G::一(一).不()&丰).iter(){if
丕[n]>F::MIN{let 丫=丕[n]+F::from(乑.test(n));if 丫>丅[一]{丅[一]=丫;}}}}丕=丅;乲.push(丕);}乲}fn 偣(l:&J,乲:&[[F;O]],乑:G,上:C,k:C,)->G{let 丰=l.map().valid;let mut 万=G::U;let
mut 一=上;let mut k=k.min(乲.len()-1);while k>0&&乲[k][一]>0{let Some(丅)=(G::一(一).不()&丰).iter().filter(|&n|乲[k-1][n]>F::MIN).max_by_key(|&n|{(F::from(乑.test(n))+乲[k-1][n],std::cmp::Reverse(n),)})else{break;};if
乑.test(丅){万.set(丅);}一=丅;k-=1;}万}fn 且(l:&J,么:&Settings1)->K<L>{let 七=l.丞();let 久=仦::of(l);let mut 丏=l.clone();let mut 万=K::new();let 丘=丸(l,l.与(1-七));let
丈:K<C> =l.乿().iter().collect();let 偄=(l.下(七)/侣).max(0)as C;let 享=match 么.乯{从::亂=>入(l,&么.乡).min(偄),从::俩=>偄,};let mut 估:K<代> =丈.iter().map(|&c|代::伯(c)).collect();估.extend(std::iter::repeat_n(代::侢,享));let
事=再(l,&久,么,估.len()+么.侽);let 冘=乻(l);let 串=久.仵();let n=估.len().max(事.len());let mut 丑=vec![vec![0i64;n];n];for(了,&丐)in 事.iter().enumerate(){for(伴,仟)in 丑[了].iter_mut().enumerate(){let
Some(&儋)=估.get(伴)else{*仟=侤;continue;};let 佢=match 儋{代::伯(一)=>{let d=l.map().丙(一,丐);if d==I::MAX{None}else{Some(F::from(d)-1)}}代::侢=>冘.iter().map(|c|l.map().丙(c,丐)).filter(|&d|d!=I::MAX).min().map(F::from),};*仟=match
佢{None=>侤,Some(佢)=>{let 个=F::from(丘[丐])-1;let 仗=么.仗*(佢-个).max(0);let 侫=if 串.test(丐){么.侳}else{0};(佢+仗-侫).max(0)}};}}let 儍=兇(&丑);let mut 係:K<(C,C)> =K::new();let
mut 俢:K<(F,C)> =K::new();for(了,&丐)in 事.iter().enumerate(){let 伴=儍[了];if 丑[了][伴]>=侤{continue;}match 估.get(伴){Some(代::伯(一))=>係.push((*一,丐)),Some(代::侢)=>俢.push((丑[了][伴],丐)),None=>{}}}let
mut 丧=l.map().valid.世(l.与(0)|l.与(1));let mut 偙:K<C> =K::new();let mut 佽=G::U;let 亦=|一:C|{let 丆=l.亝(一).unwrap_or(1);let 佌=丧|l.与(1-七);(G::一(一).不()&佌&l.乊(七,丆)).count()};係.sort_by_key(|&(一,丐)|{let
儶=if 么.侾{亦(一)}else{0};(儶,l.map().丙(一,丐),一)});for&(上,丐)in&係{let Some(丆)=丏.亝(上)else{continue;};let 丬=丏.乊(七,丆);let d=丏.map().丙(上,丐);let 乑=(丧|丏.与(1-七)).世(佽);let
伽=d.saturating_add(么.佨);let 乲=偂(&丏,乑,丐,伽.saturating_sub(1));let 伄=乲[乲.len()-1];let 丅=(G::一(上).不()&丬).iter().filter(|&c|伄[c]>F::MIN).max_by_key(|&c|{let
償=d.saturating_sub(丏.map().丙(c,丐));let 佌=丧.test(c)||丏.与(1-七).test(c);(佌&&么.信,F::from(乑.test(c))+伄[c],償,严(c,丧),std::cmp::Reverse(c),)}).filter(|&c|{丧.test(c)||丏.与(1-七).test(c)||丏.map().丙(c,丐)<d});if
let Some(丝)=丅{if 俞(&mut 丏,上,丝,&mut 万){丧.clear(丝);偙.push(上);if 么.reserve{佽|=偣(&丏,&乲,乑,丝,乲.len()-1);}}}}let 两=丸(l,l.与(七));let mut 主=G::U;for c in 丧.iter(){if
两[c]<=丘[c]{主.set(c);}}for&上 in&丈{if 偙.contains(&上)||!丏.乿().test(上){continue;}let Some(丆)=丏.亝(上)else{continue;};let 丬=丏.乊(七,丆);let 佌=(G::一(上).不()&丧&丬).iter().max_by_key(|&c|(严(c,丧),丘[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丝)=佌{if 俞(&mut 丏,上,丝,&mut 万){丧.clear(丝);}continue;}let 业=[主&丧,丧].into_iter().find_map(|丒|{丒.iter().min_by_key(|&c|(丏.map().丙(上,c),c))});if let
Some(业)=业{let d=丏.map().丙(上,业);let 丅=(G::一(上).不()&丬).iter().filter(|&c|丏.map().丙(c,业)<d).min_by_key(|&c|c);if let Some(丝)=丅{俞(&mut 丏,上,丝,&mut 万);}}}俢.sort();let
mut 俋=0usize;for&(_,丐)in&俢{if 丏.下(七)<侣{break;}let 丒=乻(&丏);let Some(乂)=丒.iter().map(|c|丏.map().丙(c,丐)).min()else{break;};let 乑=(丧|丏.与(1-七)).世(佽);let 乲=偂(&丏,乑,丐,乂.saturating_add(么.佨));let
伄=乲[乲.len()-1];let 一=丒.iter().filter(|&c|丏.map().丙(c,丐)==乂).max_by_key(|&c|(伄[c],严(c,丧),std::cmp::Reverse(c)));let Some(一)=一 else{break;};let(x,y)=俦(一);if
俑(&mut 丏,L::乀{丆:1,x,y},&mut 万){丧.clear(一);俋+=1;if 么.reserve{佽|=偣(&丏,&乲,乑,一,乲.len()-1);}}}if 么.fill&&么.乯==从::亂{while 俋<享&&丏.下(七)>=侣{let 一=(乻(&丏)&丧).iter().max_by_key(|&c|{(严(c,丧),std::cmp::Reverse(丘[c]),std::cmp::Reverse(c),)});let
Some(一)=一 else{break;};let(x,y)=俦(一);if!俑(&mut 丏,L::乀{丆:1,x,y},&mut 万){break;}丧.clear(一);俋+=1;}}万}const 傶:F=15;const 働:F=6;const 倖:F=10;const 僣:I=3;#[derive(Clone,Copy)]struct
Z{open:F,佥:F,倴:I,俅:F,佰:F,偕:F,佾:I,倳:F,倲:F,你:F,伍:F,伞:F,乬:F,伐:F,仢:F,伉:F,}impl Z{const 以:Z=Z{open:3,佥:2,倴:12,俅:4,佰:傶,偕:働,佾:僣,倳:2,倲:2,你:0,伍:0,伞:0,乬:0,伐:0,仢:0,伉:0,};const
fn 冦(self,伍:F,伞:F,乬:F)->Z{Z{伍,伞,乬,..self}}const fn 冥(self,伐:F,仢:F)->Z{Z{伐,仢,..self}}}fn 佒(一:C)->(I,I){((一%Y)as I,(一/Y)as I)}fn 丸(l:&J,上:G)->[I;O]{let mut
乆=[I::MAX;O];let 丰=l.map().valid;let mut 之=上&丰;let mut 乣=之;let mut d=0u8;while!之.is_empty(){for 一 in 之.iter(){乆[一]=d;}let 丅=(之.不()&丰).世(乣);乣|=丅;之=丅;d=d.saturating_add(1);}乆}fn
儽(l:&J)->I{let 乆=丸(l,l.与(0));l.与(1).iter().map(|一|乆[一]).min().unwrap_or(I::MAX)}struct 仐<'a>{乡:&'a Z,七:C,theirs:[I;O],伟:[I;O],free:G,主:G,ours:G,举:G,丈:G,}impl
仐<'_>{fn new<'a>(l:&J,乡:&'a Z)->仐<'a>{let 七=l.丞();let 丼=丸(l,l.与(七));let 丘=丸(l,l.与(1-七));let 伟=丸(l,G::一(T[1-七]));let 丧=l.map().valid.世(l.与(0)|l.与(1));let
mut 主=G::U;for 一 in 丧.iter(){if 丼[一]<=丘[一]{主.set(一);}}仐{乡,七,theirs:丘,伟,free:丧,主,ours:l.与(七),举:l.map().举,丈:l.丈(七),}}fn value(&self,一:C)->F{let 丁=self;let
p=丁.乡;let 严=F::from((G::一(一).不()&丁.free).count());let 亾=p.倴;let 佥=F::from(亾)-F::from(丁.theirs[一].min(亾));let 仕=if p.俅>0{F::from(丁.伟[一].min(30))/p.俅}else{0};let
丛=F::from((G::一(一).不()&丁.ours).count());let line=if 丛<=1{p.伞}else{0};let 乬=if 丁.举.test(一){p.乬}else{0};let 冉=if 严==0{p.伐}else{0};let 仢=p.仢*(22-F::from(丁.伟[一].min(22)));let
伉=p.伉*F::from((G::一(一).不()&丁.丈).count());p.open*严+p.佥*佥-仕+p.伍*丛-line+乬-冉+仢-伉}fn 丕(&self,丒:G)->Q<C>{丒.iter().max_by_key(|&一|(self.value(一),std::cmp::Reverse(一)))}fn
侴(&mut self,一:C){let 丁=self;丁.free.clear(一);丁.主.clear(一);丁.ours.set(一);丁.丈.set(一);}}fn 但(l:&mut J,专:L,万:&mut Vec<L>)->P{let 亱=l.义(专);if 亱{万.push(专);}亱}fn
偼(l:&mut J,上:C,丝:C,万:&mut Vec<L>)->P{let Some((丌,_))=l.亸(上)else{return false;};let(x,y)=佒(丝);但(l,L::乼{丌,x,y},万)}fn 军(h:F,k:F)->F{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-倖}fn
兴(l:&J,乡:&Z)->K<L>{侄(l,乡,也::亂)}#[derive(Clone,Copy,PartialEq)]enum 也{亂,人{丽:I,丈:I},}fn 侄(l:&J,乡:&Z,乯:也)->K<L>{let mut 丏=l.clone();let mut 且=仐::new(l,乡);let
mut 万=K::new();仅(&mut 丏,&mut 且,&mut 万);match 乯{也::亂=>公(&mut 丏,&mut 且,&mut 万),也::人{丽,丈}=>{儴(&mut 丏,&mut 且,丽,丈,&mut 万)}}万}fn 仅(l:&mut J,且:&mut 仐<'_>,万:&mut
Vec<L>){let 七=且.七;let mut 仂:K<C> =K::new();loop{let 丬=l.乊(七,1);let mut 优:Q<(M,C)> =None;for 上 in l.乿().iter(){if 仂.contains(&上){continue;}let 亦=(G::一(上).不()&且.free&丬).count();if
亦==0{仂.push(上);}else if 优.is_none_or(|(n,_)|亦<n){优=Some((亦,上));}}let Some((_,上))=优 else{break;};且.丈.clear(上);match 且.丕(G::一(上).不()&且.free&丬){Some(丝)if
偼(l,上,丝,万)=>且.侴(丝),_=>{且.丈.set(上);仂.push(上);}}}for 上 in 仂{if!l.乿().test(上){continue;}let 丐=[且.主,且.free].into_iter().find_map(|丒|{丒.iter().min_by_key(|&一|(l.map().丙(上,一),一))});if
let Some(丐)=丐{偼(l,上,丐,万);}}}fn 公(l:&mut J,且:&mut 仐<'_>,万:&mut Vec<L>){let 七=且.七;let p=且.乡;let h=(p.佰-F::from(l.乤())).max(p.偕);let 冔=(F::from(儽(l))-p.倳).max(0)/p.倲.max(1)+p.你;loop{let
下=l.下(七);let 丈=l.俤(七)as F;let 关=F::from(且.主.count());let 一=且.丕(l.仜(七)&且.free&l.乊(七,1));let 傌=军(h,(关/(丈+1)).min(冔));let 倬=一.is_some()&&下>=倖&&傌>0;let 偖=4*h-亞;let
乬=偗(l,且);let 儙=乬.is_some()&&下>=亞&&偖>0;if 儙&&(!倬||偖>=傌){let(x,y)=佒(乬.unwrap_or_default());if 但(l,L::亹{x,y},万){continue;}break;}let Some(一)=一.filter(|_|倬)else{break;};let(x,y)=佒(一);if!但(l,L::乀{丆:1,x,y},万){break;}且.侴(一);}}fn
偗(l:&J,且:&仐<'_>)->Q<C>{let 七=且.七;(l.map().举&l.丄(七)&l.仫()).iter().filter(|&乬|且.theirs[乬]>且.乡.佾).min_by_key(|&乬|(l.map().丙(T[七],乬),乬))}fn 儴(l:&mut J,且:&mut
仐<'_>,丽:I,丈:I,万:&mut Vec<L>,){let 七=且.七;for _ in 0..丽{let Some(乬)=偗(l,且)else{break;};let(x,y)=佒(乬);if!但(l,L::亹{x,y},万){break;}}for _ in 0..丈{let Some(一)=且.丕(l.仜(七)&且.free&l.乊(七,1))else{break;};let(x,y)=佒(一);if!但(l,L::乀{丆:1,x,y},万){break;}且.侴(一);}}fn
乙(l:&J)->P{if l.並().is_some(){return false;}let 七=l.丞();let 下=l.下(七);let 中=企::new(l,七,true);l.与(1-七).iter().any(|一|中.丑[一]<=下)}#[derive(Clone,Copy)]struct
丿{乛:I,乯:也,}const 俧:I=I::MAX;const 傞:[也;8]=[也::亂,也::人{丽:0,丈:0},也::人{丽:0,丈:1},也::人{丽:0,丈:2},也::人{丽:0,丈:俧,},也::人{丽:1,丈:0},也::人{丽:1,丈:1},也::人{丽:1,丈:俧,},];struct
侐<'a>{临:&'a[Z],仚:Z,rest:丿,什:Q<互>,么:Settings2,}impl 侐<'_>{fn 乡(&self,丢:丿)->&Z{&self.临[C::from(丢.乛)%self.临.len()]}fn 三(&self,l:&J,丢:丿)->K<L>{侄(l,self.乡(丢),丢.乯)}}fn
儉(l:&J,三:&[L],仚:&Z)->(J,P){let mut 丅=l.clone();丅.佺(三);if 丅.並().is_some()||乙(&丅){return(丅,true);}let 亅=兴(&丅,仚);丅.佺(&亅);let 仇=丅.並().is_some()||乙(&丅)||丅.乤()>俰;(丅,仇)}fn
八(l:&J,东:&[丿],仩:&侐<'_>)->K<丯>{let 七=l.丞();let mut 似=vec![佊(l,七)];let mut 乹=l.clone();for 且 in 0..俰 as C{let 丢=东.get(且).copied().unwrap_or(仩.rest);let 三=仩.三(&乹,丢);let(丅,仇)=儉(&乹,&三,&仩.仚);if
仇{break;}乹=丅;似.push(佊(&乹,七));}似}fn 佊(l:&J,七:C)->丯{let mut hash:丯=0xcbf29ce484222325;let mut 俌=|伢:丯|hash=(hash^伢).wrapping_mul(0x0100000001b3);for(册,乏)in[l.与(七),l.丈(七),l.丽()].into_iter().enumerate(){for
一 in 乏.iter(){俌(((册 as 丯)<<8)|一 as 丯);}俌(0xff);}俌(l.下(七)as 丯);hash}enum 亡{伬{丮:C,云:C,丹:仌,佼:M,},何(Settings1),}struct 亮{亨:亡,临:K<Z>,仚:Z,东:K<丿>,佪:K<丯>,仸:C,乒:M,仇:P,}impl
亮{fn new(亨:亡,临:K<Z>,仚:Z)->亮{亮{亨,临,仚,东:K::new(),佪:K::new(),仸:0,乒:0,仇:false,}}fn 全(&mut self,l:&J)->P{let 丁=self;let 兾=matches!(丁.亨,亡::伬{..}|亡::何(_));let
兪=!兾&&丁.乒>0&&丁.仸>=丁.佪.len();if!丁.仇&&(兪||乙(l)){丁.仇=true;}!丁.仇}fn end(&mut self){self.仇=true;}fn 且(&mut self,l:&J,什:Q<互>)->K<L>{let 丁=self;let 仩=侐{临:&丁.临,仚:丁.仚,rest:丿{乛:0,乯:也::亂,},什,么:Settings2::DEFAULT,};if
let 亡::何(么)=丁.亨{丁.乒+=1;丁.仸+=1;return 且(l,&么);}let 七=l.丞();let 兞=丁.佪.get(丁.仸)==Some(&佊(l,七));let 儳=matches!(丁.亨,亡::伬{佼,..}if 佼>0&&l.乤()>=佼);if!兞||儳{let
亃=丁.乒==0;丁.东=match 丁.亨{亡::伬{丮,云,丹,..}=>{let 丹=&丹;let 乢=僜;let mut 乎=东(l,&丁.临,乢,丮,云,什,丹,);乎.resize(丮,乢);乎}亡::何(_)=>K::new(),};丁.佪=match 丁.亨{亡::伬{..}=>兿(l,&丁.临,&丁.东),_=>八(l,&丁.东,&仩),};丁.仸=0;丁.乒+=1;}let
丢=丁.东.get(丁.仸).copied().unwrap_or(仩.rest);丁.仸+=1;仩.三(l,丢)}}fn 兿(l:&J,临:&[Z],东:&[丿])->K<丯>{let 七=l.丞();let mut 似=K::with_capacity(东.len());let mut 乹=l.clone();for&丢
in 东{if 乹.並().is_some(){break;}似.push(佊(&乹,七));乹=侦(&乹,&三(&乹,临,丢));}似}const 俰:M=20;const 價:F=3;#[derive(Clone,Copy)]struct Settings2{丮:F,}impl Settings2{const
DEFAULT:Settings2=Settings2{丮:價};}fn 乎(l:&J,上:G)->[I;O]{let 丰=l.map().valid;let mut 乆=[I::MAX;O];let mut 之=上&丰;let mut 乣=之;let mut d=0u8;while!之.is_empty(){for
一 in 之.iter(){乆[一]=d;}let 丅=(之.不()&丰).世(乣);乣|=丅;之=丅;d=d.saturating_add(1);}乆}fn 冟(l:&J,丂:C)->F{let 两=乎(l,l.丄(丂));let 丘=乎(l,l.丄(1-丂));l.map().valid.iter().map(|一|match
两[一].cmp(&丘[一]){_ if 两[一]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 僥:[Z;27]=临();const
僜:丿=丿{乛:16,乯:也::亂,};const fn 临()->[Z;27]{let 伍=[0,2,4];let 伞=[0,3,6];let 仢=[0,1,2];let mut 万=[Z::以;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
伐=if g>0||t>0{4}else{0};万[i]=Z::以.冦(伍[g],伞[t],0).冥(伐,仢[w]);i+=1;}万}#[derive(Default)]struct 侞{丒:F,丠:F,下:F,习:F,丷:F,住:F,中:F,份:F,}#[derive(Clone,Copy)]struct
仌{丒:F,丠:F,下:F,丷:F,习:F,住:F,中:F,份:F,串:F,伆:P,乇:争,}#[derive(Clone,Copy,PartialEq)]enum 争{倀,俲(侑),侓{亾:F},令(令),俯,}fn 冕(l:&J,丂:C)->[I;O]{let 两=丸(l,G::一(T[丂]));let
丘=丸(l,G::一(T[1-丂]));let mut 亳=G::U;let mut 主=G::U;for c in l.map().valid.iter(){if 两[c]==I::MAX||丘[c]==I::MAX{continue;}if 两[c]==丘[c]{亳.set(c);}if 两[c]<=丘[c]{主.set(c);}}let
d=丸(l,亳);let mut 万=[I::MAX;O];for c in 主.iter(){万[c]=d[c];}万}fn 儒(l:&J,丂:C,亳:&[I;O])->F{let d=丸(l,l.丄(丂));(0..O).filter(|&c|亳[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&亳[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 令{僗,}impl
令{fn 传(self,a:F,b:F,d:F)->F{let 亩=(a-b).abs()/2;match self{令::僗=>a.min(d/2),}}}fn 儷(l:&J,丂:C,便:令)->[F;O]{let 两=丸(l,G::一(T[丂]));let 丘=丸(l,G::一(T[1-丂]));let
d=F::from(两[T[1-丂]]);let mut 万=[0;O];for c in 0..O{if 两[c]!=I::MAX&&丘[c]!=I::MAX{万[c]=便.传(F::from(两[c]),F::from(丘[c]),d);}}万}#[derive(Clone,Copy,PartialEq)]enum
侑{僬(F),}impl 侑{fn 传(self,亩:F)->F{match self{侑::僬(亏)=>(亏-亩).max(0).pow(2),}}}type 佚=[F;4];fn 典(l:&J,丂:C)->[F;O]{struct 伤{丄:G,乾:[佔;O],书:[佔;O],size:[F;O],丷:[F;O],仡:佔,}fn
仼(d:&mut 伤,v:C){d.仡+=1;d.乾[v]=d.仡;d.书[v]=d.仡;d.size[v]=1;for w in(G::一(v).不()&d.丄).iter(){if d.乾[w]==0{仼(d,w);d.size[v]+=d.size[w];d.书[v]=d.书[v].min(d.书[w]);if
d.书[w]>=d.乾[v]{d.丷[v]+=d.size[w];}}else{d.书[v]=d.书[v].min(d.乾[w]);}}}let mut d=伤{丄:l.丄(丂),乾:[0;O],书:[0;O],size:[0;O],丷:[0;O],仡:0,};let 乧=T[丂];if d.丄.test(乧){仼(&mut
d,乧);}d.丷[乧]=0;d.丷}fn 兺(l:&J,丂:C)->侞{let 丄=l.丄(丂);let 丷=典(l,丂);let 伟=丸(l,G::一(T[1-丂]));let mut 万=侞{丒:F::from(丄.count()),丠:l.丠(丂),下:l.下(丂),中:F::MAX,份:冟(l,丂)/2,..侞::default()};for
一 in 丄.iter(){if 丷[一]>0{万.习+=1;万.丷=万.丷.max(丷[一]);}if 一!=T[丂]&&(G::一(一).不()&丄).count()<=1{万.住+=1;}万.中=万.中.min(F::from(伟[一]));}万}fn 冞(l:&J,丂:C,丹:&仌,伿:G)->F{let
仲=F::from((l.与(丂)&伿).count());let s=兺(l,丂);let o=丹;o.丒*s.丒+o.丠*s.丠+o.下*s.下-o.丷*s.丷-o.习*s.习-o.住*s.住-o.中*s.中.min(30)+o.份*s.份+o.串*仲}fn 仵(l:&J)->G{仦::of(l).仵()}fn
兔(l:&J)->G{let 两=丸(l,G::一(T[0]));let 丘=丸(l,G::一(T[1]));let mut 丵=G::U;for c in l.map().valid.iter(){if 两[c]!=I::MAX&&丘[c]!=I::MAX&&两[c].abs_diff(丘[c])<=2{丵.set(c);}}丵}fn
仺(l:&J)->[F;O]{let a=丸(l,G::一(T[0]));let b=丸(l,G::一(T[1]));let mut 万=[F::MAX;O];for c in 0..O{if a[c]!=I::MAX&&b[c]!=I::MAX{万[c]=F::from(a[c].abs_diff(b[c]))/2;}}万}#[derive(Clone,Copy)]struct
伃{串:G,丵:G,乛:[F;O],仺:[F;O],之:G,front:[I;O],}impl 伃{fn of(l:&J,丹:&仌)->伃{let 偒=matches!(丹.乇,争::倀);let mut 乛=[0;O];if let 争::俲(儥)=丹.乇{for(w,亩)in 乛.iter_mut().zip(仺(l)){*w=if
亩==F::MAX{0}else{儥.传(亩)};}}if let 争::令(便)=丹.乇{乛=儷(l,l.丞(),便);}let(兲,之)=match 丹.乇{争::侓{..}=>{let r=仺(l);let mut f=G::U;for(c,&亩)in r.iter().enumerate(){if
亩==0{f.set(c);}}(r,f)}_=>([F::MAX;O],G::U),};伃{乛,仺:兲,之,front:if 丹.乇==争::俯{冕(l,l.丞())}else{[I::MAX;O]},串:if 丹.串!=0||偒{仦::of(l).仵()}else{G::U},丵:if 偒{兔(l)}else{G::U},}}}fn
休(l:&J,丂:C,丹:&仌,久:伃)->佚{let 伝=冞(l,丂,丹,久.串);match 丹.乇{争::俲(_)|争::令(_)=>{let 丄=l.丄(丂);let 冢:F=丄.iter().map(|c|久.乛[c]).sum();[F::from(丄.count()),冢,伝,0]}争::侓{亾}=>{let
丄=l.丄(丂);let d=丸(l,丄);let 仭:F=久.之.iter().map(|c|F::from(d[c]).min(亾)).sum();[F::from(丄.count()),-仭,伝,0]}争::俯=>[F::from(l.丄(丂).count()),儒(l,丂,&久.front),伝,0,],争::倀=>{let
与=l.与(丂);let 串=F::from((与&久.串).count());let 丵=F::from((与&久.丵).count());match 丹.乇{_=>[F::from(l.丄(丂).count()),串,丵,伝],}}}}fn 侦(l:&J,三:&[L])->J{let mut 丅=l.clone();丅.佺(三);if
丅.並().is_none(){丅.佺(&[]);}丅}const 俨:I=I::MAX;fn 三(l:&J,临:&[Z],丢:丿)->K<L>{if 丢.乛==俨{let 么=Settings1{乡:临.first().copied().unwrap_or(Z::以),乯:if 丢.乯==也::亂{从::亂}else{从::俩},..Settings1::DEFAULT};return
且(l,&么);}侄(l,&临[C::from(丢.乛)%临.len()],丢.乯,)}fn 偦(l:&J,临:&[Z],东:&[丿],乢:丿,丮:C,)->J{let mut 乹=l.clone();for 且 in 0..丮{if 乹.並().is_some(){break;}let 丢=东.get(且).copied().unwrap_or(乢);乹=侦(&乹,&三(&乹,临,丢));}乹}fn
内(l:&J,临:&[Z],伆:P)->K<(丿,K<L>)>{let mut 万:K<(丿,K<L>)> =K::new();if 伆{for 乯 in[也::亂,也::人{丽:0,丈:I::MAX,},]{let 丢=丿{乛:俨,乯,};let c=三(l,临,丢);if!万.iter().any(|(_,乍)|*乍==c){万.push((丢,c));}}}for
乛 in 0..临.len().min(C::from(I::MAX)){for&乯 in&傞{let 丢=丿{乛:乛 as I,乯,};let c=三(l,临,丢);if!万.iter().any(|(_,乍)|*乍==c){万.push((丢,c));}}}万}fn 儐(l:&J,临:&[Z],乢:丿,丮:C,云:C,什:Q<互>,丹:&仌,)->K<丿>{let
七=l.丞();let 佄=||什.is_some_and(|t|互::now()>=t);let 久=伃::of(l,丹);let 佇=|东:&[丿]|休(&偦(l,临,东,乢,丮),七,丹,久);let mut 丕:(佚,K<丿>)=(佇(&[]),K::new());let mut 之:K<(J,K<丿>)>
=vec![(l.clone(),K::new())];for 亗 in 0..丮{let mut 亿:K<(佚,J,K<丿>)> =K::new();let mut 乣:HashMap<丯,C> =HashMap::new();for(做,东)in&之{if 佄(){break;}for(丢,c)in
内(做,临,丹.伆){let 丅=侦(做,&c);let mut 侲=东.clone();侲.push(丢);let v=休(&丅,七,丹,久);let 休=佊(&丅,七);match 乣.get(&休){Some(&丱)if 亿[丱].0>=v=>{}Some(&丱)=>亿[丱]=(v,丅,侲),None=>{乣.insert(休,亿.len());亿.push((v,丅,侲));}}}}if
亿.is_empty(){break;}亿.sort_by(|a,b|b.0 .cmp(&a.0));亿.truncate(云);let 乷=亗+1==丮||佄();for(i,(_,_,东))in 亿.iter().take(if 乷{云}else{1}).enumerate(){if i>0&&佄(){break;}let
v=佇(东);if v>丕.0{丕=(v,东.clone());}}if 乷{break;}之=亿.into_iter().map(|(_,s,p)|(s,p)).collect();}丕.1}fn 东(l:&J,临:&[Z],乢:丿,丮:C,兓:C,什:Q<互>,丹:&仌,)->K<丿>{let 七=l.丞();let
久=伃::of(l,丹);let 佇=|东:&[丿]|休(&偦(l,临,东,乢,丮),七,丹,久);let mut 丕:(佚,K<丿>)=(佇(&[]),K::new());let mut 云=2;loop{let 乘=儐(l,临,乢,丮,云,什,丹);let v=佇(&乘);if v>丕.0{丕=(v,乘);}let
佄=什.is_some_and(|t|互::now()>=t);if 佄||(什.is_none()&&云>=兓){break;}云*=2;}丕.1}const 債:C=4+12;fn 丮(俆:impl BufRead,mut 佷:impl Write,mut 侨:impl FnMut(&str,&str,M,互)->乓,)->Result<(),乓>{let
mut 乌=倁{偬:俆};let Some(亃)=乌.line()?else{return Ok(());};let mut 伏=format!("{}\n",亃);乌.take(丗(&亃,"mine spot count")?,&mut 伏)?;let mut 且=0;loop{let Some(亃)=乌.line()?else{return
Ok(());};let 乃=互::now();且+=1;let mut 些=format!("{}\n",亃);乌.take(債-1,&mut 些)?;for 乩 in["building count","unit count"]{let line=乌.偮()?;let n=丗(&line,乩)?;些.push_str(&line);些.push('\n');乌.take(n,&mut
些)?;}let 侨=侨(&伏,&些,且,乃);writeln!(佷,"{}",侨).and_then(|()|佷.flush()).map_err(|们|format!("turn {}: cannot write the answer: {}",且,们))?;}}struct 倁<R>{偬:R,}impl<R:BufRead>倁<R>{fn
line(&mut self)->Result<Q<乓>,乓>{let mut line=乓::new();match self.偬.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(们)=>Err(format!("cannot read input: {}",们)),}}fn
偮(&mut self)->Result<乓,乓>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:C,些:&mut String)->Result<(),乓>{for
_ in 0..n{些.push_str(&self.偮()?);些.push('\n');}Ok(())}}fn 丗(line:&str,乩:&str)->Result<C,乓>{match line.trim().parse::<C>(){Ok(n)if n<=僃=>Ok(n),_=>Err(format!("bad {} {line:?}",乩)),}}const
僃:C=144;const 僴:F=3;const 傼:F=2;const 僈:C=16;fn 兦(l:&J,万:&mut Vec<L>){let 丂=l.丞();let mut 亨=倉{s:l.clone(),丂,个:1-丂,丐:T[1-丂],佯:T[丂],且:l.乤(),万,};亨.run();}struct
倉<'a>{s:J,丂:C,个:C,丐:C,佯:C,且:M,万:&'a mut Vec<L>,}fn 侉(一:C)->(I,I){((一%Y)as I,(一/Y)as I)}fn 俍(丌:M,一:C)->L{let(x,y)=侉(一);L::乼{丌,x,y}}fn 傈(丆:I,一:C)->L{let(x,y)=侉(一);L::乀{丆,x,y}}impl
倉<'_>{fn run(&mut self){let 丁=self;if 丁.佳(){return;}丁.仅();if 丁.佳(){return;}丁.亊();丁.享();}fn 乆(&self,上:C,丝:C)->F{F::from(self.s.map().丙(上,丝))}fn 义(&mut self,专:L)->P{let
买=self.s.义(专);if 买{self.万.push(专);}买}fn 伡(&self,为:&J)->P{为.並()==Some(乽::亢(self.丂))}fn 佳(&mut self)->P{let 丁=self;if 丁.s.並().is_some(){return false;}for
一 in 丁.s.乿().iter(){if 丁.乆(一,丁.丐)!=1{continue;}let Some((丌,_))=丁.s.亸(一)else{continue;};let 专=俍(丌,丁.丐);let mut 为=丁.s.clone();if 为.义(专)&&丁.伡(&为){return 丁.义(专);}}let
Some(丟)=丁.儛()else{return false;};let mut 为=丁.s.clone();if!丟.iter().all(|&专|为.义(专))||!丁.伡(&为){return false;}for 专 in 丟{丁.义(专);}true}fn 儛(&self)->Q<K<L>>{let
丁=self;const 俵:F=F::MAX;let 亚=丁.s.乁().交();let 丬:K<G> =(1..=亚).map(|丆|丁.s.乊(丁.丂,丆)).collect();let 伹=|一:C|{丬.iter().position(|乏|乏.test(一)).map(|丣|(丣 as I+1,10*(丣
as F+1)))};let 丄=丁.s.丄(丁.丂);let mut 丑=[俵;O];let mut 乖=[C::MAX;O];let mut 买=[false;O];for 一 in 丄.iter(){丑[一]=0;}loop{let mut 丅:Q<C> =None;for 一 in 0..O{if!买[一]&&丑[一]<俵&&丅.is_none_or(|n|丑[一]<丑[n]){丅=Some(一);}}let
Some(一)=丅 else{break;};买[一]=true;if 一==丁.丐{break;}for&乫 in&丁.s.map().不[0][一]{if 乫==亠{break;}let 乫=C::from(乫);if 买[乫]||丄.test(乫){continue;}let Some((_,仞))=伹(乫)else{continue;};if
丑[一]+仞<丑[乫]{丑[乫]=丑[一]+仞;乖[乫]=一;}}}if 丑[丁.丐]>丁.s.下(丁.丂){return None;}let mut 丟=K::new();let mut 一=丁.丐;while!丄.test(一){let(丆,_)=伹(一)?;丟.push(傈(丆,一));一=乖[一];}丟.reverse();Some(丟)}fn
乑(&self,乪:&J)->F{let(七,仠)=(self.丂,self.个);let 乖=&self.s;let 丗=|乏:G|F::from(乏.count());let 丛=丗(乪.与(七))-丗(乖.与(七));let 亷=丗(乖.与(仠))-丗(乪.与(仠));let 俊=丗(乖.丄(仠))-丗(乪.丄(仠));let
佱=乖.伔(仠)-乪.伔(仠);let 丽=丗(乖.丽()&乖.与(仠))-丗(乪.丽()&乪.与(仠));12*丛+20*亷+8*(俊-亷).max(0)+5*佱+25*丽}fn 偹(&self,一:C)->F{let 丁=self;let 乬=丁.s.map().举.test(一);if 乬&&丁.s.乁().佡()&&!丁.s.与(丁.丂).test(一){20}else{0}}fn
值(&self,九:F)->P{let 丁=self;丁.s.丈(丁.个).iter().any(|一|丁.乆(一,丁.佯)<=九)}fn 仅(&mut self){let 丁=self;let mut 丈:K<(F,C)> =丁.s.乿().iter().map(|一|(丁.乆(一,丁.丐),一)).collect();丈.sort_unstable();for(仔,一)in
丈{if 丁.s.並().is_some(){return;}let Some((丌,丆))=丁.s.亸(一)else{continue;};let 丬=丁.s.乊(丁.丂,丆);let 乫=丁.s.map().不[丁.丂][一];let mut 丕:Q<(F,C)> =None;for&丅 in 乫.iter().take_while(|&&丅|丅!=亠){let
丅=C::from(丅);if!丬.test(丅){continue;}let mut 为=丁.s.clone();if!为.义(俍(丌,丅)){continue;}let 丫=if 丁.伡(&为){100000}else{丁.乑(&为)+10*(仔-丁.乆(丅,丁.丐))+丁.偹(丅)};if 丕.is_none_or(|(亏,_)|丫>亏){丕=Some((丫,丅));}}if
let Some((丫,丅))=丕{if 丫>0{丁.义(俍(丌,丅));}}}}fn 亊(&mut self){let 丁=self;if 丁.s.並().is_some(){return;}if 丁.值(僴){丁.冗();}else{丁.丽();}}fn 冗(&mut self){let 丁=self;if!丁.s.乁().伅()||丁.s.下(丁.丂)<亞{return;}let
乫=丁.s.map().不[丁.丂][丁.佯];let 丒:K<C> =乫.iter().take_while(|&&一|一!=亠).map(|&一|C::from(一)).collect();let 丼=丁.s.与(丁.丂);if 丒.iter().any(|&一|丼.test(一)&&丁.s.乚().test(一)){return;}for
一 in 丒{let(x,y)=侉(一);if 丁.义(L::予{x,y}){return;}}}fn 丽(&mut self){let 丁=self;if!丁.s.乁().佡(){return;}while 丁.s.下(丁.丂)>=亞{let 丧=丁.s.map().举&丁.s.与(丁.丂)&丁.s.丄(丁.丂)&丁.s.仫();let
Some(一)=丧.iter().min_by_key(|&一|(丁.乆(一,丁.佯),一))else{return;};let(x,y)=侉(一);if!丁.义(L::亹{x,y}){return;}}}fn 享(&mut self){let 丁=self;let 偯=if 丁.且<=3{0}else{5};for
_ in 0..僈{if 丁.s.並().is_some(){return;}let 下=丁.s.下(丁.丂);if 下<10{return;}let 先=丁.值(傼);let 乂=丁.s.仜(丁.丂);let mut 丕:Q<(F,L)> =None;let mut 俜:Q<(F,L)> =None;let
mut 倥=G::U;for 丆 in 1..=丁.s.乁().交(){let 丬=丁.s.乊(丁.丂,丆);let 亦=(乂&丬).世(倥);倥=丬;let 仞=10*F::from(丆);if 仞>下{continue;}for 一 in 亦.iter(){let 专=傈(丆,一);let mut
为=丁.s.clone();if!为.义(专){continue;}let 伒=为.下(丁.丂);let 丠=为.丠(丁.丂);let 佱=丁.s.伔(丁.个)-为.伔(丁.个);let 儊=if 佱>0&&先{伒+丠>=0}else{丠>=0&&(伒>=偯||佱>0)};if!儊{continue;}let
乑=丁.乑(&为);let 丫=乑+丁.偹(一)-6*丁.乆(一,丁.丐)-10*(F::from(丆)-1);if 乑>0{if 丕.is_none_or(|(亏,_)|丫>亏){丕=Some((丫,专));}}else if 丠>=3&&伒>=偯+10&&丆==1{if 俜.is_none_or(|(亏,_)|丫>亏){俜=Some((丫,专));}}}}let
Some((_,专))=丕.or(俜)else{return;};if!丁.义(专){return;}}}}fn 兀(l:&J,万:&mut Vec<L>){万.clear();兦(l,万);}const 亰:F=F::MAX;const 僇:C=8;struct 企{丑:[F;O],乖:[I;O],}fn
伈(l:&J,丂:C,仅:P,丐:C)->F{侷(l,丂,件(l,丂,仅),丐)}fn 侷(l:&J,丂:C,件:G,丐:C)->F{let mut 丑=亰;偀(l,丂,件,|丢,new|{if new.test(丐){丑=10*丢 as F;}丑!=亰});丑}fn 兮(l:&J,丂:C,件:G,乗:F)->G{let
mut 乙=G::U;偀(l,丂,件,|丢,new|{if 10*丢 as F>乗{return true;}乙|=new;false});乙.世(l.丄(丂))}fn 件(l:&J,丂:C,仅:P)->G{if 仅&&丂==l.丞(){l.乿()}else{G::U}}fn 偀(l:&J,丂:C,件:G,mut
仼:impl FnMut(C,G)->P,){let 亚=l.乁().交();let mut 伹=[G::U;3];let mut 乣=G::U;for 丆 in 1..=亚{let 乏=l.乊(丂,丆);伹[C::from(丆)-1]=乏.世(乣);乣|=乏;}let mut 严=[l.丄(丂),G::U,G::U,G::U];for
一 in(件&l.丈(丂)).iter(){if let Some(丆)=l.亝(一).filter(|&丆|丆<=亚){严[0]|=G::一(一).不()&l.乊(丂,丆);}}let mut 买=G::U;for 丢 in 0..{let new=严[丢%4].世(买);严[丢%4]=G::U;if
仼(丢,new){return;}买|=new;let 丅=new.不().世(买);for(伄,丒)in 伹.iter().enumerate(){严[(丢+伄+1)%4]|=丅&*丒;}if 严.iter().all(|乏|乏.世(买).is_empty()){break;}}}impl 企{fn
new(l:&J,丂:C,仅:P)->企{let 亚=C::from(l.乁().交());let mut 丬=[G::U;3];for(丣,乏)in 丬.iter_mut().enumerate().take(亚){*乏=l.乊(丂,丣 as I+1);}let 丄=l.丄(丂);let 不=&l.map().不[0];let
mut 丑=[亰;O];let mut 乖=[亠;O];let mut 严=[0u8;O];let mut 丗=0;let mut 側=G::U;for 一 in 丄.iter(){丑[一]=0;严[丗]=一 as I;丗+=1;}if 仅&&丂==l.丞(){for 上 in l.乿().iter(){let
Some(丆)=l.亝(上)else{continue;};let 乎=G::一(上).不()&丬[C::from(丆)-1];for 一 in 乎.iter(){if 丑[一]!=0{丑[一]=0;严[丗]=一 as I;丗+=1;}}}}while 丗>0{let mut 丕=0;for 丣 in
1..丗{let(a,b)=(C::from(严[丣]),C::from(严[丕]));if(丑[a],a)<(丑[b],b){丕=丣;}}let 一=C::from(严[丕]);丗-=1;严[丕]=严[丗];側.set(一);for&丅 in&不[一]{if 丅==亠{break;}let 丅=C::from(丅);if
側.test(丅)||丄.test(丅){continue;}let Some(丆)=丬[..亚].iter().position(|乏|乏.test(丅))else{continue;};let 傇=丑[一]+10*(丆 as F+1);if 傇<丑[丅]{if 丑[丅]==亰{严[丗]=丅 as
I;丗+=1;}丑[丅]=傇;乖[丅]=一 as I;}}}企{丑,乖}}fn path(&self,丐:C)->K<C>{let mut path=K::new();if 丐>=O||self.丑[丐]==亰{return path;}let mut 一=丐;while self.乖[一]!=亠{path.push(一);一=C::from(self.乖[一]);}path.reverse();path}}struct
佗{三:K<L>,丑:F,}fn 兒(l:&J,丂:C,一:C)->Q<I>{(1..=l.乁().交()).find(|&丆|l.乊(丂,丆).test(一))}fn 共(丏:&mut J,七:C,业:C,乗:F,三:&mut Vec<L>,亪:&mut i64,)->P{loop{if 丏.並()==Some(乽::亢(七)){return
true;}if 丏.与(七).test(业)&&丏.丄(七).test(业){return true;}let 中=企::new(丏,七,false);let 丑=中.丑[业];if 丑==亰||*亪+丑>乗{return false;}let Some(&一)=中.path(业).first()else{return
false;};let Some(丆)=兒(丏,七,一)else{return false;};let 专=L::乀{丆,x:(一%Y)as I,y:(一/Y)as I,};if!丏.义(专){return false;}*亪+=10*F::from(丆);三.push(专);}}fn 倢(l:&J,七:C,事:&[C],乗:F)->Q<佗>{let
mut 丏=l.clone();let mut 三=K::new();let mut 亪=0;for&业 in 事{if!共(&mut 丏,七,业,乗,&mut 三,&mut 亪){return None;}}(亪<=乗).then_some(佗{三,丑:亪,})}fn 写(l:&J,七:C,丐:C)->F{let
丰=l.map().valid;let 与=l.与(七)&丰;let 倰=|mut 丶:G|loop{let 丅=丶|(丶.不()&与);if 丅==丶{return 丶;}丶=丅;};let mut 丶=倰(l.丄(七));let mut 乎=0;while!丶.test(丐){let new=丶.不().世(丶)&丰;if
new.is_empty(){return F::MAX/20;}丶=倰(丶|new);乎+=1;}乎}fn 侰(l:&J,丐:C,乗:F)->Q<佗>{if 乗<0||l.並().is_some()||丐>=O||l.map().乭.test(丐){return None;}let 七=l.丞();if
l.与(七).test(丐)&&l.丄(七).test(丐){return Some(佗{三:K::new(),丑:0,});}if 10*写(l,七,丐)>乗{return None;}let mut 丕=倢(l,七,&[丐],乗);let 亃=企::new(l,七,false);let mut 乚:K<C>
=(l.乚()&l.与(1-七)).iter().filter(|&乵|乵!=丐&&亃.丑[乵]!=亰).collect();乚.sort_by_key(|&乵|(l.map().丙(乵,丐),乵));乚.truncate(僇);for 乵 in 乚{if let Some(丟)=倢(l,七,&[乵,丐],乗){if
丕.as_ref().is_none_or(|亏|丟.丑<亏.丑){丕=Some(丟);}}}丕}const 僅:C=20000;struct 佘{三:K<L>,丑:F,丒:F,丈:F,亊:F,丠:F,丳:F,}struct 侗{丒:F,丈:F,亊:F,}fn 偉(l:&J,丂:C)->侗{let 丄=l.丄(丂);let
丈=l.丈(丂)&丄;let 丫=丈.iter().filter_map(|一|l.亝(一)).map(|丆|10*F::from(丆)).sum();侗{丒:F::from(丄.count()),丈:丫,亊:F::from(((l.丽()|l.乚())&丄).count()),}}struct 倍{七:C,伣:G,乖:侗,丠:F,伓:C,乗:F,众:C,path:K<L>,乘:K<佘>,index:HashMap<伨,C,BuildHasherDefault<DefaultHasher>>,}type
伨=[丯;3];fn 冤(mut 乱:伨,一:C)->伨{乱[一/64]|=1<<(一%64);乱}impl 倍{fn 倷(&mut self,丏:&J,乷:Q<C>,亷:伨,丑:F,)->P{let 丁=self;let 七=丁.七;let 亦=match 乷{None=>丏.仜(七),Some(一)=>G::一(一).不(),}&丁.伣;let
亦=亦.世(丏.与(七));let mut 丬=[G::U;3];for 丆 in 1..=丏.乁().交(){丬[C::from(丆)-1]=丏.乊(七,丆);}for 一 in 亦.iter(){let 亚=C::from(丏.乁().交());let Some(丣)=丬[..亚].iter().position(|乏|乏.test(一))else{continue;};let
丆=丣 as I+1;let 仞=10*F::from(丆);if 丑+仞>丁.乗{continue;}if 丁.众>=僅{return false;}丁.众+=1;let mut 丅=丏.clone();let 专=L::乀{丆,x:(一%Y)as I,y:(一/Y)as I,};if!丅.义(专){continue;}丁.path.push(专);let
伜=冤(亷,一);丁.兯(&丅,伜,丑+仞);let 儦=丁.path.len()<丁.伓&&丅.並().is_none();let 儾=!儦||丁.倷(&丅,Some(一),伜,丑+仞);丁.path.pop();if!儾{return false;}}true}fn 兯(&mut self,乪:&J,伜:伨,丑:F){let
丁=self;let 佃=1-丁.七;let 佂=偉(乪,佃);let 丒=丁.乖.丒-佂.丒;let 丈=丁.乖.丈-佂.丈;if 丒<=0&&丈<=0{return;}let 亊=丁.乖.亊-佂.亊;let 丠=乪.丠(丁.七)-丁.丠;let 冧=乪.並()==Some(乽::亢(丁.七));let
丳=if 冧{F::MAX/2}else{3*丒+丈+10*亊+2*丠-丑};if let Some(&丱)=丁.index.get(&伜){if 丑>=丁.乘[丱].丑{return;}}let 丷=佘{三:丁.path.clone(),丑,丒,丈,亊,丠,丳,};match 丁.index.get(&伜){Some(&丱)=>丁.乘[丱]=丷,None=>{丁.index.insert(伜,丁.乘.len());丁.乘.push(丷);}}}}fn
倵(l:&J,伓:C,乗:F,九:C)->K<佘>{乨(l,伓,乗,九).0}fn 乨(l:&J,伓:C,乗:F,九:C)->(K<佘>,C){if l.並().is_some()||伓==0||九==0{return(K::new(),0);}let 七=l.丞();let 佃=1-七;let mut
俗=倍{七,伣:l.与(佃)|l.与(佃).不(),乖:偉(l,佃),丠:l.丠(七),伓,乗,众:0,path:K::new(),乘:K::new(),index:HashMap::default(),};俗.倷(l,None,[0;3],0);let 众=俗.众;let mut 乘=俗.乘;乘.sort_by(|a,b|b.丳.cmp(&a.丳).then(a.丑.cmp(&b.丑)));乘.truncate(九);(乘,众)}const
佟:F=1000000;const 僆:F=200;fn 俘(l:&J,丂:C)->[F;5]{let 丄=l.丄(丂);let 丈:F=(l.丈(丂)&丄).iter().filter_map(|一|l.亝(一)).map(|丆|10*F::from(丆)).sum();[l.下(丂),3*l.丠(丂),4*F::from(丄.count()),丈,10*F::from((l.乚()&丄).count()),]}fn
主(l:&J,丂:C)->F{俘(l,丂).iter().sum()}fn 偔(l:&J,丂:C,亄:G)->F{儼(l,丂,侷(l,丂,亄,T[1-丂]),)}fn 儼(l:&J,丂:C,丑:F)->F{if 丑==亰{return F::MIN;}l.下(丂)+l.丠(丂)-丑}fn 佻(亴:F)->F{亴.clamp(-僆,0)}fn
儲(l:&J,丂:C,亄:[G;2],)->(F,[F;2]){let 乍=1-丂;match l.並(){Some(乽::亢(冣))=>{return(if 冣==丂{佟}else{-佟},[0,0])}Some(乽::俳)=>return(0,[0,0]),None=>{}}let mut 丳=主(l,丂)-主(l,乍);if
丂==l.丞(){let mut 丅=l.clone();丅.伋();if 丅.並().is_none()&&伈(&丅,乍,true,T[丂])<=丅.下(乍){丳-=佟/2;}}let 付=[偔(l,丂,亄[0]),偔(l,乍,亄[1]),];(丳+佻(付[0])-佻(付[1]),付)}
