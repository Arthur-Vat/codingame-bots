// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 丱=u32;type U=u64;type 仗=i32;type C=i64;type 亜=f64;type T=bool;type 丰=String;type 乑=Instant;type K<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侈:T=true;fn 俋(l:&J,q:F,仒:C,丯:G,並:C)->T{let
丁=l.东();let 乲=1-丁;let 佛=侈&&並<=l.丈(乲)+2*l.专(乲).max(0);let 仸=G::q(q)|G::q(q).世();if!佛&&(仸&丯).is_empty(){return false;}let(x,y)=亻(q);let mut 丸=l.clone();if!丸.个(L::亐{x,y}){return
false;}if 俻(&丸,丁)<仒{return true;}佛&&伐(&丸,乲,false,Z[丁])>並}const 侎:F=4;const 伭:F=5;#[derive(Clone)]enum 乂{亿(L),仾(K<L>),}impl 乂{fn 七(&self)->&[L]{match self{乂::亿(下)=>std::slice::from_ref(下),乂::仾(七)=>七,}}}fn
亻(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 何(l:&J,万:&mut Vec<乂>){万.clear();if l.丕().is_some(){return;}let 丁=l.东();let 乕=l.丨().乩();let mut 丢=[G::Q;4];for 一
in 1..=乕{丢[F::from(一)]=l.主(丁,一);}for 丌 in l.予().iter(){let Some((丐,一))=l.仔(丌)else{continue;};let 乡=G::q(丌).世()&丢[F::from(一)];for 乫 in 乡.iter(){let(x,y)=亻(乫);万.push(乂::亿(L::亁{丐,x,y}));}}let
丈=l.丈(丁);let 仭=l.亖(丁);for 一 in 1..=乕{let 丆=10*C::from(一);if 丈<丆{break;}let mut 不=仭&丢[F::from(一)];for 俐 in 1..一{不=不.两(丢[F::from(俐)]);}for q in 不.iter(){let(x,y)=亻(q);万.push(乂::亿(L::书{一,x,y}));}}if
丈>=亏{let 亓=l.丟(丁)&仭&l.仃();if l.丨().仞(){for q in(亓&l.map().临).iter(){let(x,y)=亻(q);万.push(乂::亿(L::乾{x,y}));}}if l.丨().伅(){let 侸=l.map().valid.两(l.丟(丁)).世();let
不=(亓&侸).两(l.map().临);let 丯=丯(l,1-丁);let 仒=C::from((丯&l.丄(丁)).count());let 並=伐(l,1-丁,false,Z[丁]);for q in 不.iter().filter(|&q|俋(l,q,仒,丯,並)){let(x,y)=亻(q);万.push(乂::亿(L::亐{x,y}));}}}for
丝 in 位(l,伭,丈,侎){if 丝.七.len()>1{万.push(乂::仾(丝.七));}}if let Some(並)=伋(l,Z[1-丁],丈){if 並.七.len()>1{万.push(乂::仾(並.七));}}}const 供:I=2;const 侨:C=5;fn 乖(l:&J,z:F)->G{伟(l,z,供)}fn
伟(l:&J,z:F,丌:I)->G{let mut 乖=G::Q;if 丌>3{return 乖;}for q in(l.丅(z)&l.丄(z)).iter(){if l.亙(q).is_some_and(|一|一>=丌){乖|=G::q(q);}}乖}fn 丯(l:&J,乎:F)->G{let 丫=l.丈(乎)+l.专(乎);俠(l,乎,乖(l,乎),丫)}fn
仆(l:&J,z:F)->G{丯(l,1-z)&l.丄(z)}fn 俻(l:&J,z:F)->C{C::from(仆(l,z).count())}const 侘:C=4;fn 俓(不:C)->C{不/侘}fn 丞(l:&J,z:F,仆:G)->C{let 丄=l.丄(z);let 丞=(l.丞()&丄).两(仆);俓(C::from(丄.count()))*C::from(丞.count())}const
侤:C=2;fn 使(l:&J,z:F)->C{let 丄=l.丄(z);let[a,b,c]=丄.仵();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(伺-1);侰.with(|伉|{let mut 伉=伉.borrow_mut();let 仳=&mut 伉[与];if 仳.0==丄&&仳.1==z as I+1{return 仳.2;}let 丛=倇(丄,Z[z]);*仳=(丄,z as I+1,丛);丛})}const
伺:F=1<<12;thread_local!{static 侰:std::cell::RefCell<K<(G,I,C)>> =std::cell::RefCell::new(vec![(G::Q,0,0);伺]);}fn 倇(丄:G,乪:F)->C{struct 亼{丄:G,乄:[I;M],丳:[I;M],size:[I;M],亍:I,乵:C,们:C,}fn
亚(d:&mut 亼,v:F)->I{d.亍+=1;let 乄=d.亍;d.乄[v]=乄;let mut 丳=乄;let mut 伨=1u8;let mut 丝=0u8;for w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{let 佷=亚(d,w);伨+=佷;丳=丳.min(d.丳[w]);if
d.丳[w]>=乄{丝+=佷;}}else{丳=丳.min(d.乄[w]);}}d.丳[v]=丳;d.size[v]=伨;if 丝>0&&d.乄[v]>1{d.乵+=1;d.们=d.们.max(C::from(丝));}伨}if!丄.test(乪){return 0;}let mut d=亼{丄,乄:[0;M],丳:[0;M],size:[0;M],亍:0,乵:0,们:0,};亚(&mut
d,乪);d.乵+d.们}fn 俨(l:&J,z:F)->C{侤*(使(l,1-z)-使(l,z))}fn 低(l:&J,z:F)->C{if l.丈(z)+l.专(z)>=0{return 0;}(l.丅(z)&l.丄(z)).iter().filter_map(|q|l.亙(q)).map(|一|10*C::from(一)).sum()}const
侍:C=15;const 侮:[C;2]=[3,1];const 侬:[C;2]=[4,20];fn 俷(l:&J,z:F)->[C;2]{let mut 丑=[0;2];for q in(l.丅(z)&l.丄(z)).iter(){match l.亙(q){Some(2)=>丑[0]+=1,Some(3)=>丑[1]+=1,_=>{}}}丑}fn
佶(l:&J,z:F)->C{let[佼,俼]=俷(l,z);let 佁=|一:F|(3-侮[一])*侬[一];-侍*佼+佁(0)*佼+佁(1)*俼}const 侇:C=0;const 侊:C=2;const 侉:C=100;fn 佴(仫:C)->C{if 仫<0{return 0;}侇+侊*仫.min(侉)/2}const
伲:I=2;fn 伓(l:&J,z:F)->C{let 乖=[伟(l,z,伲),伟(l,1-z,伲),];let(丘,亪)=俄(l,z,乖);let 丘=丘+佴(亪[0])-佴(亪[1]);if l.丕().is_some(){return 丘;}let 仌=仆(l,z);let 乼=仆(l,1-z);let
仒=C::from(乼.count())-C::from(仌.count());丘-2*低(l,z)+2*低(l,1-z)+俨(l,z)+佶(l,z)-佶(l,1-z)+侨*仒/4+丞(l,z,仌)-丞(l,1-z,乼)}const 侗:F=8;const 侟:F=3;const 侏:F=32;const
侠:U=300;const 侃:亜=0.75;struct 亀{乻:P<乑>,五:P<乑>,仉:U,}#[derive(Default)]struct 伸{七:K<L>,伏:F,乳:U,丗:F,丮:F,但:F,乴:F,}const 侯:T=true;#[derive(Clone)]struct 乛{七:K<L>,伒:C,}struct
亞{七:K<L>,end:J,first:P<U>,丒:P<K<乛>>,}type 亽=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct 伀{丁:F,limit:亀,乳:U,仈:亽,亗:P<K<乂>>,}fn 伇(l:&mut J,七:&[L])->T{七.iter().all(|&下|l.个(下))}fn
乌(七:&[L])->U{let mut hash:U=0xcbf29ce484222325;for 下 in 七{let(仧,a,b,c)=match*下{L::亁{丐,x,y}=>(1,U::from(丐),x,y),L::书{一,x,y}=>(2,U::from(一),x,y),L::乾{x,y}=>(3,0,x,y),L::亐{x,y}=>(4,0,x,y),};let
人=仧|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^人).wrapping_mul(0x0100000001b3);}hash}fn 伾(乔:&J,亊:&[L])->J{let mut 丂=乔.clone();丂.仄();for&下 in 亊{丂.个(下);}丂.仄();丂}fn
仩(l:&J)->P<K<L>>{let 丁=l.东();let 丏=Z[1-丁];let 亹=|丠:&J|丠.丕()==Some(么::乧(丁));for q in l.予().iter(){if l.map().久(q,丏)!=1{continue;}let Some((丐,_))=l.仔(q)else{continue;};let(x,y)=亻(丏);let
下=L::亁{丐,x,y};let mut 丠=l.clone();if 丠.个(下)&&亹(&丠){return Some(vec![下]);}}let 並=伋(l,丏,l.丈(丁))?;let mut 丠=l.clone();(伇(&mut 丠,&並.七)&&亹(&丠)).then_some(並.七)}impl
伀{fn 习(&self)->T{self.乳>=self.limit.仉||self.limit.乻.is_some_and(|t|乑::now()>=t)}fn 丘(&mut self,乔:&J,丗:&[乛],伈:C)->C{self.乳+=1;self.伣(乔,丗,伈)}fn 伣(&self,乔:&J,丗:&[乛],伈:C)->C{if
乔.丕()==Some(么::乧(self.丁)){return 仝;}if 丗.is_empty(){return 伓(乔,self.丁);}let mut 件=C::MAX;for 亊 in 丗{件=件.min(伓(&伾(乔,&亊.七),self.丁));if 件<=伈{break;}}件}#[allow(clippy::too_many_arguments)]fn
俚(&mut self,买:&J,亢:C,乓:&[乂],亱:&[(C,F)],为:&mut[T],丗:&[乛],亣:&mut 亽,伙:T,)->(P<(F,J,C)>,T){let mut 三=None;let mut 仟=亢;for&(仮,与)in 亱{if 为[与]{continue;}if 伙&&仮!=C::MAX&&仮<=仟-亢{return(三,true);}if
self.习(){return(三,false);}为[与]=true;let 七=乓[与].七();let mut 丂=买.clone();if!伇(&mut 丂,七){亣.insert(乌(七),C::MIN/4);continue;}let 丘=self.丘(&丂,丗,仟);亣.insert(乌(七),丘-亢);if
丘>仟{仟=丘;三=Some((与,丂,丘));}}(三,false)}fn 伎(&mut self,乪:&J,丗:&[乛],什:&[U],侺:T)->亞{let j=self;let mut 乊=None;let mut 买=乪.clone();let mut 七=K::new();let mut
亢=j.丘(&买,丗,C::MIN);let mut 亣=亽::default();let mut 乓=K::new();let mut 亱=K::new();while 买.丕().is_none()&&!j.习(){if 七.is_empty()&&侺{match&j.亗{Some(亗)=>乓.clone_from(亗),None=>{何(&买,&mut
乓);j.亗=Some(乓.clone());}}}else{何(&买,&mut 乓);}亱.clear();for(与,乮)in 乓.iter().enumerate(){let k=乌(乮.七());let 仮=亣.get(&k).or(j.仈.get(&k)).copied();亱.push((仮.unwrap_or(C::MAX),与));}亱.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 为=vec![false;乓.len()];if 七.is_empty(){for(与,乮)in 乓.iter().enumerate(){为[与]=什.contains(&乌(乮.七()));}}let mut 三=None;for
伙 in[true,false]{let(丧,俰)=j.俚(&买,亢,&乓,&亱,&mut 为,丗,&mut 亣,伙,);三=丧;if 三.is_some()||!俰{break;}}let Some((与,丂,丘))=三 else{break;};if 七.is_empty(){乊=Some(乌(乓[与].七()));}七.extend_from_slice(乓[与].七());买=丂;亢=丘;}for(k,乨)in
亣{let 佗=j.仈.entry(k).or_insert(乨);*佗=(*佗).max(乨);}亞{七,end:买,first:乊,丒:None,}}fn 俤(&self,乔:&J)->K<K<L>>{let mut 丂=乔.clone();丂.仄();let mut 万=K::new();if
丂.丕().is_some(){return 万;}let 乜=丂.东();let 丈=丂.丈(乜);if let Some(並)=伋(&丂,Z[self.丁],丈){万.push(並.七);}for 丝 in 位(&丂,伭,丈,侟){万.push(丝.七);}let mut 佖=K::new();俊(&丂,&mut
佖);万.push(佖);let mut 乼=伀{丁:乜,limit:亀{仉:侠,..self.limit},乳:0,仈:亽::default(),亗:None,};万.push(乼.伎(&丂,&[],&[],true).七);万.retain(|七|!七.is_empty());万}fn 伿(&self,乔:&J)->K<乛>{self.俤(乔).into_iter().map(|七|{let
伒=伓(&伾(乔,&七),self.丁);乛{七,伒}}).collect()}fn 乴(&mut self,乪:&J,乮:&乂,丗:&[乛])->P<亞>{let mut 丸=乪.clone();if!伇(&mut 丸,乮.七()){return None;}self.乳+=1;let 乗=self.伎(&丸,丗,&[],false);let
mut 七=乮.七().to_vec();七.extend_from_slice(&乗.七);Some(亞{七,end:乗.end,first:Some(乌(乮.七())),丒:None,})}fn 佒(&self,且:&亞,丗:&[乛])->C{let 丘=self.伣(&且.end,丗,C::MIN);match&且.丒{Some(丒)if!丒.is_empty()=>丘.min(self.伣(&且.end,丒,C::MIN)),_=>丘,}}}fn
俢(丧:&[乛],丗:&mut Vec<乛>)->T{let 严:K<_> =丗.iter().map(|r|r.七.clone()).collect();for 亊 in 丧{if 丗.iter().any(|r|r.七==亊.七){continue;}丗.push(亊.clone());}丗.sort_by_key(|亊|亊.伒);丗.truncate(侗);丗.iter().map(|r|&r.七).ne(&严)}fn
俺(l:&J,丿:亀,亰:&mut 亂)->伸{let 丁=l.东();let mut 中=伸::default();if l.丕().is_some(){return 中;}if let Some(七)=仩(l){中.七=七;return 中;}let 丹=|俪:亜|match(丿.乻,丿.五){(Some(乻),Some(五))=>Some(乻+(五-乻).mul_f64(俪)),_=>None,};if
亰.保(l){中.七=亰.上(l,丹(0.4));return 中;}let mut 乍=伀{丁,limit:亀{乻:丹(侃),..丿},乳:0,仈:亽::default(),亗:None,};let mut 丗=K::new();let mut 丮:K<亞> =K::new();let mut 什=K::new();let
mut 仢=false;let mut 乴:K<乂> =K::new();while 中.伏<侏&&(丮.is_empty()||!乍.习()){let 且=if 仢{let Some(乮)=乴.pop()else{break;};match 乍.乴(l,&乮,&丗){Some(且)=>且,None=>continue,}}else{乍.伎(l,&丗,&什,true)};中.伏+=1;let
佰=丮.iter().any(|p|p.七==且.七);let 乊=且.first;let 併=乍.习();let mut 且=且;let mut 佢=false;if!併&&!佰{let 丧=乍.伿(&且.end);佢=俢(&丧,&mut 丗);且.丒=Some(丧);}if!佰{丮.push(且);}if
併{break;}if!佢&&!仢{match 乊{Some(乊)=>什.push(乊),None=>{仢=true;乴=乍.亗.clone().unwrap_or_default();乴.retain(|a|!什.contains(&乌(a.七())));let 乨=|a:&乂|{乍.仈.get(&乌(a.七())).copied().unwrap_or(C::MIN)};乴.sort_by_key(乨);}}}}中.乴=F::from(仢);中.乳=乍.乳;中.丗=丗.len();中.丮=丮.len();let
佝=||丿.五.is_some_and(|t|乑::now()>=t);let mut 亴:K<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佝(){break;}亴[与]=Some(乍.佒(&丮[与],&丗));}let
亲=|亴:&[P<C>]|{let mut 三:P<(C,F)> =None;for(与,丘)in 亴.iter().enumerate().rev(){if let Some(丘)=*丘{if 三.is_none_or(|(乽,_)|丘>乽){三=Some((丘,与));}}}三};let mut
三=亲(&亴);乍.limit.乻=丿.五;while let Some((_,与))=三{if!侯||丮[与].丒.is_some()||佝(){break;}let 丧=乍.伿(&丮[与].end);丮[与].丒=Some(丧);中.但+=1;亴[与]=Some(乍.佒(&丮[与],&丗));三=亲(&亴);}if
let Some((_,与))=三{let mut 丬=l.clone();for&下 in&丮[与].七{if 丬.个(下){中.七.push(下);}}}中}fn 俜(丿:&dyn Fn(乑,丱)->亀,mut 亰:亂,优:impl BufRead,仯:impl Write,)->Result<(),丰>{丽(优,仯,|产,乘,上,亘|match
J::read(产,乘,乤::仺,上){Ok(l)=>{let 中=俺(&l,丿(亘,上),&mut 亰);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,中.伏,中.乳,中.丗,中.但,中.乴,亘.elapsed());侵(&中.七)}Err(互)=>{eprintln!("plan: cannot read the input: {}",互);"WAIT".to_string()}},)}const
侁:亜=40.0;const 侄:亜=800.0;const 侥:亜=0.6;fn 丿(亘:乑,上:丱)->亀{if let Some(仉)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亀{乻:None,五:None,仉,};}let 仲=俽().min(100.0);let 俕=if 上==1{侄}else{侁};let 丫=Duration::from_secs_f64(俕/1000.0*仲);亀{乻:Some(亘+丫.mul_f64(侥)),五:Some(亘+丫),仉:U::MAX,}}const
侞:F=7;const 侜:亝=亝{不:3,专:1,丈:1,丝:2,乵:2,仓:2,丯:6,仕:1,};const 依:I=I::MAX;fn 亰()->亂{俘(64)}fn 俘(乣:F)->亂{let 乺=亟::仜{丽:侞,乣,乐:侜,};let 业=侣.iter().map(|&乞|Y{伦:依,..乞}).collect();亂::new(乺,业,Y::伂)}fn
main(){let 俴=io::stdin();let 俵=io::stdout();if let Err(互)=俜(&丿,亰(),俴.lock(),俵.lock()){eprintln!("plan: {}",互);std::process::exit(1);}}const 侩:&str="CG_TIME_SCALE";fn
俽()->亜{俙(std::env::var(侩).ok().as_deref())}fn 俙(丛:P<&str>)->亜{丛.and_then(|乘|乘.trim().parse::<亜>().ok()).filter(|仲|仲.is_finite()&&*仲>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乒:F=4*O;const 乥:U=(1<<乒)-1;const 付:U=1|1<<12|1<<24|1<<36;const 任:U=付<<11;const 伶:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const Q:G=G([0;3]);const 侀:G=G([乥;3]);#[inline]fn q(q:F)->G{let mut 串=G::Q;串.set(q);串}#[inline]fn test(self,q:F)->T{(self.0[q/乒]>>(q%乒))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乒]|=1<<(q%乒);}#[inline]fn clear(&mut self,q:F){self.0[q/乒]&=!(1<<(q%乒));}#[inline]fn is_empty(self)->T{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
仵(self)->[U;3]{self.0}#[inline]fn count(self)->丱{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 两(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仹{仹{伜:self.0,丹:0,}}#[inline]fn 俯(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伶)<<(乒-O)),(b>>O)|((c&伶)<<(乒-O)),c>>O,])}#[inline]fn 俬(self)->G{let[a,b,c]=self.0;G([(a<<O)&乥,((b<<O)&乥)|(a>>(乒-O)),((c<<O)&乥)|(b>>(乒-O)),])}#[inline]fn
俭(self)->G{let[a,b,c]=self.0;G([(a>>1)&!任&乥,(b>>1)&!任&乥,(c>>1)&!任&乥,])}#[inline]fn 修(self)->G{let[a,b,c]=self.0;G([(a<<1)&!付&乥,(b<<1)&!付&乥,(c<<1)&!付&乥,])}#[inline]fn
世(self)->G{let j=self;j.俯()|j.俬()|j.俭()|j.修()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::侀.两(self)}}struct 仹{伜:[U;3],丹:F,}impl Iterator for 仹{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.丹<3{let 仪=j.伜[j.丹];if 仪!=0{j.伜[j.丹]=仪&(仪-1);return Some(j.丹*乒+仪.trailing_zeros()as F);}j.丹+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{亁{丐:丱,x:I,y:I},书{一:I,x:I,y:I},乾{x:I,y:I},亐{x:I,y:I},}impl L{fn 俿(&self)->丰{match*self{L::亁{丐,x,y}=>format!("MOVE {} {x} {y}",丐),L::书{一,x,y}=>format!("TRAIN {} {x} {y}",一),L::乾{x,y}=>format!("BUILD MINE {x} {y}"),L::亐{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
侵(七:&[L])->丰{if 七.is_empty(){return "WAIT".to_string();}let mut line=丰::with_capacity(16*七.len());for(与,下)in 七.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&下.俿());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乤{伻,倍,侱,#[default]仺,}impl 乤{fn 乩(self)->I{match self{乤::伻=>1,_=>3,}}fn 仞(self)->T{matches!(self,乤::侱|乤::仺)}fn 伅(self)->T{self==乤::仺}}const 乚:I=I::MAX;struct
乭{丷:G,valid:G,临:G,久:Box<[I]>,世:[[[I;4];M];2],}thread_local!{static 侌:RefCell<P<Arc<乭>>> =const{RefCell::new(None)};}impl 乭{fn 俫(丷:G,临:G)->Arc<乭>{侌.with(|乶|{let
mut 乶=乶.borrow_mut();if let Some(亩)=乶.as_ref(){if 亩.丷==丷&&亩.临==临{return Arc::clone(亩);}}let 亩=Arc::new(乭::build(丷,临));*乶=Some(Arc::clone(&亩));亩})}fn build(丷:G,临:G)->乭{let
乬=!丷;const 侦:[[(仗,仗);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 世=[[[乚;4];M];2];for(z,乡)in 侦.iter().enumerate(){for q in
乬.iter(){let(x,y)=((q%O)as 仗,(q/O)as 仗);let mut 丑=0;for&(俁,係)in 乡{let(佤,佥)=(x+俁,y+係);if(0..O as 仗).contains(&佤)&&(0..O as 仗).contains(&佥){let 丂=佥 as F*O+佤
as F;if 乬.test(丂){世[z][q][丑]=丂 as I;丑+=1;}}}}}let mut 久=vec![乚;M*M].into_boxed_slice();let mut 伢=[0u8;M];for 丌 in 乬.iter(){let 事=&mut 久[丌*M..(丌+1)*M];事[丌]=0;伢[0]=丌
as I;let(mut 伕,mut 伪)=(0,1);while 伕<伪{let q=伢[伕]as F;伕+=1;for&丂 in&世[0][q]{if 丂!=乚&&事[丂 as F]==乚{事[丂 as F]=事[q]+1;伢[伪]=丂;伪+=1;}}}}乭{丷,valid:乬,临,久,世,}}#[inline]fn
久(&self,丌:F,乫:F)->I{self.久[丌*M+乫]}}const 侖:丱=100;const 亏:C=15;const 侭:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乧(F),伯,}enum
乙{仿,伳,伴,}#[derive(Clone)]struct J{map:Arc<乭>,丨:乤,丒:[G;2],丄:[G;2],乆:G,乇:G,丅:[G;2],一:[G;3],亇:G,亸:[丱;M],丈:[C;2],专:[C;2],东:F,了:丱,亭:丱,丕:P<么>,}fn 仇(丒:G,亦:F)->G{if!丒.test(亦){return
G::Q;}let mut 仐=G::q(亦);loop{let 丂=(仐|仐.世())&丒;if 丂==仐{return 仐;}仐=丂;}}impl J{fn 东(&self)->F{self.东}fn 了(&self)->丱{self.了}fn 丕(&self)->P<么>{self.丕}fn 丨(&self)->乤{self.丨}fn
京(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
专(&self,z:F)->C{self.专[z]}fn 伫(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&乭{&self.map}fn 丟(&self,z:F)->G{self.丒[z]}fn 丄(&self,z:F)->G{self.丄[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 仔(&self,q:F)->P<(丱,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亸[q],j.伛(q)))}fn 予(&self)->G{self.丅[self.东]&self.亇}fn
丞(&self)->G{self.乆}fn 乢(&self)->G{self.乇}fn 亙(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伛(q))}fn 俳(&mut self,z:F){let j=self;j.丄=[仇(j.丒[0],Z[0]),仇(j.丒[1],Z[1])];let
侽=j.丅[z].两(j.丄[z]);j.伤(侽);j.佇();j.丈[z]+=j.专[z];if j.丈[z]<0{j.丈[z]=0;j.伤(j.丅[z]);}j.亇|=j.丅[z];}fn 佇(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
丅=丄&j.丅[z];let mut 专=C::from(丄.count())+4*C::from((丄&j.乆).count());for(一,倅)in 侭.iter().enumerate(){专-=倅*C::from((丅&j.一[一]).count());}j.专[z]=专;}}fn 仄(&mut
self){let j=self;if j.丕.is_some(){return;}if j.东==1{if j.了>=侖{j.丕=Some(match j.京(0).cmp(&j.京(1)){std::cmp::Ordering::Greater=>么::乧(0),std::cmp::Ordering::Less=>么::乧(1),std::cmp::Ordering::Equal=>么::伯,});return;}j.了+=1;}j.东=1-j.东;j.俳(j.东);}fn
佪(&mut self,七:&[L]){let j=self;if j.丕.is_some(){return;}let 伔=j.亭;for&下 in 七{if j.俀(下,伔){continue;}j.个(下);if j.丕.is_some(){return;}}j.仄();}fn 俀(&self,下:L,伔:丱)->T{let
j=self;match 下{L::亁{丐,x,y}=>{j.亡(x,y).is_none()||丐>=伔||j.体(j.东,丐).is_none()}L::书{一,x,y}=>{j.亡(x,y).is_none()||一==0||一>j.丨.乩()}L::乾{x,y}|L::亐{x,y}=>{j.亡(x,y).is_none()}}}#[inline]fn
亡(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丷.test(y*O+x)).then_some(y*O+x)}fn 体(&self,z:F,丐:丱)->P<F>{self.丅[z].iter().find(|&q|self.亸[q]==丐)}#[inline]fn
伛(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 侷(&self,乜:F)->G{let 乢=self.乇&self.丒[乜];let 丄=self.丄[乜];乢|(丄&(丄&乢).世())}fn
主(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乜=1-z;let 丅=j.丅[0]|j.丅[1];let 促=j.仃();let 倃=match
一{1 if j.丨==乤::伻=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::Q,};let 主=促|j.丒[乜].两(倃);if 一<3{主.两(j.侷(乜))}else{主}}fn 亖(&self,z:F)->G{self.丄[z]|self.丄[z].世()}fn
仃(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 俍=G::q(Z[0])|G::q(Z[1]);j.map.valid.两(丅|j.乆|j.乇|俍)}fn 个(&mut self,下:L)->T{let j=self;if j.丕.is_some(){return
false;}let z=j.东;let 云=match 下{L::书{一,x,y}=>j.倁(z,一,x,y),L::亁{丐,x,y}=>j.俔(z,丐,x,y),L::乾{x,y}=>j.build(z,true,x,y),L::亐{x,y}=>j.build(z,false,x,y),};let
Some(云)=云 else{return false;};match 云{乙::仿=>{}乙::伳=>j.丄[z]=仇(j.丒[z],Z[z]),乙::伴=>{j.丄=[仇(j.丒[0],Z[0]),仇(j.丒[1],Z[1])];}}j.佇();if!j.丒[0].test(Z[0]){j.丕=Some(么::乧(1));}else
if!j.丒[1].test(Z[1]){j.丕=Some(么::乧(0));}true}fn 伤(&mut self,不:G){let j=self;j.丅[0]=j.丅[0].两(不);j.丅[1]=j.丅[1].两(不);for 一 in&mut j.一{*一=一.两(不);}j.亇=j.亇.两(不);}fn
佸(&mut self,z:F,q:F)->乙{let j=self;j.伤(G::q(q));j.乆.clear(q);j.乇.clear(q);let 乜=1-z;if j.丒[乜].test(q){j.丒[乜].clear(q);j.丒[z].set(q);乙::伴}else if j.丒[z].test(q){乙::仿}else{j.丒[z].set(q);乙::伳}}fn
倁(&mut self,z:F,一:I,x:I,y:I)->P<乙>{let j=self;if 一==0||一>j.丨.乩(){return None;}let q=j.亡(x,y)?;let 丆=10*C::from(一);if j.丈[z]<丆||!j.亖(z).test(q)||!j.主(z,一).test(q){return
None;}let 云=j.佸(z,q);j.佨(z,一,q,j.亭);j.亭=j.亭.saturating_add(1);j.丈[z]-=丆;Some(云)}fn 佨(&mut self,z:F,一:I,q:F,丐:丱){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亸[q]=丐;}fn
俔(&mut self,z:F,丐:丱,x:I,y:I)->P<乙>{let j=self;let 丏=j.亡(x,y)?;let 丌=j.体(z,丐)?;if!j.亇.test(丌){return None;}let 一=j.伛(丌);let 丢=j.主(z,一);let 举=j.map.世[z][丌];if
举.contains(&(丏 as I))&&!丢.test(丏){return None;}let mut 三=j.map.久(丌,丏);let mut 仁=None;for&丂 in 举.iter().take_while(|&&丂|丂!=乚){let 丂=F::from(丂);let 久=j.map.久(丂,丏);if
久<三&&丢.test(丂){三=久;仁=Some(丂);}}let 仁=仁?;let 云=j.佸(z,仁);j.丅[z].clear(丌);j.一[F::from(一)-1].clear(丌);j.亇.clear(丌);j.佨(z,一,仁,丐);Some(云)}fn build(&mut self,z:F,乆:T,x:I,y:I)->P<乙>{let
j=self;let 侴=if 乆{j.丨.仞()}else{j.丨.伅()};let q=j.亡(x,y)?;let 俖=j.map.临.test(q);if!侴||j.丈[z]<亏||俖!=乆||!j.丒[z].test(q)||!j.亖(z).test(q)||!j.仃().test(q){return
None;}if 乆{j.乆.set(q);}else{j.乇.set(q);}j.丈[z]-=亏;Some(乙::仿)}}const 侑:C=1000000000;const 仼:C=0;const 侙:C=1;const 侪:C=2;impl J{fn read(产:&str,上:&str,丨:乤,了:丱,)->Result<J,丰>{let
mut 临=K::new();let mut 丣=产.lines();let 丑=休(&mut 丣,"mine spot count")?;for _ in 0..丑{let[x,y]=乸(亮(&mut 丣,"a mine spot")?,"a mine spot")?;临.push((仡(x)?,仡(y)?));}佐(丣,"the initial input")?;let
mut 丣=上.lines();let mut 伖=[0i64;4];for(丛,丵)in 伖.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[伡]=乸(亮(&mut 丣,丵)?,丵)?;if 伡.abs()>侑{return
Err(format!("{}: {} is out of range",丵,伡));}*丛=伡;}let mut 丷=G::Q;let mut 不=[G::Q;2];let mut 丄=[G::Q;2];for y in 0..O{let 事=亮(&mut 丣,"a map row")?.trim();if
事.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,事));}for(x,仑)in 事.bytes().enumerate(){let q=y*O+x;match 仑{b'#'=>丷.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乀=F::from(仑==b'X'||仑==b'x');不[乀].set(q);if 仑.is_ascii_uppercase(){丄[乀].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仑 as char))}}}}let
侹=休(&mut 丣,"building count")?;let mut 乏=K::new();for _ in 0..侹{let 乸=乸(亮(&mut 丣,"a building")?,"a building")?;乏.push(乸);}let 伫=休(&mut 丣,"unit count")?;let
mut 丅=K::new();for _ in 0..伫{let 乸=乸(亮(&mut 丣,"a unit")?,"a unit")?;丅.push(乸);}佐(丣,"the turn's input")?;let 佧=乏.iter().find(|佃|佃[0]==0&&佃[1]==仼).ok_or("the bot's own HQ is not among the buildings")?;let
佚=伊(佧[2],佧[3],丷)?;let z=Z.iter().position(|&亦|亦==佚).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佚))?;let mut 佲=G::Q;for(x,y)in 临{if
丷.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佲.set(y*O+x);}let 伃=|乀:F|if 乀==0{z}else{1-z};let mut l=J{map:乭::俫(丷,佲),丨,丒:[G::Q;2],丄:[G::Q;2],乆:G::Q,乇:G::Q,丅:[G::Q;2],一:[G::Q;3],亇:G::Q,亸:[0;M],丈:[0;2],专:[0;2],东:z,了,亭:1,丕:None,};for
乀 in 0..2{let 九=伃(乀);l.丒[九]=不[乀];l.丄[九]=丄[乀];l.丈[九]=伖[2*乀];l.专[九]=伖[2*乀+1];}let mut 佦=G::Q;for[九,仧,x,y]in 乏{let 乀=佭(九)?;let q=伊(x,y,丷)?;if!不[乀].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佦.test(q){return Err(format!("two buildings at ({x}, {y})"));}佦.set(q);match 仧{仼
if q==Z[伃(乀)]=>{}仼=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侙=>l.乆.set(q),侪=>l.乇.set(q),_=>return Err(format!("unknown building type {}",仧)),}}let
mut 乁=G::Q;let mut 仨=0u32;for[九,丐,一,x,y]in 丅{let 乀=佭(九)?;let q=伊(x,y,丷)?;if!不[乀].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乁.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丐=丱::try_from(丐).map_err(|_|format!("unit id {} is out of range",丐))?;if
乁.iter().any(|丩|l.亸[丩]==丐){return Err(format!("two units have id {}",丐));}乁.set(q);l.丅[伃(乀)].set(q);l.一[一 as F-1].set(q);l.亸[q]=丐;仨=仨.max(丐);}if!乁.is_empty(){l.亭=仨.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仨))?;}l.亇=l.丅[z];Ok(l)}}fn
亮<'a>(丣:&mut Lines<'a>,丵:&str)->Result<&'a str,丰>{丣.next().ok_or_else(||format!("the text ends before {}",丵))}fn 佐(mut 丣:Lines<'_>,丵:&str)->Result<(),丰>{match
丣.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丵)),None=>Ok(()),}}fn 乸<const N:F>(line:&str,丵:&str)->Result<[C;N],丰>{let
mut 仵=line.split_whitespace();let mut 佾=[0i64;N];for 丛 in&mut 佾{let 人=仵.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丵))?;*丛=人.parse().map_err(|_|format!("{}: {:?} is not an integer",丵,人))?;}if
仵.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丵));}Ok(佾)}fn 休(丣:&mut Lines<'_>,丵:&str)->Result<F,丰>{let[丑]=乸(亮(丣,丵)?,丵)?;F::try_from(丑).ok().filter(|&丑|丑<=M).ok_or_else(||format!("{}: {} is out of range",丵,丑))}fn
仡(丛:C)->Result<F,丰>{F::try_from(丛).ok().filter(|&丛|丛<O).ok_or_else(||format!("coordinate {} is off the map",丛))}fn 伊(x:C,y:C,丷:G)->Result<F,丰>{let q=仡(y)?*O+仡(x)?;if
丷.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佭(九:C)->Result<F,丰>{match 九{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",九)),}}const
來:C=15;const 侚:C=6;const 伹:C=10;const 侢:I=3;#[derive(Clone,Copy)]struct Y{open:C,仠:C,佊:I,众:C,余:C,佟:C,伦:I,佉:C,佈:C,伽:C,亥:C,亶:C,丶:C,亨:C,于:C,}impl Y{const
伂:Y=Y{open:3,仠:2,佊:12,众:4,余:來,佟:侚,伦:侢,佉:2,佈:2,伽:0,亥:0,亶:0,丶:0,亨:0,于:0,};const fn 個(self,亥:C,亶:C,丶:C)->Y{Y{亥,亶,丶,..self}}const fn 倊(self,亨:C,于:C)->Y{Y{亨,于,..self}}}fn
他(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仂(l:&J,丌:G)->[I;M]{let mut 义=[I::MAX;M];let 乬=l.map().valid;let mut 之=丌&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn 俇(l:&J)->I{let 义=仂(l,l.丟(0));l.丟(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亃<'a>{乞:&'a Y,丁:F,theirs:[I;M],亷:[I;M],free:G,乎:G,ours:G,临:G,}impl 亃<'_>{fn new<'a>(l:&J,乞:&'a Y)->亃<'a>{let 丁=l.东();let 乆=仂(l,l.丟(丁));let 乼=仂(l,l.丟(1-丁));let
亷=仂(l,G::q(Z[1-丁]));let 亓=l.map().valid.两(l.丟(0)|l.丟(1));let mut 乎=G::Q;for q in 亓.iter(){if 乆[q]<=乼[q]{乎.set(q);}}亃{乞,丁,theirs:乼,亷,free:亓,乎,ours:l.丟(丁),临:l.map().临,}}fn
value(&self,q:F)->C{let j=self;let p=j.乞;let 丼=C::from((G::q(q).世()&j.free).count());let 佅=p.佊;let 仠=C::from(佅)-C::from(j.theirs[q].min(佅));let 亦=if p.众>0{C::from(j.亷[q].min(30))/p.众}else{0};let
丒=C::from((G::q(q).世()&j.ours).count());let line=if 丒<=1{p.亶}else{0};let 丶=if j.临.test(q){p.丶}else{0};let 俸=if 丼==0{p.亨}else{0};let 于=p.于*(22-C::from(j.亷[q].min(22)));p.open*丼+p.仠*仠-亦+p.亥*丒-line+丶-俸+于}fn
三(&self,不:G)->P<F>{不.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伍(&mut self,q:F){self.free.clear(q);self.乎.clear(q);self.ours.set(q);}}fn
从(l:&mut J,下:L,万:&mut Vec<L>)->T{let 似=l.个(下);if 似{万.push(下);}似}fn 丙(l:&mut J,丌:F,乫:F,万:&mut Vec<L>)->T{let Some((丐,_))=l.仔(丌)else{return false;};let(x,y)=他(乫);从(l,L::亁{丐,x,y},万)}fn
倄(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伹}#[derive(Clone,Copy)]enum 乃{仛,乿{丞:I,丅:I},}fn 佻(l:&J,乞:&Y,亅:乃)->K<L>{let mut 丬=l.clone();let
mut 上=亃::new(l,乞);let mut 万=K::new();乷(&mut 丬,&mut 上,&mut 万);match 亅{乃::仛=>俟(&mut 丬,&mut 上,&mut 万),乃::乿{丞,丅}=>{俅(&mut 丬,&mut 上,丞,丅,&mut 万)}}万}fn 乷(l:&mut
J,上:&mut 亃<'_>,万:&mut Vec<L>){let 丁=上.丁;let mut 以:K<F> =K::new();loop{let 丢=l.主(丁,1);let mut 亲:P<(丱,F)> =None;for 丌 in l.予().iter(){if 以.contains(&丌){continue;}let
乹=(G::q(丌).世()&上.free&丢).count();if 乹==0{以.push(丌);}else if 亲.is_none_or(|(n,_)|乹<n){亲=Some((乹,丌));}}let Some((_,丌))=亲 else{break;};match 上.三(G::q(丌).世()&上.free&丢){Some(乫)if
丙(l,丌,乫,万)=>上.伍(乫),_=>以.push(丌),}}for 丌 in 以{if!l.予().test(丌){continue;}let 丏=[上.乎,上.free].into_iter().find_map(|不|{不.iter().min_by_key(|&q|(l.map().久(丌,q),q))});if
let Some(丏)=丏{丙(l,丌,丏,万);}}}fn 俟(l:&mut J,上:&mut 亃<'_>,万:&mut Vec<L>){let 丁=上.丁;let p=上.乞;let h=(p.余-C::from(l.了())).max(p.佟);let 俾=(C::from(俇(l))-p.佉).max(0)/p.佈.max(1)+p.伽;loop{let
丈=l.丈(丁);let 丅=l.伫(丁)as C;let 俥=C::from(上.乎.count());let q=上.三(l.亖(丁)&上.free&l.主(丁,1));let 佽=倄(h,(俥/(丅+1)).min(俾));let 佄=q.is_some()&&丈>=伹&&佽>0;let 你=4*h-亏;let
丶=佡(l,上);let 侻=丶.is_some()&&丈>=亏&&你>0;if 侻&&(!佄||你>=佽){let(x,y)=他(丶.unwrap_or_default());if 从(l,L::乾{x,y},万){continue;}break;}let Some(q)=q.filter(|_|佄)else{break;};let(x,y)=他(q);if!从(l,L::书{一:1,x,y},万){break;}上.伍(q);}}fn
佡(l:&J,上:&亃<'_>)->P<F>{let 丁=上.丁;(l.map().临&l.丄(丁)&l.仃()).iter().filter(|&丶|上.theirs[丶]>上.乞.伦).min_by_key(|&丶|(l.map().久(Z[丁],丶),丶))}fn 俅(l:&mut J,上:&mut
亃<'_>,丞:I,丅:I,万:&mut Vec<L>,){let 丁=上.丁;for _ in 0..丞{let Some(丶)=佡(l,上)else{break;};let(x,y)=他(丶);if!从(l,L::乾{x,y},万){break;}}for _ in 0..丅{let Some(q)=上.三(l.亖(丁)&上.free&l.主(丁,1))else{break;};let(x,y)=他(q);if!从(l,L::书{一:1,x,y},万){break;}上.伍(q);}}fn
仱(l:&J)->T{if l.丕().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丯=亠::new(l,丁,true);l.丟(1-丁).iter().any(|q|丯.丆[q]<=丈)}#[derive(Clone,Copy)]struct
両{些:I,亅:乃,}const 伬:I=I::MAX;const 侂:[乃;8]=[乃::仛,乃::乿{丞:0,丅:0},乃::乿{丞:0,丅:1},乃::乿{丞:0,丅:2},乃::乿{丞:0,丅:伬,},乃::乿{丞:1,丅:0},乃::乿{丞:1,丅:1},乃::乿{丞:1,丅:伬,},];struct
伮<'a>{业:&'a[Y],今:Y,rest:両,乱:P<乑>,俦:Settings,}impl 伮<'_>{fn 乞(&self,丙:両)->&Y{&self.业[F::from(丙.些)%self.业.len()]}fn 七(&self,l:&J,丙:両)->K<L>{佻(l,self.乞(丙),丙.亅)}}fn
伧(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伝=|人:U|hash=(hash^人).wrapping_mul(0x0100000001b3);for(俹,串)in[l.丟(丁),l.丅(丁),l.丞()].into_iter().enumerate(){for
q in 串.iter(){伝(((俹 as U)<<8)|q as U);}伝(0xff);}伝(l.丈(丁)as U);hash}enum 亟{仜{丽:F,乣:F,乐:亝,},}struct 亂{乺:亟,业:K<Y>,今:Y,且:K<両>,代:K<U>,亳:F,丮:丱,仰:T,}impl 亂{fn
new(乺:亟,业:K<Y>,今:Y)->亂{亂{乺,业,今,且:K::new(),代:K::new(),亳:0,丮:0,仰:false,}}fn 保(&mut self,l:&J)->T{let j=self;let 俱=matches!(j.乺,亟::仜{..});let 俞=!俱&&j.丮>0&&j.亳>=j.代.len();if!j.仰&&(俞||仱(l)){j.仰=true;}!j.仰}fn
上(&mut self,l:&J,乱:P<乑>)->K<L>{let j=self;let 佌=伮{业:&j.业,今:j.今,rest:両{些:0,亅:乃::仛,},乱,俦:Settings::DEFAULT,};let 丁=l.东();let 俗=j.代.get(j.亳)==Some(&伧(l,丁));if!俗{let
乊=j.丮==0;j.且=match j.乺{亟::仜{丽,乣,乐,}=>{let 乐=&乐;let 乗=価;let mut 乡=且(l,&j.业,乗,丽,乣,乱,乐,);乡.resize(丽,乗);乡}};j.代=match j.乺{亟::仜{..}=>俲(l,&j.业,&j.且),};j.亳=0;j.丮+=1;}let
丙=j.且.get(j.亳).copied().unwrap_or(佌.rest);j.亳+=1;佌.七(l,丙)}}fn 俲(l:&J,业:&[Y],且:&[両])->K<U>{let 丁=l.东();let mut 作=K::with_capacity(且.len());let mut 也=l.clone();for&丙
in 且{if 也.丕().is_some(){break;}作.push(伧(&也,丁));也=伄(&也,&七(&也,业,丙));}作}const 侫:C=3;struct Settings{丽:C,}impl Settings{const DEFAULT:Settings=Settings{丽:侫};}fn
乡(l:&J,丌:G)->[I;M]{let 乬=l.map().valid;let mut 义=[I::MAX;M];let mut 之=丌&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn
倆(l:&J,z:F)->C{let 仌=乡(l,l.丄(z));let 乼=乡(l,l.丄(1-z));l.map().valid.iter().map(|q|match 仌[q].cmp(&乼[q]){_ if 仌[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侣:[Y;27]=业();const 価:両=両{些:16,亅:乃::仛,};const fn 业()->[Y;27]{let 亥=[0,2,4];let 亶=[0,3,6];let 于=[0,1,2];let mut 万=[Y::伂;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
亨=if g>0||t>0{4}else{0};万[i]=Y::伂.個(亥[g],亶[t],0).倊(亨,于[w]);i+=1;}万}#[derive(Default)]struct 企{不:C,专:C,丈:C,乵:C,丝:C,仓:C,丯:C,仕:C,}#[derive(Clone,Copy)]struct
亝{不:C,专:C,丈:C,丝:C,乵:C,仓:C,丯:C,仕:C,}fn 俧(l:&J,z:F)->[C;M]{struct 亼{丄:G,乄:[u16;M],丳:[u16;M],size:[C;M],丝:[C;M],亍:u16,}fn 亚(d:&mut 亼,v:F){d.亍+=1;d.乄[v]=d.亍;d.丳[v]=d.亍;d.size[v]=1;for
w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{亚(d,w);d.size[v]+=d.size[w];d.丳[v]=d.丳[v].min(d.丳[w]);if d.丳[w]>=d.乄[v]{d.丝[v]+=d.size[w];}}else{d.丳[v]=d.丳[v].min(d.乄[w]);}}}let
mut d=亼{丄:l.丄(z),乄:[0;M],丳:[0;M],size:[0;M],丝:[0;M],亍:0,};let 乪=Z[z];if d.丄.test(乪){亚(&mut d,乪);}d.丝[乪]=0;d.丝}fn 俩(l:&J,z:F)->企{let 丄=l.丄(z);let 丝=俧(l,z);let
亷=仂(l,G::q(Z[1-z]));let mut 万=企{不:C::from(丄.count()),专:l.专(z),丈:l.丈(z),丯:C::MAX,仕:倆(l,z)/2,..企::default()};for q in 丄.iter(){if 丝[q]>0{万.乵+=1;万.丝=万.丝.max(丝[q]);}if
q!=Z[z]&&(G::q(q).世()&丄).count()<=1{万.仓+=1;}万.丯=万.丯.min(C::from(亷[q]));}万}fn 丛(l:&J,z:F,乐:&亝)->C{let s=俩(l,z);let o=乐;o.不*s.不+o.专*s.专+o.丈*s.丈-o.丝*s.丝-o.乵*s.乵-o.仓*s.仓-o.丯*s.丯.min(30)+o.仕*s.仕}fn
伄(l:&J,七:&[L])->J{let mut 丂=l.clone();丂.佪(七);if 丂.丕().is_none(){丂.佪(&[]);}丂}fn 七(l:&J,业:&[Y],丙:両)->K<L>{佻(l,&业[F::from(丙.些)%业.len()],丙.亅,)}fn 佩(l:&J,业:&[Y],且:&[両],乗:両,丽:F,)->J{let
mut 也=l.clone();for 上 in 0..丽{if 也.丕().is_some(){break;}let 丙=且.get(上).copied().unwrap_or(乗);也=伄(&也,&七(&也,业,丙));}也}fn 俶(l:&J,业:&[Y])->K<(両,K<L>)>{let mut
万:K<(両,K<L>)> =K::new();for 些 in 0..业.len().min(F::from(I::MAX)){for&亅 in&侂{let 丙=両{些:些 as I,亅,};let c=七(l,业,丙);if!万.iter().any(|(_,丩)|*丩==c){万.push((丙,c));}}}万}fn
侶(l:&J,业:&[Y],乗:両,丽:F,乣:F,乱:P<乑>,乐:&亝,)->K<両>{let 丁=l.东();let 仍=||乱.is_some_and(|t|乑::now()>=t);let 仏=|且:&[両]|丛(&佩(l,业,且,乗,丽),丁,乐);let mut 三:(C,K<両>)=(仏(&[]),K::new());let
mut 之:K<(J,K<両>)> =vec![(l.clone(),K::new())];for 便 in 0..丽{let mut 乯:K<(C,J,K<両>)> =K::new();let mut 乁:HashMap<U,F> =HashMap::new();for(佣,且)in&之{if 仍(){break;}for(丙,c)in
俶(佣,业){let 丂=伄(佣,&c);let mut 伌=且.clone();伌.push(丙);let v=丛(&丂,丁,乐);let 乌=伧(&丂,丁);match 乁.get(&乌){Some(&丹)if 乯[丹].0>=v=>{}Some(&丹)=>乯[丹]=(v,丂,伌),None=>{乁.insert(乌,乯.len());乯.push((v,丂,伌));}}}}if
乯.is_empty(){break;}乯.sort_by(|a,b|b.0 .cmp(&a.0));乯.truncate(乣);let 乶=便+1==丽||仍();for(i,(_,_,且))in 乯.iter().take(if 乶{乣}else{1}).enumerate(){if i>0&&仍(){break;}let
v=仏(且);if v>三.0{三=(v,且.clone());}}if 乶{break;}之=乯.into_iter().map(|(_,s,p)|(s,p)).collect();}三.1}fn 且(l:&J,业:&[Y],乗:両,丽:F,俒:F,乱:P<乑>,乐:&亝,)->K<両>{let 丁=l.东();let
仏=|且:&[両]|丛(&佩(l,业,且,乗,丽),丁,乐);let mut 三:(C,K<両>)=(仏(&[]),K::new());let mut 乣=2;loop{let 丧=侶(l,业,乗,丽,乣,乱,乐);let v=仏(&丧);if v>三.0{三=(v,丧);}let 仍=乱.is_some_and(|t|乑::now()>=t);if
仍||(乱.is_none()&&乣>=俒){break;}乣*=2;}三.1}const 侅:F=4+12;fn 丽(优:impl BufRead,mut 仯:impl Write,mut 伆:impl FnMut(&str,&str,丱,乑)->丰,)->Result<(),丰>{let mut
丣=伱{佬:优};let Some(乊)=丣.line()?else{return Ok(());};let mut 产=format!("{}\n",乊);丣.take(丑(&乊,"mine spot count")?,&mut 产)?;let mut 上=0;loop{let Some(乊)=丣.line()?else{return
Ok(());};let 亘=乑::now();上+=1;let mut 乘=format!("{}\n",乊);丣.take(侅-1,&mut 乘)?;for 丵 in["building count","unit count"]{let line=丣.佮()?;let n=丑(&line,丵)?;乘.push_str(&line);乘.push('\n');丣.take(n,&mut
乘)?;}let 伆=伆(&产,&乘,上,亘);writeln!(仯,"{}",伆).and_then(|()|仯.flush()).map_err(|互|format!("turn {}: cannot write the answer: {}",上,互))?;}}struct 伱<R>{佬:R,}impl<R:BufRead>伱<R>{fn
line(&mut self)->Result<P<丰>,丰>{let mut line=丰::new();match self.佬.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(互)=>Err(format!("cannot read input: {}",互)),}}fn
佮(&mut self)->Result<丰,丰>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乘:&mut String)->Result<(),丰>{for
_ in 0..n{乘.push_str(&self.佮()?);乘.push('\n');}Ok(())}}fn 丑(line:&str,丵:&str)->Result<F,丰>{match line.trim().parse::<F>(){Ok(n)if n<=侐=>Ok(n),_=>Err(format!("bad {} {line:?}",丵)),}}const
侐:F=144;const 侧:C=3;const 例:C=2;const 侕:F=16;fn 俛(l:&J,万:&mut Vec<L>){let z=l.东();let mut 乺=伵{s:l.clone(),z,乲:1-z,丏:Z[1-z],令:Z[z],上:l.了(),万,};乺.run();}struct
伵<'a>{s:J,z:F,乲:F,丏:F,令:F,上:丱,万:&'a mut Vec<L>,}fn 价(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伞(丐:丱,q:F)->L{let(x,y)=价(q);L::亁{丐,x,y}}fn 佺(一:I,q:F)->L{let(x,y)=价(q);L::书{一,x,y}}impl
伵<'_>{fn run(&mut self){let j=self;if j.仩(){return;}j.乷();if j.仩(){return;}j.乏();j.倂();}fn 义(&self,丌:F,乫:F)->C{C::from(self.s.map().久(丌,乫))}fn 个(&mut self,下:L)->T{let
为=self.s.个(下);if 为{self.万.push(下);}为}fn 亹(&self,丠:&J)->T{丠.丕()==Some(么::乧(self.z))}fn 仩(&mut self)->T{let j=self;if j.s.丕().is_some(){return false;}for
q in j.s.予().iter(){if j.义(q,j.丏)!=1{continue;}let Some((丐,_))=j.s.仔(q)else{continue;};let 下=伞(丐,j.丏);let mut 丠=j.s.clone();if 丠.个(下)&&j.亹(&丠){return j.个(下);}}let
Some(並)=j.侼()else{return false;};let mut 丠=j.s.clone();if!並.iter().all(|&下|丠.个(下))||!j.亹(&丠){return false;}for 下 in 並{j.个(下);}true}fn 侼(&self)->P<K<L>>{let
j=self;const 估:C=C::MAX;let 乕=j.s.丨().乩();let 丢:K<G> =(1..=乕).map(|一|j.s.主(j.z,一)).collect();let 仅=|q:F|{丢.iter().position(|串|串.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 丄=j.s.丄(j.z);let mut 丆=[估;M];let mut 严=[F::MAX;M];let mut 为=[false;M];for q in 丄.iter(){丆[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!为[q]&&丆[q]<估&&丂.is_none_or(|n|丆[q]<丆[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};为[q]=true;if q==j.丏{break;}for&举 in&j.s.map().世[0][q]{if 举==乚{break;}let 举=F::from(举);if 为[举]||丄.test(举){continue;}let Some((_,争))=仅(举)else{continue;};if
丆[q]+争<丆[举]{丆[举]=丆[q]+争;严[举]=q;}}}if 丆[j.丏]>j.s.丈(j.z){return None;}let mut 並=K::new();let mut q=j.丏;while!丄.test(q){let(一,_)=仅(q)?;並.push(佺(一,q));q=严[q];}並.reverse();Some(並)}fn
乨(&self,丸:&J)->C{let(丁,二)=(self.z,self.乲);let 严=&self.s;let 丑=|串:G|C::from(串.count());let 丒=丑(丸.丟(丁))-丑(严.丟(丁));let 仴=丑(严.丟(二))-丑(丸.丟(二));let 俏=丑(严.丄(二))-丑(丸.丄(二));let
仦=严.京(二)-丸.京(二);let 丞=丑(严.丞()&严.丟(二))-丑(丸.丞()&丸.丟(二));12*丒+20*仴+8*(俏-仴).max(0)+5*仦+25*丞}fn 佳(&self,q:F)->C{let j=self;let 丶=j.s.map().临.test(q);if 丶&&j.s.丨().仞()&&!j.s.丟(j.z).test(q){20}else{0}}fn
佑(&self,丿:C)->T{let j=self;j.s.丅(j.乲).iter().any(|q|j.义(q,j.令)<=丿)}fn 乷(&mut self){let j=self;let mut 丅:K<(C,F)> =j.s.予().iter().map(|q|(j.义(q,j.丏),q)).collect();丅.sort_unstable();for(俌,q)in
丅{if j.s.丕().is_some(){return;}let Some((丐,一))=j.s.仔(q)else{continue;};let 丢=j.s.主(j.z,一);let 举=j.s.map().世[j.z][q];let mut 三:P<(C,F)> =None;for&丂 in 举.iter().take_while(|&&丂|丂!=乚){let
丂=F::from(丂);if!丢.test(丂){continue;}let mut 丠=j.s.clone();if!丠.个(伞(丐,丂)){continue;}let 丛=if j.亹(&丠){100000}else{j.乨(&丠)+10*(俌-j.义(丂,j.丏))+j.佳(丂)};if 三.is_none_or(|(乽,_)|丛>乽){三=Some((丛,丂));}}if
let Some((丛,丂))=三{if 丛>0{j.个(伞(丐,丂));}}}}fn 乏(&mut self){let j=self;if j.s.丕().is_some(){return;}if j.佑(侧){j.倀();}else{j.丞();}}fn 倀(&mut self){let j=self;if!j.s.丨().伅()||j.s.丈(j.z)<亏{return;}let
举=j.s.map().世[j.z][j.令];let 不:K<F> =举.iter().take_while(|&&q|q!=乚).map(|&q|F::from(q)).collect();let 乆=j.s.丟(j.z);if 不.iter().any(|&q|乆.test(q)&&j.s.乢().test(q)){return;}for
q in 不{let(x,y)=价(q);if j.个(L::亐{x,y}){return;}}}fn 丞(&mut self){let j=self;if!j.s.丨().仞(){return;}while j.s.丈(j.z)>=亏{let 亓=j.s.map().临&j.s.丟(j.z)&j.s.丄(j.z)&j.s.仃();let
Some(q)=亓.iter().min_by_key(|&q|(j.义(q,j.令),q))else{return;};let(x,y)=价(q);if!j.个(L::乾{x,y}){return;}}}fn 倂(&mut self){let j=self;let 佯=if j.上<=3{0}else{5};for
_ in 0..侕{if j.s.丕().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俎=j.佑(例);let 仭=j.s.亖(j.z);let mut 三:P<(C,L)> =None;let mut 伩:P<(C,L)> =None;let
mut 佂=G::Q;for 一 in 1..=j.s.丨().乩(){let 丢=j.s.主(j.z,一);let 乹=(仭&丢).两(佂);佂=丢;let 争=10*C::from(一);if 争>丈{continue;}for q in 乹.iter(){let 下=佺(一,q);let mut
丠=j.s.clone();if!丠.个(下){continue;}let 会=丠.丈(j.z);let 专=丠.专(j.z);let 仦=j.s.京(j.乲)-丠.京(j.乲);let 侲=if 仦>0&&俎{会+专>=0}else{专>=0&&(会>=佯||仦>0)};if!侲{continue;}let
乨=j.乨(&丠);let 丛=乨+j.佳(q)-6*j.义(q,j.丏)-10*(C::from(一)-1);if 乨>0{if 三.is_none_or(|(乽,_)|丛>乽){三=Some((丛,下));}}else if 专>=3&&会>=佯+10&&一==1{if 伩.is_none_or(|(乽,_)|丛>乽){伩=Some((丛,下));}}}}let
Some((_,下))=三.or(伩)else{return;};if!j.个(下){return;}}}}fn 俊(l:&J,万:&mut Vec<L>){万.clear();俛(l,万);}const 亄:C=C::MAX;const 侔:F=8;struct 亠{丆:[C;M],严:[I;M],}fn
伐(l:&J,z:F,乷:T,丏:F)->C{佋(l,z,井(l,z,乷),丏)}fn 佋(l:&J,z:F,井:G,丏:F)->C{let mut 丆=亄;佔(l,z,井,|丙,new|{if new.test(丏){丆=10*丙 as C;}丆!=亄});丆}fn 俠(l:&J,z:F,井:G,丫:C)->G{let
mut 仱=G::Q;佔(l,z,井,|丙,new|{if 10*丙 as C>丫{return true;}仱|=new;false});仱.两(l.丄(z))}fn 井(l:&J,z:F,乷:T)->G{if 乷&&z==l.东(){l.予()}else{G::Q}}fn 佔(l:&J,z:F,井:G,mut
亚:impl FnMut(F,G)->T,){let 乕=l.丨().乩();let mut 仅=[G::Q;3];let mut 乁=G::Q;for 一 in 1..=乕{let 串=l.主(z,一);仅[F::from(一)-1]=串.两(乁);乁|=串;}let mut 丼=[l.丄(z),G::Q,G::Q,G::Q];for
q in(井&l.丅(z)).iter(){if let Some(一)=l.亙(q).filter(|&一|一<=乕){丼[0]|=G::q(q).世()&l.主(z,一);}}let mut 为=G::Q;for 丙 in 0..{let new=丼[丙%4].两(为);丼[丙%4]=G::Q;if
亚(丙,new){return;}为|=new;let 丂=new.世().两(为);for(侳,不)in 仅.iter().enumerate(){丼[(丙+侳+1)%4]|=丂&*不;}if 丼.iter().all(|串|串.两(为).is_empty()){break;}}}impl 亠{fn
new(l:&J,z:F,乷:T)->亠{let 乕=F::from(l.丨().乩());let mut 丢=[G::Q;3];for(与,串)in 丢.iter_mut().enumerate().take(乕){*串=l.主(z,与 as I+1);}let 丄=l.丄(z);let 世=&l.map().世[0];let
mut 丆=[亄;M];let mut 严=[乚;M];let mut 丼=[0u8;M];let mut 丑=0;let mut 佱=G::Q;for q in 丄.iter(){丆[q]=0;丼[丑]=q as I;丑+=1;}if 乷&&z==l.东(){for 丌 in l.予().iter(){let
Some(一)=l.亙(丌)else{continue;};let 乡=G::q(丌).世()&丢[F::from(一)-1];for q in 乡.iter(){if 丆[q]!=0{丆[q]=0;丼[丑]=q as I;丑+=1;}}}}while 丑>0{let mut 三=0;for 与 in
1..丑{let(a,b)=(F::from(丼[与]),F::from(丼[三]));if(丆[a],a)<(丆[b],b){三=与;}}let q=F::from(丼[三]);丑-=1;丼[三]=丼[丑];佱.set(q);for&丂 in&世[q]{if 丂==乚{break;}let 丂=F::from(丂);if
佱.test(丂)||丄.test(丂){continue;}let Some(一)=丢[..乕].iter().position(|串|串.test(丂))else{continue;};let 佹=丆[q]+10*(一 as C+1);if 佹<丆[丂]{if 丆[丂]==亄{丼[丑]=丂 as
I;丑+=1;}丆[丂]=佹;严[丂]=q as I;}}}亠{丆,严}}fn path(&self,丏:F)->K<F>{let mut path=K::new();if 丏>=M||self.丆[丏]==亄{return path;}let mut q=丏;while self.严[q]!=乚{path.push(q);q=F::from(self.严[q]);}path.reverse();path}}struct
仙{七:K<L>,丆:C,}fn 俑(l:&J,z:F,q:F)->P<I>{(1..=l.丨().乩()).find(|&一|l.主(z,一).test(q))}fn 俣(丬:&mut J,丁:F,交:F,丫:C,七:&mut Vec<L>,习:&mut i64,)->T{loop{if 丬.丕()==Some(么::乧(丁)){return
true;}if 丬.丟(丁).test(交)&&丬.丄(丁).test(交){return true;}let 丯=亠::new(丬,丁,false);let 丆=丯.丆[交];if 丆==亄||*习+丆>丫{return false;}let Some(&q)=丯.path(交).first()else{return
false;};let Some(一)=俑(丬,丁,q)else{return false;};let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(下){return false;}*习+=10*C::from(一);七.push(下);}}fn 佀(l:&J,丁:F,俉:&[F],丫:C)->P<仙>{let
mut 丬=l.clone();let mut 七=K::new();let mut 习=0;for&交 in 俉{if!俣(&mut 丬,丁,交,丫,&mut 七,&mut 习){return None;}}(习<=丫).then_some(仙{七,丆:习,})}fn 伋(l:&J,丏:F,丫:C)->P<仙>{if
丫<0||l.丕().is_some()||丏>=M||l.map().丷.test(丏){return None;}let 丁=l.东();if l.丟(丁).test(丏)&&l.丄(丁).test(丏){return Some(仙{七:K::new(),丆:0,});}let mut 三=佀(l,丁,&[丏],丫);let
乊=亠::new(l,丁,false);let mut 乢:K<F> =(l.乢()&l.丟(1-丁)).iter().filter(|&乇|乇!=丏&&乊.丆[乇]!=亄).collect();乢.sort_by_key(|&乇|(l.map().久(乇,丏),乇));乢.truncate(侔);for
乇 in 乢{if let Some(並)=佀(l,丁,&[乇,丏],丫){if 三.as_ref().is_none_or(|乽|並.丆<乽.丆){三=Some(並);}}}三}const 侒:F=20000;struct 仚{七:K<L>,丆:C,不:C,丅:C,乏:C,专:C,丘:C,}struct
份{不:C,丅:C,乏:C,}fn 佘(l:&J,z:F)->份{let 丄=l.丄(z);let 丅=l.丅(z)&丄;let 丛=丅.iter().filter_map(|q|l.亙(q)).map(|一|10*C::from(一)).sum();份{不:C::from(丄.count()),丅:丛,乏:C::from(((l.丞()|l.乢())&丄).count()),}}struct
伷{丁:F,仸:G,严:份,专:C,享:F,丫:C,亯:F,path:K<L>,丧:K<仚>,index:HashMap<亾,F,BuildHasherDefault<DefaultHasher>>,}type 亾=[U;3];fn 倉(mut 乌:亾,q:F)->亾{乌[q/64]|=1<<(q%64);乌}impl
伷{fn 住(&mut self,丬:&J,乶:P<F>,仴:亾,丆:C,)->T{let j=self;let 丁=j.丁;let 乹=match 乶{None=>丬.亖(丁),Some(q)=>G::q(q).世(),}&j.仸;let 乹=乹.两(丬.丟(丁));let mut 丢=[G::Q;3];for
一 in 1..=丬.丨().乩(){丢[F::from(一)-1]=丬.主(丁,一);}for q in 乹.iter(){let 乕=F::from(丬.丨().乩());let Some(与)=丢[..乕].iter().position(|串|串.test(q))else{continue;};let
一=与 as I+1;let 争=10*C::from(一);if 丆+争>j.丫{continue;}if j.亯>=侒{return false;}j.亯+=1;let mut 丂=丬.clone();let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(下){continue;}j.path.push(下);let
亵=倉(仴,q);j.信(&丂,亵,丆+争);let 侾=j.path.len()<j.享&&丂.丕().is_none();let 俈=!侾||j.住(&丂,Some(q),亵,丆+争);j.path.pop();if!俈{return false;}}true}fn 信(&mut self,丸:&J,亵:亾,丆:C){let
j=self;let 介=1-j.丁;let 传=佘(丸,介);let 不=j.严.不-传.不;let 丅=j.严.丅-传.丅;if 不<=0&&丅<=0{return;}let 乏=j.严.乏-传.乏;let 专=丸.专(j.丁)-j.专;let 倌=丸.丕()==Some(么::乧(j.丁));let
丘=if 倌{C::MAX/2}else{3*不+丅+10*乏+2*专-丆};if let Some(&丹)=j.index.get(&亵){if 丆>=j.丧[丹].丆{return;}}let 丝=仚{七:j.path.clone(),丆,不,丅,乏,专,丘,};match j.index.get(&亵){Some(&丹)=>j.丧[丹]=丝,None=>{j.index.insert(亵,j.丧.len());j.丧.push(丝);}}}}fn
位(l:&J,享:F,丫:C,丿:F)->K<仚>{乍(l,享,丫,丿).0}fn 乍(l:&J,享:F,丫:C,丿:F)->(K<仚>,F){if l.丕().is_some()||享==0||丿==0{return(K::new(),0);}let 丁=l.东();let 介=1-丁;let mut
伥=伷{丁,仸:l.丟(介)|l.丟(介).世(),严:佘(l,介),专:l.专(丁),享,丫,亯:0,path:K::new(),丧:K::new(),index:HashMap::default(),};伥.住(l,None,[0;3],0);let 亯=伥.亯;let mut 丧=伥.丧;丧.sort_by(|a,b|b.丘.cmp(&a.丘).then(a.丆.cmp(&b.丆)));丧.truncate(丿);(丧,亯)}const
仝:C=1000000;const 侓:C=200;fn 乎(l:&J,z:F)->C{let 丄=l.丄(z);let 丅:C=(l.丅(z)&丄).iter().filter_map(|q|l.亙(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.专(z)+4*C::from(丄.count())+丅+10*C::from((l.乢()&丄).count())}fn
佞(l:&J,z:F,乖:G)->C{俆(l,z,佋(l,z,乖,Z[1-z]),)}fn 俆(l:&J,z:F,丆:C)->C{if 丆==亄{return C::MIN;}l.丈(z)+l.专(z)-丆}fn 佫(仫:C)->C{仫.clamp(-侓,0)}fn 俄(l:&J,z:F,乖:[G;2],)->(C,[C;2]){let
丩=1-z;match l.丕(){Some(么::乧(倈))=>{return(if 倈==z{仝}else{-仝},[0,0])}Some(么::伯)=>return(0,[0,0]),None=>{}}let mut 丘=乎(l,z)-乎(l,丩);if z==l.东(){let mut 丂=l.clone();丂.仄();if
丂.丕().is_none()&&伐(&丂,丩,true,Z[z])<=丂.丈(丩){丘-=仝/2;}}let 亪=[佞(l,z,乖[0]),佞(l,丩,乖[1]),];(丘+佫(亪[0])-佫(亪[1]),亪)}
