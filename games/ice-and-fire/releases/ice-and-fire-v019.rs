// Bundled by cg-bundler from the `ice-and-fire-bot-plan` package. Minified (ADR 0045).
type F=usize;type I=u8;type 丱=u32;type U=u64;type 付=i32;type C=i64;type 些=f64;type T=bool;type 丰=String;type 乑=Instant;type K<T>=Vec<T>;type P<T>=Option<T>;use
std::cell::RefCell;use std::collections::HashMap;use std::collections::hash_map::DefaultHasher;use std::hash::BuildHasherDefault;use std::io::{self,BufRead,Write};use
std::ops::{BitAnd,BitOr,BitOrAssign,Not};use std::str::Lines;use std::sync::Arc;use std::time::{Duration,Instant};const 侅:T=true;fn 俇(l:&J,q:F,仓:C,丯:G,並:C)->T{let
丁=l.东();let 乲=1-丁;let 余=侅&&並<=l.丈(乲)+2*l.专(乲).max(0);let 仹=G::q(q)|G::q(q).世();if!余&&(仹&丯).is_empty(){return false;}let(x,y)=人(q);let mut 丸=l.clone();if!丸.个(L::亐{x,y}){return
false;}if 俷(&丸,丁)<仓{return true;}余&&代(&丸,乲,false,Z[丁])>並}const 例:F=4;const 伬:F=5;#[derive(Clone)]enum 乂{亾(L),仿(K<L>),}impl 乂{fn 七(&self)->&[L]{match self{乂::亾(下)=>std::slice::from_ref(下),乂::仿(七)=>七,}}}fn
人(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 体(l:&J,万:&mut Vec<乂>){万.clear();if l.丕().is_some(){return;}let 丁=l.东();let 乕=l.丨().乩();let mut 丢=[G::Q;4];for 一
in 1..=乕{丢[F::from(一)]=l.主(丁,一);}for 不 in l.予().iter(){let Some((丏,一))=l.仕(不)else{continue;};let 乡=G::q(不).世()&丢[F::from(一)];for 乫 in 乡.iter(){let(x,y)=人(乫);万.push(乂::亾(L::亁{丏,x,y}));}}let
丈=l.丈(丁);let 仮=l.亖(丁);for 一 in 1..=乕{let 丆=10*C::from(一);if 丈<丆{break;}let mut 丌=仮&丢[F::from(一)];for 俌 in 1..一{丌=丌.两(丢[F::from(俌)]);}for q in 丌.iter(){let(x,y)=人(q);万.push(乂::亾(L::书{一,x,y}));}}if
丈>=亏{let 亓=l.丟(丁)&仮&l.仂();if l.丨().仟(){for q in(亓&l.map().临).iter(){let(x,y)=人(q);万.push(乂::亾(L::乾{x,y}));}}if l.丨().伆(){let 侴=l.map().valid.两(l.丟(丁)).世();let
丌=(亓&侴).两(l.map().临);let 丯=丯(l,1-丁);let 仓=C::from((丯&l.丄(丁)).count());let 並=代(l,1-丁,false,Z[丁]);for q in 丌.iter().filter(|&q|俇(l,q,仓,丯,並)){let(x,y)=人(q);万.push(乂::亾(L::亐{x,y}));}}}for
丝 in 佋(l,伬,丈,例){if 丝.七.len()>1{万.push(乂::仿(丝.七));}}if let Some(並)=伌(l,Z[1-丁],丈){if 並.七.len()>1{万.push(乂::仿(並.七));}}}const 伱:I=2;const 侤:C=5;fn 今(l:&J,z:F)->G{let
mut 今=G::Q;if 伱>3{return 今;}for q in(l.丅(z)&l.丄(z)).iter(){if l.亙(q).is_some_and(|一|一>=伱){今|=G::q(q);}}今}fn 丯(l:&J,乎:F)->G{let 丫=l.丈(乎)+l.专(乎);俜(l,乎,今(l,乎),丫)}fn
仅(l:&J,z:F)->G{丯(l,1-z)&l.丄(z)}fn 俷(l:&J,z:F)->C{C::from(仅(l,z).count())}const 侕:C=4;fn 俏(丌:C)->C{丌/侕}fn 丞(l:&J,z:F,仅:G)->C{let 丄=l.丄(z);let 丞=(l.丞()&丄).两(仅);俏(C::from(丄.count()))*C::from(丞.count())}const
侠:C=2;fn 佼(l:&J,z:F)->C{let 丄=l.丄(z);let[a,b,c]=丄.件();let mut hash=(a^b.rotate_left(21)^c.rotate_left(42)^z as U).wrapping_mul(0x9e3779b97f4a7c15);hash^=hash>>29;let
与=(hash as F)&(伹-1);侬.with(|伊|{let mut 伊=伊.borrow_mut();let 仴=&mut 伊[与];if 仴.0==丄&&仴.1==z as I+1{return 仴.2;}let 丛=倃(丄,Z[z]);*仴=(丄,z as I+1,丛);丛})}const
伹:F=1<<12;thread_local!{static 侬:std::cell::RefCell<K<(G,I,C)>> =std::cell::RefCell::new(vec![(G::Q,0,0);伹]);}fn 倃(丄:G,乪:F)->C{struct 亻{丄:G,乄:[I;M],丳:[I;M],size:[I;M],亍:I,乵:C,仭:C,}fn
乽(d:&mut 亻,v:F)->I{d.亍+=1;let 乄=d.亍;d.乄[v]=乄;let mut 丳=乄;let mut 伧=1u8;let mut 丝=0u8;for w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{let 佴=乽(d,w);伧+=佴;丳=丳.min(d.丳[w]);if
d.丳[w]>=乄{丝+=佴;}}else{丳=丳.min(d.乄[w]);}}d.丳[v]=丳;d.size[v]=伧;if 丝>0&&d.乄[v]>1{d.乵+=1;d.仭=d.仭.max(C::from(丝));}伧}if!丄.test(乪){return 0;}let mut d=亻{丄,乄:[0;M],丳:[0;M],size:[0;M],亍:0,乵:0,仭:0,};乽(&mut
d,乪);d.乵+d.仭}fn 俤(l:&J,z:F)->C{侠*(佼(l,1-z)-佼(l,z))}fn 佌(l:&J,z:F)->C{if l.丈(z)+l.专(z)>=0{return 0;}(l.丅(z)&l.丄(z)).iter().filter_map(|q|l.亙(q)).map(|一|10*C::from(一)).sum()}const
侊:C=15;const 侪:[C;2]=[3,1];const 侨:[C;2]=[4,20];fn 俳(l:&J,z:F)->[C;2]{let mut 丑=[0;2];for q in(l.丅(z)&l.丄(z)).iter(){match l.亙(q){Some(2)=>丑[0]+=1,Some(3)=>丑[1]+=1,_=>{}}}丑}fn
佳(l:&J,z:F)->C{let[佹,俸]=俳(l,z);let 佀=|一:F|(3-侪[一])*侨[一];-侊*佹+佀(0)*佹+佀(1)*俸}const 侄:C=0;const 侇:C=2;const 來:C=100;fn 佱(井:C)->C{if 井<0{return 0;}侄+侇*井.min(來)/2}fn
伓(l:&J,z:F)->C{let(丘,亩)=俀(l,z);let 丘=丘+佱(亩[0])-佱(亩[1]);if l.丕().is_some(){return 丘;}let 仍=仅(l,z);let 乻=仅(l,1-z);let 仓=C::from(乻.count())-C::from(仍.count());丘-2*佌(l,z)+2*佌(l,1-z)+俤(l,z)+佳(l,z)-佳(l,1-z)+侤*仓/4+丞(l,z,仍)-丞(l,1-z,乻)}const
侔:F=8;const 供:F=3;const 侌:F=32;const 侜:U=300;const 侀:些=0.75;struct 亀{乺:P<乑>,五:P<乑>,仈:U,}#[derive(Default)]struct 伷{七:K<L>,伐:F,乳:U,丗:F,丮:F,佅:F,乴:F,}const
侫:T=true;#[derive(Clone)]struct 乚{七:K<L>,伒:C,}struct 亝{七:K<L>,end:J,first:P<U>,丒:P<K<乚>>,}type 亼=HashMap<U,C,BuildHasherDefault<DefaultHasher>>;struct
企{丁:F,limit:亀,乳:U,仇:亼,亗:P<K<乂>>,}fn 伈(l:&mut J,七:&[L])->T{七.iter().all(|&下|l.个(下))}fn 乌(七:&[L])->U{let mut hash:U=0xcbf29ce484222325;for 下 in 七{let(仩,a,b,c)=match*下{L::亁{丏,x,y}=>(1,U::from(丏),x,y),L::书{一,x,y}=>(2,U::from(一),x,y),L::乾{x,y}=>(3,0,x,y),L::亐{x,y}=>(4,0,x,y),};let
亹=仩|a<<8|U::from(b)<<40|U::from(c)<<48;hash=(hash^亹).wrapping_mul(0x0100000001b3);}hash}fn 伽(乔:&J,亊:&[L])->J{let mut 丂=乔.clone();丂.仃();for&下 in 亊{丂.个(下);}丂.仃();丂}fn
仫(l:&J)->P<K<L>>{let 丁=l.东();let 丐=Z[1-丁];let 亸=|丠:&J|丠.丕()==Some(么::乧(丁));for q in l.予().iter(){if l.map().久(q,丐)!=1{continue;}let Some((丏,_))=l.仕(q)else{continue;};let(x,y)=人(丐);let
下=L::亁{丏,x,y};let mut 丠=l.clone();if 丠.个(下)&&亸(&丠){return Some(vec![下]);}}let 並=伌(l,丐,l.丈(丁))?;let mut 丠=l.clone();(伈(&mut 丠,&並.七)&&亸(&丠)).then_some(並.七)}impl
企{fn 习(&self)->T{self.乳>=self.limit.仈||self.limit.乺.is_some_and(|t|乑::now()>=t)}fn 丘(&mut self,乔:&J,丗:&[乚],伉:C)->C{self.乳+=1;self.伢(乔,丗,伉)}fn 伢(&self,乔:&J,丗:&[乚],伉:C)->C{if
乔.丕()==Some(么::乧(self.丁)){return 仞;}if 丗.is_empty(){return 伓(乔,self.丁);}let mut 价=C::MAX;for 亊 in 丗{价=价.min(伓(&伽(乔,&亊.七),self.丁));if 价<=伉{break;}}价}#[allow(clippy::too_many_arguments)]fn
俖(&mut self,买:&J,亡:C,乓:&[乂],亰:&[(C,F)],为:&mut[T],丗:&[乚],亢:&mut 亼,伙:T,)->(P<(F,J,C)>,T){let mut 三=None;let mut 仠=亡;for&(仯,与)in 亰{if 为[与]{continue;}if 伙&&仯!=C::MAX&&仯<=仠-亡{return(三,true);}if
self.习(){return(三,false);}为[与]=true;let 七=乓[与].七();let mut 丂=买.clone();if!伈(&mut 丂,七){亢.insert(乌(七),C::MIN/4);continue;}let 丘=self.丘(&丂,丗,仠);亢.insert(乌(七),丘-亡);if
丘>仠{仠=丘;三=Some((与,丂,丘));}}(三,false)}fn 伏(&mut self,乪:&J,丗:&[乚],亿:&[U],侶:T)->亝{let j=self;let mut 乊=None;let mut 买=乪.clone();let mut 七=K::new();let mut
亡=j.丘(&买,丗,C::MIN);let mut 亢=亼::default();let mut 乓=K::new();let mut 亰=K::new();while 买.丕().is_none()&&!j.习(){if 七.is_empty()&&侶{match&j.亗{Some(亗)=>乓.clone_from(亗),None=>{体(&买,&mut
乓);j.亗=Some(乓.clone());}}}else{体(&买,&mut 乓);}亰.clear();for(与,乮)in 乓.iter().enumerate(){let k=乌(乮.七());let 仯=亢.get(&k).or(j.仇.get(&k)).copied();亰.push((仯.unwrap_or(C::MAX),与));}亰.sort_by(|a,b|b.0
.cmp(&a.0).then(a.1 .cmp(&b.1)));let mut 为=vec![false;乓.len()];if 七.is_empty(){for(与,乮)in 乓.iter().enumerate(){为[与]=亿.contains(&乌(乮.七()));}}let mut 三=None;for
伙 in[true,false]{let(丧,俬)=j.俖(&买,亡,&乓,&亰,&mut 为,丗,&mut 亢,伙,);三=丧;if 三.is_some()||!俬{break;}}let Some((与,丂,丘))=三 else{break;};if 七.is_empty(){乊=Some(乌(乓[与].七()));}七.extend_from_slice(乓[与].七());买=丂;亡=丘;}for(k,乨)in
亢{let 何=j.仇.entry(k).or_insert(乨);*何=(*何).max(乨);}亝{七,end:买,first:乊,丒:None,}}fn 俠(&self,乔:&J)->K<K<L>>{let mut 丂=乔.clone();丂.仃();let mut 万=K::new();if
丂.丕().is_some(){return 万;}let 乜=丂.东();let 丈=丂.丈(乜);if let Some(並)=伌(&丂,Z[self.丁],丈){万.push(並.七);}for 丝 in 佋(&丂,伬,丈,供){万.push(丝.七);}let mut 佔=K::new();俆(&丂,&mut
佔);万.push(佔);let mut 乻=企{丁:乜,limit:亀{仈:侜,..self.limit},乳:0,仇:亼::default(),亗:None,};万.push(乻.伏(&丂,&[],&[],true).七);万.retain(|七|!七.is_empty());万}fn 伾(&self,乔:&J)->K<乚>{self.俠(乔).into_iter().map(|七|{let
伒=伓(&伽(乔,&七),self.丁);乚{七,伒}}).collect()}fn 乴(&mut self,乪:&J,乮:&乂,丗:&[乚])->P<亝>{let mut 丸=乪.clone();if!伈(&mut 丸,乮.七()){return None;}self.乳+=1;let 乖=self.伏(&丸,丗,&[],false);let
mut 七=乮.七().to_vec();七.extend_from_slice(&乖.七);Some(亝{七,end:乖.end,first:Some(乌(乮.七())),丒:None,})}fn 佐(&self,且:&亝,丗:&[乚])->C{let 丘=self.伢(&且.end,丗,C::MIN);match&且.丒{Some(丒)if!丒.is_empty()=>丘.min(self.伢(&且.end,丒,C::MIN)),_=>丘,}}}fn
俞(丧:&[乚],丗:&mut Vec<乚>)->T{let 严:K<_> =丗.iter().map(|r|r.七.clone()).collect();for 亊 in 丧{if 丗.iter().any(|r|r.七==亊.七){continue;}丗.push(亊.clone());}丗.sort_by_key(|亊|亊.伒);丗.truncate(侔);丗.iter().map(|r|&r.七).ne(&严)}fn
俶(l:&J,丿:亀,亯:&mut 亂)->伷{let 丁=l.东();let mut 中=伷::default();if l.丕().is_some(){return 中;}if let Some(七)=仫(l){中.七=七;return 中;}let 丹=|俦:些|match(丿.乺,丿.五){(Some(乺),Some(五))=>Some(乺+(五-乺).mul_f64(俦)),_=>None,};if
亯.俙(l){中.七=亯.上(l,丹(0.4));return 中;}let mut 乍=企{丁,limit:亀{乺:丹(侀),..丿},乳:0,仇:亼::default(),亗:None,};let mut 丗=K::new();let mut 丮:K<亝> =K::new();let mut 亿=K::new();let
mut 令=false;let mut 乴:K<乂> =K::new();while 中.伐<侌&&(丮.is_empty()||!乍.习()){let 且=if 令{let Some(乮)=乴.pop()else{break;};match 乍.乴(l,&乮,&丗){Some(且)=>且,None=>continue,}}else{乍.伏(l,&丗,&亿,true)};中.伐+=1;let
佭=丮.iter().any(|p|p.七==且.七);let 乊=且.first;let 佲=乍.习();let mut 且=且;let mut 佟=false;if!佲&&!佭{let 丧=乍.伾(&且.end);佟=俞(&丧,&mut 丗);且.丒=Some(丧);}if!佭{丮.push(且);}if
佲{break;}if!佟&&!令{match 乊{Some(乊)=>亿.push(乊),None=>{令=true;乴=乍.亗.clone().unwrap_or_default();乴.retain(|a|!亿.contains(&乌(a.七())));let 乨=|a:&乂|{乍.仇.get(&乌(a.七())).copied().unwrap_or(C::MIN)};乴.sort_by_key(乨);}}}}中.乴=F::from(令);中.乳=乍.乳;中.丗=丗.len();中.丮=丮.len();let
佛=||丿.五.is_some_and(|t|乑::now()>=t);let mut 亳:K<P<C>> =vec![None;丮.len()];for 与 in(0..丮.len()).rev(){if 与+1<丮.len()&&佛(){break;}亳[与]=Some(乍.佐(&丮[与],&丗));}let
亱=|亳:&[P<C>]|{let mut 三:P<(C,F)> =None;for(与,丘)in 亳.iter().enumerate().rev(){if let Some(丘)=*丘{if 三.is_none_or(|(乼,_)|丘>乼){三=Some((丘,与));}}}三};let mut
三=亱(&亳);乍.limit.乺=丿.五;while let Some((_,与))=三{if!侫||丮[与].丒.is_some()||佛(){break;}let 丧=乍.伾(&丮[与].end);丮[与].丒=Some(丧);中.佅+=1;亳[与]=Some(乍.佐(&丮[与],&丗));三=亱(&亳);}if
let Some((_,与))=三{let mut 丬=l.clone();for&下 in&丮[与].七{if 丬.个(下){中.七.push(下);}}}中}fn 俘(丿:&dyn Fn(乑,丱)->亀,mut 亯:亂,优:impl BufRead,仰:impl Write,)->Result<(),丰>{丽(优,仰,|亦,乗,上,亘|match
J::read(亦,乗,乤::任,上){Ok(l)=>{let 中=俶(&l,丿(亘,上),&mut 亯);eprintln!("turn {}: climbs {} tries {} memory {} checks {} forced {} {:?}",上,中.伐,中.乳,中.丗,中.佅,中.乴,亘.elapsed());侱(&中.七)}Err(互)=>{eprintln!("plan: cannot read the input: {}",互);"WAIT".to_string()}},)}const
佾:些=40.0;const 侁:些=800.0;const 価:些=0.6;fn 丿(亘:乑,上:丱)->亀{if let Some(仈)=std::env::var("CG_FIXED_ITERS").ok().and_then(|n|n.trim().parse().ok()).filter(|&n|n>0){return
亀{乺:None,五:None,仈,};}let 仳=俹().min(100.0);let 俑=if 上==1{侁}else{佾};let 丫=Duration::from_secs_f64(俑/1000.0*仳);亀{乺:Some(亘+丫.mul_f64(価)),五:Some(亘+丫),仈:U::MAX,}}const
侚:F=7;const 侘:亜=亜{丌:3,专:1,丈:1,丝:2,乵:2,仔:2,丯:6,他:1,};const 侙:I=I::MAX;fn 亯()->亂{俔(64)}fn 俔(乣:F)->亂{let 乹=亞::仝{丽:侚,乣,乐:侘,};let 业=侟.iter().map(|&乞|Y{伥:侙,..乞}).collect();亂::new(乹,业,Y::伃)}fn
main(){let 俰=io::stdin();let 俱=io::stdout();if let Err(互)=俘(&丿,亯(),俰.lock(),俱.lock()){eprintln!("plan: {}",互);std::process::exit(1);}}const 侥:&str="CG_TIME_SCALE";fn
俹()->些{俕(std::env::var(侥).ok().as_deref())}fn 俕(丛:P<&str>)->些{丛.and_then(|乗|乗.trim().parse::<些>().ok()).filter(|仳|仳.is_finite()&&*仳>0.0).unwrap_or(1.0)}const
O:F=12;const M:F=O*O;const 乒:F=4*O;const 乥:U=(1<<乒)-1;const 仙:U=1|1<<12|1<<24|1<<36;const 仼:U=仙<<11;const 伵:U=(1<<O)-1;#[derive(Clone,Copy,PartialEq)]struct
G([U;3]);impl G{const Q:G=G([0;3]);const 佽:G=G([乥;3]);#[inline]fn q(q:F)->G{let mut 串=G::Q;串.set(q);串}#[inline]fn test(self,q:F)->T{(self.0[q/乒]>>(q%乒))&1!=0}#[inline]fn
set(&mut self,q:F){self.0[q/乒]|=1<<(q%乒);}#[inline]fn clear(&mut self,q:F){self.0[q/乒]&=!(1<<(q%乒));}#[inline]fn is_empty(self)->T{self.0[0]|self.0[1]|self.0[2]==0}#[inline]fn
件(self)->[U;3]{self.0}#[inline]fn count(self)->丱{self.0[0].count_ones()+self.0[1].count_ones()+self.0[2].count_ones()}#[inline]fn 两(self,丩:G)->G{G([self.0[0]&!丩.0[0],self.0[1]&!丩.0[1],self.0[2]&!丩.0[2],])}#[inline]fn
iter(self)->仺{仺{伜:self.0,丹:0,}}#[inline]fn 俫(self)->G{let[a,b,c]=self.0;G([(a>>O)|((b&伵)<<(乒-O)),(b>>O)|((c&伵)<<(乒-O)),c>>O,])}#[inline]fn 俨(self)->G{let[a,b,c]=self.0;G([(a<<O)&乥,((b<<O)&乥)|(a>>(乒-O)),((c<<O)&乥)|(b>>(乒-O)),])}#[inline]fn
俩(self)->G{let[a,b,c]=self.0;G([(a>>1)&!仼&乥,(b>>1)&!仼&乥,(c>>1)&!仼&乥,])}#[inline]fn 俪(self)->G{let[a,b,c]=self.0;G([(a<<1)&!仙&乥,(b<<1)&!仙&乥,(c<<1)&!仙&乥,])}#[inline]fn
世(self)->G{let j=self;j.俫()|j.俨()|j.俩()|j.俪()}}impl BitAnd for G{type Output=G;#[inline]fn bitand(self,丩:G)->G{G([self.0[0]&丩.0[0],self.0[1]&丩.0[1],self.0[2]&丩.0[2],])}}impl
BitOr for G{type Output=G;#[inline]fn bitor(self,丩:G)->G{G([self.0[0]|丩.0[0],self.0[1]|丩.0[1],self.0[2]|丩.0[2],])}}impl BitOrAssign for G{#[inline]fn bitor_assign(&mut
self,丩:G){*self=*self|丩;}}impl Not for G{type Output=G;#[inline]fn not(self)->G{G::佽.两(self)}}struct 仺{伜:[U;3],丹:F,}impl Iterator for 仺{type Item=F;#[inline]fn
next(&mut self)->P<F>{let j=self;while j.丹<3{let 们=j.伜[j.丹];if 们!=0{j.伜[j.丹]=们&(们-1);return Some(j.丹*乒+们.trailing_zeros()as F);}j.丹+=1;}None}}#[derive(Clone,Copy,PartialEq)]enum
L{亁{丏:丱,x:I,y:I},书{一:I,x:I,y:I},乾{x:I,y:I},亐{x:I,y:I},}impl L{fn 俻(&self)->丰{match*self{L::亁{丏,x,y}=>format!("MOVE {} {x} {y}",丏),L::书{一,x,y}=>format!("TRAIN {} {x} {y}",一),L::乾{x,y}=>format!("BUILD MINE {x} {y}"),L::亐{x,y}=>format!("BUILD TOWER {x} {y}"),}}}fn
侱(七:&[L])->丰{if 七.is_empty(){return "WAIT".to_string();}let mut line=丰::with_capacity(16*七.len());for(与,下)in 七.iter().enumerate(){if 与>0{line.push(';');}line.push_str(&下.俻());}line}#[derive(Clone,Copy,PartialEq,Default)]enum
乤{伺,倉,侭,#[default]任,}impl 乤{fn 乩(self)->I{match self{乤::伺=>1,_=>3,}}fn 仟(self)->T{matches!(self,乤::侭|乤::任)}fn 伆(self)->T{self==乤::任}}const 乙:I=I::MAX;struct
乭{丷:G,valid:G,临:G,久:Box<[I]>,世:[[[I;4];M];2],}thread_local!{static 侉:RefCell<P<Arc<乭>>> =const{RefCell::new(None)};}impl 乭{fn 俧(丷:G,临:G)->Arc<乭>{侉.with(|乶|{let
mut 乶=乶.borrow_mut();if let Some(亨)=乶.as_ref(){if 亨.丷==丷&&亨.临==临{return Arc::clone(亨);}}let 亨=Arc::new(乭::build(丷,临));*乶=Some(Arc::clone(&亨));亨})}fn build(丷:G,临:G)->乭{let
乬=!丷;const 侢:[[(付,付);4];2]=[[(0,-1),(1,0),(0,1),(-1,0)],[(0,1),(-1,0),(0,-1),(1,0)],];let mut 世=[[[乙;4];M];2];for(z,乡)in 侢.iter().enumerate(){for q in
乬.iter(){let(x,y)=((q%O)as 付,(q/O)as 付);let mut 丑=0;for&(侽,侾)in 乡{let(佡,佢)=(x+侽,y+侾);if(0..O as 付).contains(&佡)&&(0..O as 付).contains(&佢){let 丂=佢 as F*O+佡
as F;if 乬.test(丂){世[z][q][丑]=丂 as I;丑+=1;}}}}}let mut 久=vec![乙;M*M].into_boxed_slice();let mut 伡=[0u8;M];for 不 in 乬.iter(){let 事=&mut 久[不*M..(不+1)*M];事[不]=0;伡[0]=不
as I;let(mut 伕,mut 伩)=(0,1);while 伕<伩{let q=伡[伕]as F;伕+=1;for&丂 in&世[0][q]{if 丂!=乙&&事[丂 as F]==乙{事[丂 as F]=事[q]+1;伡[伩]=丂;伩+=1;}}}}乭{丷,valid:乬,临,久,世,}}#[inline]fn
久(&self,不:F,乫:F)->I{self.久[不*M+乫]}}const 侓:丱=100;const 亏:C=15;const 侩:[C;3]=[1,4,20];const Z:[F;2]=[0,M-1];#[derive(Clone,Copy,PartialEq)]enum 么{乧(F),伮,}enum
乘{伀,伲,伳,}#[derive(Clone)]struct J{map:Arc<乭>,丨:乤,丒:[G;2],丄:[G;2],乆:G,乇:G,丅:[G;2],一:[G;3],亇:G,亷:[丱;M],丈:[C;2],专:[C;2],东:F,了:丱,京:丱,丕:P<么>,}fn 仆(丒:G,亥:F)->G{if!丒.test(亥){return
G::Q;}let mut 仑=G::q(亥);loop{let 丂=(仑|仑.世())&丒;if 丂==仑{return 仑;}仑=丂;}}impl J{fn 东(&self)->F{self.东}fn 了(&self)->丱{self.了}fn 丕(&self)->P<么>{self.丕}fn 丨(&self)->乤{self.丨}fn
享(&self,z:F)->C{let j=self;let 丅=j.丅[z];j.丈[z]+10*C::from((丅&j.一[0]).count())+20*C::from((丅&j.一[1]).count())+30*C::from((丅&j.一[2]).count())}fn 丈(&self,z:F)->C{self.丈[z]}fn
专(&self,z:F)->C{self.专[z]}fn 伪(&self,z:F)->F{self.丅[z].count()as F}fn map(&self)->&乭{&self.map}fn 丟(&self,z:F)->G{self.丒[z]}fn 丄(&self,z:F)->G{self.丄[z]}fn
丅(&self,z:F)->G{self.丅[z]}fn 仕(&self,q:F)->P<(丱,I)>{let j=self;(j.丅[0]|j.丅[1]).test(q).then(||(j.亷[q],j.伛(q)))}fn 予(&self)->G{self.丅[self.东]&self.亇}fn
丞(&self)->G{self.乆}fn 乢(&self)->G{self.乇}fn 亙(&self,q:F)->P<I>{(q<M&&(self.丅[0]|self.丅[1]).test(q)).then(||self.伛(q))}fn 俯(&mut self,z:F){let j=self;j.丄=[仆(j.丒[0],Z[0]),仆(j.丒[1],Z[1])];let
侹=j.丅[z].两(j.丄[z]);j.伣(侹);j.但();j.丈[z]+=j.专[z];if j.丈[z]<0{j.丈[z]=0;j.伣(j.丅[z]);}j.亇|=j.丅[z];}fn 但(&mut self){let j=self;for z in 0..2{let 丄=j.丄[z];let
丅=丄&j.丅[z];let mut 专=C::from(丄.count())+4*C::from((丄&j.乆).count());for(一,倁)in 侩.iter().enumerate(){专-=倁*C::from((丅&j.一[一]).count());}j.专[z]=专;}}fn 仃(&mut
self){let j=self;if j.丕.is_some(){return;}if j.东==1{if j.了>=侓{j.丕=Some(match j.享(0).cmp(&j.享(1)){std::cmp::Ordering::Greater=>么::乧(0),std::cmp::Ordering::Less=>么::乧(1),std::cmp::Ordering::Equal=>么::伮,});return;}j.了+=1;}j.东=1-j.东;j.俯(j.东);}fn
佧(&mut self,七:&[L]){let j=self;if j.丕.is_some(){return;}let 伔=j.京;for&下 in 七{if j.侼(下,伔){continue;}j.个(下);if j.丕.is_some(){return;}}j.仃();}fn 侼(&self,下:L,伔:丱)->T{let
j=self;match 下{L::亁{丏,x,y}=>{j.亠(x,y).is_none()||丏>=伔||j.佑(j.东,丏).is_none()}L::书{一,x,y}=>{j.亠(x,y).is_none()||一==0||一>j.丨.乩()}L::乾{x,y}|L::亐{x,y}=>{j.亠(x,y).is_none()}}}#[inline]fn
亠(&self,x:I,y:I)->P<F>{let(x,y)=(F::from(x),F::from(y));(x<O&&y<O&&!self.map.丷.test(y*O+x)).then_some(y*O+x)}fn 佑(&self,z:F,丏:丱)->P<F>{self.丅[z].iter().find(|&q|self.亷[q]==丏)}#[inline]fn
伛(&self,q:F)->I{if self.一[0].test(q){1}else if self.一[1].test(q){2}else{3}}fn 侳(&self,乜:F)->G{let 乢=self.乇&self.丒[乜];let 丄=self.丄[乜];乢|(丄&(丄&乢).世())}fn
主(&self,z:F,一:I)->G{let j=self;debug_assert!((1..=3).contains(&一),"unit level {} is not 1 to 3",一);let 乜=1-z;let 丅=j.丅[0]|j.丅[1];let 便=j.仂();let 俿=match
一{1 if j.丨==乤::伺=>j.一[1]|j.一[2],1=>丅,2=>j.一[1]|j.一[2],_=>G::Q,};let 主=便|j.丒[乜].两(俿);if 一<3{主.两(j.侳(乜))}else{主}}fn 亖(&self,z:F)->G{self.丄[z]|self.丄[z].世()}fn
仂(&self)->G{let j=self;let 丅=j.丅[0]|j.丅[1];let 俉=G::q(Z[0])|G::q(Z[1]);j.map.valid.两(丅|j.乆|j.乇|俉)}fn 个(&mut self,下:L)->T{let j=self;if j.丕.is_some(){return
false;}let z=j.东;let 云=match 下{L::书{一,x,y}=>j.俽(z,一,x,y),L::亁{丏,x,y}=>j.俐(z,丏,x,y),L::乾{x,y}=>j.build(z,true,x,y),L::亐{x,y}=>j.build(z,false,x,y),};let
Some(云)=云 else{return false;};match 云{乘::伀=>{}乘::伲=>j.丄[z]=仆(j.丒[z],Z[z]),乘::伳=>{j.丄=[仆(j.丒[0],Z[0]),仆(j.丒[1],Z[1])];}}j.但();if!j.丒[0].test(Z[0]){j.丕=Some(么::乧(1));}else
if!j.丒[1].test(Z[1]){j.丕=Some(么::乧(0));}true}fn 伣(&mut self,丌:G){let j=self;j.丅[0]=j.丅[0].两(丌);j.丅[1]=j.丅[1].两(丌);for 一 in&mut j.一{*一=一.两(丌);}j.亇=j.亇.两(丌);}fn
併(&mut self,z:F,q:F)->乘{let j=self;j.伣(G::q(q));j.乆.clear(q);j.乇.clear(q);let 乜=1-z;if j.丒[乜].test(q){j.丒[乜].clear(q);j.丒[z].set(q);乘::伳}else if j.丒[z].test(q){乘::伀}else{j.丒[z].set(q);乘::伲}}fn
俽(&mut self,z:F,一:I,x:I,y:I)->P<乘>{let j=self;if 一==0||一>j.丨.乩(){return None;}let q=j.亠(x,y)?;let 丆=10*C::from(一);if j.丈[z]<丆||!j.亖(z).test(q)||!j.主(z,一).test(q){return
None;}let 云=j.併(z,q);j.佥(z,一,q,j.京);j.京=j.京.saturating_add(1);j.丈[z]-=丆;Some(云)}fn 佥(&mut self,z:F,一:I,q:F,丏:丱){self.丅[z].set(q);self.一[F::from(一)-1].set(q);self.亷[q]=丏;}fn
俐(&mut self,z:F,丏:丱,x:I,y:I)->P<乘>{let j=self;let 丐=j.亠(x,y)?;let 不=j.佑(z,丏)?;if!j.亇.test(不){return None;}let 一=j.伛(不);let 丢=j.主(z,一);let 举=j.map.世[z][不];if
举.contains(&(丐 as I))&&!丢.test(丐){return None;}let mut 三=j.map.久(不,丐);let mut 什=None;for&丂 in 举.iter().take_while(|&&丂|丂!=乙){let 丂=F::from(丂);let 久=j.map.久(丂,丐);if
久<三&&丢.test(丂){三=久;什=Some(丂);}}let 什=什?;let 云=j.併(z,什);j.丅[z].clear(不);j.一[F::from(一)-1].clear(不);j.亇.clear(不);j.佥(z,一,什,丏);Some(云)}fn build(&mut self,z:F,乆:T,x:I,y:I)->P<乘>{let
j=self;let 侰=if 乆{j.丨.仟()}else{j.丨.伆()};let q=j.亠(x,y)?;let 俒=j.map.临.test(q);if!侰||j.丈[z]<亏||俒!=乆||!j.丒[z].test(q)||!j.亖(z).test(q)||!j.仂().test(q){return
None;}if 乆{j.乆.set(q);}else{j.乇.set(q);}j.丈[z]-=亏;Some(乘::伀)}}const 侎:C=1000000000;const 份:C=0;const 侖:C=1;const 侦:C=2;impl J{fn read(亦:&str,上:&str,丨:乤,了:丱,)->Result<J,丰>{let
mut 临=K::new();let mut 丣=亦.lines();let 丑=休(&mut 丣,"mine spot count")?;for _ in 0..丑{let[x,y]=乷(亭(&mut 丣,"a mine spot")?,"a mine spot")?;临.push((仢(x)?,仢(y)?));}低(丣,"the initial input")?;let
mut 丣=上.lines();let mut 伖=[0i64;4];for(丛,丵)in 伖.iter_mut().zip(["gold","income","opponent's gold","opponent's income",]){let[传]=乷(亭(&mut 丣,丵)?,丵)?;if 传.abs()>侎{return
Err(format!("{}: {} is out of range",丵,传));}*丛=传;}let mut 丷=G::Q;let mut 丌=[G::Q;2];let mut 丄=[G::Q;2];for y in 0..O{let 事=亭(&mut 丣,"a map row")?.trim();if
事.len()!=O{return Err(format!("map row {y} is not {} characters: {:?}",O,事));}for(x,仒)in 事.bytes().enumerate(){let q=y*O+x;match 仒{b'#'=>丷.set(q),b'.'=>{}b'O'|b'o'|b'X'|b'x'=>{let
乀=F::from(仒==b'X'||仒==b'x');丌[乀].set(q);if 仒.is_ascii_uppercase(){丄[乀].set(q);}}_=>{return Err(format!("map row {y} has {:?} at {x}",仒 as char))}}}}let
侵=休(&mut 丣,"building count")?;let mut 乏=K::new();for _ in 0..侵{let 乷=乷(亭(&mut 丣,"a building")?,"a building")?;乏.push(乷);}let 伪=休(&mut 丣,"unit count")?;let
mut 丅=K::new();for _ in 0..伪{let 乷=乷(亭(&mut 丣,"a unit")?,"a unit")?;丅.push(乷);}低(丣,"the turn's input")?;let 佤=乏.iter().find(|佂|佂[0]==0&&佂[1]==份).ok_or("the bot's own HQ is not among the buildings")?;let
佘=伋(佤[2],佤[3],丷)?;let z=Z.iter().position(|&亥|亥==佘).ok_or_else(||format!("the bot's HQ is at cell {}, not in a corner",佘))?;let mut 佯=G::Q;for(x,y)in 临{if
丷.test(y*O+x){return Err(format!("mine spot ({x}, {y}) is a void cell"));}佯.set(y*O+x);}let 伄=|乀:F|if 乀==0{z}else{1-z};let mut l=J{map:乭::俧(丷,佯),丨,丒:[G::Q;2],丄:[G::Q;2],乆:G::Q,乇:G::Q,丅:[G::Q;2],一:[G::Q;3],亇:G::Q,亷:[0;M],丈:[0;2],专:[0;2],东:z,了,京:1,丕:None,};for
乀 in 0..2{let 九=伄(乀);l.丒[九]=丌[乀];l.丄[九]=丄[乀];l.丈[九]=伖[2*乀];l.专[九]=伖[2*乀+1];}let mut 佣=G::Q;for[九,仩,x,y]in 乏{let 乀=佪(九)?;let q=伋(x,y,丷)?;if!丌[乀].test(q){return
Err(format!("building at ({x}, {y}) is not on its owner's cell"));}if 佣.test(q){return Err(format!("two buildings at ({x}, {y})"));}佣.set(q);match 仩{份
if q==Z[伄(乀)]=>{}份=>{return Err(format!("an HQ at ({x}, {y}), not in its owner's corner"))}侖=>l.乆.set(q),侦=>l.乇.set(q),_=>return Err(format!("unknown building type {}",仩)),}}let
mut 乁=G::Q;let mut 仪=0u32;for[九,丏,一,x,y]in 丅{let 乀=佪(九)?;let q=伋(x,y,丷)?;if!丌[乀].test(q){return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));}if
乁.test(q){return Err(format!("two units at ({x}, {y})"));}if!(1..=3).contains(&一){return Err(format!("unit level {} is not 1, 2 or 3",一));}let 丏=丱::try_from(丏).map_err(|_|format!("unit id {} is out of range",丏))?;if
乁.iter().any(|丩|l.亷[丩]==丏){return Err(format!("two units have id {}",丏));}乁.set(q);l.丅[伄(乀)].set(q);l.一[一 as F-1].set(q);l.亷[q]=丏;仪=仪.max(丏);}if!乁.is_empty(){l.京=仪.checked_add(1).ok_or_else(||format!("unit id {} is out of range",仪))?;}l.亇=l.丅[z];Ok(l)}}fn
亭<'a>(丣:&mut Lines<'a>,丵:&str)->Result<&'a str,丰>{丣.next().ok_or_else(||format!("the text ends before {}",丵))}fn 低(mut 丣:Lines<'_>,丵:&str)->Result<(),丰>{match
丣.find(|line|!line.trim().is_empty()){Some(line)=>Err(format!("extra text after the end of {}: {line:?}",丵)),None=>Ok(()),}}fn 乷<const N:F>(line:&str,丵:&str)->Result<[C;N],丰>{let
mut 件=line.split_whitespace();let mut 佻=[0i64;N];for 丛 in&mut 佻{let 亹=件.next().ok_or_else(||format!("{}: expected {N} numbers in {line:?}",丵))?;*丛=亹.parse().map_err(|_|format!("{}: {:?} is not an integer",丵,亹))?;}if
件.next().is_some(){return Err(format!("{}: expected {N} numbers in {line:?}",丵));}Ok(佻)}fn 休(丣:&mut Lines<'_>,丵:&str)->Result<F,丰>{let[丑]=乷(亭(丣,丵)?,丵)?;F::try_from(丑).ok().filter(|&丑|丑<=M).ok_or_else(||format!("{}: {} is out of range",丵,丑))}fn
仢(丛:C)->Result<F,丰>{F::try_from(丛).ok().filter(|&丛|丛<O).ok_or_else(||format!("coordinate {} is off the map",丛))}fn 伋(x:C,y:C,丷:G)->Result<F,丰>{let q=仢(y)?*O+仢(x)?;if
丷.test(q){return Err(format!("({x}, {y}) is a void cell"));}Ok(q)}fn 佪(九:C)->Result<F,丰>{match 九{0=>Ok(0),1=>Ok(1),_=>Err(format!("owner {} is not 0 or 1",九)),}}const
侃:C=15;const 侗:C=6;const 伸:C=10;const 侞:I=3;#[derive(Clone,Copy)]struct Y{open:C,仡:C,佉:I,众:C,佗:C,作:C,伥:I,佈:C,佇:C,似:C,交:C,亵:C,丶:C,产:C,于:C,}impl Y{const
伃:Y=Y{open:3,仡:2,佉:12,众:4,佗:侃,作:侗,伥:侞,佈:2,佇:2,似:0,交:0,亵:0,丶:0,产:0,于:0,};const fn 倇(self,交:C,亵:C,丶:C)->Y{Y{交,亵,丶,..self}}const fn 倆(self,产:C,于:C)->Y{Y{产,于,..self}}}fn
仗(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 仁(l:&J,不:G)->[I;M]{let mut 义=[I::MAX;M];let 乬=l.map().valid;let mut 之=不&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for
q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn 促(l:&J)->I{let 义=仁(l,l.丟(0));l.丟(1).iter().map(|q|义[q]).min().unwrap_or(I::MAX)}struct
亃<'a>{乞:&'a Y,丁:F,theirs:[I;M],亶:[I;M],free:G,乎:G,ours:G,临:G,}impl 亃<'_>{fn new<'a>(l:&J,乞:&'a Y)->亃<'a>{let 丁=l.东();let 乆=仁(l,l.丟(丁));let 乻=仁(l,l.丟(1-丁));let
亶=仁(l,G::q(Z[1-丁]));let 亓=l.map().valid.两(l.丟(0)|l.丟(1));let mut 乎=G::Q;for q in 亓.iter(){if 乆[q]<=乻[q]{乎.set(q);}}亃{乞,丁,theirs:乻,亶,free:亓,乎,ours:l.丟(丁),临:l.map().临,}}fn
value(&self,q:F)->C{let j=self;let p=j.乞;let 丼=C::from((G::q(q).世()&j.free).count());let 佄=p.佉;let 仡=C::from(佄)-C::from(j.theirs[q].min(佄));let 亥=if p.众>0{C::from(j.亶[q].min(30))/p.众}else{0};let
丒=C::from((G::q(q).世()&j.ours).count());let line=if 丒<=1{p.亵}else{0};let 丶=if j.临.test(q){p.丶}else{0};let 俴=if 丼==0{p.产}else{0};let 于=p.于*(22-C::from(j.亶[q].min(22)));p.open*丼+p.仡*仡-亥+p.交*丒-line+丶-俴+于}fn
三(&self,丌:G)->P<F>{丌.iter().max_by_key(|&q|(self.value(q),std::cmp::Reverse(q)))}fn 伎(&mut self,q:F){self.free.clear(q);self.乎.clear(q);self.ours.set(q);}}fn
仏(l:&mut J,下:L,万:&mut Vec<L>)->T{let 伻=l.个(下);if 伻{万.push(下);}伻}fn 丙(l:&mut J,不:F,乫:F,万:&mut Vec<L>)->T{let Some((丏,_))=l.仕(不)else{return false;};let(x,y)=仗(乫);仏(l,L::亁{丏,x,y},万)}fn
倀(h:C,k:C)->C{let k=k.clamp(0,(h-1).max(0));k*h-k*(k+1)/2-伸}#[derive(Clone,Copy)]enum 乃{仜,乿{丞:I,丅:I},}fn 佸(l:&J,乞:&Y,亅:乃)->K<L>{let mut 丬=l.clone();let
mut 上=亃::new(l,乞);let mut 万=K::new();乛(&mut 丬,&mut 上,&mut 万);match 亅{乃::仜=>俛(&mut 丬,&mut 上,&mut 万),乃::乿{丞,丅}=>{俁(&mut 丬,&mut 上,丞,丅,&mut 万)}}万}fn 乛(l:&mut
J,上:&mut 亃<'_>,万:&mut Vec<L>){let 丁=上.丁;let mut 仧:K<F> =K::new();loop{let 丢=l.主(丁,1);let mut 亱:P<(丱,F)> =None;for 不 in l.予().iter(){if 仧.contains(&不){continue;}let
乸=(G::q(不).世()&上.free&丢).count();if 乸==0{仧.push(不);}else if 亱.is_none_or(|(n,_)|乸<n){亱=Some((乸,不));}}let Some((_,不))=亱 else{break;};match 上.三(G::q(不).世()&上.free&丢){Some(乫)if
丙(l,不,乫,万)=>上.伎(乫),_=>仧.push(不),}}for 不 in 仧{if!l.予().test(不){continue;}let 丐=[上.乎,上.free].into_iter().find_map(|丌|{丌.iter().min_by_key(|&q|(l.map().久(不,q),q))});if
let Some(丐)=丐{丙(l,不,丐,万);}}}fn 俛(l:&mut J,上:&mut 亃<'_>,万:&mut Vec<L>){let 丁=上.丁;let p=上.乞;let h=(p.佗-C::from(l.了())).max(p.作);let 俺=(C::from(促(l))-p.佈).max(0)/p.佇.max(1)+p.似;loop{let
丈=l.丈(丁);let 丅=l.伪(丁)as C;let 信=C::from(上.乎.count());let q=上.三(l.亖(丁)&上.free&l.主(丁,1));let 佺=倀(h,(信/(丅+1)).min(俺));let 佃=q.is_some()&&丈>=伸&&佺>0;let 佝=4*h-亏;let
丶=佞(l,上);let 侷=丶.is_some()&&丈>=亏&&佝>0;if 侷&&(!佃||佝>=佺){let(x,y)=仗(丶.unwrap_or_default());if 仏(l,L::乾{x,y},万){continue;}break;}let Some(q)=q.filter(|_|佃)else{break;};let(x,y)=仗(q);if!仏(l,L::书{一:1,x,y},万){break;}上.伎(q);}}fn
佞(l:&J,上:&亃<'_>)->P<F>{let 丁=上.丁;(l.map().临&l.丄(丁)&l.仂()).iter().filter(|&丶|上.theirs[丶]>上.乞.伥).min_by_key(|&丶|(l.map().久(Z[丁],丶),丶))}fn 俁(l:&mut J,上:&mut
亃<'_>,丞:I,丅:I,万:&mut Vec<L>,){let 丁=上.丁;for _ in 0..丞{let Some(丶)=佞(l,上)else{break;};let(x,y)=仗(丶);if!仏(l,L::乾{x,y},万){break;}}for _ in 0..丅{let Some(q)=上.三(l.亖(丁)&上.free&l.主(丁,1))else{break;};let(x,y)=仗(q);if!仏(l,L::书{一:1,x,y},万){break;}上.伎(q);}}fn
仲(l:&J)->T{if l.丕().is_some(){return false;}let 丁=l.东();let 丈=l.丈(丁);let 丯=亟::new(l,丁,true);l.丟(1-丁).iter().any(|q|丯.丆[q]<=丈)}#[derive(Clone,Copy)]struct
両{亚:I,亅:乃,}const 伫:I=I::MAX;const 使:[乃;8]=[乃::仜,乃::乿{丞:0,丅:0},乃::乿{丞:0,丅:1},乃::乿{丞:0,丅:2},乃::乿{丞:0,丅:伫,},乃::乿{丞:1,丅:0},乃::乿{丞:1,丅:1},乃::乿{丞:1,丅:伫,},];struct
伭<'a>{业:&'a[Y],仉:Y,rest:両,乱:P<乑>,俢:Settings,}impl 伭<'_>{fn 乞(&self,丙:両)->&Y{&self.业[F::from(丙.亚)%self.业.len()]}fn 七(&self,l:&J,丙:両)->K<L>{佸(l,self.乞(丙),丙.亅)}}fn
伦(l:&J,丁:F)->U{let mut hash:U=0xcbf29ce484222325;let mut 伝=|亹:U|hash=(hash^亹).wrapping_mul(0x0100000001b3);for(俵,串)in[l.丟(丁),l.丅(丁),l.丞()].into_iter().enumerate(){for
q in 串.iter(){伝(((俵 as U)<<8)|q as U);}伝(0xff);}伝(l.丈(丁)as U);hash}enum 亞{仝{丽:F,乣:F,乐:亜,},}struct 亂{乹:亞,业:K<Y>,仉:Y,且:K<両>,以:K<U>,亲:F,丮:丱,仱:T,}impl 亂{fn
new(乹:亞,业:K<Y>,仉:Y)->亂{亂{乹,业,仉,且:K::new(),以:K::new(),亲:0,丮:0,仱:false,}}fn 俙(&mut self,l:&J)->T{let j=self;let 俭=matches!(j.乹,亞::仝{..});let 俚=!俭&&j.丮>0&&j.亲>=j.以.len();if!j.仱&&(俚||仲(l)){j.仱=true;}!j.仱}fn
上(&mut self,l:&J,乱:P<乑>)->K<L>{let j=self;let 佊=伭{业:&j.业,仉:j.仉,rest:両{亚:0,亅:乃::仜,},乱,俢:Settings::DEFAULT,};let 丁=l.东();let 俓=j.以.get(j.亲)==Some(&伦(l,丁));if!俓{let
乊=j.丮==0;j.且=match j.乹{亞::仝{丽,乣,乐,}=>{let 乐=&乐;let 乖=依;let mut 乡=且(l,&j.业,乖,丽,乣,乱,乐,);乡.resize(丽,乖);乡}};j.以=match j.乹{亞::仝{..}=>修(l,&j.业,&j.且),};j.亲=0;j.丮+=1;}let
丙=j.且.get(j.亲).copied().unwrap_or(佊.rest);j.亲+=1;佊.七(l,丙)}}fn 修(l:&J,业:&[Y],且:&[両])->K<U>{let 丁=l.东();let mut 佚=K::with_capacity(且.len());let mut 也=l.clone();for&丙
in 且{if 也.丕().is_some(){break;}佚.push(伦(&也,丁));也=伅(&也,&七(&也,业,丙));}佚}const 侧:C=3;struct Settings{丽:C,}impl Settings{const DEFAULT:Settings=Settings{丽:侧};}fn
乡(l:&J,不:G)->[I;M]{let 乬=l.map().valid;let mut 义=[I::MAX;M];let mut 之=不&乬;let mut 乁=之;let mut d=0u8;while!之.is_empty(){for q in 之.iter(){义[q]=d;}let 丂=(之.世()&乬).两(乁);乁|=丂;之=丂;d=d.saturating_add(1);}义}fn
倂(l:&J,z:F)->C{let 仍=乡(l,l.丄(z));let 乻=乡(l,l.丄(1-z));l.map().valid.iter().map(|q|match 仍[q].cmp(&乻[q]){_ if 仍[q]==I::MAX=>0,std::cmp::Ordering::Less=>2,std::cmp::Ordering::Equal=>1,std::cmp::Ordering::Greater=>0,}).sum()}const
侟:[Y;27]=业();const 依:両=両{亚:16,亅:乃::仜,};const fn 业()->[Y;27]{let 交=[0,2,4];let 亵=[0,3,6];let 于=[0,1,2];let mut 万=[Y::伃;27];let mut i=0;while i<27{let(g,t,w)=(i/9,(i/3)%3,i%3);let
产=if g>0||t>0{4}else{0};万[i]=Y::伃.倇(交[g],亵[t],0).倆(产,于[w]);i+=1;}万}#[derive(Default)]struct 伂{丌:C,专:C,丈:C,乵:C,丝:C,仔:C,丯:C,他:C,}#[derive(Clone,Copy)]struct
亜{丌:C,专:C,丈:C,丝:C,乵:C,仔:C,丯:C,他:C,}fn 俣(l:&J,z:F)->[C;M]{struct 亻{丄:G,乄:[u16;M],丳:[u16;M],size:[C;M],丝:[C;M],亍:u16,}fn 乽(d:&mut 亻,v:F){d.亍+=1;d.乄[v]=d.亍;d.丳[v]=d.亍;d.size[v]=1;for
w in(G::q(v).世()&d.丄).iter(){if d.乄[w]==0{乽(d,w);d.size[v]+=d.size[w];d.丳[v]=d.丳[v].min(d.丳[w]);if d.丳[w]>=d.乄[v]{d.丝[v]+=d.size[w];}}else{d.丳[v]=d.丳[v].min(d.乄[w]);}}}let
mut d=亻{丄:l.丄(z),乄:[0;M],丳:[0;M],size:[0;M],丝:[0;M],亍:0,};let 乪=Z[z];if d.丄.test(乪){乽(&mut d,乪);}d.丝[乪]=0;d.丝}fn 俥(l:&J,z:F)->伂{let 丄=l.丄(z);let 丝=俣(l,z);let
亶=仁(l,G::q(Z[1-z]));let mut 万=伂{丌:C::from(丄.count()),专:l.专(z),丈:l.丈(z),丯:C::MAX,他:倂(l,z)/2,..伂::default()};for q in 丄.iter(){if 丝[q]>0{万.乵+=1;万.丝=万.丝.max(丝[q]);}if
q!=Z[z]&&(G::q(q).世()&丄).count()<=1{万.仔+=1;}万.丯=万.丯.min(C::from(亶[q]));}万}fn 丛(l:&J,z:F,乐:&亜)->C{let s=俥(l,z);let o=乐;o.丌*s.丌+o.专*s.专+o.丈*s.丈-o.丝*s.丝-o.乵*s.乵-o.仔*s.仔-o.丯*s.丯.min(30)+o.他*s.他}fn
伅(l:&J,七:&[L])->J{let mut 丂=l.clone();丂.佧(七);if 丂.丕().is_none(){丂.佧(&[]);}丂}fn 七(l:&J,业:&[Y],丙:両)->K<L>{佸(l,&业[F::from(丙.亚)%业.len()],丙.亅,)}fn 佦(l:&J,业:&[Y],且:&[両],乖:両,丽:F,)->J{let
mut 也=l.clone();for 上 in 0..丽{if 也.丕().is_some(){break;}let 丙=且.get(上).copied().unwrap_or(乖);也=伅(&也,&七(&也,业,丙));}也}fn 俲(l:&J,业:&[Y])->K<(両,K<L>)>{let mut
万:K<(両,K<L>)> =K::new();for 亚 in 0..业.len().min(F::from(I::MAX)){for&亅 in&使{let 丙=両{亚:亚 as I,亅,};let c=七(l,业,丙);if!万.iter().any(|(_,丩)|*丩==c){万.push((丙,c));}}}万}fn
侲(l:&J,业:&[Y],乖:両,丽:F,乣:F,乱:P<乑>,乐:&亜,)->K<両>{let 丁=l.东();let 从=||乱.is_some_and(|t|乑::now()>=t);let 仐=|且:&[両]|丛(&佦(l,业,且,乖,丽),丁,乐);let mut 三:(C,K<両>)=(仐(&[]),K::new());let
mut 之:K<(J,K<両>)> =vec![(l.clone(),K::new())];for 侻 in 0..丽{let mut 乯:K<(C,J,K<両>)> =K::new();let mut 乁:HashMap<U,F> =HashMap::new();for(你,且)in&之{if 从(){break;}for(丙,c)in
俲(你,业){let 丂=伅(你,&c);let mut 伍=且.clone();伍.push(丙);let v=丛(&丂,丁,乐);let 乌=伦(&丂,丁);match 乁.get(&乌){Some(&丹)if 乯[丹].0>=v=>{}Some(&丹)=>乯[丹]=(v,丂,伍),None=>{乁.insert(乌,乯.len());乯.push((v,丂,伍));}}}}if
乯.is_empty(){break;}乯.sort_by(|a,b|b.0 .cmp(&a.0));乯.truncate(乣);let 乶=侻+1==丽||从();for(i,(_,_,且))in 乯.iter().take(if 乶{乣}else{1}).enumerate(){if i>0&&从(){break;}let
v=仐(且);if v>三.0{三=(v,且.clone());}}if 乶{break;}之=乯.into_iter().map(|(_,s,p)|(s,p)).collect();}三.1}fn 且(l:&J,业:&[Y],乖:両,丽:F,俎:F,乱:P<乑>,乐:&亜,)->K<両>{let 丁=l.东();let
仐=|且:&[両]|丛(&佦(l,业,且,乖,丽),丁,乐);let mut 三:(C,K<両>)=(仐(&[]),K::new());let mut 乣=2;loop{let 丧=侲(l,业,乖,丽,乣,乱,乐);let v=仐(&丧);if v>三.0{三=(v,丧);}let 从=乱.is_some_and(|t|乑::now()>=t);if
从||(乱.is_none()&&乣>=俎){break;}乣*=2;}三.1}const 侂:F=4+12;fn 丽(优:impl BufRead,mut 仰:impl Write,mut 伇:impl FnMut(&str,&str,丱,乑)->丰,)->Result<(),丰>{let mut
丣=估{佩:优};let Some(乊)=丣.line()?else{return Ok(());};let mut 亦=format!("{}\n",乊);丣.take(丑(&乊,"mine spot count")?,&mut 亦)?;let mut 上=0;loop{let Some(乊)=丣.line()?else{return
Ok(());};let 亘=乑::now();上+=1;let mut 乗=format!("{}\n",乊);丣.take(侂-1,&mut 乗)?;for 丵 in["building count","unit count"]{let line=丣.佫()?;let n=丑(&line,丵)?;乗.push_str(&line);乗.push('\n');丣.take(n,&mut
乗)?;}let 伇=伇(&亦,&乗,上,亘);writeln!(仰,"{}",伇).and_then(|()|仰.flush()).map_err(|互|format!("turn {}: cannot write the answer: {}",上,互))?;}}struct 估<R>{佩:R,}impl<R:BufRead>估<R>{fn
line(&mut self)->Result<P<丰>,丰>{let mut line=丰::new();match self.佩.read_line(&mut line){Ok(0)=>Ok(None),Ok(_)=>Ok(Some(line.trim_end_matches(['\r','\n']).to_string())),Err(互)=>Err(format!("cannot read input: {}",互)),}}fn
佫(&mut self)->Result<丰,丰>{self.line()?.ok_or_else(||"input ended in the middle of a turn".to_string())}fn take(&mut self,n:F,乗:&mut String)->Result<(),丰>{for
_ in 0..n{乗.push_str(&self.佫()?);乗.push('\n');}Ok(())}}fn 丑(line:&str,丵:&str)->Result<F,丰>{match line.trim().parse::<F>(){Ok(n)if n<=侍=>Ok(n),_=>Err(format!("bad {} {line:?}",丵)),}}const
侍:F=144;const 侣:C=3;const 侈:C=2;const 侒:F=16;fn 俗(l:&J,万:&mut Vec<L>){let z=l.东();let mut 乹=伴{s:l.clone(),z,乲:1-z,丐:Z[1-z],仦:Z[z],上:l.了(),万,};乹.run();}struct
伴<'a>{s:J,z:F,乲:F,丐:F,仦:F,上:丱,万:&'a mut Vec<L>,}fn 仸(q:F)->(I,I){((q%O)as I,(q/O)as I)}fn 伞(丏:丱,q:F)->L{let(x,y)=仸(q);L::亁{丏,x,y}}fn 佷(一:I,q:F)->L{let(x,y)=仸(q);L::书{一,x,y}}impl
伴<'_>{fn run(&mut self){let j=self;if j.仫(){return;}j.乛();if j.仫(){return;}j.乏();j.俾();}fn 义(&self,不:F,乫:F)->C{C::from(self.s.map().久(不,乫))}fn 个(&mut self,下:L)->T{let
为=self.s.个(下);if 为{self.万.push(下);}为}fn 亸(&self,丠:&J)->T{丠.丕()==Some(么::乧(self.z))}fn 仫(&mut self)->T{let j=self;if j.s.丕().is_some(){return false;}for
q in j.s.予().iter(){if j.义(q,j.丐)!=1{continue;}let Some((丏,_))=j.s.仕(q)else{continue;};let 下=伞(丏,j.丐);let mut 丠=j.s.clone();if 丠.个(下)&&j.亸(&丠){return j.个(下);}}let
Some(並)=j.侸()else{return false;};let mut 丠=j.s.clone();if!並.iter().all(|&下|丠.个(下))||!j.亸(&丠){return false;}for 下 in 並{j.个(下);}true}fn 侸(&self)->P<K<L>>{let
j=self;const 伯:C=C::MAX;let 乕=j.s.丨().乩();let 丢:K<G> =(1..=乕).map(|一|j.s.主(j.z,一)).collect();let 仄=|q:F|{丢.iter().position(|串|串.test(q)).map(|与|(与 as I+1,10*(与
as C+1)))};let 丄=j.s.丄(j.z);let mut 丆=[伯;M];let mut 严=[F::MAX;M];let mut 为=[false;M];for q in 丄.iter(){丆[q]=0;}loop{let mut 丂:P<F> =None;for q in 0..M{if!为[q]&&丆[q]<伯&&丂.is_none_or(|n|丆[q]<丆[n]){丂=Some(q);}}let
Some(q)=丂 else{break;};为[q]=true;if q==j.丐{break;}for&举 in&j.s.map().世[0][q]{if 举==乙{break;}let 举=F::from(举);if 为[举]||丄.test(举){continue;}let Some((_,争))=仄(举)else{continue;};if
丆[q]+争<丆[举]{丆[举]=丆[q]+争;严[举]=q;}}}if 丆[j.丐]>j.s.丈(j.z){return None;}let mut 並=K::new();let mut q=j.丐;while!丄.test(q){let(一,_)=仄(q)?;並.push(佷(一,q));q=严[q];}並.reverse();Some(並)}fn
乨(&self,丸:&J)->C{let(丁,二)=(self.z,self.乲);let 严=&self.s;let 丑=|串:G|C::from(串.count());let 丒=丑(丸.丟(丁))-丑(严.丟(丁));let 仵=丑(严.丟(二))-丑(丸.丟(二));let 俋=丑(严.丄(二))-丑(丸.丄(二));let
仨=严.享(二)-丸.享(二);let 丞=丑(严.丞()&严.丟(二))-丑(丸.丞()&丸.丟(二));12*丒+20*仵+8*(俋-仵).max(0)+5*仨+25*丞}fn 佰(&self,q:F)->C{let j=self;let 丶=j.s.map().临.test(q);if 丶&&j.s.丨().仟()&&!j.s.丟(j.z).test(q){20}else{0}}fn
住(&self,丿:C)->T{let j=self;j.s.丅(j.乲).iter().any(|q|j.义(q,j.仦)<=丿)}fn 乛(&mut self){let j=self;let mut 丅:K<(C,F)> =j.s.予().iter().map(|q|(j.义(q,j.丐),q)).collect();丅.sort_unstable();for(俈,q)in
丅{if j.s.丕().is_some(){return;}let Some((丏,一))=j.s.仕(q)else{continue;};let 丢=j.s.主(j.z,一);let 举=j.s.map().世[j.z][q];let mut 三:P<(C,F)> =None;for&丂 in 举.iter().take_while(|&&丂|丂!=乙){let
丂=F::from(丂);if!丢.test(丂){continue;}let mut 丠=j.s.clone();if!丠.个(伞(丏,丂)){continue;}let 丛=if j.亸(&丠){100000}else{j.乨(&丠)+10*(俈-j.义(丂,j.丐))+j.佰(丂)};if 三.is_none_or(|(乼,_)|丛>乼){三=Some((丛,丂));}}if
let Some((丛,丂))=三{if 丛>0{j.个(伞(丏,丂));}}}}fn 乏(&mut self){let j=self;if j.s.丕().is_some(){return;}if j.住(侣){j.俼();}else{j.丞();}}fn 俼(&mut self){let j=self;if!j.s.丨().伆()||j.s.丈(j.z)<亏{return;}let
举=j.s.map().世[j.z][j.仦];let 丌:K<F> =举.iter().take_while(|&&q|q!=乙).map(|&q|F::from(q)).collect();let 乆=j.s.丟(j.z);if 丌.iter().any(|&q|乆.test(q)&&j.s.乢().test(q)){return;}for
q in 丌{let(x,y)=仸(q);if j.个(L::亐{x,y}){return;}}}fn 丞(&mut self){let j=self;if!j.s.丨().仟(){return;}while j.s.丈(j.z)>=亏{let 亓=j.s.map().临&j.s.丟(j.z)&j.s.丄(j.z)&j.s.仂();let
Some(q)=亓.iter().min_by_key(|&q|(j.义(q,j.仦),q))else{return;};let(x,y)=仸(q);if!j.个(L::乾{x,y}){return;}}}fn 俾(&mut self){let j=self;let 佬=if j.上<=3{0}else{5};for
_ in 0..侒{if j.s.丕().is_some(){return;}let 丈=j.s.丈(j.z);if 丈<10{return;}let 俊=j.住(侈);let 仮=j.s.亖(j.z);let mut 三:P<(C,L)> =None;let mut 伨:P<(C,L)> =None;let
mut 佁=G::Q;for 一 in 1..=j.s.丨().乩(){let 丢=j.s.主(j.z,一);let 乸=(仮&丢).两(佁);佁=丢;let 争=10*C::from(一);if 争>丈{continue;}for q in 乸.iter(){let 下=佷(一,q);let mut
丠=j.s.clone();if!丠.个(下){continue;}let 会=丠.丈(j.z);let 专=丠.专(j.z);let 仨=j.s.享(j.乲)-丠.享(j.乲);let 侮=if 仨>0&&俊{会+专>=0}else{专>=0&&(会>=佬||仨>0)};if!侮{continue;}let
乨=j.乨(&丠);let 丛=乨+j.佰(q)-6*j.义(q,j.丐)-10*(C::from(一)-1);if 乨>0{if 三.is_none_or(|(乼,_)|丛>乼){三=Some((丛,下));}}else if 专>=3&&会>=佬+10&&一==1{if 伨.is_none_or(|(乼,_)|丛>乼){伨=Some((丛,下));}}}}let
Some((_,下))=三.or(伨)else{return;};if!j.个(下){return;}}}}fn 俆(l:&J,万:&mut Vec<L>){万.clear();俗(l,万);}const 亄:C=C::MAX;const 侑:F=8;struct 亟{丆:[C;M],严:[I;M],}fn
代(l:&J,z:F,乛:T,丐:F)->C{let mut 丆=亄;係(l,z,乛,|丙,new|{if new.test(丐){丆=10*丙 as C;}丆!=亄});丆}fn 俜(l:&J,z:F,介:G,丫:C)->G{let mut 仲=G::Q;佒(l,z,介,|丙,new|{if 10*丙
as C>丫{return true;}仲|=new;false});仲.两(l.丄(z))}fn 係(l:&J,z:F,乛:T,乽:impl FnMut(F,G)->T){佒(l,z,介(l,z,乛),乽);}fn 介(l:&J,z:F,乛:T)->G{if 乛&&z==l.东(){l.予()}else{G::Q}}fn
佒(l:&J,z:F,介:G,mut 乽:impl FnMut(F,G)->T,){let 乕=l.丨().乩();let mut 仄=[G::Q;3];let mut 乁=G::Q;for 一 in 1..=乕{let 串=l.主(z,一);仄[F::from(一)-1]=串.两(乁);乁|=串;}let
mut 丼=[l.丄(z),G::Q,G::Q,G::Q];for q in(介&l.丅(z)).iter(){if let Some(一)=l.亙(q).filter(|&一|一<=乕){丼[0]|=G::q(q).世()&l.主(z,一);}}let mut 为=G::Q;for 丙 in 0..{let
new=丼[丙%4].两(为);丼[丙%4]=G::Q;if 乽(丙,new){return;}为|=new;let 丂=new.世().两(为);for(侯,丌)in 仄.iter().enumerate(){丼[(丙+侯+1)%4]|=丂&*丌;}if 丼.iter().all(|串|串.两(为).is_empty()){break;}}}impl
亟{fn new(l:&J,z:F,乛:T)->亟{let 乕=F::from(l.丨().乩());let mut 丢=[G::Q;3];for(与,串)in 丢.iter_mut().enumerate().take(乕){*串=l.主(z,与 as I+1);}let 丄=l.丄(z);let
世=&l.map().世[0];let mut 丆=[亄;M];let mut 严=[乙;M];let mut 丼=[0u8;M];let mut 丑=0;let mut 佮=G::Q;for q in 丄.iter(){丆[q]=0;丼[丑]=q as I;丑+=1;}if 乛&&z==l.东(){for
不 in l.予().iter(){let Some(一)=l.亙(不)else{continue;};let 乡=G::q(不).世()&丢[F::from(一)-1];for q in 乡.iter(){if 丆[q]!=0{丆[q]=0;丼[丑]=q as I;丑+=1;}}}}while 丑>0{let
mut 三=0;for 与 in 1..丑{let(a,b)=(F::from(丼[与]),F::from(丼[三]));if(丆[a],a)<(丆[b],b){三=与;}}let q=F::from(丼[三]);丑-=1;丼[三]=丼[丑];佮.set(q);for&丂 in&世[q]{if 丂==乙{break;}let
丂=F::from(丂);if 佮.test(丂)||丄.test(丂){continue;}let Some(一)=丢[..乕].iter().position(|串|串.test(丂))else{continue;};let 佶=丆[q]+10*(一 as C+1);if 佶<丆[丂]{if 丆[丂]==亄{丼[丑]=丂
as I;丑+=1;}丆[丂]=佶;严[丂]=q as I;}}}亟{丆,严}}fn path(&self,丐:F)->K<F>{let mut path=K::new();if 丐>=M||self.丆[丐]==亄{return path;}let mut q=丐;while self.严[q]!=乙{path.push(q);q=F::from(self.严[q]);}path.reverse();path}}struct
仚{七:K<L>,丆:C,}fn 俍(l:&J,z:F,q:F)->P<I>{(1..=l.丨().乩()).find(|&一|l.主(z,一).test(q))}fn 俟(丬:&mut J,丁:F,亣:F,丫:C,七:&mut Vec<L>,习:&mut i64,)->T{loop{if 丬.丕()==Some(么::乧(丁)){return
true;}if 丬.丟(丁).test(亣)&&丬.丄(丁).test(亣){return true;}let 丯=亟::new(丬,丁,false);let 丆=丯.丆[亣];if 丆==亄||*习+丆>丫{return false;}let Some(&q)=丯.path(亣).first()else{return
false;};let Some(一)=俍(丬,丁,q)else{return false;};let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丬.个(下){return false;}*习+=10*C::from(一);七.push(下);}}fn 伿(l:&J,丁:F,俅:&[F],丫:C)->P<仚>{let
mut 丬=l.clone();let mut 七=K::new();let mut 习=0;for&亣 in 俅{if!俟(&mut 丬,丁,亣,丫,&mut 七,&mut 习){return None;}}(习<=丫).then_some(仚{七,丆:习,})}fn 伌(l:&J,丐:F,丫:C)->P<仚>{if
丫<0||l.丕().is_some()||丐>=M||l.map().丷.test(丐){return None;}let 丁=l.东();if l.丟(丁).test(丐)&&l.丄(丁).test(丐){return Some(仚{七:K::new(),丆:0,});}let mut 三=伿(l,丁,&[丐],丫);let
乊=亟::new(l,丁,false);let mut 乢:K<F> =(l.乢()&l.丟(1-丁)).iter().filter(|&乇|乇!=丐&&乊.丆[乇]!=亄).collect();乢.sort_by_key(|&乇|(l.map().久(乇,丐),乇));乢.truncate(侑);for
乇 in 乢{if let Some(並)=伿(l,丁,&[乇,丐],丫){if 三.as_ref().is_none_or(|乼|並.丆<乼.丆){三=Some(並);}}}三}const 侏:F=20000;struct 仛{七:K<L>,丆:C,丌:C,丅:C,乏:C,专:C,丘:C,}struct
仾{丌:C,丅:C,乏:C,}fn 佖(l:&J,z:F)->仾{let 丄=l.丄(z);let 丅=l.丅(z)&丄;let 丛=丅.iter().filter_map(|q|l.亙(q)).map(|一|10*C::from(一)).sum();仾{丌:C::from(丄.count()),丅:丛,乏:C::from(((l.丞()|l.乢())&丄).count()),}}struct
伶{丁:F,仹:G,严:仾,专:C,亪:F,丫:C,亮:F,path:K<L>,丧:K<仛>,index:HashMap<亽,F,BuildHasherDefault<DefaultHasher>>,}type 亽=[U;3];fn 倅(mut 乌:亽,q:F)->亽{乌[q/64]|=1<<(q%64);乌}impl
伶{fn 位(&mut self,丬:&J,乶:P<F>,仵:亽,丆:C,)->T{let j=self;let 丁=j.丁;let 乸=match 乶{None=>丬.亖(丁),Some(q)=>G::q(q).世(),}&j.仹;let 乸=乸.两(丬.丟(丁));let mut 丢=[G::Q;3];for
一 in 1..=丬.丨().乩(){丢[F::from(一)-1]=丬.主(丁,一);}for q in 乸.iter(){let 乕=F::from(丬.丨().乩());let Some(与)=丢[..乕].iter().position(|串|串.test(q))else{continue;};let
一=与 as I+1;let 争=10*C::from(一);if 丆+争>j.丫{continue;}if j.亮>=侏{return false;}j.亮+=1;let mut 丂=丬.clone();let 下=L::书{一,x:(q%O)as I,y:(q/O)as I,};if!丂.个(下){continue;}j.path.push(下);let
亴=倅(仵,q);j.保(&丂,亴,丆+争);let 侺=j.path.len()<j.亪&&丂.丕().is_none();let 俄=!侺||j.位(&丂,Some(q),亴,丆+争);j.path.pop();if!俄{return false;}}true}fn 保(&mut self,丸:&J,亴:亽,丆:C){let
j=self;let 仌=1-j.丁;let 伟=佖(丸,仌);let 丌=j.严.丌-伟.丌;let 丅=j.严.丅-伟.丅;if 丌<=0&&丅<=0{return;}let 乏=j.严.乏-伟.乏;let 专=丸.专(j.丁)-j.专;let 倈=丸.丕()==Some(么::乧(j.丁));let
丘=if 倈{C::MAX/2}else{3*丌+丅+10*乏+2*专-丆};if let Some(&丹)=j.index.get(&亴){if 丆>=j.丧[丹].丆{return;}}let 丝=仛{七:j.path.clone(),丆,丌,丅,乏,专,丘,};match j.index.get(&亴){Some(&丹)=>j.丧[丹]=丝,None=>{j.index.insert(亴,j.丧.len());j.丧.push(丝);}}}}fn
佋(l:&J,亪:F,丫:C,丿:F)->K<仛>{乍(l,亪,丫,丿).0}fn 乍(l:&J,亪:F,丫:C,丿:F)->(K<仛>,F){if l.丕().is_some()||亪==0||丿==0{return(K::new(),0);}let 丁=l.东();let 仌=1-丁;let mut
伤=伶{丁,仹:l.丟(仌)|l.丟(仌).世(),严:佖(l,仌),专:l.专(丁),亪,丫,亮:0,path:K::new(),丧:K::new(),index:HashMap::default(),};伤.位(l,None,[0;3],0);let 亮=伤.亮;let mut 丧=伤.丧;丧.sort_by(|a,b|b.丘.cmp(&a.丘).then(a.丆.cmp(&b.丆)));丧.truncate(丿);(丧,亮)}const
仞:C=1000000;const 侐:C=200;fn 乎(l:&J,z:F)->C{let 丄=l.丄(z);let 丅:C=(l.丅(z)&丄).iter().filter_map(|q|l.亙(q)).map(|一|10*C::from(一)).sum();l.丈(z)+3*l.专(z)+4*C::from(丄.count())+丅+10*C::from((l.乢()&丄).count())}fn
井(l:&J,z:F)->C{let 丆=代(l,z,false,Z[1-z]);if 丆==亄{return C::MIN;}l.丈(z)+l.专(z)-丆}fn 佨(井:C)->C{井.clamp(-侐,0)}fn 俀(l:&J,z:F)->(C,[C;2]){let 丩=1-z;match l.丕(){Some(么::乧(倄))=>{return(if
倄==z{仞}else{-仞},[0,0])}Some(么::伮)=>return(0,[0,0]),None=>{}}let mut 丘=乎(l,z)-乎(l,丩);if z==l.东(){let mut 丂=l.clone();丂.仃();if 丂.丕().is_none()&&代(&丂,丩,true,Z[z])<=丂.丈(丩){丘-=仞/2;}}let
亩=[井(l,z),井(l,丩)];(丘+佨(亩[0])-佨(亩[1]),亩)}
