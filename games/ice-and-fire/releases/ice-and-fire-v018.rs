// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type C=usize;type I=u8;type 丱=u32;type U=u64;type 他=i32;type F=i64;type 亚=f64;type T=bool;type 丰=String;type 乑=Instant;type K<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侂:T=true;fn 俁(l:&J,q:C,仑:F,丯:G,並:F)->T{let
丁=l.东();let 乲=1-丁;let 佘=侂&&並<=l.丈(乲)+2*l.专(乲).max(0);let 价=G::q(q)|G::q(q).世();if!佘&&(价&丯).is_empty(){return false;}let(x,y)=亸(q);let mut 丸=l.clone();if!丸.个(L::亐{x,y}){return
false;}if 俱(&丸,丁)<仑{return true;}佘&&仡(&丸,乲,false,Z[丁])>並}const 來:C=4;const 伪:C=5;#[derive(Clone)]enum 乂{亼(L),份(K<L>),}impl 乂{fn 七(&self)->&[L]{match self{乂::亼(下)=>std::slice::from_ref(下),乂::份(七)=>七,}}}fn
亸(q:C)->(I,I){((q%O)as I,(q/O)as I)}fn 佒(l:&J,万:&mut Vec<乂>){万.clear();if l.丕().is_some(){return;}let 丁=l.东();let 乕=l.丨().乩();let mut 丢=[G::Q;4];for 一
in 1..=乕{丢[C::from(一)]=l.主(丁,一);}for 不 in l.予().iter(){let Some((丏,一))=l.仓(不)else{continue;};let 乡=G::q(不).世()&丢[C::from(一)];for 乫 in 乡.iter(){let(x,y)=亸(乫);万.push(乂::亼(L::亁{丏,x,y}));}}let
丈=l.丈(丁);let 们=l.井(丁);for 一 in 1..=乕{let 丆=10*F::from(一);if 丈<丆{break;}let mut 丌=们&丢[C::from(一)];for 俆 in 1..一{丌=丌.两(丢[C::from(俆)]);}for q in 丌.iter(){let(x,y)=亸(q);万.push(乂::亼(L::书{一,x,y}));}}if
丈>=亏{let 亓=l.丟(丁)&们&l.什();if l.丨().仝(){for q in(亓&l.map().临).iter(){let(x,y)=亸(q);万.push(乂::亼(L::乾{x,y}));}}if l.丨().伄(){let 侮=l.map().valid.两(l.丟(丁)).世();let
丌=(亓&侮).两(l.map().临);let 丯=丯(l,1-丁);let 仑=F::from((丯&l.丄(丁)).count());let 並=仡(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俁(l,q,仑,丯,並)){let(x,y)=亸(q);万.push(乂::亼(L::亐{x,y}));}}}for
丝 in 佊(l,伪,丈,來){if 丝.七.len()>1{万.push(乂::份(丝.七));}}if let Some(並)=伊(l,Z[1-丁],丈){if 並.七.len()>1{万.push(乂::份(並.七));}}}const 估:I=2;const 侞:F=5;fn 仈(l:&J,z:C)->G{let
mut 仈=G::Q;if 估>3{return 仈;}for q in(l.丅(z)&l.丄(z)).iter(){if l.亘(q).is_some_and(|一|一>=估){仈|=G::q(q);}}仈}fn 丯(l:&J,乎:C)->G{let 丫=l.丈(乎)+l.专(乎);俖(l,乎,仈(l,乎),丫)}fn
仃(l:&J,z:C)->G{丯(l,1-z)&l.丄(z)}fn 俱(l:&J,z:C)->F{F::from(仃(l,z).count())}const 侏:F=4;fn 俉(丌:F)->F{丌/侏}fn 丞(l:&J,z:C,仃:G)->F{let 丄=l.丄(z);let 丞=(l.丞()&丄).两(仃);俉(F::from(丄.count()))*F::from(丞.count())}const
侚:F=2;fn 佺(l:&J,z:C)->F{let 丄=l.丄(z);let[a,b,c]=丄.仴();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as C)&(伸-1);侦.with(|伈|{let mut 伈=伈.borrow_mut();let 仲=&mut 伈[与];if 仲.0==丄&&仲.1==z as I+1{return 仲.2;}let 业=俽(丄,Z[z]);*仲=(丄,z as I+1,业);业})}const
伸:C=1<<12;thread_local!{static 侦:std::cell::RefCell<K<(G,I,F)>> =std::cell::RefCell::new(vec![(G::Q,0,0);伸]);}fn 俽(丄:G,乪:C)->F{struct 亹{丄:G,乄:[I;M],丳:[I;M],size:[I;M],亍:I,乵:F,仫:F,}fn
乽(d:&mut 亹,v:C)->I{d.亍+=1;let 乄=d.亍;d.乄[v]=乄;let mut 丳=乄;let mut 伥=1u8;let mut 丝=0u8;for w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{let 佲=乽(d,w);伥+=佲;丳=丳.min(d.丳[w]);if
d.丳[w]>=乄{丝+=佲;}}else{丳=丳.min(d.乄[w]);}}d.丳[v]=丳;d.size[v]=伥;if 丝>0&&d.乄[v]>1{d.乵+=1;d.仫=d.仫.max(F::from(丝));}伥}if!丄.test(乪){return 0;}let mut d=亹{丄,乄:[0;M],丳:[0;M],size:[0;M],亍:0,乵:0,仫:0,};乽(&mut
d,乪);d.乵+d.仫}fn 俞(l:&J,z:C)->F{侚*(佺(l,1-z)-佺(l,z))}fn 佋(l:&J,z:C)->F{if l.丈(z)+l.专(z)>=0{return 0;}(l.丅(z)&l.丄(z)).iter().filter_map(|q|l.亘(q)).map(|一|10*F::from(一)).sum()}const
侅:F=15;const 侤:[F;2]=[3,1];const 侢:[F;2]=[4,20];fn 俭(l:&J,z:C)->[F;2]{let mut 丑=[0;2];for q in(l.丅(z)&l.丄(z)).iter(){match l.亘(q){Some(2)=>丑[0]+=1,Some(3)=>丑[1]+=1,_=>{}}}丑}fn
佱(l:&J,z:C)->F{let[佷,俲]=俭(l,z);let 伿=|一:C|(3-侤[一])*侢[一];-侅*佷+伿(0)*佷+伿(1)*俲}fn 休(l:&J,z:C)->F{let 丛=侺(l,z);if l.丕().is_some(){return 丛;}let 介=仃(l,z);let
乻=仃(l,1-z);let 仑=F::from(乻.count())-F::from(介.count());丛-2*佋(l,z)+2*佋(l,1-z)+俞(l,z)+佱(l,z)-佱(l,1-z)+侞*仑/4+丞(l,z,介)-丞(l,1-z,乻)}const 侎:C=8;const 侕:C=3;const
侇:C=32;const 侖:U=300;const 佾:亚=0.75;struct 亀{乺:P<乑>,五:P<乑>,仆:U,}#[derive(Default)]struct 伶{七:K<L>,伎:C,乳:U,丗:C,丮:C,佄:C,乴:C,}const 侥:T=true;#[derive(Clone)]struct
乚{七:K<L>,伐:F,}struct 亜{七:K<L>,end:J,first:P<U>,丒:P<K<乚>>,}type 人=HashMap<U,F,BuildHasherDefault<DefaultHasher>>;struct 仿{丁:C,limit:亀,乳:U,仅:人,亖:P<K<乂>>,}fn
伆(l:&mut J,七:&[L])->T{七.iter().all(|&下|l.个(下))}fn 乌(七:&[L])->U{let mut hash:U=0xcbf29ce484222325;for 下 in 七{let(仧,a,b,c)=match*下{L::亁{丏,x,y}=>(1,U::from(丏),x,y),L::书{一,x,y}=>(2,U::from(一),x,y),L::乾{x,y}=>(3,0,x,y),L::亐{x,y}=>(4,0,x,y),};let
亷=仧|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亷).wrapping_mul(0x0100000001b3);}hash}fn 似(乔:&J,亊:&[L])->J{let mut 丂=乔.clone();丂.仁();for&下 in 亊{丂.个(下);}丂.仁();丂}fn
仩(l:&J)->P<K<L>>{let 丁=l.东();let 丐=Z[1-丁];let 亶=|丠:&J|丠.丕()==Some(么::乧(丁));for q in l.予().iter(){if l.map().久(q,丐)!=1{continue;}let Some((丏,_))=l.仓(q)else{continue;};let(x,y)=亸(丐);let
下=L::亁{丏,x,y};let mut 丠=l.clone();if 丠.个(下)&&亶(&丠){return Some(vec![下]);}}let 並=伊(l,丐,l.丈(丁))?;let mut 丠=l.clone();(伆(&mut 丠,&並.七)&&亶(&丠)).then_some(並.七)}impl
仿{fn 习(&self)->T{self.乳>=self.limit.仆||self.limit.乺.is_some_and(|t|乑::now()>=t)}fn 丛(&mut self,乔:&J,丗:&[乚],伇:F)->F{self.乳+=1;self.传(乔,丗,伇)}fn 传(&self,乔:&J,丗:&[乚],伇:F)->F{if
乔.丕()==Some(么::乧(self.丁)){return 仜;}if 丗.is_empty(){return 休(乔,self.丁);}let mut 仵=F::MAX;for 亊 in 丗{仵=仵.min(休(&似(乔,&亊.七),self.丁));if 仵<=伇{break;}}仵}#[allow(clippy::too_many_arguments)]fn
俐(&mut self,买:&J,亠:F,乓:&[乂],亮:&[(F,C)],为:&mut[T],丗:&[乚],亡:&mut 人,众:T,)->(P<(C,J,F)>,T){let mut 三=None;let mut 仞=亠;for&(仭,与)in 亮{if 为[与]{continue;}if 众&&仭!=F::MAX&&仭<=仞-亠{return(三,true);}if
self.习(){return(三,false);}为[与]=true;let 七=乓[与].七();let mut 丂=买.clone();if!伆(&mut 丂,七){亡.insert(乌(七),F::MIN/4);continue;}let 丛=self.丛(&丂,丗,仞);亡.insert(乌(七),丛-亠);if
丛>仞{仞=丛;三=Some((与,丂,丛));}}(三,false)}fn 伍(&mut self,乪:&J,丗:&[乚],亽:&[U],侰:T)->亜{let j=self;let mut 乊=None;let mut 买=乪.clone();let mut 七=K::new();let mut
亠=j.丛(&买,丗,F::MIN);let mut 亡=人::default();let mut 乓=K::new();let mut 亮=K::new();while 买.丕().is_none()&&!j.习(){if 七.is_empty()&&侰{match&j.亖{Some(亖)=>乓.clone_from(亖),None=>{佒(&买,&mut
乓);j.亖=Some(乓.clone());}}}else{佒(&买,&mut 乓);}亮.clear();for(与,乮)in 乓.iter().enumerate(){let k=乌(乮.七());let 仭=亡.get(&k).or(j.仅.get(&k)).copied();亮.push((仭.unwrap_or(F::MAX),与));}亮.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 为=vec![false;乓.len()];if 七.is_empty(){for(与,乮)in 乓.iter().enumerate(){为[与]=亽.contains(&乌(乮.七()));}}let mut 三=None;for
众 in[true,false]{let(丧,俦)=j.俐(&买,亠,&乓,&亮,&mut 为,丗,&mut 亡,众,);三=丧;if 三.is_some()||!俦{break;}}let Some((与,丂,丛))=三 else{break;};if 七.is_empty(){乊=Some(乌(乓[与].七()));}七.extend_from_slice(乓[与].七());买=丂;亠=丛;}for(k,乨)in
亡{let 佔=j.仅.entry(k).or_insert(乨);*佔=(*佔).max(乨);}亜{七,end:买,first:乊,丒:None,}}fn 俚(&self,乔:&J)->K<K<L>>{let mut 丂=乔.clone();丂.仁();let mut 万=K::new();if
丂.丕().is_some(){return 万;}let 乜=丂.东();let 丈=丂.丈(乜);if let Some(並)=伊(&丂,Z[self.丁],丈){万.push(並.七);}for 丝 in 佊(&丂,伪,丈,侕){万.push(丝.七);}let mut 体=K::new();俀(&丂,&mut
体);万.push(体);let mut 乻=仿{丁:乜,limit:亀{仆:侖,..self.limit},乳:0,仅:人::default(),亖:None,};万.push(乻.伍(&丂,&[],&[],true).七);万.retain(|七|!七.is_empty());万}fn 伽(&self,乔:&J)->K<乚>{self.俚(乔).into_iter().map(|七|{let
伐=休(&似(乔,&七),self.丁);乚{七,伐}}).collect()}fn 乴(&mut self,乪:&J,乮:&乂,丗:&[乚])->P<亜>{let mut 丸=乪.clone();if!伆(&mut 丸,乮.七()){return None;}self.乳+=1;let 乖=self.伍(&丸,丗,&[],false);let
mut 七=乮.七().to_vec();七.extend_from_slice(&乖.七);Some(亜{七,end:乖.end,first:Some(乌(乮.七())),丒:None,})}fn 住(&self,且:&亜,丗:&[乚])->F{let 丛=self.传(&且.end,丗,F::MIN);match&且.丒{Some(丒)if!丒.is_empty()=>丛.min(self.传(&且.end,丒,F::MIN)),_=>丛,}}}fn
俘(丧:&[乚],丗:&mut Vec<乚>)->T{let 严:K<_> =丗.iter().map(|r|r.七.clone()).collect();for 亊 in 丧{if 丗.iter().any(|r|r.七==亊.七){continue;}丗.push(亊.clone());}丗.sort_by_key(|亊|亊.伐);丗.truncate(侎);丗.iter().map(|r|&r.七).ne(&严)}fn
俰(l:&J,丿:亀,亭:&mut 亂)->伶{let 丁=l.东();let mut 中=伶::default();if l.丕().is_some(){return 中;}if let Some(七)=仩(l){中.七=七;return 中;}let 丹=|俠:亚|match(丿.乺,丿.五){(Some(乺),Some(五))=>Some(乺+(五-乺).mul_f64(俠)),_=>None,};if
亭.俓(l){中.七=亭.上(l,丹(0.4));return 中;}let mut 乍=仿{丁,limit:亀{乺:丹(佾),..丿},乳:0,仅:人::default(),亖:None,};let mut 丗=K::new();let mut 丮:K<亜> =K::new();let mut 亽=K::new();let
mut 仢=false;let mut 乴:K<乂> =K::new();while 中.伎<侇&&(丮.is_empty()||!乍.习()){let 且=if 仢{let Some(乮)=乴.pop()else{break;};match 乍.乴(l,&乮,&丗){Some(且)=>且,None=>continue,}}else{乍.伍(l,&丗,&亽,true)};中.伎+=1;let
佬=丮.iter().any(|p|p.七==且.七);let 乊=且.first;let 佰=乍.习();let mut 且=且;let mut 佞=false;if!佰&&!佬{let 丧=乍.伽(&且.end);佞=俘(&丧,&mut 丗);且.丒=Some(丧);}if!佬{丮.push(且);}if
佰{break;}if!佞&&!仢{match 乊{Some(乊)=>亽.push(乊),None=>{仢=true;乴=乍.亖.clone().unwrap_or_default();乴.retain(|a|!亽.contains(&乌(a.七())));let 乨=|a:&乂|{乍.仅.get(&乌(a.七())).copied().unwrap_or(F::MIN)};乴.sort_by_key(乨);}}}}中.乴=C::from(仢);中.乳=乍.乳;中.丗=丗.len();中.丮=丮.len();let
佚=||丿.五.is_some_and(|t|乑::now()>=t);let mut 亱:K<P<F>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佚(){break;}亱[与]=Some(乍.住(&丮[与],&丗));}let
亯=|亱:&[P<F>]|{let mut 三:P<(F,C)> =None;for(与,丛)in 亱.iter().enumerate().rev(){if let Some(丛)=*丛{if 三.is_none_or(|(乼,_)|丛>乼){三=Some((丛,与));}}}三};let mut
三=亯(&亱);乍.limit.乺=丿.五;while let Some((_,与))=三{if!侥||丮[与].丒.is_some()||佚(){break;}let 丧=乍.伽(&丮[与].end);丮[与].丒=Some(丧);中.佄+=1;亱[与]=Some(乍.住(&丮[与],&丗));三=亯(&亱);}if
let Some((_,与))=三{let mut 丬=l.clone();for&下 in&丮[与].七{if 丬.个(下){中.七.push(下);}}}中}fn 俒(丿:&dyn Fn(乑,丱)->亀,mut 亭:亂,伖:impl BufRead,仮:impl Write,)->Result<(),丰>{丽(伖,仮,|亥,乗,上,亗|match
J::read(亥,乗,乤::仹,上){Ok(l)=>{let 中=俰(&l,丿(亗,上),&mut 亭);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,中.伎,中.乳,中.丗,中.佄,中.乴,亗.elapsed());侫(&中.七)}Err(互)=>{eprintln!("plan: cannot read the input: {}",互);"WAIT".to_string()}},)}const
佼:亚=40.0;const 使:亚=800.0;const 供:亚=0.6;fn 丿(亗:乑,上:丱)->亀{if let Some(仆)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亀{乺:None,五:None,仆,};}let 仱=俳().min(100.0);let 俋=if 上==1{使}else{佼};let 丫=Duration::from_secs_f64(俋/1000.0*仱);亀{乺:Some(亗+丫.mul_f64(供)),五:Some(亗+丫),仆:U::MAX,}}const
侔:C=7;const 侒:些=些{丌:3,专:1,丈:1,丝:2,乵:2,仒:2,丯:6,仔:1,};const 侓:I=I::MAX;fn 亭()->亂{俎(64)}fn 俎(乣:C)->亂{let 乹=亝::仛{丽:侔,乣,乐:侒,};let 丙=侙.iter().map(|&乞|Y{伣:侓,..乞}).collect();亂::new(乹,丙,Y::企)}fn
main(){let 俪=io::stdin();let 俫=io::stdout();if let Err(互)=俒(&丿,亭(),俪.lock(),俫.lock()){eprintln!("plan: {}",互);std::process::exit(1);}}const 侟:&str="CG_TIME_SCALE";fn
俳()->亚{俏(std::env::var(侟).ok().as_deref())}fn 俏(业:P<&str>)->亚{业.and_then(|乗|乗.trim().parse::<亚>().ok()).filter(|仱|仱.is_finite()&&*仱>0.0).unwrap_or(1.0)}const
O:C=12;const M:C=O*O;const 乒:C=4*O;const 乥:U=(1<<乒)-1;const 仗:U=1|1<<12|1<<24|1<<36;const 仺:U=仗<<11;const 伴:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const Q:G=G([0;3]);const 佻:G=G([乥;3]);#[inline]fn q(q:C)->G{let mut 串=G::Q;串.set(q);串}#[inline]fn test(self,q:C)->T{(self.0[q/乒]>>(q%乒))&1!=0}#[inline]fn
set(&mut self,q:C){self.0[q/乒]|=1<<(q%乒);}#[inline]fn clear(&mut self,q:C){self.0[q/乒]&=!(1<<(q%乒));}#[inline]fn is_empty(self)->T{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仴(self)->[U;3]{self.0}#[inline]fn count(self)->丱{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 两(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仸{仸{会:self.0,丹:0,}}#[inline]fn 俥(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伴)<<(乒-O)),(b>>O)|((c&伴)<<(乒-O)),c>>O,])}#[inline]fn 俢(self)->G{let[a,b,c]=self.0;G([(a<<O)&乥,((b<<O)&乥)|(a>>(乒-O)),((c<<O)&乥)|(b>>(乒-O)),])}#[inline]fn
俣(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仺&乥,(b>>1)&!仺&乥,(c>>1)&!仺&乥,])}#[inline]fn 俤(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仗&乥,(b<<1)&!仗&乥,(c<<1)&!仗&乥,])}#[inline]fn
世(self)->G{let j=self;j.俥()|j.俢()|j.俣()|j.俤()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::佻.两(self)}}struct 仸{会:[U;3],丹:C,}impl Iterator for 仸{type Item=C;#[inline]fn
next(&mut self)->P<C>{let j=self;while j.丹<3{let 仪=j.会[j.丹];if 仪!=0{j.会[j.丹]=仪&(仪-1);return Some(j.丹*乒+仪.trailing_zeros()as C);}j.丹+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{亁{丏:丱,x:I,y:I},书{一:I,x:I,y:I},乾{x:I,y:I},亐{x:I,y:I},}impl L{fn 俵(&self)->丰{match*self{L::亁{丏,x,y}=>format!("MOVE {} {x} {y}",丏),L::书{一,x,y}=>format!("TRAIN {} {x} {y}",一),L::乾{x,y}=>format!("BUILD MINE {x} {y}"),L::亐{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
侫(七:&[L])->丰{if 七.is_empty(){return "WAIT".to_string();}let mut line=丰::with_capacity(16*七.len());for(与,下)in 七.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&下.俵());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乤{伹,倃,侧,#[default]仹,}impl 乤{fn 乩(self)->I{match self{乤::伹=>1,_=>3,}}fn 仝(self)->T{matches!(self,乤::侧|乤::仹)}fn 伄(self)->T{self==乤::仹}}const 乙:I=I::MAX;struct
乭{丷:G,valid:G,临:G,久:Box<[I]>,世:[[[I;4];M];2],}thread_local!{static 侄:RefCell<P<Arc<乭>>> =const{RefCell::new(None)};}impl 乭{fn 信(丷:G,临:G)->Arc<乭>{侄.with(|乶|{let
mut 乶=乶.borrow_mut();if let Some(产)=乶.as_ref(){if 产.丷==丷&&产.临==临{return Arc::clone(产);}}let 产=Arc::new(乭::build(丷,临));*乶=Some(Arc::clone(&产));产})}fn build(丷:G,临:G)->乭{let
乬=!丷;const 侜:[[(他,他);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 世=[[[乙;4];M];2];for(z,乡)in 侜.iter().enumerate(){for q in
乬.iter(){let(x,y)=((q%O)as 他,(q/O)as 他);let mut 丑=0;for&(侷,侸)in 乡{let(你,佡)=(x+侷,y+侸);if(0..O as 他).contains(&你)&&(0..O as 他).contains(&佡){let 丂=佡 as C*O+你
as C;if 乬.test(丂){世[z][q][丑]=丂 as I;丑+=1;}}}}}let mut 久=vec![乙;M*M].into_boxed_slice();let mut 伟=[0u8;M];for 不 in 乬.iter(){let 事=&mut 久[不*M..(不+1)*M];事[不]=0;伟[0]=不
as I;let(mut 伓,mut 伧)=(0,1);while 伓<伧{let q=伟[伓]as C;伓+=1;for&丂 in&世[0][q]{if 丂!=乙&&事[丂 as C]==乙{事[丂 as C]=事[q]+1;伟[伧]=丂;伧+=1;}}}}乭{丷,valid:乬,临,久,世,}}#[inline]fn
久(&self,不:C,乫:C)->I{self.久[不*M+乫]}}const 侍:丱=100;const 亏:F=15;const 侣:[F;3]=[1,4,20];const Z:[C;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乧(C),伬,}enum
乘{仾,伱,伲,}#[derive(Clone)]struct J{map:Arc<乭>,丨:乤,丒:[G;2],丄:[G;2],乆:G,乇:G,丅:[G;2],一:[G;3],亇:G,亵:[丱;M],丈:[F;2],专:[F;2],东:C,了:丱,亪:丱,丕:P<么>,}fn 仄(丒:G,交:C)->G{if!丒.test(交){return
G::Q;}let mut 仏=G::q(交);loop{let 丂=(仏|仏.世())&丒;if 丂==仏{return 仏;}仏=丂;}}impl J{fn 东(&self)->C{self.东}fn 了(&self)->丱{self.了}fn 丕(&self)->P<么>{self.丕}fn 丨(&self)->乤{self.丨}fn
亩(&self,z:C)->F{let j=self;let 丅=j.丅[z];j.丈[z]+10*F::from((丅&j.一[0]).count())+20*F::from((丅&j.一[1]).count())+30*F::from((丅&j.一[2]).count())}fn 丈(&self,z:C)->F{self.丈[z]}fn
专(&self,z:C)->F{self.专[z]}fn 伨(&self,z:C)->C{self.丅[z].count()as C}fn map(&self)->&乭{&self.map}fn 丟(&self,z:C)->G{self.丒[z]}fn 丄(&self,z:C)->G{self.丄[z]}fn
丅(&self,z:C)->G{self.丅[z]}fn 仓(&self,q:C)->P<(丱,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亵[q],j.伙(q)))}fn 予(&self)->G{self.丅[self.东]&self.亇}fn
丞(&self)->G{self.乆}fn 乢(&self)->G{self.乇}fn 亘(&self,q:C)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伙(q))}fn 俩(&mut self,z:C){let j=self;j.丄=[仄(j.丒[0],Z[0]),仄(j.丒[1],Z[1])];let
侳=j.丅[z].两(j.丄[z]);j.伡(侳);j.佅();j.丈[z]+=j.专[z];if j.丈[z]<0{j.丈[z]=0;j.伡(j.丅[z]);}j.亇|=j.丅[z];}fn 佅(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
丅=丄&j.丅[z];let mut 专=F::from(丄.count())+4*F::from((丄&j.乆).count());for(一,俻)in 侣.iter().enumerate(){专-=俻*F::from((丅&j.一[一]).count());}j.专[z]=专;}}fn 仁(&mut
self){let j=self;if j.丕.is_some(){return;}if j.东==1{if j.了>=侍{j.丕=Some(match j.亩(0).cmp(&j.亩(1)){std::cmp::Ordering::Greater=>么::乧(0),std::cmp::Ordering::Less=>么::乧(1),std::cmp::Ordering::Equal=>么::伬,});return;}j.了+=1;}j.东=1-j.东;j.俩(j.东);}fn
佦(&mut self,七:&[L]){let j=self;if j.丕.is_some(){return;}let 伒=j.亪;for&下 in 七{if j.侶(下,伒){continue;}j.个(下);if j.丕.is_some(){return;}}j.仁();}fn 侶(&self,下:L,伒:丱)->T{let
j=self;match 下{L::亁{丏,x,y}=>{j.亟(x,y).is_none()||丏>=伒||j.佐(j.东,丏).is_none()}L::书{一,x,y}=>{j.亟(x,y).is_none()||一==0||一>j.丨.乩()}L::乾{x,y}|L::亐{x,y}=>{j.亟(x,y).is_none()}}}#[inline]fn
亟(&self,x:I,y:I)->P<C>{let(x,y)=(C::from(x),C::from(y));(x<O&&y<O&&!self.map.丷.test(y*O+x)).then_some(y*O+x)}fn 佐(&self,z:C,丏:丱)->P<C>{self.丅[z].iter().find(|&q|self.亵[q]==丏)}#[inline]fn
伙(&self,q:C)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 侭(&self,乜:C)->G{let 乢=self.乇&self.丒[乜];let 丄=self.丄[乜];乢|(丄&(丄&乢).世())}fn
主(&self,z:C,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乜=1-z;let 丅=j.丅[0]|j.丅[1];let 侹=j.什();let 俹=match
一{1 if j.丨==乤::伹=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::Q,};let 主=侹|j.丒[乜].两(俹);if 一<3{主.两(j.侭(乜))}else{主}}fn 井(&self,z:C)->G{self.丄[z]|self.丄[z].世()}fn
什(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 促=G::q(Z[0])|G::q(Z[1]);j.map.valid.两(丅|j.乆|j.乇|促)}fn 个(&mut self,下:L)->T{let j=self;if j.丕.is_some(){return
false;}let z=j.东;let 云=match 下{L::书{一,x,y}=>j.俷(z,一,x,y),L::亁{丏,x,y}=>j.俊(z,丏,x,y),L::乾{x,y}=>j.build(z,true,x,y),L::亐{x,y}=>j.build(z,false,x,y),};let
Some(云)=云 else{return false;};match 云{乘::仾=>{}乘::伱=>j.丄[z]=仄(j.丒[z],Z[z]),乘::伲=>{j.丄=[仄(j.丒[0],Z[0]),仄(j.丒[1],Z[1])];}}j.佅();if!j.丒[0].test(Z[0]){j.丕=Some(么::乧(1));}else
if!j.丒[1].test(Z[1]){j.丕=Some(么::乧(0));}true}fn 伡(&mut self,丌:G){let j=self;j.丅[0]=j.丅[0].两(丌);j.丅[1]=j.丅[1].两(丌);for 一 in&mut j.一{*一=一.两(丌);}j.亇=j.亇.两(丌);}fn
佳(&mut self,z:C,q:C)->乘{let j=self;j.伡(G::q(q));j.乆.clear(q);j.乇.clear(q);let 乜=1-z;if j.丒[乜].test(q){j.丒[乜].clear(q);j.丒[z].set(q);乘::伲}else if j.丒[z].test(q){乘::仾}else{j.丒[z].set(q);乘::伱}}fn
俷(&mut self,z:C,一:I,x:I,y:I)->P<乘>{let j=self;if 一==0||一>j.丨.乩(){return None;}let q=j.亟(x,y)?;let 丆=10*F::from(一);if j.丈[z]<丆||!j.井(z).test(q)||!j.主(z,一).test(q){return
None;}let 云=j.佳(z,q);j.佤(z,一,q,j.亪);j.亪=j.亪.saturating_add(1);j.丈[z]-=丆;Some(云)}fn 佤(&mut self,z:C,一:I,q:C,丏:丱){self.丅[z].set(q);self.一[C::from(一)-1].set(q);self.亵[q]=丏;}fn
俊(&mut self,z:C,丏:丱,x:I,y:I)->P<乘>{let j=self;let 丐=j.亟(x,y)?;let 不=j.佐(z,丏)?;if!j.亇.test(不){return None;}let 一=j.伙(不);let 丢=j.主(z,一);let 举=j.map.世[z][不];if
举.contains(&(丐 as I))&&!丢.test(丐){return None;}let mut 三=j.map.久(不,丐);let mut 亾=None;for&丂 in 举.iter().take_while(|&&丂|丂!=乙){let 丂=C::from(丂);let 久=j.map.久(丂,丐);if
久<三&&丢.test(丂){三=久;亾=Some(丂);}}let 亾=亾?;let 云=j.佳(z,亾);j.丅[z].clear(不);j.一[C::from(一)-1].clear(不);j.亇.clear(不);j.佤(z,一,亾,丏);Some(云)}fn build(&mut self,z:C,乆:T,x:I,y:I)->P<乘>{let
j=self;let 侪=if 乆{j.丨.仝()}else{j.丨.伄()};let q=j.亟(x,y)?;let 俌=j.map.临.test(q);if!侪||j.丈[z]<亏||俌!=乆||!j.丒[z].test(q)||!j.井(z).test(q)||!j.什().test(q){return
None;}if 乆{j.乆.set(q);}else{j.乇.set(q);}j.丈[z]-=亏;Some(乘::仾)}}const 侉:F=1000000000;const 任:F=0;const 侐:F=1;const 侠:F=2;impl J{fn read(亥:&str,上:&str,丨:乤,了:丱,)->Result<J,丰>{let
mut 临=K::new();let mut 丣=亥.lines();let 丑=伏(&mut 丣,"mine spot count")?;for _ in 0..丑{let[x,y]=乷(享(&mut 丣,"a mine spot")?,"a mine spot")?;临.push((仠(x)?,仠(y)?));}位(丣,"the initial input")?;let
mut 丣=上.lines();let mut 伔=[0i64;4];for(业,丵)in 伔.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伞]=乷(享(&mut 丣,丵)?,丵)?;if 伞.abs()>侉{return
Err(format!("{}: {} is out of range",丵,伞));}*业=伞;}let mut 丷=G::Q;let mut 丌=[G::Q;2];let mut 丄=[G::Q;2];for y in 0..O{let 事=享(&mut 丣,"a map row")?.trim();if
事.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,事));}for(x,仐)in 事.bytes().enumerate(){let q=y*O+x;match 仐{b'#'=>丷.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乀=C::from(仐==b'X'||仐==b'x');丌[乀].set(q);if 仐.is_ascii_uppercase(){丄[乀].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仐 as char))}}}}let
侯=伏(&mut 丣,"building count")?;let mut 乏=K::new();for _ in 0..侯{let 乷=乷(享(&mut 丣,"a building")?,"a building")?;乏.push(乷);}let 伨=伏(&mut 丣,"unit count")?;let
mut 丅=K::new();for _ in 0..伨{let 乷=乷(享(&mut 丣,"a unit")?,"a unit")?;丅.push(乷);}位(丣,"the turn's input")?;let 佣=乏.iter().find(|佁|佁[0]==0&&佁[1]==任).ok_or("the bot's own HQ is not among the buildings")?;let
佗=伉(佣[2],佣[3],丷)?;let z=Z.iter().position(|&交|交==佗).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佗))?;let mut 佮=G::Q;for(x,y)in 临{if
丷.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佮.set(y*O+x);}let 伂=|乀:C|if 乀==0{z}else{1-z};let mut l=J{map:乭::信(丷,佮),丨,丒:[G::Q;2],丄:[G::Q;2],乆:G::Q,乇:G::Q,丅:[G::Q;2],一:[G::Q;3],亇:G::Q,亵:[0;M],丈:[0;2],专:[0;2],东:z,了,亪:1,丕:None,};for
乀 in 0..2{let 九=伂(乀);l.丒[九]=丌[乀];l.丄[九]=丄[乀];l.丈[九]=伔[2*乀];l.专[九]=伔[2*乀+1];}let mut 佢=G::Q;for[九,仧,x,y]in 乏{let 乀=佩(九)?;let q=伉(x,y,丷)?;if!丌[乀].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佢.test(q){return Err(format!("two buildings at ({x}, {y})"));}佢.set(q);match 仧{任
if q==Z[伂(乀)]=>{}任=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侐=>l.乆.set(q),侠=>l.乇.set(q),_=>return Err(format!("unknown building type {}",仧)),}}let
mut 乁=G::Q;let mut 仨=0u32;for[九,丏,一,x,y]in 丅{let 乀=佩(九)?;let q=伉(x,y,丷)?;if!丌[乀].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乁.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丏=丱::try_from(丏).map_err(|_|format!("unit id {} is out of range",丏))?;if
乁.iter().any(|丩|l.亵[丩]==丏){return Err(format!("two units have id {}",丏));}乁.set(q);l.丅[伂(乀)].set(q);l.一[一 as C-1].set(q);l.亵[q]=丏;仨=仨.max(丏);}if!乁.is_empty(){l.亪=仨.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仨))?;}l.亇=l.丅[z];Ok(l)}}fn
享<'a>(丣:&mut Lines<'a>,丵:&str)->Result<&'a str,丰>{丣.next().ok_or_else(||format!("the text ends before {}",丵))}fn 位(mut 丣:Lines<'_>,丵:&str)->Result<(),丰>{match
丣.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丵)),None=>Ok(()),}}fn 乷<const N:C>(line:&str,丵:&str)->Result<[F;N],丰>{let
mut 仴=line.split_whitespace();let mut 佹=[0i64;N];for 业 in&mut 佹{let 亷=仴.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丵))?;*业=亷.parse().map_err(|_|format!("{}: {:?} is not an integer",丵,亷))?;}if
仴.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丵));}Ok(佹)}fn 伏(丣:&mut Lines<'_>,丵:&str)->Result<C,丰>{let[丑]=乷(享(丣,丵)?,丵)?;C::try_from(丑).ok().filter(|&丑|丑<=M).ok_or_else(||format!("{}: {} is out of range",丵,丑))}fn
仠(业:F)->Result<C,丰>{C::try_from(业).ok().filter(|&业|业<O).ok_or_else(||format!("coordinate {} is off the map",业))}fn 伉(x:F,y:F,丷:G)->Result<C,丰>{let q=仠(y)?*O+仠(x)?;if
丷.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佩(九:F)->Result<C,丰>{match 九{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",九)),}}const
侁:F=15;const 侑:F=6;const 伷:F=10;const 侘:I=3;#[derive(Clone,Copy)]struct Y{open:F,仟:F,佈:I,伕:F,佖:F,佛:F,伣:I,佇:F,但:F,伻:F,亣:F,亳:F,丶:F,亦:F,于:F,}impl Y{const
企:Y=Y{open:3,仟:2,佈:12,伕:4,佖:侁,佛:侑,伣:侘,佇:2,但:2,伻:0,亣:0,亳:0,丶:0,亦:0,于:0,};const fn 倁(self,亣:F,亳:F,丶:F)->Y{Y{亣,亳,丶,..self}}const fn 倀(self,亦:F,于:F)->Y{Y{亦,于,..self}}}fn
仕(q:C)->(I,I){((q%O)as I,(q/O)as I)}fn 亿(l:&J,不:G)->[I;M]{let mut 义=[I::MAX;M];let 乬=l.map().valid;let mut 之=不&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn 侽(l:&J)->I{let 义=亿(l,l.丟(0));l.丟(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亃<'a>{乞:&'a Y,丁:C,theirs:[I;M],亴:[I;M],free:G,乎:G,ours:G,临:G,}impl 亃<'_>{fn new<'a>(l:&J,乞:&'a Y)->亃<'a>{let 丁=l.东();let 乆=亿(l,l.丟(丁));let 乻=亿(l,l.丟(1-丁));let
亴=亿(l,G::q(Z[1-丁]));let 亓=l.map().valid.两(l.丟(0)|l.丟(1));let mut 乎=G::Q;for q in 亓.iter(){if 乆[q]<=乻[q]{乎.set(q);}}亃{乞,丁,theirs:乻,亴,free:亓,乎,ours:l.丟(丁),临:l.map().临,}}fn
value(&self,q:C)->F{let j=self;let p=j.乞;let 丼=F::from((G::q(q).世()&j.free).count());let 佃=p.佈;let 仟=F::from(佃)-F::from(j.theirs[q].min(佃));let 交=if p.伕>0{F::from(j.亴[q].min(30))/p.伕}else{0};let
丒=F::from((G::q(q).世()&j.ours).count());let line=if 丒<=1{p.亳}else{0};let 丶=if j.临.test(q){p.丶}else{0};let 修=if 丼==0{p.亦}else{0};let 于=p.于*(22-F::from(j.亴[q].min(22)));p.open*丼+p.仟*仟-交+p.亣*丒-line+丶-修+于}fn
三(&self,丌:G)->P<C>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伌(&mut self,q:C){self.free.clear(q);self.乎.clear(q);self.ours.set(q);}}fn
仍(l:&mut J,下:L,万:&mut Vec<L>)->T{let 伺=l.个(下);if 伺{万.push(下);}伺}fn 丘(l:&mut J,不:C,乫:C,万:&mut Vec<L>)->T{let Some((丏,_))=l.仓(不)else{return false;};let(x,y)=仕(乫);仍(l,L::亁{丏,x,y},万)}fn
俺(h:F,k:F)->F{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伷}#[derive(Clone,Copy)]enum 乃{仚,乿{丞:I,丅:I},}fn 佶(l:&J,乞:&Y,亅:乃)->K<L>{let mut 丬=l.clone();let
mut 上=亃::new(l,乞);let mut 万=K::new();乛(&mut 丬,&mut 上,&mut 万);match 亅{乃::仚=>俕(&mut 丬,&mut 上,&mut 万),乃::乿{丞,丅}=>{侻(&mut 丬,&mut 上,丞,丅,&mut 万)}}万}fn 乛(l:&mut
J,上:&mut 亃<'_>,万:&mut Vec<L>){let 丁=上.丁;let mut 以:K<C> =K::new();loop{let 丢=l.主(丁,1);let mut 亯:P<(丱,C)> =None;for 不 in l.予().iter(){if 以.contains(&不){continue;}let
乸=(G::q(不).世()&上.free&丢).count();if 乸==0{以.push(不);}else if 亯.is_none_or(|(n,_)|乸<n){亯=Some((乸,不));}}let Some((_,不))=亯 else{break;};match 上.三(G::q(不).世()&上.free&丢){Some(乫)if
丘(l,不,乫,万)=>上.伌(乫),_=>以.push(不),}}for 不 in 以{if!l.予().test(不){continue;}let 丐=[上.乎,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().久(不,q),q))});if
let Some(丐)=丐{丘(l,不,丐,万);}}}fn 俕(l:&mut J,上:&mut 亃<'_>,万:&mut Vec<L>){let 丁=上.丁;let p=上.乞;let h=(p.佖-F::from(l.了())).max(p.佛);let 俴=(F::from(侽(l))-p.佇).max(0)/p.但.max(1)+p.伻;loop{let
丈=l.丈(丁);let 丅=l.伨(丁)as F;let 俛=F::from(上.乎.count());let q=上.三(l.井(丁)&上.free&l.主(丁,1));let 佸=俺(h,(俛/(丅+1)).min(俴));let 佂=q.is_some()&&丈>=伷&&佸>0;let 作=4*h-亏;let
丶=佝(l,上);let 侱=丶.is_some()&&丈>=亏&&作>0;if 侱&&(!佂||作>=佸){let(x,y)=仕(丶.unwrap_or_default());if 仍(l,L::乾{x,y},万){continue;}break;}let Some(q)=q.filter(|_|佂)else{break;};let(x,y)=仕(q);if!仍(l,L::书{一:1,x,y},万){break;}上.伌(q);}}fn
佝(l:&J,上:&亃<'_>)->P<C>{let 丁=上.丁;(l.map().临&l.丄(丁)&l.什()).iter().filter(|&丶|上.theirs[丶]>上.乞.伣).min_by_key(|&丶|(l.map().久(Z[丁],丶),丶))}fn 侻(l:&mut J,上:&mut
亃<'_>,丞:I,丅:I,万:&mut Vec<L>,){let 丁=上.丁;for _ in 0..丞{let Some(丶)=佝(l,上)else{break;};let(x,y)=仕(丶);if!仍(l,L::乾{x,y},万){break;}}for _ in 0..丅{let Some(q)=上.三(l.井(丁)&上.free&l.主(丁,1))else{break;};let(x,y)=仕(q);if!仍(l,L::书{一:1,x,y},万){break;}上.伌(q);}}fn
仰(l:&J)->T{if l.丕().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丯=亞::new(l,丁,true);l.丟(1-丁).iter().any(|q|丯.丆[q]<=丈)}#[derive(Clone,Copy)]struct
両{亙:I,亅:乃,}const 伩:I=I::MAX;const 佽:[乃;8]=[乃::仚,乃::乿{丞:0,丅:0},乃::乿{丞:0,丅:1},乃::乿{丞:0,丅:2},乃::乿{丞:0,丅:伩,},乃::乿{丞:1,丅:0},乃::乿{丞:1,丅:1},乃::乿{丞:1,丅:伩,},];struct
伫<'a>{丙:&'a[Y],仇:Y,rest:両,乱:P<乑>,俜:Settings,}impl 伫<'_>{fn 乞(&self,丘:両)->&Y{&self.丙[C::from(丘.亙)%self.丙.len()]}fn 七(&self,l:&J,丘:両)->K<L>{佶(l,self.乞(丘),丘.亅)}}fn
伤(l:&J,丁:C)->U{let mut hash:U=0xcbf29ce484222325;let mut 伛=|亷:U|hash=(hash^亷).wrapping_mul(0x0100000001b3);for(俯,串)in[l.丟(丁),l.丅(丁),l.丞()].into_iter().enumerate(){for
q in 串.iter(){伛(((俯 as U)<<8)|q as U);}伛(0xff);}伛(l.丈(丁)as U);hash}enum 亝{仛{丽:C,乣:C,乐:些,},}struct 亂{乹:亝,丙:K<Y>,仇:Y,且:K<両>,代:K<U>,亰:C,丮:丱,仯:T,}impl 亂{fn
new(乹:亝,丙:K<Y>,仇:Y)->亂{亂{乹,丙,仇,且:K::new(),代:K::new(),亰:0,丮:0,仯:false,}}fn 俓(&mut self,l:&J)->T{let j=self;let 俧=matches!(j.乹,亝::仛{..});let 俔=!俧&&j.丮>0&&j.亰>=j.代.len();if!j.仯&&(俔||仰(l)){j.仯=true;}!j.仯}fn
上(&mut self,l:&J,乱:P<乑>)->K<L>{let j=self;let 佉=伫{丙:&j.丙,仇:j.仇,rest:両{亙:0,亅:乃::仚,},乱,俜:Settings::DEFAULT,};let 丁=l.东();let 俍=j.代.get(j.亰)==Some(&伤(l,丁));if!俍{let
乊=j.丮==0;j.且=match j.乹{亝::仛{丽,乣,乐,}=>{let 乐=&乐;let 乖=侗;let mut 乡=且(l,&j.丙,乖,丽,乣,乱,乐,);乡.resize(丽,乖);乡}};j.代=match j.乹{亝::仛{..}=>俨(l,&j.丙,&j.且),};j.亰=0;j.丮+=1;}let
丘=j.且.get(j.亰).copied().unwrap_or(佉.rest);j.亰+=1;佉.七(l,丘)}}fn 俨(l:&J,丙:&[Y],且:&[両])->K<U>{let 丁=l.东();let mut 余=K::with_capacity(且.len());let mut 也=l.clone();for&丘
in 且{if 也.丕().is_some(){break;}余.push(伤(&也,丁));也=伃(&也,&七(&也,丙,丘));}余}const 価:F=3;struct Settings{丽:F,}impl Settings{const DEFAULT:Settings=Settings{丽:価};}fn
乡(l:&J,不:G)->[I;M]{let 乬=l.map().valid;let mut 义=[I::MAX;M];let mut 之=不&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn
俼(l:&J,z:C)->F{let 介=乡(l,l.丄(z));let 乻=乡(l,l.丄(1-z));l.map().valid.iter().map(|q|match 介[q].cmp(&乻[q]){_ if 介[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侙:[Y;27]=丙();const 侗:両=両{亙:16,亅:乃::仚,};const fn 丙()->[Y;27]{let 亣=[0,2,4];let 亳=[0,3,6];let 于=[0,1,2];let mut 万=[Y::企;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亦=if g>0||t>0{4}else{0};万[i]=Y::企.倁(亣[g],亳[t],0).倀(亦,于[w]);i+=1;}万}#[derive(Default)]struct 伀{丌:F,专:F,丈:F,乵:F,丝:F,仒:F,丯:F,仔:F,}#[derive(Clone,Copy)]struct
些{丌:F,专:F,丈:F,丝:F,乵:F,仒:F,丯:F,仔:F,}fn 保(l:&J,z:C)->[F;M]{struct 亹{丄:G,乄:[u16;M],丳:[u16;M],size:[F;M],丝:[F;M],亍:u16,}fn 乽(d:&mut 亹,v:C){d.亍+=1;d.乄[v]=d.亍;d.丳[v]=d.亍;d.size[v]=1;for
w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{乽(d,w);d.size[v]+=d.size[w];d.丳[v]=d.丳[v].min(d.丳[w]);if d.丳[w]>=d.乄[v]{d.丝[v]+=d.size[w];}}else{d.丳[v]=d.丳[v].min(d.乄[w]);}}}let
mut d=亹{丄:l.丄(z),乄:[0;M],丳:[0;M],size:[0;M],丝:[0;M],亍:0,};let 乪=Z[z];if d.丄.test(乪){乽(&mut d,乪);}d.丝[乪]=0;d.丝}fn 俟(l:&J,z:C)->伀{let 丄=l.丄(z);let 丝=保(l,z);let
亴=亿(l,G::q(Z[1-z]));let mut 万=伀{丌:F::from(丄.count()),专:l.专(z),丈:l.丈(z),丯:F::MAX,仔:俼(l,z)/2,..伀::default()};for q in 丄.iter(){if 丝[q]>0{万.乵+=1;万.丝=万.丝.max(丝[q]);}if
q!=Z[z]&&(G::q(q).世()&丄).count()<=1{万.仒+=1;}万.丯=万.丯.min(F::from(亴[q]));}万}fn 业(l:&J,z:C,乐:&些)->F{let s=俟(l,z);let o=乐;o.丌*s.丌+o.专*s.专+o.丈*s.丈-o.丝*s.丝-o.乵*s.乵-o.仒*s.仒-o.丯*s.丯.min(30)+o.仔*s.仔}fn
伃(l:&J,七:&[L])->J{let mut 丂=l.clone();丂.佦(七);if 丂.丕().is_none(){丂.佦(&[]);}丂}fn 七(l:&J,丙:&[Y],丘:両)->K<L>{佶(l,&丙[C::from(丘.亙)%丙.len()],丘.亅,)}fn 佥(l:&J,丙:&[Y],且:&[両],乖:両,丽:C,)->J{let
mut 也=l.clone();for 上 in 0..丽{if 也.丕().is_some(){break;}let 丘=且.get(上).copied().unwrap_or(乖);也=伃(&也,&七(&也,丙,丘));}也}fn 俬(l:&J,丙:&[Y])->K<(両,K<L>)>{let mut
万:K<(両,K<L>)> =K::new();for 亙 in 0..丙.len().min(C::from(I::MAX)){for&亅 in&佽{let 丘=両{亙:亙 as I,亅,};let c=七(l,丙,丘);if!万.iter().any(|(_,丩)|*丩==c){万.push((丘,c));}}}万}fn
侬(l:&J,丙:&[Y],乖:両,丽:C,乣:C,乱:P<乑>,乐:&些,)->K<両>{let 丁=l.东();let 仌=||乱.is_some_and(|t|乑::now()>=t);let 从=|且:&[両]|业(&佥(l,丙,且,乖,丽),丁,乐);let mut 三:(F,K<両>)=(从(&[]),K::new());let
mut 之:K<(J,K<両>)> =vec![(l.clone(),K::new())];for 侵 in 0..丽{let mut 乯:K<(F,J,K<両>)> =K::new();let mut 乁:HashMap<U,C> =HashMap::new();for(佟,且)in&之{if 仌(){break;}for(丘,c)in
俬(佟,丙){let 丂=伃(佟,&c);let mut 伋=且.clone();伋.push(丘);let v=业(&丂,丁,乐);let 乌=伤(&丂,丁);match 乁.get(&乌){Some(&丹)if 乯[丹].0>=v=>{}Some(&丹)=>乯[丹]=(v,丂,伋),None=>{乁.insert(乌,乯.len());乯.push((v,丂,伋));}}}}if
乯.is_empty(){break;}乯.sort_by(|a,b|b.0 .cmp(&a.0));乯.truncate(乣);let 乶=侵+1==丽||仌();for(i,(_,_,且))in 乯.iter().take(if 乶{乣}else{1}).enumerate(){if i>0&&仌(){break;}let
v=从(且);if v>三.0{三=(v,且.clone());}}if 乶{break;}之=乯.into_iter().map(|(_,s,p)|(s,p)).collect();}三.1}fn 且(l:&J,丙:&[Y],乖:両,丽:C,俈:C,乱:P<乑>,乐:&些,)->K<両>{let 丁=l.东();let
从=|且:&[両]|业(&佥(l,丙,且,乖,丽),丁,乐);let mut 三:(F,K<両>)=(从(&[]),K::new());let mut 乣=2;loop{let 丧=侬(l,丙,乖,丽,乣,乱,乐);let v=从(&丧);if v>三.0{三=(v,丧);}let 仌=乱.is_some_and(|t|乑::now()>=t);if
仌||(乱.is_none()&&乣>=俈){break;}乣*=2;}三.1}const 侀:C=4+12;fn 丽(伖:impl BufRead,mut 仮:impl Write,mut 伅:impl FnMut(&str,&str,丱,乑)->丰,)->Result<(),丰>{let mut
丣=伮{佨:伖};let Some(乊)=丣.line()?else{return Ok(());};let mut 亥=format!("{}\n",乊);丣.take(丑(&乊,"mine spot count")?,&mut 亥)?;let mut 上=0;loop{let Some(乊)=丣.line()?else{return
Ok(());};let 亗=乑::now();上+=1;let mut 乗=format!("{}\n",乊);丣.take(侀-1,&mut 乗)?;for 丵 in["building count","unit count"]{let line=丣.佪()?;let n=丑(&line,丵)?;乗.push_str(&line);乗.push('\n');丣.take(n,&mut
乗)?;}let 伅=伅(&亥,&乗,上,亗);writeln!(仮,"{}",伅).and_then(|()|仮.flush()).map_err(|互|format!("turn {}: cannot write the answer: {}",上,互))?;}}struct 伮<R>{佨:R,}impl<R:BufRead>伮<R>{fn
line(&mut self)->Result<P<丰>,丰>{let mut line=丰::new();match self.佨.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(互)=>Err(format!("cannot read input: {}",互)),}}fn
佪(&mut self)->Result<丰,丰>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:C,乗:&mut String)->Result<(),丰>{for
_ in 0..n{乗.push_str(&self.佪()?);乗.push('\n');}Ok(())}}fn 丑(line:&str,丵:&str)->Result<C,丰>{match line.trim().parse::<C>(){Ok(n)if n<=侈=>Ok(n),_=>Err(format!("bad {} {line:?}",丵)),}}const
侈:C=144;const 依:F=3;const 侃:F=2;const 侌:C=16;fn 俑(l:&J,万:&mut Vec<L>){let z=l.东();let mut 乹=伳{s:l.clone(),z,乲:1-z,丐:Z[1-z],令:Z[z],上:l.了(),万,};乹.run();}struct
伳<'a>{s:J,z:C,乲:C,丐:C,令:C,上:丱,万:&'a mut Vec<L>,}fn 件(q:C)->(I,I){((q%O)as I,(q/O)as I)}fn 伜(丏:丱,q:C)->L{let(x,y)=件(q);L::亁{丏,x,y}}fn 併(一:I,q:C)->L{let(x,y)=件(q);L::书{一,x,y}}impl
伳<'_>{fn run(&mut self){let j=self;if j.仩(){return;}j.乛();if j.仩(){return;}j.乏();j.俸();}fn 义(&self,不:C,乫:C)->F{F::from(self.s.map().久(不,乫))}fn 个(&mut self,下:L)->T{let
为=self.s.个(下);if 为{self.万.push(下);}为}fn 亶(&self,丠:&J)->T{丠.丕()==Some(么::乧(self.z))}fn 仩(&mut self)->T{let j=self;if j.s.丕().is_some(){return false;}for
q in j.s.予().iter(){if j.义(q,j.丐)!=1{continue;}let Some((丏,_))=j.s.仓(q)else{continue;};let 下=伜(丏,j.丐);let mut 丠=j.s.clone();if 丠.个(下)&&j.亶(&丠){return j.个(下);}}let
Some(並)=j.侲()else{return false;};let mut 丠=j.s.clone();if!並.iter().all(|&下|丠.个(下))||!j.亶(&丠){return false;}for 下 in 並{j.个(下);}true}fn 侲(&self)->P<K<L>>{let
j=self;const 伭:F=F::MAX;let 乕=j.s.丨().乩();let 丢:K<G> =(1..=乕).map(|一|j.s.主(j.z,一)).collect();let 仂=|q:C|{丢.iter().position(|串|串.test(q)).map(|与|(与 as I+1,10*(与
as F+1)))};let 丄=j.s.丄(j.z);let mut 丆=[伭;M];let mut 严=[C::MAX;M];let mut 为=[false;M];for q in 丄.iter(){丆[q]=0;}loop{let mut 丂:P<C> =None;for q in 0..M{if!为[q]&&丆[q]<伭&&丂.is_none_or(|n|丆[q]<丆[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};为[q]=true;if q==j.丐{break;}for&举 in&j.s.map().世[0][q]{if 举==乙{break;}let 举=C::from(举);if 为[举]||丄.test(举){continue;}let Some((_,争))=仂(举)else{continue;};if
丆[q]+争<丆[举]{丆[举]=丆[q]+争;严[举]=q;}}}if 丆[j.丐]>j.s.丈(j.z){return None;}let mut 並=K::new();let mut q=j.丐;while!丄.test(q){let(一,_)=仂(q)?;並.push(併(一,q));q=严[q];}並.reverse();Some(並)}fn
乨(&self,丸:&J)->F{let(丁,二)=(self.z,self.乲);let 严=&self.s;let 丑=|串:G|F::from(串.count());let 丒=丑(丸.丟(丁))-丑(严.丟(丁));let 仳=丑(严.丟(二))-丑(丸.丟(二));let 俅=丑(严.丄(二))-丑(丸.丄(二));let
仦=严.亩(二)-丸.亩(二);let 丞=丑(严.丞()&严.丟(二))-丑(丸.丞()&丸.丟(二));12*丒+20*仳+8*(俅-仳).max(0)+5*仦+25*丞}fn 佯(&self,q:C)->F{let j=self;let 丶=j.s.map().临.test(q);if 丶&&j.s.丨().仝()&&!j.s.丟(j.z).test(q){20}else{0}}fn
低(&self,丿:F)->T{let j=self;j.s.丅(j.乲).iter().any(|q|j.义(q,j.令)<=丿)}fn 乛(&mut self){let j=self;let mut 丅:K<(F,C)> =j.s.予().iter().map(|q|(j.义(q,j.丐),q)).collect();丅.sort_unstable();for(係,q)in
丅{if j.s.丕().is_some(){return;}let Some((丏,一))=j.s.仓(q)else{continue;};let 丢=j.s.主(j.z,一);let 举=j.s.map().世[j.z][q];let mut 三:P<(F,C)> =None;for&丂 in 举.iter().take_while(|&&丂|丂!=乙){let
丂=C::from(丂);if!丢.test(丂){continue;}let mut 丠=j.s.clone();if!丠.个(伜(丏,丂)){continue;}let 业=if j.亶(&丠){100000}else{j.乨(&丠)+10*(係-j.义(丂,j.丐))+j.佯(丂)};if 三.is_none_or(|(乼,_)|业>乼){三=Some((业,丂));}}if
let Some((业,丂))=三{if 业>0{j.个(伜(丏,丂));}}}}fn 乏(&mut self){let j=self;if j.s.丕().is_some(){return;}if j.低(依){j.俶();}else{j.丞();}}fn 俶(&mut self){let j=self;if!j.s.丨().伄()||j.s.丈(j.z)<亏{return;}let
举=j.s.map().世[j.z][j.令];let 丌:K<C> =举.iter().take_while(|&&q|q!=乙).map(|&q|C::from(q)).collect();let 乆=j.s.丟(j.z);if 丌.iter().any(|&q|乆.test(q)&&j.s.乢().test(q)){return;}for
q in 丌{let(x,y)=件(q);if j.个(L::亐{x,y}){return;}}}fn 丞(&mut self){let j=self;if!j.s.丨().仝(){return;}while j.s.丈(j.z)>=亏{let 亓=j.s.map().临&j.s.丟(j.z)&j.s.丄(j.z)&j.s.什();let
Some(q)=亓.iter().min_by_key(|&q|(j.义(q,j.令),q))else{return;};let(x,y)=件(q);if!j.个(L::乾{x,y}){return;}}}fn 俸(&mut self){let j=self;let 佫=if j.上<=3{0}else{5};for
_ in 0..侌{if j.s.丕().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俄=j.低(侃);let 们=j.s.井(j.z);let mut 三:P<(F,L)> =None;let mut 伦:P<(F,L)> =None;let
mut 佀=G::Q;for 一 in 1..=j.s.丨().乩(){let 丢=j.s.主(j.z,一);let 乸=(们&丢).两(佀);佀=丢;let 争=10*F::from(一);if 争>丈{continue;}for q in 乸.iter(){let 下=併(一,q);let mut
丠=j.s.clone();if!丠.个(下){continue;}let 优=丠.丈(j.z);let 专=丠.专(j.z);let 仦=j.s.亩(j.乲)-丠.亩(j.乲);let 侨=if 仦>0&&俄{优+专>=0}else{专>=0&&(优>=佫||仦>0)};if!侨{continue;}let
乨=j.乨(&丠);let 业=乨+j.佯(q)-6*j.义(q,j.丐)-10*(F::from(一)-1);if 乨>0{if 三.is_none_or(|(乼,_)|业>乼){三=Some((业,下));}}else if 专>=3&&优>=佫+10&&一==1{if 伦.is_none_or(|(乼,_)|业>乼){伦=Some((业,下));}}}}let
Some((_,下))=三.or(伦)else{return;};if!j.个(下){return;}}}}fn 俀(l:&J,万:&mut Vec<L>){万.clear();俑(l,万);}const 亄:F=F::MAX;const 例:C=8;struct 亞{丆:[F;M],严:[I;M],}fn
仡(l:&J,z:C,乛:T,丐:C)->F{let mut 丆=亄;侼(l,z,乛,|丘,new|{if new.test(丐){丆=10*丘 as F;}丆!=亄});丆}fn 俖(l:&J,z:C,仉:G,丫:F)->G{let mut 仰=G::Q;佑(l,z,仉,|丘,new|{if 10*丘
as F>丫{return true;}仰|=new;false});仰.两(l.丄(z))}fn 侼(l:&J,z:C,乛:T,乽:impl FnMut(C,G)->T){佑(l,z,仉(l,z,乛),乽);}fn 仉(l:&J,z:C,乛:T)->G{if 乛&&z==l.东(){l.予()}else{G::Q}}fn
佑(l:&J,z:C,仉:G,mut 乽:impl FnMut(C,G)->T,){let 乕=l.丨().乩();let mut 仂=[G::Q;3];let mut 乁=G::Q;for 一 in 1..=乕{let 串=l.主(z,一);仂[C::from(一)-1]=串.两(乁);乁|=串;}let
mut 丼=[l.丄(z),G::Q,G::Q,G::Q];for q in(仉&l.丅(z)).iter(){if let Some(一)=l.亘(q).filter(|&一|一<=乕){丼[0]|=G::q(q).世()&l.主(z,一);}}let mut 为=G::Q;for 丘 in 0..{let
new=丼[丘%4].两(为);丼[丘%4]=G::Q;if 乽(丘,new){return;}为|=new;let 丂=new.世().两(为);for(侩,丌)in 仂.iter().enumerate(){丼[(丘+侩+1)%4]|=丂&*丌;}if 丼.iter().all(|串|串.两(为).is_empty()){break;}}}impl
亞{fn new(l:&J,z:C,乛:T)->亞{let 乕=C::from(l.丨().乩());let mut 丢=[G::Q;3];for(与,串)in 丢.iter_mut().enumerate().take(乕){*串=l.主(z,与 as I+1);}let 丄=l.丄(z);let
世=&l.map().世[0];let mut 丆=[亄;M];let mut 严=[乙;M];let mut 丼=[0u8;M];let mut 丑=0;let mut 佭=G::Q;for q in 丄.iter(){丆[q]=0;丼[丑]=q as I;丑+=1;}if 乛&&z==l.东(){for
不 in l.予().iter(){let Some(一)=l.亘(不)else{continue;};let 乡=G::q(不).世()&丢[C::from(一)-1];for q in 乡.iter(){if 丆[q]!=0{丆[q]=0;丼[丑]=q as I;丑+=1;}}}}while 丑>0{let
mut 三=0;for 与 in 1..丑{let(a,b)=(C::from(丼[与]),C::from(丼[三]));if(丆[a],a)<(丆[b],b){三=与;}}let q=C::from(丼[三]);丑-=1;丼[三]=丼[丑];佭.set(q);for&丂 in&世[q]{if 丂==乙{break;}let
丂=C::from(丂);if 佭.test(丂)||丄.test(丂){continue;}let Some(一)=丢[..乕].iter().position(|串|串.test(丂))else{continue;};let 佴=丆[q]+10*(一 as F+1);if 佴<丆[丂]{if 丆[丂]==亄{丼[丑]=丂
as I;丑+=1;}丆[丂]=佴;严[丂]=q as I;}}}亞{丆,严}}fn path(&self,丐:C)->K<C>{let mut path=K::new();if 丐>=M||self.丆[丐]==亄{return path;}let mut q=丐;while self.严[q]!=乙{path.push(q);q=C::from(self.严[q]);}path.reverse();path}}struct
付{七:K<L>,丆:F,}fn 俇(l:&J,z:C,q:C)->P<I>{(1..=l.丨().乩()).find(|&一|l.主(z,一).test(q))}fn 俙(丬:&mut J,丁:C,亢:C,丫:F,七:&mut Vec<L>,习:&mut i64,)->T{loop{if 丬.丕()==Some(么::乧(丁)){return
true;}if 丬.丟(丁).test(亢)&&丬.丄(丁).test(亢){return true;}let 丯=亞::new(丬,丁,false);let 丆=丯.丆[亢];if 丆==亄||*习+丆>丫{return false;}let Some(&q)=丯.path(亢).first()else{return
false;};let Some(一)=俇(丬,丁,q)else{return false;};let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(下){return false;}*习+=10*F::from(一);七.push(下);}}fn 伾(l:&J,丁:C,便:&[C],丫:F)->P<付>{let
mut 丬=l.clone();let mut 七=K::new();let mut 习=0;for&亢 in 便{if!俙(&mut 丬,丁,亢,丫,&mut 七,&mut 习){return None;}}(习<=丫).then_some(付{七,丆:习,})}fn 伊(l:&J,丐:C,丫:F)->P<付>{if
丫<0||l.丕().is_some()||丐>=M||l.map().丷.test(丐){return None;}let 丁=l.东();if l.丟(丁).test(丐)&&l.丄(丁).test(丐){return Some(付{七:K::new(),丆:0,});}let mut 三=伾(l,丁,&[丐],丫);let
乊=亞::new(l,丁,false);let mut 乢:K<C> =(l.乢()&l.丟(1-丁)).iter().filter(|&乇|乇!=丐&&乊.丆[乇]!=亄).collect();乢.sort_by_key(|&乇|(l.map().久(乇,丐),乇));乢.truncate(例);for
乇 in 乢{if let Some(並)=伾(l,丁,&[乇,丐],丫){if 三.as_ref().is_none_or(|乼|並.丆<乼.丆){三=Some(並);}}}三}const 侊:C=20000;struct 仙{七:K<L>,丆:F,丌:F,丅:F,乏:F,专:F,丛:F,}struct
仼{丌:F,丅:F,乏:F,}fn 何(l:&J,z:C)->仼{let 丄=l.丄(z);let 丅=l.丅(z)&丄;let 业=丅.iter().filter_map(|q|l.亘(q)).map(|一|10*F::from(一)).sum();仼{丌:F::from(丄.count()),丅:业,乏:F::from(((l.丞()|l.乢())&丄).count()),}}struct
伵{丁:C,价:G,严:仼,专:F,亨:C,丫:F,京:C,path:K<L>,丧:K<仙>,index:HashMap<亻,C,BuildHasherDefault<DefaultHasher>>,}type 亻=[U;3];fn 俿(mut 乌:亻,q:C)->亻{乌[q/64]|=1<<(q%64);乌}impl
伵{fn 佌(&mut self,丬:&J,乶:P<C>,仳:亻,丆:F,)->T{let j=self;let 丁=j.丁;let 乸=match 乶{None=>丬.井(丁),Some(q)=>G::q(q).世(),}&j.价;let 乸=乸.两(丬.丟(丁));let mut 丢=[G::Q;3];for
一 in 1..=丬.丨().乩(){丢[C::from(一)-1]=丬.主(丁,一);}for q in 乸.iter(){let 乕=C::from(丬.丨().乩());let Some(与)=丢[..乕].iter().position(|串|串.test(q))else{continue;};let
一=与 as I+1;let 争=10*F::from(一);if 丆+争>j.丫{continue;}if j.京>=侊{return false;}j.京+=1;let mut 丂=丬.clone();let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(下){continue;}j.path.push(下);let
亲=俿(仳,q);j.俗(&丂,亲,丆+争);let 侴=j.path.len()<j.亨&&丂.丕().is_none();let 侾=!侴||j.佌(&丂,Some(q),亲,丆+争);j.path.pop();if!侾{return false;}}true}fn 俗(&mut self,丸:&J,亲:亻,丆:F){let
j=self;let 今=1-j.丁;let 伝=何(丸,今);let 丌=j.严.丌-伝.丌;let 丅=j.严.丅-伝.丅;if 丌<=0&&丅<=0{return;}let 乏=j.严.乏-伝.乏;let 专=丸.专(j.丁)-j.专;let 倂=丸.丕()==Some(么::乧(j.丁));let
丛=if 倂{F::MAX/2}else{3*丌+丅+10*乏+2*专-丆};if let Some(&丹)=j.index.get(&亲){if 丆>=j.丧[丹].丆{return;}}let 丝=仙{七:j.path.clone(),丆,丌,丅,乏,专,丛,};match j.index.get(&亲){Some(&丹)=>j.丧[丹]=丝,None=>{j.index.insert(亲,j.丧.len());j.丧.push(丝);}}}}fn
佊(l:&J,亨:C,丫:F,丿:C)->K<仙>{乍(l,亨,丫,丿).0}fn 乍(l:&J,亨:C,丫:F,丿:C)->(K<仙>,C){if l.丕().is_some()||亨==0||丿==0{return(K::new(),0);}let 丁=l.东();let 今=1-丁;let mut
伢=伵{丁,价:l.丟(今)|l.丟(今).世(),严:何(l,今),专:l.专(丁),亨,丫,京:0,path:K::new(),丧:K::new(),index:HashMap::default(),};伢.佌(l,None,[0;3],0);let 京=伢.京;let mut 丧=伢.丧;丧.sort_by(|a,b|b.丛.cmp(&a.丛).then(a.丆.cmp(&b.丆)));丧.truncate(丿);(丧,京)}const
仜:F=1000000;const 伯:F=200;fn 乎(l:&J,z:C)->F{let 丄=l.丄(z);let 丅:F=(l.丅(z)&丄).iter().filter_map(|q|l.亘(q)).map(|一|10*F::from(一)).sum();l.丈(z)+3*l.专(z)+4*F::from(丄.count())+丅+10*F::from((l.乢()&丄).count())}fn
佧(l:&J,z:C)->F{let 丆=仡(l,z,false,Z[1-z]);if 丆==亄{return-伯;}(l.丈(z)+l.专(z)-丆).clamp(-伯,0)}fn 侺(l:&J,z:C)->F{let 丩=1-z;match l.丕(){Some(么::乧(俾))=>return
if 俾==z{仜}else{-仜},Some(么::伬)=>return 0,None=>{}}let mut 丛=乎(l,z)-乎(l,丩);if z==l.东(){let mut 丂=l.clone();丂.仁();if 丂.丕().is_none()&&仡(&丂,丩,true,Z[z])<=丂.丈(丩){丛-=仜/2;}}丛+佧(l,z)-佧(l,丩)}
