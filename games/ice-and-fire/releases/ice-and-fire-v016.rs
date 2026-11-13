// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 丰=u32;type U=u64;type 仔=i32;type C=i64;type 亙=f64;type Q=bool;type 丯=String;type 乑=Instant;type K<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 佾:Q=true;fn 侹(l:&J,q:F,仏:C,丼:G,丧:C)->Q{let
丁=l.东();let 乏=1-丁;let 佔=佾&&丧<=l.丈(乏)+2*l.丑(乏).max(0);let 仵=G::q(q)|G::q(q).世();if!佔&&(仵&丼).is_empty(){return false;}let(x,y)=亸(q);let mut 丷=l.clone();if!丷.丫(L::亏{x,y}){return
false;}if 俧(&丷,丁)<仏{return true;}佔&&仟(&丷,乏,false,Z[丁])>丧}const 侂:F=4;const 伨:F=5;#[derive(Clone)]enum 乂{亼(L),任(K<L>),}impl 乂{fn 七(&self)->&[L]{match self{乂::亼(下)=>std::slice::from_ref(下),乂::任(七)=>七,}}}fn
亸(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 低(l:&J,万:&mut Vec<乂>){万.clear();if l.且().is_some(){return;}let 丁=l.东();let 乕=l.丩().乩();let mut 丢=[G::T;4];for 一
in 1..=乕{丢[F::from(一)]=l.为(丁,一);}for 不 in l.亇().iter(){let Some((丏,一))=l.仑(不)else{continue;};let 乡=G::q(不).世()&丢[F::from(一)];for 乫 in 乡.iter(){let(x,y)=亸(乫);万.push(乂::亼(L::亀{丏,x,y}));}}let
丈=l.丈(丁);let 仪=l.五(丁);for 一 in 1..=乕{let 丆=10*C::from(一);if 丈<丆{break;}let mut 丌=仪&丢[F::from(一)];for 侾 in 1..一{丌=丌.两(丢[F::from(侾)]);}for q in 丌.iter(){let(x,y)=亸(q);万.push(乂::亼(L::书{一,x,y}));}}if
丈>=于{let 互=l.丟(丁)&仪&l.什();if l.丩().仛(){for q in(互&l.map().丳).iter(){let(x,y)=亸(q);万.push(乂::亼(L::乽{x,y}));}}if l.丩().伂(){let 侧=l.map().valid.两(l.丟(丁)).世();let
丌=(互&侧).两(l.map().丳);let 严=l.丈(1-丁)+l.丑(1-丁);let 丼=佤(l,1-丁,false,严);let 仏=C::from((丼&l.丄(丁)).count());let 丧=仟(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|侹(l,q,仏,丼,丧)){let(x,y)=亸(q);万.push(乂::亼(L::亏{x,y}));}}}for
丝 in 但(l,伨,丈,侂){if 丝.七.len()>1{万.push(乂::任(丝.七));}}if let Some(丧)=伈(l,Z[1-丁],丈){if 丧.七.len()>1{万.push(乂::任(丧.七));}}}fn 仃(l:&J,z:F)->G{let 乏=1-z;let 严=l.丈(乏)+l.丑(乏);佤(l,乏,false,严)&l.丄(z)}fn
俧(l:&J,z:F)->C{C::from(仃(l,z).count())}const 例:C=4;fn 俁(丌:C)->C{丌/例}fn 丞(l:&J,z:F,仃:G)->C{let 丄=l.丄(z);let 丞=(l.丞()&丄).两(仃);俁(C::from(丄.count()))*C::from(丞.count())}const
侖:C=2;fn 佶(l:&J,z:F)->C{let 丄=l.丄(z);let[a,b,c]=丄.仲();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(伵-1);侟.with(|伆|{let mut 伆=伆.borrow_mut();let 仰=&mut 伆[与];if 仰.0==丄&&仰.1==z as I+1{return 仰.2;}let 业=俲(丄,Z[z]);*仰=(丄,z as I+1,业);业})}const
伵:F=1<<12;thread_local!{static 侟:std::cell::RefCell<K<(G,I,C)>> =std::cell::RefCell::new(vec![(G::T,0,0);伵]);}fn 俲(丄:G,乪:F)->C{struct 亹{丄:G,乄:[I;M],串:[I;M],size:[I;M],二:I,乴:C,仩:C,}fn
亗(d:&mut 亹,v:F)->I{d.二+=1;let 乄=d.二;d.乄[v]=乄;let mut 串=乄;let mut 伣=1u8;let mut 丝=0u8;for w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{let 佮=亗(d,w);伣+=佮;串=串.min(d.串[w]);if
d.串[w]>=乄{丝+=佮;}}else{串=串.min(d.乄[w]);}}d.串[v]=串;d.size[v]=伣;if 丝>0&&d.乄[v]>1{d.乴+=1;d.仩=d.仩.max(C::from(丝));}伣}if!丄.test(乪){return 0;}let mut d=亹{丄,乄:[0;M],串:[0;M],size:[0;M],二:0,乴:0,仩:0,};亗(&mut
d,乪);d.乴+d.仩}fn 俕(l:&J,z:F)->C{侖*(佶(l,1-z)-佶(l,z))}fn 佇(l:&J,z:F)->C{if l.丈(z)+l.丑(z)>=0{return 0;}(l.丅(z)&l.丄(z)).iter().filter_map(|q|l.亵(q)).map(|一|10*C::from(一)).sum()}const
侁:C=15;fn 佳(l:&J,z:F)->C{(l.丅(z)&l.丄(z)).iter().filter(|&q|l.亵(q)==Some(2)).count()as C}fn 伏(l:&J,z:F)->C{let 丛=侳(l,z);if l.且().is_some(){return 丛;}let
仉=仃(l,z);let 乻=仃(l,1-z);let 仏=C::from(乻.count())-C::from(仉.count());丛-2*佇(l,z)+2*佇(l,1-z)+俕(l,z)+侁*(佳(l,1-z)-佳(l,z))+7*仏/4+丞(l,z,仉)-丞(l,1-z,乻)}const 侊:F=8;const
侑:F=3;const 侃:F=32;const 侒:U=300;const 佺:亙=0.75;struct 乿{乺:P<乑>,亓:P<乑>,仆:U,}#[derive(Default)]struct 伳{七:K<L>,伌:F,乲:U,丗:F,丮:F,佀:F,乳:F,}const 侞:Q=true;#[derive(Clone)]struct
乚{七:K<L>,伎:C,}struct 些{七:K<L>,end:J,first:P<U>,丒:P<K<乚>>,}type 人=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct 份{丁:F,limit:乿,乲:U,仅:人,井:P<K<乂>>,}fn
伄(l:&mut J,七:&[L])->Q{七.iter().all(|&下|l.丫(下))}fn 乌(七:&[L])->U{let mut hash:U=0xcbf29ce484222325;for 下 in 七{let(以,a,b,c)=match*下{L::亀{丏,x,y}=>(1,U::from(丏),x,y),L::书{一,x,y}=>(2,U::from(一),x,y),L::乽{x,y}=>(3,0,x,y),L::亏{x,y}=>(4,0,x,y),};let
亷=以|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亷).wrapping_mul(0x0100000001b3);}hash}fn 伹(乔:&J,争:&[L])->J{let mut 丂=乔.clone();丂.仁();for&下 in 争{丂.丫(下);}丂.仁();丂}fn
仧(l:&J)->P<K<L>>{let 丁=l.东();let 丐=Z[1-丁];let 亶=|丠:&J|丠.且()==Some(么::乧(丁));for q in l.亇().iter(){if l.map().久(q,丐)!=1{continue;}let Some((丏,_))=l.仑(q)else{continue;};let(x,y)=亸(丐);let
下=L::亀{丏,x,y};let mut 丠=l.clone();if 丠.丫(下)&&亶(&丠){return Some(vec![下]);}}let 丧=伈(l,丐,l.丈(丁))?;let mut 丠=l.clone();(伄(&mut 丠,&丧.七)&&亶(&丠)).then_some(丧.七)}impl
份{fn 习(&self)->Q{self.乲>=self.limit.仆||self.limit.乺.is_some_and(|t|乑::now()>=t)}fn 丛(&mut self,乔:&J,丗:&[乚],伅:C)->C{self.乲+=1;self.伞(乔,丗,伅)}fn 伞(&self,乔:&J,丗:&[乚],伅:C)->C{if
乔.且()==Some(么::乧(self.丁)){return 仚;}if 丗.is_empty(){return 伏(乔,self.丁);}let mut 仳=C::MAX;for 争 in 丗{仳=仳.min(伏(&伹(乔,&争.七),self.丁));if 仳<=伅{break;}}仳}#[allow(clippy::too_many_arguments)]fn
俈(&mut self,买:&J,亟:C,乓:&[乂],亭:&[(C,F)],丹:&mut[Q],丗:&[乚],亠:&mut 人,伕:Q,)->(P<(F,J,C)>,Q){let mut 三=None;let mut 仜=亟;for&(仫,与)in 亭{if 丹[与]{continue;}if 伕&&仫!=C::MAX&&仫<=仜-亟{return(三,true);}if
self.习(){return(三,false);}丹[与]=true;let 七=乓[与].七();let mut 丂=买.clone();if!伄(&mut 丂,七){亠.insert(乌(七),C::MIN/4);continue;}let 丛=self.丛(&丂,丗,仜);亠.insert(乌(七),丛-亟);if
丛>仜{仜=丛;三=Some((与,丂,丛));}}(三,false)}fn 伋(&mut self,乪:&J,丗:&[乚],亽:&[U],侩:Q)->些{let j=self;let mut 乊=None;let mut 买=乪.clone();let mut 七=K::new();let mut
亟=j.丛(&买,丗,C::MIN);let mut 亠=人::default();let mut 乓=K::new();let mut 亭=K::new();while 买.且().is_none()&&!j.习(){if 七.is_empty()&&侩{match&j.井{Some(井)=>乓.clone_from(井),None=>{低(&买,&mut
乓);j.井=Some(乓.clone());}}}else{低(&买,&mut 乓);}亭.clear();for(与,乮)in 乓.iter().enumerate(){let k=乌(乮.七());let 仫=亠.get(&k).or(j.仅.get(&k)).copied();亭.push((仫.unwrap_or(C::MAX),与));}亭.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 丹=vec![false;乓.len()];if 七.is_empty(){for(与,乮)in 乓.iter().enumerate(){丹[与]=亽.contains(&乌(乮.七()));}}let mut 三=None;for
伕 in[true,false]{let(丨,保)=j.俈(&买,亟,&乓,&亭,&mut 丹,丗,&mut 亠,伕,);三=丨;if 三.is_some()||!保{break;}}let Some((与,丂,丛))=三 else{break;};if 七.is_empty(){乊=Some(乌(乓[与].七()));}七.extend_from_slice(乓[与].七());买=丂;亟=丛;}for(k,乨)in
亠{let 佐=j.仅.entry(k).or_insert(乨);*佐=(*佐).max(乨);}些{七,end:买,first:乊,丒:None,}}fn 俑(&self,乔:&J)->K<K<L>>{let mut 丂=乔.clone();丂.仁();let mut 万=K::new();if
丂.且().is_some(){return 万;}let 乜=丂.东();let 丈=丂.丈(乜);if let Some(丧)=伈(&丂,Z[self.丁],丈){万.push(丧.七);}for 丝 in 但(&丂,伨,丈,侑){万.push(丝.七);}let mut 住=K::new();侸(&丂,&mut
住);万.push(住);let mut 乻=份{丁:乜,limit:乿{仆:侒,..self.limit},乲:0,仅:人::default(),井:None,};万.push(乻.伋(&丂,&[],&[],true).七);万.retain(|七|!七.is_empty());万}fn 伺(&self,乔:&J)->K<乚>{self.俑(乔).into_iter().map(|七|{let
伎=伏(&伹(乔,&七),self.丁);乚{七,伎}}).collect()}fn 乳(&mut self,乪:&J,乮:&乂,丗:&[乚])->P<些>{let mut 丷=乪.clone();if!伄(&mut 丷,乮.七()){return None;}self.乲+=1;let 乖=self.伋(&丷,丗,&[],false);let
mut 七=乮.七().to_vec();七.extend_from_slice(&乖.七);Some(些{七,end:乖.end,first:Some(乌(乮.七())),丒:None,})}fn 佋(&self,专:&些,丗:&[乚])->C{let 丛=self.伞(&专.end,丗,C::MIN);match&专.丒{Some(丒)if!丒.is_empty()=>丛.min(self.伞(&专.end,丒,C::MIN)),_=>丛,}}}fn
俏(丨:&[乚],丗:&mut Vec<乚>)->Q{let 並:K<_> =丗.iter().map(|r|r.七.clone()).collect();for 争 in 丨{if 丗.iter().any(|r|r.七==争.七){continue;}丗.push(争.clone());}丗.sort_by_key(|争|争.伎);丗.truncate(侊);丗.iter().map(|r|&r.七).ne(&並)}fn
俦(l:&J,丿:乿,京:&mut 亁)->伳{let 丁=l.东();let mut 中=伳::default();if l.且().is_some(){return 中;}if let Some(七)=仧(l){中.七=七;return 中;}let 丸=|俗:亙|match(丿.乺,丿.亓){(Some(乺),Some(亓))=>Some(乺+(亓-乺).mul_f64(俗)),_=>None,};if
京.俋(l){中.七=京.上(l,丸(0.4));return 中;}let mut 乍=份{丁,limit:乿{乺:丸(佺),..丿},乲:0,仅:人::default(),井:None,};let mut 丗=K::new();let mut 丮:K<些> =K::new();let mut 亽=K::new();let
mut 仠=false;let mut 乳:K<乂> =K::new();while 中.伌<侃&&(丮.is_empty()||!乍.习()){let 专=if 仠{let Some(乮)=乳.pop()else{break;};match 乍.乳(l,&乮,&丗){Some(专)=>专,None=>continue,}}else{乍.伋(l,&丗,&亽,true)};中.伌+=1;let
佩=丮.iter().any(|p|p.七==专.七);let 乊=专.first;let 佭=乍.习();let mut 专=专;let mut 佚=false;if!佭&&!佩{let 丨=乍.伺(&专.end);佚=俏(&丨,&mut 丗);专.丒=Some(丨);}if!佩{丮.push(专);}if
佭{break;}if!佚&&!仠{match 乊{Some(乊)=>亽.push(乊),None=>{仠=true;乳=乍.井.clone().unwrap_or_default();乳.retain(|a|!亽.contains(&乌(a.七())));let 乨=|a:&乂|{乍.仅.get(&乌(a.七())).copied().unwrap_or(C::MIN)};乳.sort_by_key(乨);}}}}中.乳=F::from(仠);中.乲=乍.乲;中.丗=丗.len();中.丮=丮.len();let
佖=||丿.亓.is_some_and(|t|乑::now()>=t);let mut 亰:K<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佖(){break;}亰[与]=Some(乍.佋(&丮[与],&丗));}let
亮=|亰:&[P<C>]|{let mut 三:P<(C,F)> =None;for(与,丛)in 亰.iter().enumerate().rev(){if let Some(丛)=*丛{if 三.is_none_or(|(乼,_)|丛>乼){三=Some((丛,与));}}}三};let mut
三=亮(&亰);乍.limit.乺=丿.亓;while let Some((_,与))=三{if!侞||丮[与].丒.is_some()||佖(){break;}let 丨=乍.伺(&丮[与].end);丮[与].丒=Some(丨);中.佀+=1;亰[与]=Some(乍.佋(&丮[与],&丗));三=亮(&亰);}if
let Some((_,与))=三{let mut 丬=l.clone();for&下 in&丮[与].七{if 丬.丫(下){中.七.push(下);}}}中}fn 俊(丿:&dyn Fn(乑,丰)->乿,mut 京:亁,伔:impl BufRead,们:impl Write,)->Result<(),丯>{丽(伔,们,|交,乗,上,亖|match
J::read(交,乗,乤::价,上){Ok(l)=>{let 中=俦(&l,丿(亖,上),&mut 京);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,中.伌,中.乲,中.丗,中.佀,中.乳,亖.elapsed());侤(&中.七)}Err(云)=>{eprintln!("plan: cannot read the input: {}",云);"WAIT".to_string()}},)}const
佸:亙=40.0;const 佻:亙=800.0;const 侗:亙=0.6;fn 丿(亖:乑,上:丰)->乿{if let Some(仆)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
乿{乺:None,亓:None,仆,};}let 仯=俨().min(100.0);let 促=if 上==1{佻}else{佸};let 严=Duration::from_secs_f64(促/1000.0*仯);乿{乺:Some(亖+严.mul_f64(侗)),亓:Some(亖+严),仆:U::MAX,}}const
侐:F=7;const 侎:亚=亚{丌:3,丑:1,丈:1,丝:2,乴:2,仐:2,丼:6,仒:1,};const 侏:I=I::MAX;fn 京()->亁{俆(64)}fn 俆(乣:F)->亁{let 乸=亜::仙{丽:侐,乣,乐:侎,};let 丙=侕.iter().map(|&乞|Y{伡:侏,..乞}).collect();亁::new(乸,丙,Y::仿)}fn
main(){let 信=io::stdin();let 俢=io::stdout();if let Err(云)=俊(&丿,京(),信.lock(),俢.lock()){eprintln!("plan: {}",云);std::process::exit(1);}}const 侚:&str="CG_TIME_SCALE";fn
俨()->亙{俇(std::env::var(侚).ok().as_deref())}fn 俇(业:P<&str>)->亙{业.and_then(|乗|乗.trim().parse::<亙>().ok()).filter(|仯|仯.is_finite()&&*仯>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乒:F=4*O;const 乥:U=(1<<乒)-1;const 仕:U=1|1<<12|1<<24|1<<36;const 仸:U=仕<<11;const 伱:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const T:G=G([0;3]);const 佷:G=G([乥;3]);#[inline]fn q(q:F)->G{let mut 丱=G::T;丱.set(q);丱}#[inline]fn test(self,q:F)->Q{(self.0[q/乒]>>(q%乒))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乒]|=1<<(q%乒);}#[inline]fn clear(&mut self,q:F){self.0[q/乒]&=!(1<<(q%乒));}#[inline]fn is_empty(self)->Q{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仲(self)->[U;3]{self.0}#[inline]fn count(self)->丰{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 两(self,个:G)->G{G([self.0[0]&!个.0[0],self.0[1]&!个.0[1],self.0[2]&!个.0[2],])}#[inline]fn
iter(self)->件{件{优:self.0,丸:0,}}#[inline]fn 俜(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伱)<<(乒-O)),(b>>O)|((c&伱)<<(乒-O)),c>>O,])}#[inline]fn 俙(self)->G{let[a,b,c]=self.0;G([(a<<O)&乥,((b<<O)&乥)|(a>>(乒-O)),((c<<O)&乥)|(b>>(乒-O)),])}#[inline]fn
俚(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仸&乥,(b>>1)&!仸&乥,(c>>1)&!仸&乥,])}#[inline]fn 俛(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仕&乥,(b<<1)&!仕&乥,(c<<1)&!仕&乥,])}#[inline]fn
世(self)->G{let j=self;j.俜()|j.俙()|j.俚()|j.俛()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,个:G)->G{G([self.0[0]&个.0[0],self.0[1]&个.0[1],self.0[2]&个.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,个:G)->G{G([self.0[0]|个.0[0],self.0[1]|个.0[1],self.0[2]|个.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,个:G){*self=*self|个;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::佷.两(self)}}struct 件{优:[U;3],丸:F,}impl Iterator for 件{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.丸<3{let 仨=j.优[j.丸];if 仨!=0{j.优[j.丸]=仨&(仨-1);return Some(j.丸*乒+仨.trailing_zeros()as F);}j.丸+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{亀{丏:丰,x:I,y:I},书{一:I,x:I,y:I},乽{x:I,y:I},亏{x:I,y:I},}impl L{fn 俪(&self)->丯{match*self{L::亀{丏,x,y}=>format!("MOVE {} {x} {y}",丏),L::书{一,x,y}=>format!("TRAIN {} {x} {y}",一),L::乽{x,y}=>format!("BUILD MINE {x} {y}"),L::亏{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
侤(七:&[L])->丯{if 七.is_empty(){return "WAIT".to_string();}let mut line=丯::with_capacity(16*七.len());for(与,下)in 七.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&下.俪());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乤{伶,俸,侠,#[default]价,}impl 乤{fn 乩(self)->I{match self{乤::伶=>1,_=>3,}}fn 仛(self)->Q{matches!(self,乤::侠|乤::价)}fn 伂(self)->Q{self==乤::价}}const 乙:I=I::MAX;struct
乭{丶:G,valid:G,丳:G,久:Box<[I]>,世:[[[I;4];M];2],}thread_local!{static 侀:RefCell<P<Arc<乭>>> =const{RefCell::new(None)};}impl 乭{fn 俘(丶:G,丳:G)->Arc<乭>{侀.with(|乵|{let
mut 乵=乵.borrow_mut();if let Some(亦)=乵.as_ref(){if 亦.丶==丶&&亦.丳==丳{return Arc::clone(亦);}}let 亦=Arc::new(乭::build(丶,丳));*乵=Some(Arc::clone(&亦));亦})}fn build(丶:G,丳:G)->乭{let
乬=!丶;const 侘:[[(仔,仔);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 世=[[[乙;4];M];2];for(z,乡)in 侘.iter().enumerate(){for q in
乬.iter(){let(x,y)=((q%O)as 仔,(q/O)as 仔);let mut 丕=0;for&(侰,侱)in 乡{let(作,佝)=(x+侰,y+侱);if(0..O as 仔).contains(&作)&&(0..O as 仔).contains(&佝){let 丂=佝 as F*O+作
as F;if 乬.test(丂){世[z][q][丕]=丂 as I;丕+=1;}}}}}let mut 久=vec![乙;M*M].into_boxed_slice();let mut 伝=[0u8;M];for 不 in 乬.iter(){let 亊=&mut 久[不*M..(不+1)*M];亊[不]=0;伝[0]=不
as I;let(mut 休,mut 伥)=(0,1);while 休<伥{let q=伝[休]as F;休+=1;for&丂 in&世[0][q]{if 丂!=乙&&亊[丂 as F]==乙{亊[丂 as F]=亊[q]+1;伝[伥]=丂;伥+=1;}}}}乭{丶,valid:乬,丳,久,世,}}#[inline]fn
久(&self,不:F,乫:F)->I{self.久[不*M+乫]}}const 侉:丰=100;const 于:C=15;const 依:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乧(F),伪,}enum
乘{仼,伮,伯,}#[derive(Clone)]struct J{map:Arc<乭>,丩:乤,丒:[G;2],丄:[G;2],乆:G,乇:G,丅:[G;2],一:[G;3],了:G,亴:[丰;M],丈:[C;2],丑:[C;2],东:F,亅:丰,亩:丰,且:P<么>,}fn 仄(丒:G,亣:F)->G{if!丒.test(亣){return
G::T;}let mut 仍=G::q(亣);loop{let 丂=(仍|仍.世())&丒;if 丂==仍{return 仍;}仍=丂;}}impl J{fn 东(&self)->F{self.东}fn 亅(&self)->丰{self.亅}fn 且(&self)->P<么>{self.且}fn 丩(&self)->乤{self.丩}fn
亨(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
丑(&self,z:F)->C{self.丑[z]}fn 伦(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&乭{&self.map}fn 丟(&self,z:F)->G{self.丒[z]}fn 丄(&self,z:F)->G{self.丄[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 仑(&self,q:F)->P<(丰,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亴[q],j.众(q)))}fn 亇(&self)->G{self.丅[self.东]&self.了}fn
丞(&self)->G{self.乆}fn 乢(&self)->G{self.乇}fn 亵(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.众(q))}fn 俠(&mut self,z:F){let j=self;j.丄=[仄(j.丒[0],Z[0]),仄(j.丒[1],Z[1])];let
侬=j.丅[z].两(j.丄[z]);j.伟(侬);j.佁();j.丈[z]+=j.丑[z];if j.丈[z]<0{j.丈[z]=0;j.伟(j.丅[z]);}j.了|=j.丅[z];}fn 佁(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
丅=丄&j.丅[z];let mut 丑=C::from(丄.count())+4*C::from((丄&j.乆).count());for(一,俰)in 依.iter().enumerate(){丑-=俰*C::from((丅&j.一[一]).count());}j.丑[z]=丑;}}fn 仁(&mut
self){let j=self;if j.且.is_some(){return;}if j.东==1{if j.亅>=侉{j.且=Some(match j.亨(0).cmp(&j.亨(1)){std::cmp::Ordering::Greater=>么::乧(0),std::cmp::Ordering::Less=>么::乧(1),std::cmp::Ordering::Equal=>么::伪,});return;}j.亅+=1;}j.东=1-j.东;j.俠(j.东);}fn
佢(&mut self,七:&[L]){let j=self;if j.且.is_some(){return;}let 伐=j.亩;for&下 in 七{if j.侯(下,伐){continue;}j.丫(下);if j.且.is_some(){return;}}j.仁();}fn 侯(&self,下:L,伐:丰)->Q{let
j=self;match 下{L::亀{丏,x,y}=>{j.亞(x,y).is_none()||丏>=伐||j.佌(j.东,丏).is_none()}L::书{一,x,y}=>{j.亞(x,y).is_none()||一==0||一>j.丩.乩()}L::乽{x,y}|L::亏{x,y}=>{j.亞(x,y).is_none()}}}#[inline]fn
亞(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丶.test(y*O+x)).then_some(y*O+x)}fn 佌(&self,z:F,丏:丰)->P<F>{self.丅[z].iter().find(|&q|self.亴[q]==丏)}#[inline]fn
众(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 侦(&self,乜:F)->G{let 乢=self.乇&self.丒[乜];let 丄=self.丄[乜];乢|(丄&(丄&乢).世())}fn
为(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乜=1-z;let 丅=j.丅[0]|j.丅[1];let 侲=j.什();let 修=match
一{1 if j.丩==乤::伶=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::T,};let 为=侲|j.丒[乜].两(修);if 一<3{为.两(j.侦(乜))}else{为}}fn 五(&self,z:F)->G{self.丄[z]|self.丄[z].世()}fn
什(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 侻=G::q(Z[0])|G::q(Z[1]);j.map.valid.两(丅|j.乆|j.乇|侻)}fn 丫(&mut self,下:L)->Q{let j=self;if j.且.is_some(){return
false;}let z=j.东;let 亐=match 下{L::书{一,x,y}=>j.俬(z,一,x,y),L::亀{丏,x,y}=>j.係(z,丏,x,y),L::乽{x,y}=>j.build(z,true,x,y),L::亏{x,y}=>j.build(z,false,x,y),};let
Some(亐)=亐 else{return false;};match 亐{乘::仼=>{}乘::伮=>j.丄[z]=仄(j.丒[z],Z[z]),乘::伯=>{j.丄=[仄(j.丒[0],Z[0]),仄(j.丒[1],Z[1])];}}j.佁();if!j.丒[0].test(Z[0]){j.且=Some(么::乧(1));}else
if!j.丒[1].test(Z[1]){j.且=Some(么::乧(0));}true}fn 伟(&mut self,丌:G){let j=self;j.丅[0]=j.丅[0].两(丌);j.丅[1]=j.丅[1].两(丌);for 一 in&mut j.一{*一=一.两(丌);}j.了=j.了.两(丌);}fn
佯(&mut self,z:F,q:F)->乘{let j=self;j.伟(G::q(q));j.乆.clear(q);j.乇.clear(q);let 乜=1-z;if j.丒[乜].test(q){j.丒[乜].clear(q);j.丒[z].set(q);乘::伯}else if j.丒[z].test(q){乘::仼}else{j.丒[z].set(q);乘::伮}}fn
俬(&mut self,z:F,一:I,x:I,y:I)->P<乘>{let j=self;if 一==0||一>j.丩.乩(){return None;}let q=j.亞(x,y)?;let 丆=10*C::from(一);if j.丈[z]<丆||!j.五(z).test(q)||!j.为(z,一).test(q){return
None;}let 亐=j.佯(z,q);j.你(z,一,q,j.亩);j.亩=j.亩.saturating_add(1);j.丈[z]-=丆;Some(亐)}fn 你(&mut self,z:F,一:I,q:F,丏:丰){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亴[q]=丏;}fn
係(&mut self,z:F,丏:丰,x:I,y:I)->P<乘>{let j=self;let 丐=j.亞(x,y)?;let 不=j.佌(z,丏)?;if!j.了.test(不){return None;}let 一=j.众(不);let 丢=j.为(z,一);let 举=j.map.世[z][不];if
举.contains(&(丐 as I))&&!丢.test(丐){return None;}let mut 三=j.map.久(不,丐);let mut 亾=None;for&丂 in 举.iter().take_while(|&&丂|丂!=乙){let 丂=F::from(丂);let 久=j.map.久(丂,丐);if
久<三&&丢.test(丂){三=久;亾=Some(丂);}}let 亾=亾?;let 亐=j.佯(z,亾);j.丅[z].clear(不);j.一[F::from(一)-1].clear(不);j.了.clear(不);j.你(z,一,亾,丏);Some(亐)}fn build(&mut self,z:F,乆:Q,x:I,y:I)->P<乘>{let
j=self;let 侣=if 乆{j.丩.仛()}else{j.丩.伂()};let q=j.亞(x,y)?;let 俄=j.map.丳.test(q);if!侣||j.丈[z]<于||俄!=乆||!j.丒[z].test(q)||!j.五(z).test(q)||!j.什().test(q){return
None;}if 乆{j.乆.set(q);}else{j.乇.set(q);}j.丈[z]-=于;Some(乘::仼)}}const 侅:C=1000000000;const 仹:C=0;const 侌:C=1;const 供:C=2;impl J{fn read(交:&str,上:&str,丩:乤,亅:丰,)->Result<J,丯>{let
mut 丳=K::new();let mut 丣=交.lines();let 丕=伍(&mut 丣,"mine spot count")?;for _ in 0..丕{let[x,y]=乶(亪(&mut 丣,"a mine spot")?,"a mine spot")?;丳.push((仞(x)?,仞(y)?));}佉(丣,"the initial input")?;let
mut 丣=上.lines();let mut 伒=[0i64;4];for(业,临)in 伒.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伜]=乶(亪(&mut 丣,临)?,临)?;if 伜.abs()>侅{return
Err(format!("{}: {} is out of range",临,伜));}*业=伜;}let mut 丶=G::T;let mut 丌=[G::T;2];let mut 丄=[G::T;2];for y in 0..O{let 亊=亪(&mut 丣,"a map row")?.trim();if
亊.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,亊));}for(x,从)in 亊.bytes().enumerate(){let q=y*O+x;match 从{b'#'=>丶.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乀=F::from(从==b'X'||从==b'x');丌[乀].set(q);if 从.is_ascii_uppercase(){丄[乀].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",从 as char))}}}}let
侨=伍(&mut 丣,"building count")?;let mut 乎=K::new();for _ in 0..侨{let 乶=乶(亪(&mut 丣,"a building")?,"a building")?;乎.push(乶);}let 伦=伍(&mut 丣,"unit count")?;let
mut 丅=K::new();for _ in 0..伦{let 乶=乶(亪(&mut 丣,"a unit")?,"a unit")?;丅.push(乶);}佉(丣,"the turn's input")?;let 佟=乎.iter().find(|伽|伽[0]==0&&伽[1]==仹).ok_or("the bot's own HQ is not among the buildings")?;let
体=伇(佟[2],佟[3],丶)?;let z=Z.iter().position(|&亣|亣==体).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",体))?;let mut 佫=G::T;for(x,y)in 丳{if
丶.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佫.set(y*O+x);}let 伀=|乀:F|if 乀==0{z}else{1-z};let mut l=J{map:乭::俘(丶,佫),丩,丒:[G::T;2],丄:[G::T;2],乆:G::T,乇:G::T,丅:[G::T;2],一:[G::T;3],了:G::T,亴:[0;M],丈:[0;2],丑:[0;2],东:z,亅,亩:1,且:None,};for
乀 in 0..2{let 九=伀(乀);l.丒[九]=丌[乀];l.丄[九]=丄[乀];l.丈[九]=伒[2*乀];l.丑[九]=伒[2*乀+1];}let mut 佞=G::T;for[九,以,x,y]in 乎{let 乀=佦(九)?;let q=伇(x,y,丶)?;if!丌[乀].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佞.test(q){return Err(format!("two buildings at ({x}, {y})"));}佞.set(q);match 以{仹
if q==Z[伀(乀)]=>{}仹=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侌=>l.乆.set(q),供=>l.乇.set(q),_=>return Err(format!("unknown building type {}",以)),}}let
mut 乁=G::T;let mut 仦=0u32;for[九,丏,一,x,y]in 丅{let 乀=佦(九)?;let q=伇(x,y,丶)?;if!丌[乀].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乁.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丏=丰::try_from(丏).map_err(|_|format!("unit id {} is out of range",丏))?;if
乁.iter().any(|个|l.亴[个]==丏){return Err(format!("two units have id {}",丏));}乁.set(q);l.丅[伀(乀)].set(q);l.一[一 as F-1].set(q);l.亴[q]=丏;仦=仦.max(丏);}if!乁.is_empty(){l.亩=仦.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仦))?;}l.了=l.丅[z];Ok(l)}}fn
亪<'a>(丣:&mut Lines<'a>,临:&str)->Result<&'a str,丯>{丣.next().ok_or_else(||format!("the text ends before {}",临))}fn 佉(mut 丣:Lines<'_>,临:&str)->Result<(),丯>{match
丣.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",临)),None=>Ok(()),}}fn 乶<const N:F>(line:&str,临:&str)->Result<[C;N],丯>{let
mut 仲=line.split_whitespace();let mut 併=[0i64;N];for 业 in&mut 併{let 亷=仲.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",临))?;*业=亷.parse().map_err(|_|format!("{}: {:?} is not an integer",临,亷))?;}if
仲.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",临));}Ok(併)}fn 伍(丣:&mut Lines<'_>,临:&str)->Result<F,丯>{let[丕]=乶(亪(丣,临)?,临)?;F::try_from(丕).ok().filter(|&丕|丕<=M).ok_or_else(||format!("{}: {} is out of range",临,丕))}fn
仞(业:C)->Result<F,丯>{F::try_from(业).ok().filter(|&业|业<O).ok_or_else(||format!("coordinate {} is off the map",业))}fn 伇(x:C,y:C,丶:G)->Result<F,丯>{let q=仞(y)?*O+仞(x)?;if
丶.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佦(九:C)->Result<F,丯>{match 九{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",九)),}}const
佽:C=15;const 侍:C=6;const 伴:C=10;const 侔:I=3;#[derive(Clone,Copy)]struct Y{open:C,仝:C,佄:I,伓:C,佒:C,佗:C,伡:I,佃:C,佂:C,伸:C,亢:C,亲:C,丵:C,亥:C,亍:C,}impl Y{const
仿:Y=Y{open:3,仝:2,佄:12,伓:4,佒:佽,佗:侍,伡:侔,佃:2,佂:2,伸:0,亢:0,亲:0,丵:0,亥:0,亍:0,};const fn 俶(self,亢:C,亲:C,丵:C)->Y{Y{亢,亲,丵,..self}}const fn 俵(self,亥:C,亍:C)->Y{Y{亥,亍,..self}}}fn
仓(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 亿(l:&J,不:G)->[I;M]{let mut 义=[I::MAX;M];let 乬=l.map().valid;let mut 之=不&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn 侵(l:&J)->I{let 义=亿(l,l.丟(0));l.丟(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亂<'a>{乞:&'a Y,丁:F,theirs:[I;M],亳:[I;M],free:G,乹:G,ours:G,丳:G,}impl 亂<'_>{fn new<'a>(l:&J,乞:&'a Y)->亂<'a>{let 丁=l.东();let 乆=亿(l,l.丟(丁));let 乻=亿(l,l.丟(1-丁));let
亳=亿(l,G::q(Z[1-丁]));let 互=l.map().valid.两(l.丟(0)|l.丟(1));let mut 乹=G::T;for q in 互.iter(){if 乆[q]<=乻[q]{乹.set(q);}}亂{乞,丁,theirs:乻,亳,free:互,乹,ours:l.丟(丁),丳:l.map().丳,}}fn
value(&self,q:F)->C{let j=self;let p=j.乞;let 主=C::from((G::q(q).世()&j.free).count());let 伿=p.佄;let 仝=C::from(伿)-C::from(j.theirs[q].min(伿));let 亣=if p.伓>0{C::from(j.亳[q].min(30))/p.伓}else{0};let
丒=C::from((G::q(q).世()&j.ours).count());let line=if 丒<=1{p.亲}else{0};let 丵=if j.丳.test(q){p.丵}else{0};let 俤=if 主==0{p.亥}else{0};let 亍=p.亍*(22-C::from(j.亳[q].min(22)));p.open*主+p.仝*仝-亣+p.亢*丒-line+丵-俤+亍}fn
三(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伊(&mut self,q:F){self.free.clear(q);self.乹.clear(q);self.ours.set(q);}}fn
介(l:&mut J,下:L,万:&mut Vec<L>)->Q{let 伷=l.丫(下);if 伷{万.push(下);}伷}fn 丘(l:&mut J,不:F,乫:F,万:&mut Vec<L>)->Q{let Some((丏,_))=l.仑(不)else{return false;};let(x,y)=仓(乫);介(l,L::亀{丏,x,y},万)}fn
俯(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伴}#[derive(Clone,Copy)]enum 乃{付,乾{丞:I,丅:I},}fn 佲(l:&J,乞:&Y,亄:乃)->K<L>{let mut 丬=l.clone();let
mut 上=亂::new(l,乞);let mut 万=K::new();乛(&mut 丬,&mut 上,&mut 万);match 亄{乃::付=>俍(&mut 丬,&mut 上,&mut 万),乃::乾{丞,丅}=>{侴(&mut 丬,&mut 上,丞,丅,&mut 万)}}万}fn 乛(l:&mut
J,上:&mut 亂<'_>,万:&mut Vec<L>){let 丁=上.丁;let mut 代:K<F> =K::new();loop{let 丢=l.为(丁,1);let mut 亮:P<(丰,F)> =None;for 不 in l.亇().iter(){if 代.contains(&不){continue;}let
乷=(G::q(不).世()&上.free&丢).count();if 乷==0{代.push(不);}else if 亮.is_none_or(|(n,_)|乷<n){亮=Some((乷,不));}}let Some((_,不))=亮 else{break;};match 上.三(G::q(不).世()&上.free&丢){Some(乫)if
丘(l,不,乫,万)=>上.伊(乫),_=>代.push(不),}}for 不 in 代{if!l.亇().test(不){continue;}let 丐=[上.乹,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().久(不,q),q))});if
let Some(丐)=丐{丘(l,不,丐,万);}}}fn 俍(l:&mut J,上:&mut 亂<'_>,万:&mut Vec<L>){let 丁=上.丁;let p=上.乞;let h=(p.佒-C::from(l.亅())).max(p.佗);let 俩=(C::from(侵(l))-p.佃).max(0)/p.佂.max(1)+p.伸;loop{let
丈=l.丈(丁);let 丅=l.伦(丁)as C;let 俒=C::from(上.乹.count());let q=上.三(l.五(丁)&上.free&l.为(丁,1));let 佴=俯(h,(俒/(丅+1)).min(俩));let 伾=q.is_some()&&丈>=伴&&佴>0;let 佘=4*h-于;let
丵=余(l,上);let 侪=丵.is_some()&&丈>=于&&佘>0;if 侪&&(!伾||佘>=佴){let(x,y)=仓(丵.unwrap_or_default());if 介(l,L::乽{x,y},万){continue;}break;}let Some(q)=q.filter(|_|伾)else{break;};let(x,y)=仓(q);if!介(l,L::书{一:1,x,y},万){break;}上.伊(q);}}fn
余(l:&J,上:&亂<'_>)->P<F>{let 丁=上.丁;(l.map().丳&l.丄(丁)&l.什()).iter().filter(|&丵|上.theirs[丵]>上.乞.伡).min_by_key(|&丵|(l.map().久(Z[丁],丵),丵))}fn 侴(l:&mut J,上:&mut
亂<'_>,丞:I,丅:I,万:&mut Vec<L>,){let 丁=上.丁;for _ in 0..丞{let Some(丵)=余(l,上)else{break;};let(x,y)=仓(丵);if!介(l,L::乽{x,y},万){break;}}for _ in 0..丅{let Some(q)=上.三(l.五(丁)&上.free&l.为(丁,1))else{break;};let(x,y)=仓(q);if!介(l,L::书{一:1,x,y},万){break;}上.伊(q);}}fn
仮(l:&J)->Q{if l.且().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丼=亝::new(l,丁,true);l.丟(1-丁).iter().any(|q|丼.丆[q]<=丈)}#[derive(Clone,Copy)]struct
両{亘:I,亄:乃,}const 伧:I=I::MAX;const 佹:[乃;8]=[乃::付,乃::乾{丞:0,丅:0},乃::乾{丞:0,丅:1},乃::乾{丞:0,丅:2},乃::乾{丞:0,丅:伧,},乃::乾{丞:1,丅:0},乃::乾{丞:1,丅:1},乃::乾{丞:1,丅:伧,},];struct
伩<'a>{丙:&'a[Y],仇:Y,rest:両,乱:P<乑>,俓:Settings,}impl 伩<'_>{fn 乞(&self,丘:両)->&Y{&self.丙[F::from(丘.亘)%self.丙.len()]}fn 七(&self,l:&J,丘:両)->K<L>{佲(l,self.乞(丘),丘.亄)}}fn
伢(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伙=|亷:U|hash=(hash^亷).wrapping_mul(0x0100000001b3);for(俥,丱)in[l.丟(丁),l.丅(丁),l.丞()].into_iter().enumerate(){for
q in 丱.iter(){伙(((俥 as U)<<8)|q as U);}伙(0xff);}伙(l.丈(丁)as U);hash}enum 亜{仙{丽:F,乣:F,乐:亚,},}struct 亁{乸:亜,丙:K<Y>,仇:Y,专:K<両>,仡:K<U>,亯:F,丮:丰,仭:Q,}impl 亁{fn
new(乸:亜,丙:K<Y>,仇:Y)->亁{亁{乸,丙,仇,专:K::new(),仡:K::new(),亯:0,丮:0,仭:false,}}fn 俋(&mut self,l:&J)->Q{let j=self;let 俞=matches!(j.乸,亜::仙{..});let 俌=!俞&&j.丮>0&&j.亯>=j.仡.len();if!j.仭&&(俌||仮(l)){j.仭=true;}!j.仭}fn
上(&mut self,l:&J,乱:P<乑>)->K<L>{let j=self;let 佅=伩{丙:&j.丙,仇:j.仇,rest:両{亘:0,亄:乃::付,},乱,俓:Settings::DEFAULT,};let 丁=l.东();let 俅=j.仡.get(j.亯)==Some(&伢(l,丁));if!俅{let
乊=j.丮==0;j.专=match j.乸{亜::仙{丽,乣,乐,}=>{let 乐=&乐;let 乖=侓;let mut 乡=专(l,&j.丙,乖,丽,乣,乱,乐,);乡.resize(丽,乖);乡}};j.仡=match j.乸{亜::仙{..}=>俟(l,&j.丙,&j.专),};j.亯=0;j.丮+=1;}let
丘=j.专.get(j.亯).copied().unwrap_or(佅.rest);j.亯+=1;佅.七(l,丘)}}fn 俟(l:&J,丙:&[Y],专:&[両])->K<U>{let 丁=l.东();let mut 何=K::with_capacity(专.len());let mut 也=l.clone();for&丘
in 专{if 也.且().is_some(){break;}何.push(伢(&也,丁));也=企(&也,&七(&也,丙,丘));}何}const 侜:C=3;struct Settings{丽:C,}impl Settings{const DEFAULT:Settings=Settings{丽:侜};}fn
乡(l:&J,不:G)->[I;M]{let 乬=l.map().valid;let mut 义=[I::MAX;M];let mut 之=不&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn
俱(l:&J,z:F)->C{let 仉=乡(l,l.丄(z));let 乻=乡(l,l.丄(1-z));l.map().valid.iter().map(|q|match 仉[q].cmp(&乻[q]){_ if 仉[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侕:[Y;27]=丙();const 侓:両=両{亘:16,亄:乃::付,};const fn 丙()->[Y;27]{let 亢=[0,2,4];let 亲=[0,3,6];let 亍=[0,1,2];let mut 万=[Y::仿;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亥=if g>0||t>0{4}else{0};万[i]=Y::仿.俶(亢[g],亲[t],0).俵(亥,亍[w]);i+=1;}万}#[derive(Default)]struct 仾{丌:C,丑:C,丈:C,乴:C,丝:C,仐:C,丼:C,仒:C,}#[derive(Clone,Copy)]struct
亚{丌:C,丑:C,丈:C,丝:C,乴:C,仐:C,丼:C,仒:C,}fn 俔(l:&J,z:F)->[C;M]{struct 亹{丄:G,乄:[u16;M],串:[u16;M],size:[C;M],丝:[C;M],二:u16,}fn 亗(d:&mut 亹,v:F){d.二+=1;d.乄[v]=d.二;d.串[v]=d.二;d.size[v]=1;for
w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{亗(d,w);d.size[v]+=d.size[w];d.串[v]=d.串[v].min(d.串[w]);if d.串[w]>=d.乄[v]{d.丝[v]+=d.size[w];}}else{d.串[v]=d.串[v].min(d.乄[w]);}}}let
mut d=亹{丄:l.丄(z),乄:[0;M],串:[0;M],size:[0;M],丝:[0;M],二:0,};let 乪=Z[z];if d.丄.test(乪){亗(&mut d,乪);}d.丝[乪]=0;d.丝}fn 俖(l:&J,z:F)->仾{let 丄=l.丄(z);let 丝=俔(l,z);let
亳=亿(l,G::q(Z[1-z]));let mut 万=仾{丌:C::from(丄.count()),丑:l.丑(z),丈:l.丈(z),丼:C::MAX,仒:俱(l,z)/2,..仾::default()};for q in 丄.iter(){if 丝[q]>0{万.乴+=1;万.丝=万.丝.max(丝[q]);}if
q!=Z[z]&&(G::q(q).世()&丄).count()<=1{万.仐+=1;}万.丼=万.丼.min(C::from(亳[q]));}万}fn 业(l:&J,z:F,乐:&亚)->C{let s=俖(l,z);let o=乐;o.丌*s.丌+o.丑*s.丑+o.丈*s.丈-o.丝*s.丝-o.乴*s.乴-o.仐*s.仐-o.丼*s.丼.min(30)+o.仒*s.仒}fn
企(l:&J,七:&[L])->J{let mut 丂=l.clone();丂.佢(七);if 丂.且().is_none(){丂.佢(&[]);}丂}fn 七(l:&J,丙:&[Y],丘:両)->K<L>{佲(l,&丙[F::from(丘.亘)%丙.len()],丘.亄,)}fn 佡(l:&J,丙:&[Y],专:&[両],乖:両,丽:F,)->J{let
mut 也=l.clone();for 上 in 0..丽{if 也.且().is_some(){break;}let 丘=专.get(上).copied().unwrap_or(乖);也=企(&也,&七(&也,丙,丘));}也}fn 俣(l:&J,丙:&[Y])->K<(両,K<L>)>{let mut
万:K<(両,K<L>)> =K::new();for 亘 in 0..丙.len().min(F::from(I::MAX)){for&亄 in&佹{let 丘=両{亘:亘 as I,亄,};let c=七(l,丙,丘);if!万.iter().any(|(_,个)|*个==c){万.push((丘,c));}}}万}fn
侥(l:&J,丙:&[Y],乖:両,丽:F,乣:F,乱:P<乑>,乐:&亚,)->K<両>{let 丁=l.东();let 今=||乱.is_some_and(|t|乑::now()>=t);let 仌=|专:&[両]|业(&佡(l,丙,专,乖,丽),丁,乐);let mut 三:(C,K<両>)=(仌(&[]),K::new());let
mut 之:K<(J,K<両>)> =vec![(l.clone(),K::new())];for 侮 in 0..丽{let mut 乯:K<(C,J,K<両>)> =K::new();let mut 乁:HashMap<U,F> =HashMap::new();for(佛,专)in&之{if 今(){break;}for(丘,c)in
俣(佛,丙){let 丂=企(佛,&c);let mut 伉=专.clone();伉.push(丘);let v=业(&丂,丁,乐);let 乌=伢(&丂,丁);match 乁.get(&乌){Some(&丸)if 乯[丸].0>=v=>{}Some(&丸)=>乯[丸]=(v,丂,伉),None=>{乁.insert(乌,乯.len());乯.push((v,丂,伉));}}}}if
乯.is_empty(){break;}乯.sort_by(|a,b|b.0 .cmp(&a.0));乯.truncate(乣);let 乵=侮+1==丽||今();for(i,(_,_,专))in 乯.iter().take(if 乵{乣}else{1}).enumerate(){if i>0&&今(){break;}let
v=仌(专);if v>三.0{三=(v,专.clone());}}if 乵{break;}之=乯.into_iter().map(|(_,s,p)|(s,p)).collect();}三.1}fn 专(l:&J,丙:&[Y],乖:両,丽:F,俀:F,乱:P<乑>,乐:&亚,)->K<両>{let 丁=l.东();let
仌=|专:&[両]|业(&佡(l,丙,专,乖,丽),丁,乐);let mut 三:(C,K<両>)=(仌(&[]),K::new());let mut 乣=2;loop{let 丨=侥(l,丙,乖,丽,乣,乱,乐);let v=仌(&丨);if v>三.0{三=(v,丨);}let 今=乱.is_some_and(|t|乑::now()>=t);if
今||(乱.is_none()&&乣>=俀){break;}乣*=2;}三.1}const 佼:F=4+12;fn 丽(伔:impl BufRead,mut 们:impl Write,mut 伃:impl FnMut(&str,&str,丰,乑)->丯,)->Result<(),丯>{let mut
丣=伬{佥:伔};let Some(乊)=丣.line()?else{return Ok(());};let mut 交=format!("{}\n",乊);丣.take(丕(&乊,"mine spot count")?,&mut 交)?;let mut 上=0;loop{let Some(乊)=丣.line()?else{return
Ok(());};let 亖=乑::now();上+=1;let mut 乗=format!("{}\n",乊);丣.take(佼-1,&mut 乗)?;for 临 in["building count","unit count"]{let line=丣.佧()?;let n=丕(&line,临)?;乗.push_str(&line);乗.push('\n');丣.take(n,&mut
乗)?;}let 伃=伃(&交,&乗,上,亖);writeln!(们,"{}",伃).and_then(|()|们.flush()).map_err(|云|format!("turn {}: cannot write the answer: {}",上,云))?;}}struct 伬<R>{佥:R,}impl<R:BufRead>伬<R>{fn
line(&mut self)->Result<P<丯>,丯>{let mut line=丯::new();match self.佥.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(云)=>Err(format!("cannot read input: {}",云)),}}fn
佧(&mut self)->Result<丯,丯>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乗:&mut String)->Result<(),丯>{for
_ in 0..n{乗.push_str(&self.佧()?);乗.push('\n');}Ok(())}}fn 丕(line:&str,临:&str)->Result<F,丯>{match line.trim().parse::<F>(){Ok(n)if n<=侄=>Ok(n),_=>Err(format!("bad {} {line:?}",临)),}}const
侄:F=144;const 侙:C=3;const 使:C=2;const 侈:F=16;fn 俉(l:&J,万:&mut Vec<L>){let z=l.东();let mut 乸=估{s:l.clone(),z,乏:1-z,丐:Z[1-z],仢:Z[z],上:l.亅(),万,};乸.run();}struct
估<'a>{s:J,z:F,乏:F,丐:F,仢:F,上:丰,万:&'a mut Vec<L>,}fn 仴(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 会(丏:丰,q:F)->L{let(x,y)=仴(q);L::亀{丏,x,y}}fn 佱(一:I,q:F)->L{let(x,y)=仴(q);L::书{一,x,y}}impl
估<'_>{fn run(&mut self){let j=self;if j.仧(){return;}j.乛();if j.仧(){return;}j.乎();j.俭();}fn 义(&self,不:F,乫:F)->C{C::from(self.s.map().久(不,乫))}fn 丫(&mut self,下:L)->Q{let
丹=self.s.丫(下);if 丹{self.万.push(下);}丹}fn 亶(&self,丠:&J)->Q{丠.且()==Some(么::乧(self.z))}fn 仧(&mut self)->Q{let j=self;if j.s.且().is_some(){return false;}for
q in j.s.亇().iter(){if j.义(q,j.丐)!=1{continue;}let Some((丏,_))=j.s.仑(q)else{continue;};let 下=会(丏,j.丐);let mut 丠=j.s.clone();if 丠.丫(下)&&j.亶(&丠){return j.丫(下);}}let
Some(丧)=j.侫()else{return false;};let mut 丠=j.s.clone();if!丧.iter().all(|&下|丠.丫(下))||!j.亶(&丠){return false;}for 下 in 丧{j.丫(下);}true}fn 侫(&self)->P<K<L>>{let
j=self;const 伫:C=C::MAX;let 乕=j.s.丩().乩();let 丢:K<G> =(1..=乕).map(|一|j.s.为(j.z,一)).collect();let 仂=|q:F|{丢.iter().position(|丱|丱.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 丄=j.s.丄(j.z);let mut 丆=[伫;M];let mut 並=[F::MAX;M];let mut 丹=[false;M];for q in 丄.iter(){丆[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!丹[q]&&丆[q]<伫&&丂.is_none_or(|n|丆[q]<丆[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};丹[q]=true;if q==j.丐{break;}for&举 in&j.s.map().世[0][q]{if 举==乙{break;}let 举=F::from(举);if 丹[举]||丄.test(举){continue;}let Some((_,予))=仂(举)else{continue;};if
丆[q]+予<丆[举]{丆[举]=丆[q]+予;並[举]=q;}}}if 丆[j.丐]>j.s.丈(j.z){return None;}let mut 丧=K::new();let mut q=j.丐;while!丄.test(q){let(一,_)=仂(q)?;丧.push(佱(一,q));q=並[q];}丧.reverse();Some(丧)}fn
乨(&self,丷:&J)->C{let(丁,事)=(self.z,self.乏);let 並=&self.s;let 丕=|丱:G|C::from(丱.count());let 丒=丕(丷.丟(丁))-丕(並.丟(丁));let 仱=丕(並.丟(事))-丕(丷.丟(事));let 侽=丕(並.丄(事))-丕(丷.丄(事));let
令=並.亨(事)-丷.亨(事);let 丞=丕(並.丞()&並.丟(事))-丕(丷.丞()&丷.丟(事));12*丒+20*仱+8*(侽-仱).max(0)+5*令+25*丞}fn 佬(&self,q:F)->C{let j=self;let 丵=j.s.map().丳.test(q);if 丵&&j.s.丩().仛()&&!j.s.丟(j.z).test(q){20}else{0}}fn
佊(&self,丿:C)->Q{let j=self;j.s.丅(j.乏).iter().any(|q|j.义(q,j.仢)<=丿)}fn 乛(&mut self){let j=self;let mut 丅:K<(C,F)> =j.s.亇().iter().map(|q|(j.义(q,j.丐),q)).collect();丅.sort_unstable();for(侺,q)in
丅{if j.s.且().is_some(){return;}let Some((丏,一))=j.s.仑(q)else{continue;};let 丢=j.s.为(j.z,一);let 举=j.s.map().世[j.z][q];let mut 三:P<(C,F)> =None;for&丂 in 举.iter().take_while(|&&丂|丂!=乙){let
丂=F::from(丂);if!丢.test(丂){continue;}let mut 丠=j.s.clone();if!丠.丫(会(丏,丂)){continue;}let 业=if j.亶(&丠){100000}else{j.乨(&丠)+10*(侺-j.义(丂,j.丐))+j.佬(丂)};if 三.is_none_or(|(乼,_)|业>乼){三=Some((业,丂));}}if
let Some((业,丂))=三{if 业>0{j.丫(会(丏,丂));}}}}fn 乎(&mut self){let j=self;if j.s.且().is_some(){return;}if j.佊(侙){j.俫();}else{j.丞();}}fn 俫(&mut self){let j=self;if!j.s.丩().伂()||j.s.丈(j.z)<于{return;}let
举=j.s.map().世[j.z][j.仢];let 丌:K<F> =举.iter().take_while(|&&q|q!=乙).map(|&q|F::from(q)).collect();let 乆=j.s.丟(j.z);if 丌.iter().any(|&q|乆.test(q)&&j.s.乢().test(q)){return;}for
q in 丌{let(x,y)=仴(q);if j.丫(L::亏{x,y}){return;}}}fn 丞(&mut self){let j=self;if!j.s.丩().仛(){return;}while j.s.丈(j.z)>=于{let 互=j.s.map().丳&j.s.丟(j.z)&j.s.丄(j.z)&j.s.什();let
Some(q)=互.iter().min_by_key(|&q|(j.义(q,j.仢),q))else{return;};let(x,y)=仴(q);if!j.丫(L::乽{x,y}){return;}}}fn 俭(&mut self){let j=self;let 佨=if j.上<=3{0}else{5};for
_ in 0..侈{if j.s.且().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 侼=j.佊(使);let 仪=j.s.五(j.z);let mut 三:P<(C,L)> =None;let mut 伤:P<(C,L)> =None;let
mut 似=G::T;for 一 in 1..=j.s.丩().乩(){let 丢=j.s.为(j.z,一);let 乷=(仪&丢).两(似);似=丢;let 予=10*C::from(一);if 予>丈{continue;}for q in 乷.iter(){let 下=佱(一,q);let mut
丠=j.s.clone();if!丠.丫(下){continue;}let 伖=丠.丈(j.z);let 丑=丠.丑(j.z);let 令=j.s.亨(j.乏)-丠.亨(j.乏);let 価=if 令>0&&侼{伖+丑>=0}else{丑>=0&&(伖>=佨||令>0)};if!価{continue;}let
乨=j.乨(&丠);let 业=乨+j.佬(q)-6*j.义(q,j.丐)-10*(C::from(一)-1);if 乨>0{if 三.is_none_or(|(乼,_)|业>乼){三=Some((业,下));}}else if 丑>=3&&伖>=佨+10&&一==1{if 伤.is_none_or(|(乼,_)|业>乼){伤=Some((业,下));}}}}let
Some((_,下))=三.or(伤)else{return;};if!j.丫(下){return;}}}}fn 侸(l:&J,万:&mut Vec<L>){万.clear();俉(l,万);}const 亃:C=C::MAX;const 侇:F=8;struct 亝{丆:[C;M],並:[I;M],}fn
仟(l:&J,z:F,乛:Q,丐:F)->C{let mut 丆=亃;位(l,z,乛,|丘,new|{if new.test(丐){丆=10*丘 as C;}丆!=亃});丆}fn 佤(l:&J,z:F,乛:Q,严:C)->G{let mut 仮=G::T;位(l,z,乛,|丘,new|{if 10*丘
as C>严{return true;}仮|=new;false});仮.两(l.丄(z))}fn 位(l:&J,z:F,乛:Q,mut 亗:impl FnMut(F,G)->Q,){let 乕=l.丩().乩();let mut 仂=[G::T;3];let mut 乁=G::T;for 一 in
1..=乕{let 丱=l.为(z,一);仂[F::from(一)-1]=丱.两(乁);乁|=丱;}let mut 主=[l.丄(z),G::T,G::T,G::T];if 乛&&z==l.东(){for q in l.亇().iter(){if let Some(一)=l.亵(q).filter(|&一|一<=乕){主[0]|=G::q(q).世()&l.为(z,一);}}}let
mut 丹=G::T;for 丘 in 0..{let new=主[丘%4].两(丹);主[丘%4]=G::T;if 亗(丘,new){return;}丹|=new;let 丂=new.世().两(丹);for(侢,丌)in 仂.iter().enumerate(){主[(丘+侢+1)%4]|=丂&*丌;}if
主.iter().all(|丱|丱.两(丹).is_empty()){break;}}}impl 亝{fn new(l:&J,z:F,乛:Q)->亝{let 乕=F::from(l.丩().乩());let mut 丢=[G::T;3];for(与,丱)in 丢.iter_mut().enumerate().take(乕){*丱=l.为(z,与
as I+1);}let 丄=l.丄(z);let 世=&l.map().世[0];let mut 丆=[亃;M];let mut 並=[乙;M];let mut 主=[0u8;M];let mut 丕=0;let mut 佪=G::T;for q in 丄.iter(){丆[q]=0;主[丕]=q
as I;丕+=1;}if 乛&&z==l.东(){for 不 in l.亇().iter(){let Some(一)=l.亵(不)else{continue;};let 乡=G::q(不).世()&丢[F::from(一)-1];for q in 乡.iter(){if 丆[q]!=0{丆[q]=0;主[丕]=q
as I;丕+=1;}}}}while 丕>0{let mut 三=0;for 与 in 1..丕{let(a,b)=(F::from(主[与]),F::from(主[三]));if(丆[a],a)<(丆[b],b){三=与;}}let q=F::from(主[三]);丕-=1;主[三]=主[丕];佪.set(q);for&丂
in&世[q]{if 丂==乙{break;}let 丂=F::from(丂);if 佪.test(丂)||丄.test(丂){continue;}let Some(一)=丢[..乕].iter().position(|丱|丱.test(丂))else{continue;};let 佰=丆[q]+10*(一
as C+1);if 佰<丆[丂]{if 丆[丂]==亃{主[丕]=丂 as I;丕+=1;}丆[丂]=佰;並[丂]=q as I;}}}亝{丆,並}}fn path(&self,丐:F)->K<F>{let mut path=K::new();if 丐>=M||self.丆[丐]==亃{return
path;}let mut q=丐;while self.並[q]!=乙{path.push(q);q=F::from(self.並[q]);}path.reverse();path}}struct 他{七:K<L>,丆:C,}fn 便(l:&J,z:F,q:F)->P<I>{(1..=l.丩().乩()).find(|&一|l.为(z,一).test(q))}fn
俐(丬:&mut J,丁:F,亡:F,严:C,七:&mut Vec<L>,习:&mut i64,)->Q{loop{if 丬.且()==Some(么::乧(丁)){return true;}if 丬.丟(丁).test(亡)&&丬.丄(丁).test(亡){return true;}let 丼=亝::new(丬,丁,false);let
丆=丼.丆[亡];if 丆==亃||*习+丆>严{return false;}let Some(&q)=丼.path(亡).first()else{return false;};let Some(一)=便(丬,丁,q)else{return false;};let 下=L::书{一,x:(q%O)as
I,y:(q/O)as I,};if!丬.丫(下){return false;}*习+=10*C::from(一);七.push(下);}}fn 伻(l:&J,丁:F,侷:&[F],严:C)->P<他>{let mut 丬=l.clone();let mut 七=K::new();let mut 习=0;for&亡
in 侷{if!俐(&mut 丬,丁,亡,严,&mut 七,&mut 习){return None;}}(习<=严).then_some(他{七,丆:习,})}fn 伈(l:&J,丐:F,严:C)->P<他>{if 严<0||l.且().is_some()||丐>=M||l.map().丶.test(丐){return
None;}let 丁=l.东();if l.丟(丁).test(丐)&&l.丄(丁).test(丐){return Some(他{七:K::new(),丆:0,});}let mut 三=伻(l,丁,&[丐],严);let 乊=亝::new(l,丁,false);let mut 乢:K<F> =(l.乢()&l.丟(1-丁)).iter().filter(|&乇|乇!=丐&&乊.丆[乇]!=亃).collect();乢.sort_by_key(|&乇|(l.map().久(乇,丐),乇));乢.truncate(侇);for
乇 in 乢{if let Some(丧)=伻(l,丁,&[乇,丐],严){if 三.as_ref().is_none_or(|乼|丧.丆<乼.丆){三=Some(丧);}}}三}const 來:F=20000;struct 仗{七:K<L>,丆:C,丌:C,丅:C,乎:C,丑:C,丛:C,}struct
仺{丌:C,丅:C,乎:C,}fn 佑(l:&J,z:F)->仺{let 丄=l.丄(z);let 丅=l.丅(z)&丄;let 业=丅.iter().filter_map(|q|l.亵(q)).map(|一|10*C::from(一)).sum();仺{丌:C::from(丄.count()),丅:业,乎:C::from(((l.丞()|l.乢())&丄).count()),}}struct
伲{丁:F,仵:G,並:仺,丑:C,产:F,严:C,享:F,path:K<L>,丨:K<仗>,index:HashMap<亻,F,BuildHasherDefault<DefaultHasher>>,}type 亻=[U;3];fn 俴(mut 乌:亻,q:F)->亻{乌[q/64]|=1<<(q%64);乌}impl
伲{fn 佈(&mut self,丬:&J,乵:P<F>,仱:亻,丆:C,)->Q{let j=self;let 丁=j.丁;let 乷=match 乵{None=>丬.五(丁),Some(q)=>G::q(q).世(),}&j.仵;let 乷=乷.两(丬.丟(丁));let mut 丢=[G::T;3];for
一 in 1..=丬.丩().乩(){丢[F::from(一)-1]=丬.为(丁,一);}for q in 乷.iter(){let 乕=F::from(丬.丩().乩());let Some(与)=丢[..乕].iter().position(|丱|丱.test(q))else{continue;};let
一=与 as I+1;let 予=10*C::from(一);if 丆+予>j.严{continue;}if j.享>=來{return false;}j.享+=1;let mut 丂=丬.clone();let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丂.丫(下){continue;}j.path.push(下);let
亱=俴(仱,q);j.俎(&丂,亱,丆+予);let 侭=j.path.len()<j.产&&丂.且().is_none();let 侶=!侭||j.佈(&丂,Some(q),亱,丆+予);j.path.pop();if!侶{return false;}}true}fn 俎(&mut self,丷:&J,亱:亻,丆:C){let
j=self;let 仈=1-j.丁;let 伛=佑(丷,仈);let 丌=j.並.丌-伛.丌;let 丅=j.並.丅-伛.丅;if 丌<=0&&丅<=0{return;}let 乎=j.並.乎-伛.乎;let 丑=丷.丑(j.丁)-j.丑;let 俷=丷.且()==Some(么::乧(j.丁));let
丛=if 俷{C::MAX/2}else{3*丌+丅+10*乎+2*丑-丆};if let Some(&丸)=j.index.get(&亱){if 丆>=j.丨[丸].丆{return;}}let 丝=仗{七:j.path.clone(),丆,丌,丅,乎,丑,丛,};match j.index.get(&亱){Some(&丸)=>j.丨[丸]=丝,None=>{j.index.insert(亱,j.丨.len());j.丨.push(丝);}}}}fn
但(l:&J,产:F,严:C,丿:F)->K<仗>{乍(l,产,严,丿).0}fn 乍(l:&J,产:F,严:C,丿:F)->(K<仗>,F){if l.且().is_some()||产==0||丿==0{return(K::new(),0);}let 丁=l.东();let 仈=1-丁;let mut
传=伲{丁,仵:l.丟(仈)|l.丟(仈).世(),並:佑(l,仈),丑:l.丑(丁),产,严,享:0,path:K::new(),丨:K::new(),index:HashMap::default(),};传.佈(l,None,[0;3],0);let 享=传.享;let mut 丨=传.丨;丨.sort_by(|a,b|b.丛.cmp(&a.丛).then(a.丆.cmp(&b.丆)));丨.truncate(丿);(丨,享)}const
仚:C=1000000;const 伭:C=200;fn 乹(l:&J,z:F)->C{let 丄=l.丄(z);let 丅:C=(l.丅(z)&丄).iter().filter_map(|q|l.亵(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.丑(z)+4*C::from(丄.count())+丅+10*C::from((l.乢()&丄).count())}fn
佣(l:&J,z:F)->C{let 丆=仟(l,z,false,Z[1-z]);if 丆==亃{return-伭;}(l.丈(z)+l.丑(z)-丆).clamp(-伭,0)}fn 侳(l:&J,z:F)->C{let 个=1-z;match l.且(){Some(么::乧(俳))=>return
if 俳==z{仚}else{-仚},Some(么::伪)=>return 0,None=>{}}let mut 丛=乹(l,z)-乹(l,个);if z==l.东(){let mut 丂=l.clone();丂.仁();if 丂.且().is_none()&&仟(&丂,个,true,Z[z])<=丂.丈(个){丛-=仚/2;}}丛+佣(l,z)-佣(l,个)}
