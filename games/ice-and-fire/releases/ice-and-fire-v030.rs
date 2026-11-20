// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 伉=u16;type 並=u32;type Z=u64;type 伈=i32;type C=i64;type 交=f64;type P=bool;type 之=String;type 乯=Instant;type K<T>=Vec<T>;type
U<T>=Option<T>;use std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use
std::io::{self,BufRead,Write};use std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const
倐:P=true;fn 偧(l:&J,一:F,伃:C,丵:G,为:C)->P{let 丂=l.丝();let 乼=1-丂;let 俋=倐&&为<=l.下(乼)+2*l.丛(乼).max(0);let 代=G::一(一)|G::一(一).丑();if!俋&&(代&丵).is_empty(){return
false;}let(x,y)=亏(一);let mut 乏=l.clone();if!乏.丸(L::亇{x,y}){return false;}if 侀(&乏,丂)<伃{return true;}俋&&会(&乏,乼,false,T[丂])>为}const 倽:P=false;const 倩:I=2;const
侤:I=1;const 倗:F=4;const 侍:F=5;#[derive(Clone)]enum 乇{仃(L),伐(K<L>),}impl 乇{fn 万(&self)->&[L]{match self{乇::仃(不)=>std::slice::from_ref(不),乇::伐(万)=>万,}}}fn
亏(一:F)->(I,I){((一%Q)as I,(一/Q)as I)}fn 俅(l:&J,丆:&mut Vec<乇>){丆.clear();if l.丙().is_some(){return;}let 丂=l.丝();let 乲=l.丰().乾();let mut 丢=[G::O;4];for 七
in 1..=乲{丢[F::from(七)]=l.乂(丂,七);}for 三 in l.乿().iter(){let Some((世,七))=l.什(三)else{continue;};let 乛=G::一(三).丑()&丢[F::from(七)];for 丷 in 乛.iter(){let(x,y)=亏(丷);丆.push(乇::仃(L::乷{世,x,y}));}}let
下=l.下(丂);let 丅=l.丅(丂);let 亠=l.京(丂);let 伅=match 倩{0=>亠,1=>{let 低=l.map().valid.丗(丅).丑()&丅;亠.丗(丅)|低}_=>亠.丗(丅),};for 七 in 1..=乲{let 上=10*C::from(七);if 下<上{break;}let
mut 丐=伅&丢[F::from(七)];for 偮 in 1..七{丐=丐.丗(丢[F::from(偮)]);}for 一 in 丐.iter(){let(x,y)=亏(一);丆.push(乇::仃(L::乖{七,x,y}));}}if 下>=亐{let 丠=l.专(丂)&亠&l.介();if l.丰().伓(){for
一 in(丠&l.map().久).iter(){let(x,y)=亏(一);丆.push(乇::仃(L::云{x,y}));}}if l.丰().伔(){let 低=l.map().valid.丗(l.专(丂)).丑();let 丐=(丠&低).丗(l.map().久);let 丵=丵(l,1-丂);let
伃=C::from((丵&l.丅(丂)).count());let 为=会(l,1-丂,false,T[丂]);let 串=if 倽{let(事,伟)=临(l,丂);事.丗(伟)}else{G::O};for 一 in 丐.iter().filter(|&一|{let 代=G::一(一)|G::一(一).丑();!(代&串).is_empty()||偧(l,一,伃,丵,为)}){let(x,y)=亏(一);丆.push(乇::仃(L::亇{x,y}));}}}let
偦=match 侤{1=>亐,2=>10,_=>C::MAX,};if 下>=偦{let 丵=丵(l,1-丂);let 佰=l.map().valid.丗(l.专(丂)|l.专(1-丂));for 三 in(l.乿()&丵).iter(){let Some((世,七))=l.什(三)else{continue;};let
乛=G::一(三).丑()&丢[F::from(七)]&佰;let(係,促)=亏(三);let 健=if 侤==1{if l.map().久.test(三){continue;}L::亇{x:係,y:促}}else{L::乖{七:1,x:係,y:促,}};for 丷 in 乛.iter(){let(x,y)=亏(丷);丆.push(乇::伐(vec![L::乷{世,x,y},健]));}}}for
个 in 侺(l,侍,下,倗){if 个.万.len()>1{丆.push(乇::伐(个.万));}}if let Some(为)=佒(l,T[1-丂],下){if 为.万.len()>1{丆.push(乇::伐(为.万));}}}const 値:I=2;const 倵:C=5;fn 乳(l:&J,丁:F)->G{佮(l,丁,値)}fn
佮(l:&J,丁:F,三:I)->G{let mut 乳=G::O;if 三>3{return 乳;}for 一 in(l.丈(丁)&l.丅(丁)).iter(){if l.亅(一).is_some_and(|七|七>=三){乳|=G::一(一);}}乳}fn 丵(l:&J,乊:F)->G{let 丿=l.下(乊)+l.丛(乊);傂(l,乊,乳(l,乊),丿)}fn
仱(l:&J,丁:F)->G{丵(l,1-丁)&l.丅(丁)}fn 侀(l:&J,丁:F)->C{C::from(仱(l,丁).count())}const 侚:C=4;fn 偲(丐:C)->C{丐/侚}fn 丧(l:&J,丁:F,仱:G)->C{let 丅=l.丅(丁);let 丧=(l.丧()&丅).丗(仱);let
丐=C::from(丅.count());let 位=if l.乧()<=倃{丐/倂+倁}else{偲(丐)};位*C::from(丧.count())}const 倃:並=15;const 倁:C=6;const 倂:C=侚;const 倱:C=2;fn 俷(l:&J,丁:F)->C{let 丅=l.丅(丁);let[a,b,c]=丅.伵();let
mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^丁 as Z).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let 丘=(hash as F)&(侩-1);倾.with(|佐|{let mut 佐=佐.borrow_mut();let
亾=&mut 佐[丘];if 亾.0==丅&&亾.1==丁 as I+1{return 亾.2;}let 严=傮(丅,T[丁]);*亾=(丅,丁 as I+1,严);严})}const 侩:F=1<<12;thread_local!{static 倾:std::cell::RefCell<K<(G,I,C)>>
=std::cell::RefCell::new(vec![(G::O,0,0);侩]);}fn 傮(丅:G,于:F)->C{struct 令{丅:G,乡:[I;M],乌:[I;M],size:[I;M],亯:I,亝:C,伪:C,}fn 仁(d:&mut 令,v:F)->I{d.亯+=1;let 乡=d.亯;d.乡[v]=乡;let
mut 乌=乡;let mut 佸=1u8;let mut 个=0u8;for w in(G::一(v).丑()&d.丅).iter(){if d.乡[w]==0{let 修=仁(d,w);佸+=修;乌=乌.min(d.乌[w]);if d.乌[w]>=乡{个+=修;}}else{乌=乌.min(d.乡[w]);}}d.乌[v]=乌;d.size[v]=佸;if
个>0&&d.乡[v]>1{d.亝+=1;d.伪=d.伪.max(C::from(个));}佸}if!丅.test(于){return 0;}let mut d=令{丅,乡:[0;M],乌:[0;M],size:[0;M],亯:0,亝:0,伪:0,};仁(&mut d,于);d.亝+d.伪}fn 傊(l:&J,丁:F)->C{倱*(俷(l,1-丁)-俷(l,丁))}fn
侻(l:&J,丁:F)->C{if l.下(丁)+l.丛(丁)>=0{return 0;}(l.丈(丁)&l.丅(丁)).iter().filter_map(|一|l.亅(一)).map(|七|10*C::from(七)).sum()}const 倖:C=15;const 倻:[C;2]=[3,1];const
倹:[C;2]=[4,20];fn 傜(l:&J,丁:F)->[C;2]{let mut 且=[0;2];for 一 in(l.丈(丁)&l.丅(丁)).iter(){match l.亅(一){Some(2)=>且[0]+=1,Some(3)=>且[1]+=1,_=>{}}}且}fn 俭(l:&J,丁:F)->C{let[俲,傡]=傜(l,丁);let
侮=|七:F|(3-倻[七])*倹[七];-倖*俲+侮(0)*俲+侮(1)*傡}const 倏:C=0;const 倒:C=2;const 們:C=100;fn 俩(伩:C)->C{if 伩<0{return 0;}倏+倒*伩.min(們)/2}const 侙:I=2;const 侕:C=2;const
侔:C=4;const 倓:I=3;fn 些(l:&J,丁:F)->C{if 侕==0{return 0;}let 中=l.map().valid;let mut 丈=G::O;for 一 in(l.丈(丁)&l.丅(丁)).iter(){if l.亅(一).is_some_and(|七|七<=倓){丈|=G::一(一);}}let
mut 仂=中.丗(l.专(丁));let mut 佚=G::O;let mut 乛=0;for 两 in 1..=侔{仂=(仂|仂.丑())&中;let 传=(丈&仂).丗(佚);乛+=(两-1)*C::from(传.count());佚|=传;}乛+=侔*C::from(丈.丗(佚).count());侕*乛}const
侨:C=8;fn 临(l:&J,丁:F)->(G,G){let 丅=l.丅(丁);let 事=丅&l.专(1-丁).丑();let 乆=l.乆()&丅;(事,事&(乆|乆.丑()))}fn 俵(l:&J,丁:F)->C{if 侨==0{return 0;}侨*C::from(临(l,丁).1 .count())}const
伽:C=0;const 侑:I=1;const 侒:C=4;const 伾:C=0;const 侐:C=0;const 倇:並=10;const 倆:C=2;struct 侓{valid:G,乑:G,丯:G,}thread_local!{static 倈:std::cell::RefCell<U<侓>>
=const{std::cell::RefCell::new(None)};}fn 偣(l:&J)->(G,G){倈.with(|一|{let mut 丮=一.borrow_mut();let 中=l.map().valid;if 丮.as_ref().is_none_or(|z|z.valid!=中){let
仛=仛(l);let mut 乑=G::O;for c in 中.iter(){if 仛[c]<=倆{乑.set(c);}}*丮=Some(侓{valid:中,乑,丯:亹(l),});}丮.as_ref().map_or((G::O,G::O),|z|(z.乑,z.丯))})}fn 伟(l:&J,丁:F)->G{let
丅=l.丅(丁);let 乆=l.乆()&丅;(乆|乆.丑())&丅}fn 乮(l:&J,丁:F)->C{if(伽==0&&侒==0&&伾==0&&侐==0)||l.乧()>倇{return 0;}let(乑,丯)=偣(l);let mut 亰=0;if 伽!=0{let 丫=乁(l,l.丅(丁));let
丟=乁(l,l.丅(1-丁));let mut 佯=0;for c in 乑.iter(){let(a,b)=(丫[c],丟[c]);if a!=I::MAX&&a.saturating_add(侑)<=b{佯+=1;}else if b!=I::MAX&&b.saturating_add(侑)<=a{佯-=1;}}亰+=伽*佯;}let
且=|举:G|C::from(举.count());let(乃,丟)=(l.丅(丁),l.丅(1-丁));亰+=侒*(且(乃&乑)-且(丟&乑));if 伾!=0{亰+=伾*(且(伟(l,丁)&乑)-且(伟(l,1-丁)&乑));}亰+=侐*(且(乃&丯)-且(丟&丯));亰}fn 作(l:&J,丁:F)->C{let
乳=[佮(l,丁,侙),佮(l,1-丁,侙),];let(丣,仔)=做(l,丁,乳);let 丣=丣+俩(仔[0])-俩(仔[1]);if l.丙().is_some(){return 丣;}let 丫=仱(l,丁);let 丟=仱(l,1-丁);let 伃=C::from(丟.count())-C::from(丫.count());丣-2*侻(l,丁)+2*侻(l,1-丁)+傊(l,丁)+俭(l,丁)-俭(l,1-丁)-些(l,丁)+些(l,1-丁)+倵*伃/4+丧(l,丁,丫)-丧(l,1-丁,丟)+俵(l,丁)-俵(l,1-丁)+乮(l,丁)}const
倠:F=8;const 倬:F=3;const 倘:F=32;const 倭:Z=300;const 俿:交=0.75;const 倌:U<並> =Some(7);const 倊:P=false;const 倉:C=10;const 個:並=6;struct 亥{亣:U<乯>,亸:U<乯>,价:Z,}#[derive(Default)]struct
侦{万:K<L>,佗:F,亙:Z,丞:F,乄:F,侵:F,亚:F,二:P,}const 值:P=true;#[derive(Clone)]struct 乱{万:K<L>,佛:C,}struct 仄{万:K<L>,end:J,first:U<Z>,东:U<K<乱>>,}type 以=HashMap<Z,C,BuildHasherDefault<DefaultHasher>>;struct
佃{丂:F,limit:亥,亙:Z,仴:以,亽:U<K<乇>>,佼:P,}const 佁:C=0;const 侊:P=true;const 俻:P=false;const 侉:U<I> =None;fn 偉(l:&J,丞:&[乱],丆:&mut Vec<乇>){if!侊&&侉.is_none(){return;}let
仿=丞.iter().map(|乨|乨.万.as_slice());偊(l,仿,(侊,俻,侉),丆,);}fn 偊<'a>(l:&J,仿:impl Iterator<Item=&'a[L]>,(乆,丑,七):(P,P,U<I>),丆:&mut Vec<乇>,){let 丂=l.丝();let 乃=l.丅(丂);let
mut 仯=G::O;for 乨 in 仿{for 不 in 乨{if let L::乖{x,y,..}|L::乷{x,y,..}=*不{let 一=F::from(y)*Q+F::from(x);if 乃.test(一){仯|=G::一(一);}}}}if 仯.is_empty(){return;}let
丠=l.介()&乃;let 下=l.下(丂);let 俟=|不:L,丆:&mut Vec<乇>|{if!丆.iter().any(|a|a.万()==[不]){丆.push(乇::仃(不));}};if 乆&&下>=亐&&l.丰().伔(){let mut 丐=仯;if 丑{丐|=仯.丑();}for
一 in(丐&丠).丗(l.map().久).iter(){let(x,y)=亏(一);俟(L::亇{x,y},丆);}}if let Some(七)=七.filter(|&七|七<=l.丰().乾()){if 下>=10*C::from(七){for 一 in(仯&丠).iter(){let(x,y)=亏(一);俟(L::乖{七,x,y},丆);}}}}fn
佌(l:&mut J,万:&[L])->P{万.iter().all(|&不|l.丸(不))}fn 乽(万:&[L])->Z{let mut hash:Z=0xcbf29ce484222325;for 不 in 万{let(伤,a,b,c)=match*不{L::乷{世,x,y}=>(1,Z::from(世),x,y),L::乖{七,x,y}=>(2,Z::from(七),x,y),L::云{x,y}=>(3,0,x,y),L::亇{x,y}=>(4,0,x,y),};let
仢=伤|a<<8|Z::from(b)<<40|Z::from(c)<<48;hash=(hash^仢).wrapping_mul(0x0100000001b3);}hash}fn 侫(乢:&J,乨:&[L])->J{let mut 丄=乢.clone();丄.仌();for&不 in 乨{丄.丸(不);}丄.仌();丄}fn
伧(l:&J)->U<K<L>>{let 丂=l.丝();let 丌=T[1-丂];let 仡=|丬:&J|丬.丙()==Some(习::乻(丂));for 一 in l.乿().iter(){if l.map().两(一,丌)!=1{continue;}let Some((世,_))=l.什(一)else{continue;};let(x,y)=亏(丌);let
不=L::乷{世,x,y};let mut 丬=l.clone();if 丬.丸(不)&&仡(&丬){return Some(vec![不]);}}let 为=佒(l,丌,l.下(丂))?;let mut 丬=l.clone();(佌(&mut 丬,&为.万)&&仡(&丬)).then_some(为.万)}fn
俬(l:&J,丁:F,乍:C)->P{let 上=佘(l,丁,l.丈(丁),T[1-丁]);上!=井&&l.下(丁)+乍*l.丛(丁)>=上}fn 傛(l:&J)->P{let 丂=l.丝();if 佁==0||l.丙().is_some(){return false;}let mut 丄=l.clone();丄.仌();let
侀=丄.丙().is_none()&&会(&丄,1-丂,true,T[丂])<=丄.下(1-丂);!侀&&俬(l,丂,佁)}impl 佃{fn 侰(&self,乢:&J)->P{self.佼&&乢.丙()!=Some(习::乻(self.丂))&&!俬(乢,self.丂,佁)}fn 亄(&self)->P{self.亙>=self.limit.价||self.limit.亣.is_some_and(|t|乯::now()>=t)}fn
丣(&mut self,乢:&J,丞:&[乱],住:C)->C{self.亙+=1;self.併(乢,丞,住)}fn 併(&self,乢:&J,丞:&[乱],住:C)->C{if 乢.丙()==Some(习::乻(self.丂)){return 休;}if 丞.is_empty(){return 作(乢,self.丂);}let
mut 伶=C::MAX;for 乨 in 丞{伶=伶.min(作(&侫(乢,&乨.万),self.丂));if 伶<=住{break;}}伶}#[allow(clippy::too_many_arguments)]fn 偺(&mut self,乭:&J,今:C,乫:&[乇],乓:&[(C,F)],乙:&mut[P],丞:&[乱],仍:&mut
以,佦:P,)->(U<(F,J,C)>,P){let mut 丏=None;let mut 众=今;for&(伫,丘)in 乓{if 乙[丘]{continue;}if 佦&&伫!=C::MAX&&伫<=众-今{return(丏,true);}if self.亄(){return(丏,false);}乙[丘]=true;let
万=乫[丘].万();let mut 丄=乭.clone();if!佌(&mut 丄,万)||self.侰(&丄){仍.insert(乽(万),C::MIN/4);continue;}let 丣=self.丣(&丄,丞,众);仍.insert(乽(万),丣-今);if 丣>众{众=丣;丏=Some((丘,丄,丣));}}(丏,false)}fn
佖(&mut self,于:&J,丞:&[乱],仪:&[Z],偍:P)->仄{let q=self;let mut 书=None;let mut 乭=于.clone();let mut 万=K::new();let mut 今=q.丣(&乭,丞,C::MIN);let mut 仍=以::default();let
mut 乫=K::new();let mut 乓=K::new();while 乭.丙().is_none()&&!q.亄(){if 万.is_empty()&&偍{match&q.亽{Some(亽)=>乫.clone_from(亽),None=>{俅(&乭,&mut 乫);q.亽=Some(乫.clone());}}}else{俅(&乭,&mut
乫);}偉(&乭,丞,&mut 乫);乓.clear();for(丘,亖)in 乫.iter().enumerate(){let k=乽(亖.万());let 伫=仍.get(&k).or(q.仴.get(&k)).copied();乓.push((伫.unwrap_or(C::MAX),丘));}乓.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 乙=vec![false;乫.len()];if 万.is_empty(){for(丘,亖)in 乫.iter().enumerate(){乙[丘]=仪.contains(&乽(亖.万()));}}let mut 丏=None;for
佦 in[true,false]{let(主,傒)=q.偺(&乭,今,&乫,&乓,&mut 乙,丞,&mut 仍,佦,);丏=主;if 丏.is_some()||!傒{break;}}let Some((丘,丄,丣))=丏 else{break;};if 万.is_empty(){书=Some(乽(乫[丘].万()));}万.extend_from_slice(乫[丘].万());乭=丄;今=丣;}for(k,丼)in
仍{let 俈=q.仴.entry(k).or_insert(丼);*俈=(*俈).max(丼);}仄{万,end:乭,first:书,东:None,}}fn 仿(&self,乢:&J)->K<K<L>>{let mut 丄=乢.clone();丄.仌();let mut 丆=K::new();if
丄.丙().is_some(){return 丆;}let 亀=丄.丝();let 下=丄.下(亀);if let Some(为)=佒(&丄,T[self.丂],下){丆.push(为.万);}for 个 in 侺(&丄,侍,下,倬){丆.push(个.万);}let mut 俇=K::new();偤(&丄,&mut
俇);丆.push(俇);let mut 丟=佃{丂:亀,limit:亥{价:倭,..self.limit},亙:0,仴:以::default(),亽:None,佼:false,};丆.push(丟.佖(&丄,&[],&[],true).万);丆.retain(|万|!万.is_empty());丆}fn
侬(&self,乢:&J)->K<乱>{self.仿(乢).into_iter().map(|万|{let 佛=作(&侫(乢,&万),self.丂);乱{万,佛}}).collect()}fn 亚(&mut self,于:&J,亖:&乇,丞:&[乱])->U<仄>{let mut 乏=于.clone();if!佌(&mut
乏,亖.万())||self.侰(&乏){return None;}self.亙+=1;let 乴=self.佖(&乏,丞,&[],false);let mut 万=亖.万().to_vec();万.extend_from_slice(&乴.万);Some(仄{万,end:乴.end,first:Some(乽(亖.万())),东:None,})}fn
便(&self,业:&仄,丞:&[乱])->C{let 丣=self.併(&业.end,丞,C::MIN);match&业.东{Some(东)if!东.is_empty()=>丣.min(self.併(&业.end,东,C::MIN)),_=>丣,}}}fn 傄(主:&[乱],丞:&mut Vec<乱>)->P{let
丹:K<_> =丞.iter().map(|r|r.万.clone()).collect();for 乨 in 主{if 丞.iter().any(|r|r.万==乨.万){continue;}丞.push(乨.clone());}丞.sort_by_key(|乨|乨.佛);丞.truncate(倠);丞.iter().map(|r|&r.万).ne(&丹)}fn
傠(l:&J,乒:亥,二:&mut 五)->侦{let 丂=l.丝();let mut 丽=侦::default();if l.丙().is_some(){return 丽;}if let Some(万)=伧(l){丽.万=万;return 丽;}let 乗=|傌:交|match(乒.亣,乒.亸){(Some(亣),Some(亸))=>Some(亣+(亸-亣).mul_f64(傌)),_=>None,};if
倌.is_some_and(|与|l.乧()>=与){二.end();}if 二.偾(l){let 万=二.与(l,乗(0.4));let 偌=万.iter().any(|不|{matches!(不,L::乖{..}|L::云{..}|L::亇{..})});let 些=倊&&!偌&&l.乧()>=個&&l.下(丂)>=倉;if!些{丽.二=true;丽.万=万;return
丽;}二.end();}let mut 乪=佃{丂,limit:亥{亣:乗(俿),..乒},亙:0,仴:以::default(),亽:None,佼:傛(l),};let mut 丞=K::new();let mut 乄:K<仄> =K::new();let mut 仪=K::new();let mut
伝=false;let mut 亚:K<乇> =K::new();while 丽.佗<倘&&(乄.is_empty()||!乪.亄()){let 业=if 伝{let Some(亖)=亚.pop()else{break;};match 乪.亚(l,&亖,&丞){Some(业)=>业,None=>continue,}}else{乪.佖(l,&丞,&仪,true)};丽.佗+=1;let
俥=乄.iter().any(|p|p.万==业.万);let 书=业.first;let 俫=乪.亄();let mut 业=业;let mut 俓=false;if!俫&&!俥{let 主=乪.侬(&业.end);俓=傄(&主,&mut 丞);业.东=Some(主);}if!俥{乄.push(业);}if
俫{break;}if!俓&&!伝{match 书{Some(书)=>仪.push(书),None=>{伝=true;亚=乪.亽.clone().unwrap_or_default();亚.retain(|a|!仪.contains(&乽(a.万())));let 丼=|a:&乇|{乪.仴.get(&乽(a.万())).copied().unwrap_or(C::MIN)};亚.sort_by_key(丼);}}}}丽.亚=F::from(伝);丽.亙=乪.亙;丽.丞=丞.len();丽.乄=乄.len();let
亩=||乒.亸.is_some_and(|t|乯::now()>=t);let mut 仜:K<U<C>> =vec![None;乄.len()];for 丘 in(0..乄.len()).rev(){if 丘+1<乄.len()&&亩(){break;}仜[丘]=Some(乪.便(&乄[丘],&丞));}let
仚=|仜:&[U<C>]|{let mut 丏:U<(C,F)> =None;for(丘,丣)in 仜.iter().enumerate().rev(){if let Some(丣)=*丣{if 丏.is_none_or(|(乜,_)|丣>乜){丏=Some((丣,丘));}}}丏};let mut
丏=仚(&仜);乪.limit.亣=乒.亸;while let Some((_,丘))=丏{if!值||乄[丘].东.is_some()||亩(){break;}let 主=乪.侬(&乄[丘].end);乄[丘].东=Some(主);丽.侵+=1;仜[丘]=Some(乪.便(&乄[丘],&丞));丏=仚(&仜);}if
let Some((_,丘))=丏{let mut 丒=l.clone();for&不 in&乄[丘].万{if 丒.丸(不){丽.万.push(不);}}}丽}fn 偽(乒:&dyn Fn(乯,並)->亥,mut 二:五,佥:impl BufRead,伬:impl Write,)->Result<(),之>{乍(佥,伬,|仐,乵,与,亿|match
J::read(仐,乵,争::伹,与){Ok(l)=>{let 丽=傠(&l,乒(亿,与),&mut 二);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",与,丽.佗,丽.亙,丽.丞,丽.侵,丽.亚,亿.elapsed());偄(&丽.万)}Err(亷)=>{eprintln!("plan: cannot read the input: {}",亷);"WAIT".to_string()}},)}const
俼:交=45.0;const 倅:交=800.0;const 倲:交=0.6;fn 乒(亿:乯,与:並)->亥{if let Some(价)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亥{亣:None,亸:None,价,};}let 伱=傢().min(100.0);let 側=if 与==1{倅}else{俼};let 丿=Duration::from_secs_f64(側/1000.0*伱);亥{亣:Some(亿+丿.mul_f64(倲)),亸:Some(亿+丿),价:Z::MAX,}}const
倧:F=9;const 倥:亦=亦{丐:3,丛:1,下:1,个:2,亝:2,伄:2,丵:6,伆:1,丯:0,伖:false,乓:倨,};const 倫:並=0;const 倨:乹=乹::伻(伎::侗(7));const 侟:I=I::MAX;const 依:C=2;const 侞:C=18;const
侜:C=0;const 倦:F=32;const 俹:P=false;const 俺:Settings=Settings{佺:3,佹:1,佔:4,亩:2,佝:2,伜:2,使:true,reserve:true,佞:true,乘:产::乥,fill:true,义:Y{估:侟,仉:依,伢:侞,伒:侜,..Y::亳},};fn
二()->五{if 俹{return 五::new(乺::伊(俺),K::new(),Y::亳);}偸(倦)}fn 偸(了:F)->五{let 亃=乺::仧{乍:倧,了,丨:倥,伮:倫,};let 両=倰.iter().map(|&义|Y{估:侟,仉:依,伢:侞,伒:侜,..义}).collect();五::new(亃,両,Y::亳)}fn
main(){let 傗=io::stdin();let 傘=io::stdout();if let Err(亷)=偽(&乒,二(),傗.lock(),傘.lock()){eprintln!("plan: {}",亷);std::process::exit(1);}}const 倶:&str="CG_TIME_SCALE";fn
傢()->交{偹(std::env::var(倶).ok().as_deref())}fn 偹(严:U<&str>)->交{严.and_then(|乵|乵.trim().parse::<交>().ok()).filter(|伱|伱.is_finite()&&*伱>0.0).unwrap_or(1.0)}const
_:()=assert!(倄==16);const 倄:F=16;const 俾:交=0.4;const 倡:I=4;#[derive(Clone,Copy)]struct 亴{丯:G,侴:G,仸:G,}impl 亴{fn of(l:&J)->亴{let(a,们)=俛(l,T[0]);let(b,伛)=俛(l,T[1]);let
亰=a[T[1]];let 偬=们[T[1]];let mut 丮=亴{丯:G::O,侴:G::O,仸:G::O,};for 一 in l.map().valid.iter(){if 们[一]==I::MAX||伛[一]==I::MAX{continue;}if 们[一].abs_diff(伛[一])<=1{丮.侴.set(一);}if
们[一].abs_diff(伛[一])<=倡{丮.仸.set(一);}let 偵=伉::from(们[一])+伉::from(伛[一])==伉::from(偬);if 亰>0.0&&偵&&一!=T[0]&&一!=T[1]&&a[一]*b[一]/亰>=俾{丮.丯.set(一);}}丮}fn 亹(&self)->G{self.丯&self.仸}}fn
傚(l:&J,傕:G)->[I;M]{let 中=l.map().valid;let mut 丆=[I::MAX;M];let mut 亍=傕&中;let mut 伦=亍;let mut 丕=0u8;while!伦.is_empty(){for 一 in 伦.iter(){丆[一]=丕;}let 丄=(伦.丑()&中).丗(亍);亍|=丄;伦=丄;丕=丕.saturating_add(1);}丆}fn
俛(l:&J,三:F)->([交;M],[I;M]){let 乀=傚(l,G::一(三));let mut 乓:K<F> =l.map().valid.iter().filter(|&c|乀[c]!=I::MAX).collect();乓.sort_by_key(|&c|乀[c]);let mut 且=[0.0;M];且[三]=1.0;for
一 in 乓{if 一==三{continue;}let 乐=G::一(一).丑()&l.map().valid;且[一]=乐.iter().filter(|&n|乀[n]!=I::MAX&&乀[n]+1==乀[一]).map(|n|且[n]).sum();}(且,乀)}const Q:F=12;const
M:F=Q*Q;const 买:F=4*Q;const 亊:Z=(1<<买)-1;const 伋:Z=1|1<<12|1<<24|1<<36;const 伺:Z=伋<<11;const 侢:Z=(1<<Q)-1;#[derive(Clone,Copy,PartialEq)]struct G([Z;3]);impl
G{const O:G=G([0;3]);const 俸:G=G([亊;3]);#[inline]fn 一(一:F)->G{let mut 举=G::O;举.set(一);举}#[inline]fn test(self,一:F)->P{(self.0[一/买]>>(一%买))&1!=0}#[inline]fn
set(&mut self,一:F){self.0[一/买]|=1<<(一%买);}#[inline]fn clear(&mut self,一:F){self.0[一/买]&=!(1<<(一%买));}#[inline]fn is_empty(self)->P{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
伵(self)->[Z;3]{self.0}#[inline]fn count(self)->並{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 丗(self,丳:G)->G{G([self.0[0]&!丳.0[0],self.0[1]&!丳.0[1],self.0[2]&!丳.0[2],])}#[inline]fn
iter(self)->伸{伸{佪:self.0,乗:0,}}#[inline]fn 傑(self)->G{let[a,b,c]=self.0;G([(a>>Q)|((b&侢)<<(买-Q)),(b>>Q)|((c&侢)<<(买-Q)),c>>Q,])}#[inline]fn 傎(self)->G{let[a,b,c]=self.0;G([(a<<Q)&亊,((b<<Q)&亊)|(a>>(买-Q)),((c<<Q)&亊)|(b>>(买-Q)),])}#[inline]fn
傏(self)->G{let[a,b,c]=self.0;G([(a>>1)&!伺&亊,(b>>1)&!伺&亊,(c>>1)&!伺&亊,])}#[inline]fn 傐(self)->G{let[a,b,c]=self.0;G([(a<<1)&!伋&亊,(b<<1)&!伋&亊,(c<<1)&!伋&亊,])}#[inline]fn
丑(self)->G{let q=self;q.傑()|q.傎()|q.傏()|q.傐()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丳:G)->G{G([self.0[0]&丳.0[0],self.0[1]&丳.0[1],self.0[2]&丳.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丳:G)->G{G([self.0[0]|丳.0[0],self.0[1]|丳.0[1],self.0[2]|丳.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丳:G){*self=*self|丳;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::俸.丗(self)}}struct 伸{佪:[Z;3],乗:F,}impl Iterator for 伸{type Item=F;#[inline]fn
next(&mut self)->U<F>{let q=self;while q.乗<3{let 伨=q.佪[q.乗];if 伨!=0{q.佪[q.乗]=伨&(伨-1);return Some(q.乗*买+伨.trailing_zeros()as F);}q.乗+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{乷{世:並,x:I,y:I},乖{七:I,x:I,y:I},云{x:I,y:I},亇{x:I,y:I},}impl L{fn 傥(&self)->之{match*self{L::乷{世,x,y}=>format!("MOVE {} {x} {y}",世),L::乖{七,x,y}=>format!("TRAIN {} {x} {y}",七),L::云{x,y}=>format!("BUILD MINE {x} {y}"),L::亇{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
偄(万:&[L])->之{if 万.is_empty(){return "WAIT".to_string();}let mut line=之::with_capacity(16*万.len());for(丘,不)in 万.iter().enumerate(){if 丘>0{line.push(';');}line.push_str(&不.傥());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
争{侪,債,倿,#[default]伹,}impl 争{fn 乾(self)->I{match self{争::侪=>1,_=>3,}}fn 伓(self)->P{matches!(self,争::倿|争::伹)}fn 伔(self)->P{self==争::伹}}const 乸:I=I::MAX;struct
亓{乕:G,valid:G,久:G,两:Box<[I]>,丑:[[[I;4];M];2],}thread_local!{static 倕:RefCell<U<Arc<亓>>> =const{RefCell::new(None)};}impl 亓{fn 傍(乕:G,久:G)->Arc<亓>{倕.with(|亞|{let
mut 亞=亞.borrow_mut();if let Some(仓)=亞.as_ref(){if 仓.乕==乕&&仓.久==久{return Arc::clone(仓);}}let 仓=Arc::new(亓::build(乕,久));*亞=Some(Arc::clone(&仓));仓})}fn build(乕:G,久:G)->亓{let
中=!乕;const 倳:[[(伈,伈);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 丑=[[[乸;4];M];2];for(丁,乛)in 倳.iter().enumerate(){for 一 in
中.iter(){let(x,y)=((一%Q)as 伈,(一/Q)as 伈);let mut 且=0;for&(偗,偘)in 乛{let(俖,俗)=(x+偗,y+偘);if(0..Q as 伈).contains(&俖)&&(0..Q as 伈).contains(&俗){let 丄=俗 as F*Q+俖
as F;if 中.test(丄){丑[丁][一][且]=丄 as I;且+=1;}}}}}let mut 两=vec![乸;M*M].into_boxed_slice();let mut 佴=[0u8;M];for 三 in 中.iter(){let 乩=&mut 两[三*M..(三+1)*M];乩[三]=0;佴[0]=三
as I;let(mut 佢,mut 佾)=(0,1);while 佢<佾{let 一=佴[佢]as F;佢+=1;for&丄 in&丑[0][一]{if 丄!=乸&&乩[丄 as F]==乸{乩[丄 as F]=乩[一]+1;佴[佾]=丄;佾+=1;}}}}亓{乕,valid:中,久,两,丑,}}#[inline]fn
两(&self,三:F,丷:F)->I{self.两[三*M+丷]}}const 借:並=100;const 亐:C=15;const 债:[C;3]=[1,4,20];const T:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 习{乻(F),侎,}enum
乶{佂,供,侠,}#[derive(Clone)]struct J{map:Arc<亓>,丰:争,东:[G;2],丅:[G;2],乃:G,乣:G,丈:[G;2],七:[G;3],享:G,仠:[並;M],下:[C;2],丛:[C;2],丝:F,乧:並,仗:並,丙:U<习>,}fn 仲(东:G,仏:F)->G{if!东.test(仏){return
G::O;}let mut 仾=G::一(仏);loop{let 丄=(仾|仾.丑())&东;if 丄==仾{return 仾;}仾=丄;}}impl J{fn 丝(&self)->F{self.丝}fn 乧(&self)->並{self.乧}fn 丙(&self)->U<习>{self.丙}fn 丰(&self)->争{self.丰}fn
他(&self,丁:F)->C{let q=self;let 丈=q.丈[丁];q.下[丁]+10*C::from((丈&q.七[0]).count())+20*C::from((丈&q.七[1]).count())+30*C::from((丈&q.七[2]).count())}fn 下(&self,丁:F)->C{self.下[丁]}fn
丛(&self,丁:F)->C{self.丛[丁]}fn 侂(&self,丁:F)->F{self.丈[丁].count()as F}fn map(&self)->&亓{&self.map}fn 专(&self,丁:F)->G{self.东[丁]}fn 丅(&self,丁:F)->G{self.丅[丁]}fn
丈(&self,丁:F)->G{self.丈[丁]}fn 什(&self,一:F)->U<(並,I)>{let q=self;(q.丈[0]|q.丈[1]).test(一).then(||(q.仠[一],q.佩(一)))}fn 乿(&self)->G{self.丈[self.丝]&self.享}fn
丧(&self)->G{self.乃}fn 乆(&self)->G{self.乣}fn 亅(&self,一:F)->U<I>{(一<M&&(self.丈[0]|self.丈[1]).test(一)).then(||self.佩(一))}fn 傖(&mut self,丁:F){let q=self;q.丅=[仲(q.东[0],T[0]),仲(q.东[1],T[1])];let
偒=q.丈[丁].丗(q.丅[丁]);q.佶(偒);q.侶();q.下[丁]+=q.丛[丁];if q.下[丁]<0{q.下[丁]=0;q.佶(q.丈[丁]);}q.享|=q.丈[丁];}fn 侶(&mut self){let q=self;for 丁 in 0..2{let 丅=q.丅[丁];let
丈=丅&q.丈[丁];let mut 丛=C::from(丅.count())+4*C::from((丅&q.乃).count());for(七,傫)in 债.iter().enumerate(){丛-=傫*C::from((丈&q.七[七]).count());}q.丛[丁]=丛;}}fn 仌(&mut
self){let q=self;if q.丙.is_some(){return;}if q.丝==1{if q.乧>=借{q.丙=Some(match q.他(0).cmp(&q.他(1)){std::cmp::Ordering::Greater=>习::乻(0),std::cmp::Ordering::Less=>习::乻(1),std::cmp::Ordering::Equal=>习::侎,});return;}q.乧+=1;}q.丝=1-q.丝;q.傖(q.丝);}fn
伭(&mut self,万:&[L]){let q=self;if q.丙.is_some(){return;}let 你=q.仗;for&不 in 万{if q.偖(不,你){continue;}q.丸(不);if q.丙.is_some(){return;}}q.仌();}fn 偖(&self,不:L,你:並)->P{let
q=self;match 不{L::乷{世,x,y}=>{q.仈(x,y).is_none()||世>=你||q.俀(q.丝,世).is_none()}L::乖{七,x,y}=>{q.仈(x,y).is_none()||七==0||七>q.丰.乾()}L::云{x,y}|L::亇{x,y}=>{q.仈(x,y).is_none()}}}#[inline]fn
仈(&self,x:I,y:I)->U<F>{let(x,y)=(F::from(x),F::from(y));(x<Q&&y<Q&&!self.map.乕.test(y*Q+x)).then_some(y*Q+x)}fn 俀(&self,丁:F,世:並)->U<F>{self.丈[丁].iter().find(|&一|self.仠[一]==世)}#[inline]fn
佩(&self,一:F)->I{if self.七[0].test(一){1}else if self.七[1].test(一){2}else{3}}fn 偈(&self,亀:F)->G{let 乆=self.乣&self.东[亀];let 丅=self.丅[亀];乆|(丅&(丅&乆).丑())}fn
乂(&self,丁:F,七:I)->G{let q=self;debug_assert!((1..=3).contains(&七),"unit level {} is not 1 to 3",七);let 亀=1-丁;let 丈=q.丈[0]|q.丈[1];let 偙=q.介();let 傩=match
七{1 if q.丰==争::侪=>q.七[1]|q.七[2],1=>丈,2=>q.七[1]|q.七[2],_=>G::O,};let 乂=偙|q.东[亀].丗(傩);if 七<3{乂.丗(q.偈(亀))}else{乂}}fn 京(&self,丁:F)->G{self.丅[丁]|self.丅[丁].丑()}fn
介(&self)->G{let q=self;let 丈=q.丈[0]|q.丈[1];let 偩=G::一(T[0])|G::一(T[1]);q.map.valid.丗(丈|q.乃|q.乣|偩)}fn 丸(&mut self,不:L)->P{let q=self;if q.丙.is_some(){return
false;}let 丁=q.丝;let 亵=match 不{L::乖{七,x,y}=>q.傧(丁,七,x,y),L::乷{世,x,y}=>q.偳(丁,世,x,y),L::云{x,y}=>q.build(丁,true,x,y),L::亇{x,y}=>q.build(丁,false,x,y),};let
Some(亵)=亵 else{return false;};match 亵{乶::佂=>{}乶::供=>q.丅[丁]=仲(q.东[丁],T[丁]),乶::侠=>{q.丅=[仲(q.东[0],T[0]),仲(q.东[1],T[1])];}}q.侶();if!q.东[0].test(T[0]){q.丙=Some(习::乻(1));}else
if!q.东[1].test(T[1]){q.丙=Some(习::乻(0));}true}fn 佶(&mut self,丐:G){let q=self;q.丈[0]=q.丈[0].丗(丐);q.丈[1]=q.丈[1].丗(丐);for 七 in&mut q.七{*七=七.丗(丐);}q.享=q.享.丗(丐);}fn
俯(&mut self,丁:F,一:F)->乶{let q=self;q.佶(G::一(一));q.乃.clear(一);q.乣.clear(一);let 亀=1-丁;if q.东[亀].test(一){q.东[亀].clear(一);q.东[丁].set(一);乶::侠}else if q.东[丁].test(一){乶::佂}else{q.东[丁].set(一);乶::供}}fn
傧(&mut self,丁:F,七:I,x:I,y:I)->U<乶>{let q=self;if 七==0||七>q.丰.乾(){return None;}let 一=q.仈(x,y)?;let 上=10*C::from(七);if q.下[丁]<上||!q.京(丁).test(一)||!q.乂(丁,七).test(一){return
None;}let 亵=q.俯(丁,一);q.俜(丁,七,一,q.仗);q.仗=q.仗.saturating_add(1);q.下[丁]-=上;Some(亵)}fn 俜(&mut self,丁:F,七:I,一:F,世:並){self.丈[丁].set(一);self.七[F::from(七)-1].set(一);self.仠[一]=世;}fn
偳(&mut self,丁:F,世:並,x:I,y:I)->U<乶>{let q=self;let 丌=q.仈(x,y)?;let 三=q.俀(丁,世)?;if!q.享.test(三){return None;}let 七=q.佩(三);let 丢=q.乂(丁,七);let 乐=q.map.丑[丁][三];if
乐.contains(&(丌 as I))&&!丢.test(丌){return None;}let mut 丏=q.map.两(三,丌);let mut 仮=None;for&丄 in 乐.iter().take_while(|&&丄|丄!=乸){let 丄=F::from(丄);let 两=q.map.两(丄,丌);if
两<丏&&丢.test(丄){丏=两;仮=Some(丄);}}let 仮=仮?;let 亵=q.俯(丁,仮);q.丈[丁].clear(三);q.七[F::from(七)-1].clear(三);q.享.clear(三);q.俜(丁,七,仮,世);Some(亵)}fn build(&mut self,丁:F,乃:P,x:I,y:I)->U<乶>{let
q=self;let 偃=if 乃{q.丰.伓()}else{q.丰.伔()};let 一=q.仈(x,y)?;let 偶=q.map.久.test(一);if!偃||q.下[丁]<亐||偶!=乃||!q.东[丁].test(一)||!q.京(丁).test(一)||!q.介().test(一){return
None;}if 乃{q.乃.set(一);}else{q.乣.set(一);}q.下[丁]-=亐;Some(乶::佂)}}const 倚:C=1000000000;const 伿:C=0;const 倢:C=1;const 倷:C=2;impl J{fn read(仐:&str,与:&str,丰:争,乧:並,)->Result<J,之>{let
mut 久=K::new();let mut 丱=仐.lines();let 且=余(&mut 丱,"mine spot count")?;for _ in 0..且{let[x,y]=亡(付(&mut 丱,"a mine spot")?,"a mine spot")?;久.push((伙(x)?,伙(y)?));}侽(丱,"the initial input")?;let
mut 丱=与.lines();let mut 佣=[0i64;4];for(严,乎)in 佣.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[佲]=亡(付(&mut 丱,乎)?,乎)?;if 佲.abs()>倚{return
Err(format!("{}: {} is out of range",乎,佲));}*严=佲;}let mut 乕=G::O;let mut 丐=[G::O;2];let mut 丅=[G::O;2];for y in 0..Q{let 乩=付(&mut 丱,"a map row")?.trim();if
乩.len()!=Q{return Err(format!("map row {y} is not {} characters: {:?}",Q,乩));}for(x,企)in 乩.bytes().enumerate(){let 一=y*Q+x;match 企{b'#'=>乕.set(一),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
九=F::from(企==b'X'||企==b'x');丐[九].set(一);if 企.is_ascii_uppercase(){丅[九].set(一);}}_=>{return Err(format!("map row {y} has {:?} at {x}",企 as char))}}}}let
偋=余(&mut 丱,"building count")?;let mut 乬=K::new();for _ in 0..偋{let 亡=亡(付(&mut 丱,"a building")?,"a building")?;乬.push(亡);}let 侂=余(&mut 丱,"unit count")?;let
mut 丈=K::new();for _ in 0..侂{let 亡=亡(付(&mut 丱,"a unit")?,"a unit")?;丈.push(亡);}侽(丱,"the turn's input")?;let 俙=乬.iter().find(|侱|侱[0]==0&&侱[1]==伿).ok_or("the bot's own HQ is not among the buildings")?;let
俊=佑(俙[2],俙[3],乕)?;let 丁=T.iter().position(|&仏|仏==俊).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",俊))?;let mut 俧=G::O;for(x,y)in 久{if
乕.test(y*Q+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}俧.set(y*Q+x);}let 佉=|九:F|if 九==0{丁}else{1-丁};let mut l=J{map:亓::傍(乕,俧),丰,东:[G::O;2],丅:[G::O;2],乃:G::O,乣:G::O,丈:[G::O;2],七:[G::O;3],享:G::O,仠:[0;M],下:[0;2],丛:[0;2],丝:丁,乧,仗:1,丙:None,};for
九 in 0..2{let 亂=佉(九);l.东[亂]=丐[九];l.丅[亂]=丅[九];l.下[亂]=佣[2*九];l.丛[亂]=佣[2*九+1];}let mut 俘=G::O;for[亂,伤,x,y]in 乬{let 九=俢(亂)?;let 一=佑(x,y,乕)?;if!丐[九].test(一){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 俘.test(一){return Err(format!("two buildings at ({x}, {y})"));}俘.set(一);match 伤{伿
if 一==T[佉(九)]=>{}伿=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}倢=>l.乃.set(一),倷=>l.乣.set(一),_=>return Err(format!("unknown building type {}",伤)),}}let
mut 乞=G::O;let mut 伥=0u32;for[亂,世,七,x,y]in 丈{let 九=俢(亂)?;let 一=佑(x,y,乕)?;if!丐[九].test(一){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乞.test(一){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&七){return Err(format!("unit level {} is not 1, 2 or 3",七));}let 世=並::try_from(世).map_err(|_|format!("unit id {} is out of range",世))?;if
乞.iter().any(|丳|l.仠[丳]==世){return Err(format!("two units have id {}",世));}乞.set(一);l.丈[佉(九)].set(一);l.七[七 as F-1].set(一);l.仠[一]=世;伥=伥.max(世);}if!乞.is_empty(){l.仗=伥.checked_add(1).ok_or_else(||format!("unit id {} is out of range",伥))?;}l.享=l.丈[丁];Ok(l)}}fn
付<'a>(丱:&mut Lines<'a>,乎:&str)->Result<&'a str,之>{丱.next().ok_or_else(||format!("the text ends before {}",乎))}fn 侽(mut 丱:Lines<'_>,乎:&str)->Result<(),之>{match
丱.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",乎)),None=>Ok(()),}}fn 亡<const N:F>(line:&str,乎:&str)->Result<[C;N],之>{let
mut 伵=line.split_whitespace();let mut 俴=[0i64;N];for 严 in&mut 俴{let 仢=伵.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",乎))?;*严=仢.parse().map_err(|_|format!("{}: {:?} is not an integer",乎,仢))?;}if
伵.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",乎));}Ok(俴)}fn 余(丱:&mut Lines<'_>,乎:&str)->Result<F,之>{let[且]=亡(付(丱,乎)?,乎)?;F::try_from(且).ok().filter(|&且|且<=M).ok_or_else(||format!("{}: {} is out of range",乎,且))}fn
伙(严:C)->Result<F,之>{F::try_from(严).ok().filter(|&严|严<Q).ok_or_else(||format!("coordinate {} is off the map",严))}fn 佑(x:C,y:C,乕:G)->Result<F,之>{let 一=伙(y)?*Q+伙(x)?;if
乕.test(一){return Err(format!("({x}, {y}) is a void cell"));}Ok(一)}fn 俢(亂:C)->Result<F,之>{match 亂{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",亂)),}}#[derive(Clone,Copy,PartialEq)]enum
产{乥,侈,}#[derive(Clone,Copy)]struct Settings{佺:I,佹:I,佔:C,亩:C,佝:F,伜:I,使:P,reserve:P,佞:P,乘:产,fill:P,义:Y,}impl Settings{const DEFAULT:Settings=Settings{佺:3,佹:1,佔:4,亩:2,佝:2,伜:2,使:true,reserve:true,佞:true,乘:产::乥,fill:true,义:Y::亳,};}const
佇:C=1000;const 但:C=10;fn 侅(一:F)->(I,I){((一%Q)as I,(一/Q)as I)}fn 佳(l:&mut J,不:L,丆:&mut Vec<L>)->P{let 仨=l.丸(不);if 仨{丆.push(不);}仨}fn 丕(l:&mut J,三:F,丷:F,丆:&mut
Vec<L>)->P{let Some((世,_))=l.什(三)else{return false;};let(x,y)=侅(丷);佳(l,L::乷{世,x,y},丆)}fn 傟(l:&J,丮:&亴,丶:&Settings,乒:F,)->K<F>{let 丂=l.丝();let 丫=乁(l,l.专(丂));let
丟=乁(l,l.专(1-丂));let 佰=l.map().valid.丗(l.专(0)|l.专(1));let 俠=|c:F|丫[c]!=I::MAX&&丟[c]!=I::MAX;let mut 丯:K<F> =丮.亹().丗(l.专(丂)).iter().filter(|&c|俠(c)).collect();丯.sort_by_key(|&c|(丫[c],c));let
mut 临:K<F> =佰.iter().filter(|&c|俠(c)&&丫[c].abs_diff(丟[c])<=丶.佹).collect();临.sort_by_key(|&c|(丫[c].abs_diff(丟[c]),丫[c],c));let mut 丆:K<F> =K::new();for
一 in 丯.into_iter().chain(临){if 丆.len()>=乒{break;}let 偝=丆.iter().all(|&t|l.map().两(t,一)>=丶.佺);let 偐=丮.亹().test(一);if!丆.contains(&一)&&(偐||偝){丆.push(一);}}丆}fn
偪(上:&[K<C>])->K<F>{let n=上.len();let 俍=C::MAX/4;let mut u=vec![0i64;n+1];let mut v=vec![0i64;n+1];let mut p=vec![0usize;n+1];let mut 俶=vec![0usize;n+1];for
i in 1..=n{p[0]=i;let mut 亜=0usize;let mut 仹=vec![俍;n+1];let mut 侃=vec![false;n+1];loop{侃[亜]=true;let 俌=p[亜];let mut 仭=俍;let mut 仵=0usize;for j in 1..=n{if!侃[j]{let
乭=上[俌-1][j-1]-u[俌]-v[j];if 乭<仹[j]{仹[j]=乭;俶[j]=亜;}if 仹[j]<仭{仭=仹[j];仵=j;}}}for j in 0..=n{if 侃[j]{u[p[j]]+=仭;v[j]-=仭;}else{仹[j]-=仭;}}亜=仵;if p[亜]==0{break;}}loop{let
仵=俶[亜];p[亜]=p[仵];亜=仵;if 亜==0{break;}}}let mut 丆=vec![0usize;n];for j in 1..=n{if p[j]>0{丆[p[j]-1]=j-1;}}丆}#[derive(Clone,Copy)]enum 亲{佈(F),佅,}fn 伅(l:&J)->G{let
丂=l.丝();(l.京(丂)&l.乂(丂,1)).丗(l.专(丂))}fn 偻(l:&J,义:&Y)->F{伴(l,义,么::乥).iter().filter(|c|matches!(c,L::乖{..})).count()}fn 串(一:F,丠:G)->並{(G::一(一).丑()&丠).count()}fn
俄(l:&J,丼:G,丌:F,佨:I)->K<[C;M]>{let 中=l.map().valid;let mut 丏=[C::MIN;M];丏[丌]=0;let mut 乚=vec![丏];for _ in 0..佨{let mut 丄=丏;for 一 in 中.iter(){for n in(G::一(一).丑()&中).iter(){if
丏[n]>C::MIN{let 严=丏[n]+C::from(丼.test(n));if 严>丄[一]{丄[一]=严;}}}}丏=丄;乚.push(丏);}乚}fn 俚(l:&J,乚:&[[C;M]],丼:G,三:F,k:F,)->G{let 中=l.map().valid;let mut 丆=G::O;let
mut 一=三;let mut k=k.min(乚.len()-1);while k>0&&乚[k][一]>0{let Some(丄)=(G::一(一).丑()&中).iter().filter(|&n|乚[k-1][n]>C::MIN).max_by_key(|&n|{(C::from(丼.test(n))+乚[k-1][n],std::cmp::Reverse(n),)})else{break;};if
丼.test(丄){丆.set(丄);}一=丄;k-=1;}丆}fn 与(l:&J,丶:&Settings)->K<L>{let 丂=l.丝();let 丮=亴::of(l);let mut 丒=l.clone();let mut 丆=K::new();let 丟=乁(l,l.专(1-丂));let
丈:K<F> =l.乿().iter().collect();let 俆=(l.下(丂)/但).max(0)as F;let 伳=match 丶.乘{产::乥=>偻(l,&丶.义).min(俆),产::侈=>俆,};let mut 仩:K<亲> =丈.iter().map(|&c|亲::佈(c)).collect();仩.extend(std::iter::repeat_n(亲::佅,伳));let
仳=傟(l,&丮,丶,仩.len()+丶.佝);let 储=伅(l);let 丯=丮.亹();let n=仩.len().max(仳.len());let mut 上=vec![vec![0i64;n];n];for(乩,&丌)in 仳.iter().enumerate(){for(仫,亾)in 上[乩].iter_mut().enumerate(){let
Some(&偂)=仩.get(仫)else{*亾=佇;continue;};let 伕=match 偂{亲::佈(一)=>{let d=l.map().两(一,丌);if d==I::MAX{None}else{Some(C::from(d)-1)}}亲::佅=>储.iter().map(|c|l.map().两(c,丌)).filter(|&d|d!=I::MAX).min().map(C::from),};*亾=match
伕{None=>佇,Some(伕)=>{let 乼=C::from(丟[丌])-1;let 亩=丶.亩*(伕-乼).max(0);let 位=if 丯.test(丌){丶.佔}else{0};(伕+亩-位).max(0)}};}}let 偅=偪(&上);let mut 佡:K<(F,F)> =K::new();let
mut 侁:K<(C,F)> =K::new();for(乩,&丌)in 仳.iter().enumerate(){let 仫=偅[乩];if 上[乩][仫]>=佇{continue;}match 仩.get(仫){Some(亲::佈(一))=>佡.push((*一,丌)),Some(亲::佅)=>侁.push((上[乩][仫],丌)),None=>{}}}let
mut 丠=l.map().valid.丗(l.专(0)|l.专(1));let mut 俔:K<F> =K::new();let mut 伯=G::O;let 亁=|一:F|{let 七=l.亅(一).unwrap_or(1);let 伂=丠|l.专(1-丂);(G::一(一).丑()&伂&l.乂(丂,七)).count()};佡.sort_by_key(|&(一,丌)|{let
偞=if 丶.佞{亁(一)}else{0};(偞,l.map().两(一,丌),一)});for&(三,丌)in&佡{let Some(七)=丒.亅(三)else{continue;};let 丢=丒.乂(丂,七);let d=丒.map().两(三,丌);let 丼=(丠|丒.专(1-丂)).丗(伯);let
佨=d.saturating_add(丶.伜);let 乚=俄(&丒,丼,丌,佨.saturating_sub(1));let 仇=乚[乚.len()-1];let 丄=(G::一(三).丑()&丢).iter().filter(|&c|仇[c]>C::MIN).max_by_key(|&c|{let
偑=d.saturating_sub(丒.map().两(c,丌));let 伂=丠.test(c)||丒.专(1-丂).test(c);(伂&&丶.使,C::from(丼.test(c))+仇[c],偑,串(c,丠),std::cmp::Reverse(c),)}).filter(|&c|{丠.test(c)||丒.专(1-丂).test(c)||丒.map().两(c,丌)<d});if
let Some(丷)=丄{if 丕(&mut 丒,三,丷,&mut 丆){丠.clear(丷);俔.push(三);if 丶.reserve{伯|=俚(&丒,&乚,丼,丷,乚.len()-1);}}}}let 丫=乁(l,l.专(丂));let mut 乊=G::O;for c in 丠.iter(){if
丫[c]<=丟[c]{乊.set(c);}}for&三 in&丈{if 俔.contains(&三)||!丒.乿().test(三){continue;}let Some(七)=丒.亅(三)else{continue;};let 丢=丒.乂(丂,七);let 伂=(G::一(三).丑()&丠&丢).iter().max_by_key(|&c|(串(c,丠),丟[c]==I::MAX,std::cmp::Reverse(c)));if
let Some(丷)=伂{if 丕(&mut 丒,三,丷,&mut 丆){丠.clear(丷);}continue;}let 乮=[乊&丠,丠].into_iter().find_map(|丐|{丐.iter().min_by_key(|&c|(丒.map().两(三,c),c))});if let
Some(乮)=乮{let d=丒.map().两(三,乮);let 丄=(G::一(三).丑()&丢).iter().filter(|&c|丒.map().两(c,乮)<d).min_by_key(|&c|c);if let Some(丷)=丄{丕(&mut 丒,三,丷,&mut 丆);}}}侁.sort();let
mut 佫=0usize;for&(_,丌)in&侁{if 丒.下(丂)<但{break;}let 丐=伅(&丒);let Some(亠)=丐.iter().map(|c|丒.map().两(c,丌)).min()else{break;};let 丼=(丠|丒.专(1-丂)).丗(伯);let 乚=俄(&丒,丼,丌,亠.saturating_add(丶.伜));let
仇=乚[乚.len()-1];let 一=丐.iter().filter(|&c|丒.map().两(c,丌)==亠).max_by_key(|&c|(仇[c],串(c,丠),std::cmp::Reverse(c)));let Some(一)=一 else{break;};let(x,y)=侅(一);if
佳(&mut 丒,L::乖{七:1,x,y},&mut 丆){丠.clear(一);佫+=1;if 丶.reserve{伯|=俚(&丒,&乚,丼,一,乚.len()-1);}}}if 丶.fill&&丶.乘==产::乥{while 佫<伳&&丒.下(丂)>=但{let 一=(伅(&丒)&丠).iter().max_by_key(|&c|{(串(c,丠),std::cmp::Reverse(丟[c]),std::cmp::Reverse(c),)});let
Some(一)=一 else{break;};let(x,y)=侅(一);if!佳(&mut 丒,L::乖{七:1,x,y},&mut 丆){break;}丠.clear(一);佫+=1;}}丆}const 倎:C=15;const 倣:C=6;const 侧:C=10;const 倯:I=3;#[derive(Clone,Copy)]struct
Y{open:C,优:C,侹:I,佤:C,伢:C,俐:C,估:I,侸:C,侷:C,伒:C,从:C,仞:C,乔:C,仑:C,亱:C,仉:C,}impl Y{const 亳:Y=Y{open:3,优:2,侹:12,佤:4,伢:倎,俐:倣,估:倯,侸:2,侷:2,伒:0,从:0,仞:0,乔:0,仑:0,亱:0,仉:0,};const
fn 傳(self,从:C,仞:C,乔:C)->Y{Y{从,仞,乔,..self}}const fn 傲(self,仑:C,亱:C)->Y{Y{仑,亱,..self}}}fn 伇(一:F)->(I,I){((一%Q)as I,(一/Q)as I)}fn 乁(l:&J,三:G)->[I;M]{let mut
乀=[I::MAX;M];let 中=l.map().valid;let mut 临=三&中;let mut 乞=临;let mut d=0u8;while!临.is_empty(){for 一 in 临.iter(){乀[一]=d;}let 丄=(临.丑()&中).丗(乞);乞|=丄;临=丄;d=d.saturating_add(1);}乀}fn
偡(l:&J)->I{let 乀=乁(l,l.专(0));l.专(1).iter().map(|一|乀[一]).min().unwrap_or(I::MAX)}struct 亨<'a>{义:&'a Y,丂:F,theirs:[I;M],仟:[I;M],free:G,乊:G,ours:G,久:G,丈:G,}impl
亨<'_>{fn new<'a>(l:&J,义:&'a Y)->亨<'a>{let 丂=l.丝();let 乃=乁(l,l.专(丂));let 丟=乁(l,l.专(1-丂));let 仟=乁(l,G::一(T[1-丂]));let 丠=l.map().valid.丗(l.专(0)|l.专(1));let
mut 乊=G::O;for 一 in 丠.iter(){if 乃[一]<=丟[一]{乊.set(一);}}亨{义,丂,theirs:丟,仟,free:丠,乊,ours:l.专(丂),久:l.map().久,丈:l.丈(丂),}}fn value(&self,一:F)->C{let q=self;let
p=q.义;let 串=C::from((G::一(一).丑()&q.free).count());let 侳=p.侹;let 优=C::from(侳)-C::from(q.theirs[一].min(侳));let 仏=if p.佤>0{C::from(q.仟[一].min(30))/p.佤}else{0};let
东=C::from((G::一(一).丑()&q.ours).count());let line=if 东<=1{p.仞}else{0};let 乔=if q.久.test(一){p.乔}else{0};let 傝=if 串==0{p.仑}else{0};let 亱=p.亱*(22-C::from(q.仟[一].min(22)));let
仉=p.仉*C::from((G::一(一).丑()&q.丈).count());p.open*串+p.优*优-仏+p.从*东-line+乔-傝+亱-仉}fn 丏(&self,丐:G)->U<F>{丐.iter().max_by_key(|&一|(self.value(一),std::cmp::Reverse(一)))}fn
何(&mut self,一:F){let q=self;q.free.clear(一);q.乊.clear(一);q.ours.set(一);q.丈.set(一);}}fn 仼(l:&mut J,不:L,丆:&mut Vec<L>)->P{let 仨=l.丸(不);if 仨{丆.push(不);}仨}fn
俪(l:&mut J,三:F,丷:F,丆:&mut Vec<L>)->P{let Some((世,_))=l.什(三)else{return false;};let(x,y)=伇(丷);仼(l,L::乷{世,x,y},丆)}fn 傪(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-侧}fn
傈(l:&J,义:&Y)->K<L>{伴(l,义,么::乥)}#[derive(Clone,Copy,PartialEq)]enum 么{乥,互{丧:I,丈:I},}fn 伴(l:&J,义:&Y,乘:么)->K<L>{let mut 丒=l.clone();let mut 与=亨::new(l,义);let
mut 丆=K::new();亟(&mut 丒,&mut 与,&mut 丆);match 乘{么::乥=>傁(&mut 丒,&mut 与,&mut 丆),么::互{丧,丈}=>{停(&mut 丒,&mut 与,丧,丈,&mut 丆)}}丆}fn 亟(l:&mut J,与:&mut 亨<'_>,丆:&mut
Vec<L>){let 丂=与.丂;let mut 些:K<F> =K::new();loop{let 丢=l.乂(丂,1);let mut 仚:U<(並,F)> =None;for 三 in l.乿().iter(){if 些.contains(&三){continue;}let 亁=(G::一(三).丑()&与.free&丢).count();if
亁==0{些.push(三);}else if 仚.is_none_or(|(n,_)|亁<n){仚=Some((亁,三));}}let Some((_,三))=仚 else{break;};与.丈.clear(三);match 与.丏(G::一(三).丑()&与.free&丢){Some(丷)if
俪(l,三,丷,丆)=>与.何(丷),_=>{与.丈.set(三);些.push(三);}}}for 三 in 些{if!l.乿().test(三){continue;}let 丌=[与.乊,与.free].into_iter().find_map(|丐|{丐.iter().min_by_key(|&一|(l.map().两(三,一),一))});if
let Some(丌)=丌{俪(l,三,丌,丆);}}}fn 傁(l:&mut J,与:&mut 亨<'_>,丆:&mut Vec<L>){let 丂=与.丂;let p=与.义;let h=(p.伢-C::from(l.乧())).max(p.俐);let 傣=(C::from(偡(l))-p.侸).max(0)/p.侷.max(1)+p.伒;loop{let
下=l.下(丂);let 丈=l.侂(丂)as C;let 傇=C::from(与.乊.count());let 一=与.丏(l.京(丂)&与.free&l.乂(丂,1));let 俳=傪(h,(傇/(丈+1)).min(傣));let 侲=一.is_some()&&下>=侧&&俳>0;let 俑=4*h-亐;let
乔=俒(l,与);let 偎=乔.is_some()&&下>=亐&&俑>0;if 偎&&(!侲||俑>=俳){let(x,y)=伇(乔.unwrap_or_default());if 仼(l,L::云{x,y},丆){continue;}break;}let Some(一)=一.filter(|_|侲)else{break;};let(x,y)=伇(一);if!仼(l,L::乖{七:1,x,y},丆){break;}与.何(一);}}fn
俒(l:&J,与:&亨<'_>)->U<F>{let 丂=与.丂;(l.map().久&l.丅(丂)&l.介()).iter().filter(|&乔|与.theirs[乔]>与.义.估).min_by_key(|&乔|(l.map().两(T[丂],乔),乔))}fn 停(l:&mut J,与:&mut
亨<'_>,丧:I,丈:I,丆:&mut Vec<L>,){let 丂=与.丂;for _ in 0..丧{let Some(乔)=俒(l,与)else{break;};let(x,y)=伇(乔);if!仼(l,L::云{x,y},丆){break;}}for _ in 0..丈{let Some(一)=与.丏(l.京(丂)&与.free&l.乂(丂,1))else{break;};let(x,y)=伇(一);if!仼(l,L::乖{七:1,x,y},丆){break;}与.何(一);}}fn
亍(l:&J)->P{if l.丙().is_some(){return false;}let 丂=l.丝();let 下=l.下(丂);let 丵=仅::new(l,丂,true);l.专(1-丂).iter().any(|一|丵.上[一]<=下)}#[derive(Clone,Copy)]struct
丩{乤:I,乘:么,}const 來:I=I::MAX;const 俽:[么;8]=[么::乥,么::互{丧:0,丈:0},么::互{丧:0,丈:1},么::互{丧:0,丈:2},么::互{丧:0,丈:來,},么::互{丧:1,丈:0},么::互{丧:1,丈:1},么::互{丧:1,丈:來,},];struct
似<'a>{両:&'a[Y],亪:Y,rest:丩,亘:U<乯>,丶:Settings1,}impl 似<'_>{fn 义(&self,丕:丩)->&Y{&self.両[F::from(丕.乤)%self.両.len()]}fn 万(&self,l:&J,丕:丩)->K<L>{伴(l,self.义(丕),丕.乘)}}fn
偀(l:&J,万:&[L],亪:&Y)->(J,P){let mut 丄=l.clone();丄.伭(万);if 丄.丙().is_some()||亍(&丄){return(丄,true);}let 乨=傈(&丄,亪);丄.伭(&乨);let 亢=丄.丙().is_some()||亍(&丄)||丄.乧()>侌;(丄,亢)}fn
傀(l:&J,业:&[丩],亶:&似<'_>)->K<Z>{let 丂=l.丝();let mut 件=vec![伀(l,丂)];let mut 也=l.clone();for 与 in 0..侌 as F{let 丕=业.get(与).copied().unwrap_or(亶.rest);let 万=亶.万(&也,丕);let(丄,亢)=偀(&也,&万,&亶.亪);if
亢{break;}也=丄;件.push(伀(&也,丂));}件}fn 伀(l:&J,丂:F)->Z{let mut hash:Z=0xcbf29ce484222325;let mut 佬=|仢:Z|hash=(hash^仢).wrapping_mul(0x0100000001b3);for(傞,举)in[l.专(丂),l.丈(丂),l.丧()].into_iter().enumerate(){for
一 in 举.iter(){佬(((傞 as Z)<<8)|一 as Z);}佬(0xff);}佬(l.下(丂)as Z);hash}enum 乺{仧{乍:F,了:F,丨:亦,伮:並,},伊(Settings),}struct 五{亃:乺,両:K<Y>,亪:Y,业:K<丩>,伞:K<Z>,亻:F,乄:並,亢:P,}impl
五{fn new(亃:乺,両:K<Y>,亪:Y)->五{五{亃,両,亪,业:K::new(),伞:K::new(),亻:0,乄:0,亢:false,}}fn 偾(&mut self,l:&J)->P{let q=self;let 傓=matches!(q.亃,乺::仧{..}|乺::伊(_));let
偿=!傓&&q.乄>0&&q.亻>=q.伞.len();if!q.亢&&(偿||亍(l)){q.亢=true;}!q.亢}fn end(&mut self){self.亢=true;}fn 与(&mut self,l:&J,亘:U<乯>)->K<L>{let q=self;let 亶=似{両:&q.両,亪:q.亪,rest:丩{乤:0,乘:么::乥,},亘,丶:Settings1::DEFAULT,};if
let 乺::伊(丶)=q.亃{q.乄+=1;q.亻+=1;return 与(l,&丶);}let 丂=l.丝();let 偷=q.伞.get(q.亻)==Some(&伀(l,丂));let 偛=matches!(q.亃,乺::仧{伮,..}if 伮>0&&l.乧()>=伮);if!偷||偛{let
书=q.乄==0;q.业=match q.亃{乺::仧{乍,了,丨,..}=>{let 丨=&丨;let 乴=倮;let mut 乛=业(l,&q.両,乴,乍,了,亘,丨,);乛.resize(乍,乴);乛}乺::伊(_)=>K::new(),};q.伞=match q.亃{乺::仧{..}=>傔(l,&q.両,&q.业),_=>傀(l,&q.业,&亶),};q.亻=0;q.乄+=1;}let
丕=q.业.get(q.亻).copied().unwrap_or(亶.rest);q.亻+=1;亶.万(l,丕)}}fn 傔(l:&J,両:&[Y],业:&[丩])->K<Z>{let 丂=l.丝();let mut 件=K::with_capacity(业.len());let mut 也=l.clone();for&丕
in 业{if 也.丙().is_some(){break;}件.push(伀(&也,丂));也=佊(&也,&万(&也,両,丕));}件}const 侌:並=20;const 倸:C=3;#[derive(Clone,Copy)]struct Settings1{乍:C,}impl Settings1{const
DEFAULT:Settings1=Settings1{乍:倸};}fn 乛(l:&J,三:G)->[I;M]{let 中=l.map().valid;let mut 乀=[I::MAX;M];let mut 临=三&中;let mut 乞=临;let mut d=0u8;while!临.is_empty(){for
一 in 临.iter(){乀[一]=d;}let 丄=(临.丑()&中).丗(乞);乞|=丄;临=丄;d=d.saturating_add(1);}乀}fn 傭(l:&J,丁:F)->C{let 丫=乛(l,l.丅(丁));let 丟=乛(l,l.丅(1-丁));l.map().valid.iter().map(|一|match
丫[一].cmp(&丟[一]){_ if 丫[一]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const 倰:[Y;27]=両();const
倮:丩=丩{乤:16,乘:么::乥,};const fn 両()->[Y;27]{let 从=[0,2,4];let 仞=[0,3,6];let 亱=[0,1,2];let mut 丆=[Y::亳;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
仑=if g>0||t>0{4}else{0};丆[i]=Y::亳.傳(从[g],仞[t],0).傲(仑,亱[w]);i+=1;}丆}#[derive(Default)]struct 佄{丐:C,丛:C,下:C,亝:C,个:C,伄:C,丵:C,伆:C,}#[derive(Clone,Copy)]struct
亦{丐:C,丛:C,下:C,个:C,亝:C,伄:C,丵:C,伆:C,丯:C,伖:P,乓:乹,}#[derive(Clone,Copy,PartialEq)]enum 乹{侖,伻(伎),予(予),例,}fn 傤(l:&J,丁:F)->[I;M]{let 丫=乁(l,G::一(T[丁]));let 丟=乁(l,G::一(T[1-丁]));let
mut 事=G::O;let mut 乊=G::O;for c in l.map().valid.iter(){if 丫[c]==I::MAX||丟[c]==I::MAX{continue;}if 丫[c]==丟[c]{事.set(c);}if 丫[c]<=丟[c]{乊.set(c);}}let d=乁(l,事);let
mut 丆=[I::MAX;M];for c in 乊.iter(){丆[c]=d[c];}丆}fn 假(l:&J,丁:F,事:&[I;M])->C{let d=乁(l,l.丅(丁));(0..M).filter(|&c|事[c]!=I::MAX&&d[c]!=I::MAX).map(|c|match
d[c].cmp(&事[c]){std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}#[derive(Clone,Copy,PartialEq)]enum 予{倀{乜:C},侣{乜:C,仂:C},倪,}impl
予{fn 侄(self,a:C,b:C,d:C)->C{let 亼=(a-b).abs()/2;match self{予::倀{乜}=>(乜-亼-(a+b-d)/2).max(0),予::侣{乜,仂}if a<=仂=>(乜-亼).max(0),予::侣{..}=>0,予::倪=>a.min(d/2),}}}fn
偟(l:&J,丁:F,佟:予)->[C;M]{let 丫=乁(l,G::一(T[丁]));let 丟=乁(l,G::一(T[1-丁]));let d=C::from(丫[T[1-丁]]);let mut 丆=[0;M];for c in 0..M{if 丫[c]!=I::MAX&&丟[c]!=I::MAX{丆[c]=佟.侄(C::from(丫[c]),C::from(丟[c]),d);}}丆}#[derive(Clone,Copy,PartialEq)]enum
伎{侗(C),}impl 伎{fn 侄(self,亼:C)->C{match self{伎::侗(乜)=>(乜-亼).max(0),}}}type 伏=[C;4];fn 傉(l:&J,丁:F)->[C;M]{struct 令{丅:G,乡:[伉;M],乌:[伉;M],size:[C;M],个:[C;M],亯:伉,}fn
仁(d:&mut 令,v:F){d.亯+=1;d.乡[v]=d.亯;d.乌[v]=d.亯;d.size[v]=1;for w in(G::一(v).丑()&d.丅).iter(){if d.乡[w]==0{仁(d,w);d.size[v]+=d.size[w];d.乌[v]=d.乌[v].min(d.乌[w]);if
d.乌[w]>=d.乡[v]{d.个[v]+=d.size[w];}}else{d.乌[v]=d.乌[v].min(d.乡[w]);}}}let mut d=令{丅:l.丅(丁),乡:[0;M],乌:[0;M],size:[0;M],个:[0;M],亯:0,};let 于=T[丁];if d.丅.test(于){仁(&mut
d,于);}d.个[于]=0;d.个}fn 傋(l:&J,丁:F)->佄{let 丅=l.丅(丁);let 个=傉(l,丁);let 仟=乁(l,G::一(T[1-丁]));let mut 丆=佄{丐:C::from(丅.count()),丛:l.丛(丁),下:l.下(丁),丵:C::MAX,伆:傭(l,丁)/2,..佄::default()};for
一 in 丅.iter(){if 个[一]>0{丆.亝+=1;丆.个=丆.个.max(个[一]);}if 一!=T[丁]&&(G::一(一).丑()&丅).count()<=1{丆.伄+=1;}丆.丵=丆.丵.min(C::from(仟[一]));}丆}fn 催(l:&J,丁:F,丨:&亦,仸:G)->C{let
偨=C::from((l.专(丁)&仸).count());let s=傋(l,丁);let o=丨;o.丐*s.丐+o.丛*s.丛+o.下*s.下-o.个*s.个-o.亝*s.亝-o.伄*s.伄-o.丵*s.丵.min(30)+o.伆*s.伆+o.丯*偨}fn 亹(l:&J)->G{亴::of(l).亹()}fn
偱(l:&J)->G{let 丫=乁(l,G::一(T[0]));let 丟=乁(l,G::一(T[1]));let mut 乑=G::O;for c in l.map().valid.iter(){if 丫[c]!=I::MAX&&丟[c]!=I::MAX&&丫[c].abs_diff(丟[c])<=2{乑.set(c);}}乑}fn
仛(l:&J)->[C;M]{let a=乁(l,G::一(T[0]));let b=乁(l,G::一(T[1]));let mut 丆=[C::MAX;M];for c in 0..M{if a[c]!=I::MAX&&b[c]!=I::MAX{丆[c]=C::from(a[c].abs_diff(b[c]))/2;}}丆}#[derive(Clone,Copy)]struct
仆{丯:G,乑:G,乤:[C;M],仛:[C;M],临:G,front:[I;M],}impl 仆{fn of(l:&J,丨:&亦)->仆{let 俎=matches!(丨.乓,乹::侖);let mut 乤=[0;M];if let 乹::伻(偓)=丨.乓{for(w,亼)in 乤.iter_mut().zip(仛(l)){*w=if
亼==C::MAX{0}else{偓.侄(亼)};}}if let 乹::予(佟)=丨.乓{乤=偟(l,l.丝(),佟);}let(傆,临)=match 丨.乓{_=>([C::MAX;M],G::O),};仆{乤,仛:傆,临,front:if 丨.乓==乹::例{傤(l,l.丝())}else{[I::MAX;M]},丯:if
丨.丯!=0||俎{亴::of(l).亹()}else{G::O},乑:if 俎{偱(l)}else{G::O},}}}fn 仒(l:&J,丁:F,丨:&亦,丮:仆)->伏{let 佽=催(l,丁,丨,丮.丯);match 丨.乓{乹::伻(_)|乹::予(_)=>{let 丅=l.丅(丁);let
傯:C=丅.iter().map(|c|丮.乤[c]).sum();[C::from(丅.count()),傯,佽,0]}乹::例=>[C::from(l.丅(丁).count()),假(l,丁,&丮.front),佽,0,],乹::侖=>{let 专=l.专(丁);let 丯=C::from((专&丮.丯).count());let
乑=C::from((专&丮.乑).count());match 丨.乓{_=>[C::from(l.丅(丁).count()),丯,乑,佽],}}}}fn 佊(l:&J,万:&[L])->J{let mut 丄=l.clone();丄.伭(万);if 丄.丙().is_none(){丄.伭(&[]);}丄}const
侇:I=I::MAX;fn 万(l:&J,両:&[Y],丕:丩)->K<L>{if 丕.乤==侇{let 丶=Settings{义:両.first().copied().unwrap_or(Y::亳),乘:if 丕.乘==么::乥{产::乥}else{产::侈},..Settings::DEFAULT};return
与(l,&丶);}伴(l,&両[F::from(丕.乤)%両.len()],丕.乘,)}fn 保(l:&J,両:&[Y],业:&[丩],乴:丩,乍:F,)->J{let mut 也=l.clone();for 与 in 0..乍{if 也.丙().is_some(){break;}let 丕=业.get(与).copied().unwrap_or(乴);也=佊(&也,&万(&也,両,丕));}也}fn
備(l:&J,両:&[Y],伖:P)->K<(丩,K<L>)>{let mut 丆:K<(丩,K<L>)> =K::new();if 伖{for 乘 in[么::乥,么::互{丧:0,丈:I::MAX,},]{let 丕=丩{乤:侇,乘,};let c=万(l,両,丕);if!丆.iter().any(|(_,丳)|*丳==c){丆.push((丕,c));}}}for
乤 in 0..両.len().min(F::from(I::MAX)){for&乘 in&俽{let 丕=丩{乤:乤 as I,乘,};let c=万(l,両,丕);if!丆.iter().any(|(_,丳)|*丳==c){丆.push((丕,c));}}}丆}fn 偆(l:&J,両:&[Y],乴:丩,乍:F,了:F,亘:U<乯>,丨:&亦,)->K<丩>{let
丂=l.丝();let 任=||亘.is_some_and(|t|乯::now()>=t);let 丮=仆::of(l,丨);let 份=|业:&[丩]|仒(&保(l,両,业,乴,乍),丂,丨,丮);let mut 丏:(伏,K<丩>)=(份(&[]),K::new());let mut 临:K<(J,K<丩>)>
=vec![(l.clone(),K::new())];for 偕 in 0..乍{let mut 亗:K<(伏,J,K<丩>)> =K::new();let mut 乞:HashMap<Z,F> =HashMap::new();for(俕,业)in&临{if 任(){break;}for(丕,c)in
備(俕,両,丨.伖){let 丄=佊(俕,&c);let mut 体=业.clone();体.push(丕);let v=仒(&丄,丂,丨,丮);let 仒=伀(&丄,丂);match 乞.get(&仒){Some(&乗)if 亗[乗].0>=v=>{}Some(&乗)=>亗[乗]=(v,丄,体),None=>{乞.insert(仒,亗.len());亗.push((v,丄,体));}}}}if
亗.is_empty(){break;}亗.sort_by(|a,b|b.0 .cmp(&a.0));亗.truncate(了);let 亞=偕+1==乍||任();for(i,(_,_,业))in 亗.iter().take(if 亞{了}else{1}).enumerate(){if i>0&&任(){break;}let
v=份(业);if v>丏.0{丏=(v,业.clone());}}if 亞{break;}临=亗.into_iter().map(|(_,s,p)|(s,p)).collect();}丏.1}fn 业(l:&J,両:&[Y],乴:丩,乍:F,偰:F,亘:U<乯>,丨:&亦,)->K<丩>{let 丂=l.丝();let
丮=仆::of(l,丨);let 份=|业:&[丩]|仒(&保(l,両,业,乴,乍),丂,丨,丮);let mut 丏:(伏,K<丩>)=(份(&[]),K::new());let mut 了=2;loop{let 主=偆(l,両,乴,乍,了,亘,丨);let v=份(&主);if v>丏.0{丏=(v,主);}let
任=亘.is_some_and(|t|乯::now()>=t);if 任||(亘.is_none()&&了>=偰){break;}了*=2;}丏.1}const 倍:F=4+12;fn 乍(佥:impl BufRead,mut 伬:impl Write,mut 佋:impl FnMut(&str,&str,並,乯)->之,)->Result<(),之>{let
mut 丱=侘{信:佥};let Some(书)=丱.line()?else{return Ok(());};let mut 仐=format!("{}\n",书);丱.take(且(&书,"mine spot count")?,&mut 仐)?;let mut 与=0;loop{let Some(书)=丱.line()?else{return
Ok(());};let 亿=乯::now();与+=1;let mut 乵=format!("{}\n",书);丱.take(倍-1,&mut 乵)?;for 乎 in["building count","unit count"]{let line=丱.俣()?;let n=且(&line,乎)?;乵.push_str(&line);乵.push('\n');丱.take(n,&mut
乵)?;}let 佋=佋(&仐,&乵,与,亿);writeln!(伬,"{}",佋).and_then(|()|伬.flush()).map_err(|亷|format!("turn {}: cannot write the answer: {}",与,亷))?;}}struct 侘<R>{信:R,}impl<R:BufRead>侘<R>{fn
line(&mut self)->Result<U<之>,之>{let mut line=之::new();match self.信.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(亷)=>Err(format!("cannot read input: {}",亷)),}}fn
俣(&mut self)->Result<之,之>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乵:&mut String)->Result<(),之>{for
_ in 0..n{乵.push_str(&self.俣()?);乵.push('\n');}Ok(())}}fn 且(line:&str,乎:&str)->Result<F,之>{match line.trim().parse::<F>(){Ok(n)if n<=候=>Ok(n),_=>Err(format!("bad {} {line:?}",乎)),}}const
候:F=144;const 倴:C=3;const 倔:C=2;const 倞:F=16;fn 偼(l:&J,丆:&mut Vec<L>){let 丁=l.丝();let mut 亃=価{s:l.clone(),丁,乼:1-丁,丌:T[1-丁],伡:T[丁],与:l.乧(),丆,};亃.run();}struct
価<'a>{s:J,丁:F,乼:F,丌:F,伡:F,与:並,丆:&'a mut Vec<L>,}fn 伷(一:F)->(I,I){((一%Q)as I,(一/Q)as I)}fn 佭(世:並,一:F)->L{let(x,y)=伷(一);L::乷{世,x,y}}fn 俱(七:I,一:F)->L{let(x,y)=伷(一);L::乖{七,x,y}}impl
価<'_>{fn run(&mut self){let q=self;if q.伧(){return;}q.亟();if q.伧(){return;}q.乬();q.伳();}fn 乀(&self,三:F,丷:F)->C{C::from(self.s.map().两(三,丷))}fn 丸(&mut self,不:L)->P{let
乙=self.s.丸(不);if 乙{self.丆.push(不);}乙}fn 仡(&self,丬:&J)->P{丬.丙()==Some(习::乻(self.丁))}fn 伧(&mut self)->P{let q=self;if q.s.丙().is_some(){return false;}for
一 in q.s.乿().iter(){if q.乀(一,q.丌)!=1{continue;}let Some((世,_))=q.s.什(一)else{continue;};let 不=佭(世,q.丌);let mut 丬=q.s.clone();if 丬.丸(不)&&q.仡(&丬){return q.丸(不);}}let
Some(为)=q.偏()else{return false;};let mut 丬=q.s.clone();if!为.iter().all(|&不|丬.丸(不))||!q.仡(&丬){return false;}for 不 in 为{q.丸(不);}true}fn 偏(&self)->U<K<L>>{let
q=self;const 侏:C=C::MAX;let 乲=q.s.丰().乾();let 丢:K<G> =(1..=乲).map(|七|q.s.乂(q.丁,七)).collect();let 仰=|一:F|{丢.iter().position(|举|举.test(一)).map(|丘|(丘 as I+1,10*(丘
as C+1)))};let 丅=q.s.丅(q.丁);let mut 上=[侏;M];let mut 丹=[F::MAX;M];let mut 乙=[false;M];for 一 in 丅.iter(){上[一]=0;}loop{let mut 丄:U<F> =None;for 一 in 0..M{if!乙[一]&&上[一]<侏&&丄.is_none_or(|n|上[一]<上[n]){丄=Some(一);}}let
Some(一)=丄 else{break;};乙[一]=true;if 一==q.丌{break;}for&乐 in&q.s.map().丑[0][一]{if 乐==乸{break;}let 乐=F::from(乐);if 乙[乐]||丅.test(乐){continue;}let Some((_,亭))=仰(乐)else{continue;};if
上[一]+亭<上[乐]{上[乐]=上[一]+亭;丹[乐]=一;}}}if 上[q.丌]>q.s.下(q.丁){return None;}let mut 为=K::new();let mut 一=q.丌;while!丅.test(一){let(七,_)=仰(一)?;为.push(俱(七,一));一=丹[一];}为.reverse();Some(为)}fn
丼(&self,乏:&J)->C{let(丂,亮)=(self.丁,self.乼);let 丹=&self.s;let 且=|举:G|C::from(举.count());let 东=且(乏.专(丂))-且(丹.专(丂));let 伲=且(丹.专(亮))-且(乏.专(亮));let 偭=且(丹.丅(亮))-且(乏.丅(亮));let
伣=丹.他(亮)-乏.他(亮);let 丧=且(丹.丧()&丹.专(亮))-且(乏.丧()&乏.专(亮));12*东+20*伲+8*(偭-伲).max(0)+5*伣+25*丧}fn 俨(&self,一:F)->C{let q=self;let 乔=q.s.map().久.test(一);if 乔&&q.s.丰().伓()&&!q.s.专(q.丁).test(一){20}else{0}}fn
侾(&self,乒:C)->P{let q=self;q.s.丈(q.乼).iter().any(|一|q.乀(一,q.伡)<=乒)}fn 亟(&mut self){let q=self;let mut 丈:K<(C,F)> =q.s.乿().iter().map(|一|(q.乀(一,q.丌),一)).collect();丈.sort_unstable();for(传,一)in
丈{if q.s.丙().is_some(){return;}let Some((世,七))=q.s.什(一)else{continue;};let 丢=q.s.乂(q.丁,七);let 乐=q.s.map().丑[q.丁][一];let mut 丏:U<(C,F)> =None;for&丄 in 乐.iter().take_while(|&&丄|丄!=乸){let
丄=F::from(丄);if!丢.test(丄){continue;}let mut 丬=q.s.clone();if!丬.丸(佭(世,丄)){continue;}let 严=if q.仡(&丬){100000}else{q.丼(&丬)+10*(传-q.乀(丄,q.丌))+q.俨(丄)};if 丏.is_none_or(|(乜,_)|严>乜){丏=Some((严,丄));}}if
let Some((严,丄))=丏{if 严>0{q.丸(佭(世,丄));}}}}fn 乬(&mut self){let q=self;if q.s.丙().is_some(){return;}if q.侾(倴){q.傦();}else{q.丧();}}fn 傦(&mut self){let q=self;if!q.s.丰().伔()||q.s.下(q.丁)<亐{return;}let
乐=q.s.map().丑[q.丁][q.伡];let 丐:K<F> =乐.iter().take_while(|&&一|一!=乸).map(|&一|F::from(一)).collect();let 乃=q.s.专(q.丁);if 丐.iter().any(|&一|乃.test(一)&&q.s.乆().test(一)){return;}for
一 in 丐{let(x,y)=伷(一);if q.丸(L::亇{x,y}){return;}}}fn 丧(&mut self){let q=self;if!q.s.丰().伓(){return;}while q.s.下(q.丁)>=亐{let 丠=q.s.map().久&q.s.专(q.丁)&q.s.丅(q.丁)&q.s.介();let
Some(一)=丠.iter().min_by_key(|&一|(q.乀(一,q.伡),一))else{return;};let(x,y)=伷(一);if!q.丸(L::云{x,y}){return;}}}fn 伳(&mut self){let q=self;let 俤=if q.与<=3{0}else{5};for
_ in 0..倞{if q.s.丙().is_some(){return;}let 下=q.s.下(q.丁);if 下<10{return;}let 偫=q.侾(倔);let 亠=q.s.京(q.丁);let mut 丏:U<(C,L)> =None;let mut 佻:U<(C,L)> =None;let
mut 侯=G::O;for 七 in 1..=q.s.丰().乾(){let 丢=q.s.乂(q.丁,七);let 亁=(亠&丢).丗(侯);侯=丢;let 亭=10*C::from(七);if 亭>下{continue;}for 一 in 亁.iter(){let 不=俱(七,一);let mut
丬=q.s.clone();if!丬.丸(不){continue;}let 佧=丬.下(q.丁);let 丛=丬.丛(q.丁);let 伣=q.s.他(q.乼)-丬.他(q.乼);let 偁=if 伣>0&&偫{佧+丛>=0}else{丛>=0&&(佧>=俤||伣>0)};if!偁{continue;}let
丼=q.丼(&丬);let 严=丼+q.俨(一)-6*q.乀(一,q.丌)-10*(C::from(七)-1);if 丼>0{if 丏.is_none_or(|(乜,_)|严>乜){丏=Some((严,不));}}else if 丛>=3&&佧>=俤+10&&七==1{if 佻.is_none_or(|(乜,_)|严>乜){佻=Some((严,不));}}}}let
Some((_,不))=丏.or(佻)else{return;};if!q.丸(不){return;}}}}fn 偤(l:&J,丆:&mut Vec<L>){丆.clear();偼(l,丆);}const 井:C=C::MAX;const 倝:F=8;struct 仅{上:[C;M],丹:[I;M],}fn
会(l:&J,丁:F,亟:P,丌:F)->C{佘(l,丁,人(l,丁,亟),丌)}fn 佘(l:&J,丁:F,人:G,丌:F)->C{let mut 上=井;俁(l,丁,人,|丕,new|{if new.test(丌){上=10*丕 as C;}上!=井});上}fn 傂(l:&J,丁:F,人:G,丿:C)->G{let
mut 亍=G::O;俁(l,丁,人,|丕,new|{if 10*丕 as C>丿{return true;}亍|=new;false});亍.丗(l.丅(丁))}fn 人(l:&J,丁:F,亟:P)->G{if 亟&&丁==l.丝(){l.乿()}else{G::O}}fn 俁(l:&J,丁:F,人:G,mut
仁:impl FnMut(F,G)->P,){let 乲=l.丰().乾();let mut 仰=[G::O;3];let mut 乞=G::O;for 七 in 1..=乲{let 举=l.乂(丁,七);仰[F::from(七)-1]=举.丗(乞);乞|=举;}let mut 串=[l.丅(丁),G::O,G::O,G::O];for
一 in(人&l.丈(丁)).iter(){if let Some(七)=l.亅(一).filter(|&七|七<=乲){串[0]|=G::一(一).丑()&l.乂(丁,七);}}let mut 乙=G::O;for 丕 in 0..{let new=串[丕%4].丗(乙);串[丕%4]=G::O;if
仁(丕,new){return;}乙|=new;let 丄=new.丑().丗(乙);for(仇,丐)in 仰.iter().enumerate(){串[(丕+仇+1)%4]|=丄&*丐;}if 串.iter().all(|举|举.丗(乙).is_empty()){break;}}}impl 仅{fn
new(l:&J,丁:F,亟:P)->仅{let 乲=F::from(l.丰().乾());let mut 丢=[G::O;3];for(丘,举)in 丢.iter_mut().enumerate().take(乲){*举=l.乂(丁,丘 as I+1);}let 丅=l.丅(丁);let 丑=&l.map().丑[0];let
mut 上=[井;M];let mut 丹=[乸;M];let mut 串=[0u8;M];let mut 且=0;let mut 俦=G::O;for 一 in 丅.iter(){上[一]=0;串[且]=一 as I;且+=1;}if 亟&&丁==l.丝(){for 三 in l.乿().iter(){let
Some(七)=l.亅(三)else{continue;};let 乛=G::一(三).丑()&丢[F::from(七)-1];for 一 in 乛.iter(){if 上[一]!=0{上[一]=0;串[且]=一 as I;且+=1;}}}}while 且>0{let mut 丏=0;for 丘 in
1..且{let(a,b)=(F::from(串[丘]),F::from(串[丏]));if(上[a],a)<(上[b],b){丏=丘;}}let 一=F::from(串[丏]);且-=1;串[丏]=串[且];俦.set(一);for&丄 in&丑[一]{if 丄==乸{break;}let 丄=F::from(丄);if
俦.test(丄)||丅.test(丄){continue;}let Some(七)=丢[..乲].iter().position(|举|举.test(丄))else{continue;};let 俰=上[一]+10*(七 as C+1);if 俰<上[丄]{if 上[丄]==井{串[且]=丄 as
I;且+=1;}上[丄]=俰;丹[丄]=一 as I;}}}仅{上,丹}}fn path(&self,丌:F)->K<F>{let mut path=K::new();if 丌>=M||self.上[丌]==井{return path;}let mut 一=丌;while self.丹[一]!=乸{path.push(一);一=F::from(self.丹[一]);}path.reverse();path}}struct
伌{万:K<L>,上:C,}fn 偯(l:&J,丁:F,一:F)->U<I>{(1..=l.丰().乾()).find(|&七|l.乂(丁,七).test(一))}fn 傅(丒:&mut J,丂:F,乮:F,丿:C,万:&mut Vec<L>,亄:&mut i64,)->P{loop{if 丒.丙()==Some(习::乻(丂)){return
true;}if 丒.专(丂).test(乮)&&丒.丅(丂).test(乮){return true;}let 丵=仅::new(丒,丂,false);let 上=丵.上[乮];if 上==井||*亄+上>丿{return false;}let Some(&一)=丵.path(乮).first()else{return
false;};let Some(七)=偯(丒,丂,一)else{return false;};let 不=L::乖{七,x:(一%Q)as I,y:(一/Q)as I,};if!丒.丸(不){return false;}*亄+=10*C::from(七);万.push(不);}}fn 侭(l:&J,丂:F,仳:&[F],丿:C)->U<伌>{let
mut 丒=l.clone();let mut 万=K::new();let mut 亄=0;for&乮 in 仳{if!傅(&mut 丒,丂,乮,丿,&mut 万,&mut 亄){return None;}}(亄<=丿).then_some(伌{万,上:亄,})}fn 佒(l:&J,丌:F,丿:C)->U<伌>{if
丿<0||l.丙().is_some()||丌>=M||l.map().乕.test(丌){return None;}let 丂=l.丝();if l.专(丂).test(丌)&&l.丅(丂).test(丌){return Some(伌{万:K::new(),上:0,});}let mut 丏=侭(l,丂,&[丌],丿);let
书=仅::new(l,丂,false);let mut 乆:K<F> =(l.乆()&l.专(1-丂)).iter().filter(|&乣|乣!=丌&&书.上[乣]!=井).collect();乆.sort_by_key(|&乣|(l.map().两(乣,丌),乣));乆.truncate(倝);for
乣 in 乆{if let Some(为)=侭(l,丂,&[乣,丌],丿){if 丏.as_ref().is_none_or(|乜|为.上<乜.上){丏=Some(为);}}}丏}const 倛:F=20000;struct 伍{万:K<L>,上:C,丐:C,丈:C,乬:C,丛:C,丣:C,}struct
佀{丐:C,丈:C,乬:C,}fn 俉(l:&J,丁:F)->佀{let 丅=l.丅(丁);let 丈=l.丈(丁)&丅;let 严=丈.iter().filter_map(|一|l.亅(一)).map(|七|10*C::from(七)).sum();佀{丐:C::from(丅.count()),丈:严,乬:C::from(((l.丧()|l.乆())&丅).count()),}}struct
侥{丂:F,代:G,丹:佀,丛:C,仕:F,丿:C,仙:F,path:K<L>,主:K<伍>,index:HashMap<仦,F,BuildHasherDefault<DefaultHasher>>,}type 仦=[Z;3];fn 傱(mut 乽:仦,一:F)->仦{乽[一/64]|=1<<(一%64);乽}impl
侥{fn 侼(&mut self,丒:&J,亞:U<F>,伲:仦,上:C,)->P{let q=self;let 丂=q.丂;let 亁=match 亞{None=>丒.京(丂),Some(一)=>G::一(一).丑(),}&q.代;let 亁=亁.丗(丒.专(丂));let mut 丢=[G::O;3];for
七 in 1..=丒.丰().乾(){丢[F::from(七)-1]=丒.乂(丂,七);}for 一 in 亁.iter(){let 乲=F::from(丒.丰().乾());let Some(丘)=丢[..乲].iter().position(|举|举.test(一))else{continue;};let
七=丘 as I+1;let 亭=10*C::from(七);if 上+亭>q.丿{continue;}if q.仙>=倛{return false;}q.仙+=1;let mut 丄=丒.clone();let 不=L::乖{七,x:(一%Q)as I,y:(一/Q)as I,};if!丄.丸(不){continue;}q.path.push(不);let
仝=傱(伲,一);q.傃(&丄,仝,上+亭);let 偔=q.path.len()<q.仕&&丄.丙().is_none();let 偢=!偔||q.侼(&丄,Some(一),仝,上+亭);q.path.pop();if!偢{return false;}}true}fn 傃(&mut self,乏:&J,仝:仦,上:C){let
q=self;let 仺=1-q.丂;let 佱=俉(乏,仺);let 丐=q.丹.丐-佱.丐;let 丈=q.丹.丈-佱.丈;if 丐<=0&&丈<=0{return;}let 乬=q.丹.乬-佱.乬;let 丛=乏.丛(q.丂)-q.丛;let 傴=乏.丙()==Some(习::乻(q.丂));let
丣=if 傴{C::MAX/2}else{3*丐+丈+10*乬+2*丛-上};if let Some(&乗)=q.index.get(&仝){if 上>=q.主[乗].上{return;}}let 个=伍{万:q.path.clone(),上,丐,丈,乬,丛,丣,};match q.index.get(&仝){Some(&乗)=>q.主[乗]=个,None=>{q.index.insert(仝,q.主.len());q.主.push(个);}}}}fn
侺(l:&J,仕:F,丿:C,乒:F)->K<伍>{乪(l,仕,丿,乒).0}fn 乪(l:&J,仕:F,丿:C,乒:F)->(K<伍>,F){if l.丙().is_some()||仕==0||乒==0{return(K::new(),0);}let 丂=l.丝();let 仺=1-丂;let mut
佷=侥{丂,代:l.专(仺)|l.专(仺).丑(),丹:俉(l,仺),丛:l.丛(丂),仕,丿,仙:0,path:K::new(),主:K::new(),index:HashMap::default(),};佷.侼(l,None,[0;3],0);let 仙=佷.仙;let mut 主=佷.主;主.sort_by(|a,b|b.丣.cmp(&a.丣).then(a.上.cmp(&b.上)));主.truncate(乒);(主,仙)}const
休:C=1000000;const 倜:C=200;fn 乊(l:&J,丁:F)->C{let 丅=l.丅(丁);let 丈:C=(l.丈(丁)&丅).iter().filter_map(|一|l.亅(一)).map(|七|10*C::from(七)).sum();l.下(丁)+3*l.丛(丁)+4*C::from(丅.count())+丈+10*C::from((l.乆()&丅).count())}fn
俏(l:&J,丁:F,乳:G)->C{偠(l,丁,佘(l,丁,乳,T[1-丁]),)}fn 偠(l:&J,丁:F,上:C)->C{if 上==井{return C::MIN;}l.下(丁)+l.丛(丁)-上}fn 俞(伩:C)->C{伩.clamp(-倜,0)}fn 做(l:&J,丁:F,乳:[G;2],)->(C,[C;2]){let
丳=1-丁;match l.丙(){Some(习::乻(傰))=>{return(if 傰==丁{休}else{-休},[0,0])}Some(习::侎)=>return(0,[0,0]),None=>{}}let mut 丣=乊(l,丁)-乊(l,丳);if 丁==l.丝(){let mut 丄=l.clone();丄.仌();if
丄.丙().is_none()&&会(&丄,丳,true,T[丁])<=丄.下(丳){丣-=休/2;}}let 仔=[俏(l,丁,乳[0]),俏(l,丳,乳[1]),];(丣+俞(仔[0])-俞(仔[1]),仔)}
