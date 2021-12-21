
# fuzzy_logic
Fuzzy logic inference implementation.

[Documentation](http://kineticcookie.github.io/fuzzy_logic/fuzzy_logic/index.html)

[![Build Status](https://travis-ci.org/KineticCookie/fuzzy_logic.svg?branch=master)](https://travis-ci.org/KineticCookie/fuzzy_logic)
## Usage
This crate is not in `crates.io` yet.
So you can pull it from GitHub:

```

toml
[dependencies.fuzzy_logic]
git = "https://github.com/KineticCookie/fuzzy_logic"
```
And declare the new crate with:
```rust
extern crate fuzzy_logic;
```

## Twenkid
15.12.2021
Debugging examples ... use.rs ...
Why NaN?

functions.rs singleton ...

16.12.2021
Studying the code, debugging. Singleton - discrete, doesn't interpolate ...

lechev-space: debug prints done the rust-way:
https://github.com/LechevSpace/fuzzy_logic/commit/252b7b2de9de9b32ead7d48765029319c2ad5d4b

## Разсъждения и пр.

[ 16 декември 2021 г. 12:03 ] ⁨Todor⁩: Вече разбрах повече кода, пуснах и с различни параметри, и това value = 0.5 ми се струва странно (дали не е нормално да връща NaN).
0.5 е в рамките на обхватите за output, а му се подава -1.9, което е в рамките на x, x_dest. 
На singleton се подава само една стойност, всички други са с повече (интерполация), а тук връща само 1 или 0 което в коментарите на кода значи дали се среща в множеството или не (т.е. не връща стойността на интерполацията, за разлика от другите функции).

Когато го викам с параметри, които са кръгли, като -2.0 и 2.0 (NB, PB), тогава връща за изходното множество -0.5 (NB), което е правилно по таблицата.
```
Също сега, като гледам как е коментирано:
   // NB
        //("x_dest".into(), -1.9_f32), //-1.9_f32),
        ("x_dest".into(), -2.0_f32), //-1.9_f32),
        // NS
        ("x".into(), 2.0_f32),
		//("x".into(), 0.0_f32),
```
Според разбирането ми засега -1.9 не е NB, само 2.0, а функцията не интерполира нищо, само връща (според коментара) дали е в множеството или не е - и наистина не е.
```
[ 16 декември 2021 г. 12:04 ] ⁨Todor⁩: value, xt, value-xt < eps = -0.5,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = -0.25,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = 0,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = 0.25,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = 0.5,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
```
[src\inference.rs:78] &result = Set { name: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB
cache: k:-0.5 v:1
 }
Set: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB value: -0.5

(с принтове на това което трябва да изчисли в singleton)
[ 16 декември 2021 г. 12:05 ] ⁨Todor⁩:``` let xt = -2.0; //-1.9;
println!("value, xt, value-xt < eps = {},{},{}", value, xt, value-xt < eps);```
[ 16 декември 2021 г. 12:07 ] ⁨Todor⁩: Съобщение със снимка
[ 16 декември 2021 г. 12:07 ] ⁨Todor⁩: https://codecrucks.com/fuzzy-terminologies-all-you-need-to-know/
[ 16 декември 2021 г. 12:22 ] ⁨Todor⁩: Хм, още изясняване/объркване: сингълтона е за изхода, pitch, а за входа е триъгълно, там трябва да има интерполация. Обаче на сингълтона му се подават параметри, които са за триъгълника?
[ 16 декември 2021 г. 12:23 ] ⁨Todor⁩: Пускам и триъгълника: получава се деление на нула май:
```
 pub fn triangular(a: f32, b: f32, c: f32) -> Box<MembershipFunction> {
		println!("a,b,c={},{},{}", a,b,c);
		let mut t = 0.0;
		let xt = -2.0;
		if a <= xt && xt <= b {
                t = 1.0 - (b - xt) / (b - a)
            } else if b <= xt && xt <= c {
                t = 1.0 - (xt - b) / (c - b)
            } else {
                t= 0.0
            }
			
	    println!("Triangular, t == {}", t);
        Box::new(move |x: f32| {
        ...
 
[ 16 декември 2021 г. 12:23 ] ⁨Todor⁩: a,b,c=-2,-2,-1
Triangular, t == NaN
a,b,c=-2,-1,0
Triangular, t == 0
a,b,c=-1,0,1
Triangular, t == 0
a,b,c=0,1,2
Triangular, t == 0
a,b,c=1,2,2
Triangular, t == 0
a,b,c=-2,-2,-1
Triangular, t == NaN
a,b,c=-2,-1,0
Triangular, t == 0
a,b,c=-1,0,1
Triangular, t == 0
a,b,c=0,1,2
Triangular, t == 0
a,b,c=1,2,2
Triangular, t == 0
  ```
[ 16 декември 2021 г. 12:24 ] ⁨Todor⁩: Макар че така, ако се подадат кръгли числа -2.0, 2.0 за x_dest, x дава май верен изход -0.5

[ 16 декември 2021 г. 12:26 ] ⁨Todor⁩: При -1.9, триъгълната интерполация дава числови стойности, но после - НаН
```
a,b,c=-2,-2,-1
Triangular, t == 0.9
a,b,c=-2,-1,0
Triangular, t == 0.100000024
a,b,c=-1,0,1
Triangular, t == 0
a,b,c=0,1,2
Triangular, t == 0
a,b,c=1,2,2
Triangular, t == 0
a,b,c=-2,-2,-1
Triangular, t == 0.9
a,b,c=-2,-1,0
Triangular, t == 0.100000024
a,b,c=-1,0,1
Triangular, t == 0
a,b,c=0,1,2
Triangular, t == 0
a,b,c=1,2,2
Triangular, t == 0
value, xt, value-xt < eps = -0.5,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = -0.25,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = 0,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = 0.25,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]
value, xt, value-xt < eps = 0.5,-2,false
singleton if num::abs(value - xt) < eps 1.0  else 0.0; t = ? [0]

[src\inference.rs:78] &result = Set { name: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB
cache:  }
Set: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB value: NaN
```
[ 16 декември 2021 г. 12:32 ] ⁨Todor⁩: Гледам и че трябва да смята уж по центъра на тежестта:  ```      defuzz_func: DefuzzFactory::center_of_mass(),```
(...)

[ 16 декември 2021 г. 12:59 ] ⁨LechevSpace⁩: Може и да използваш dbg! Като тя може и да ти върне подадената стойност.
Там при тестовете трябва да ползваш cargo test -- --nocapture
[ 16 декември 2021 г. 13:00 ] LechevSpace⁩: my_fn(dbg!(func_param))

[ 16 декември 2021 г. 13:00 ] ⁨Todor⁩: ok 10x

[ 16 декември 2021 г. 13:00 ] ⁨LechevSpace⁩: Така ще ти изкара debug стойността за структурата

[ 16 декември 2021 г. 13:00 ] ⁨Todor⁩: но това подаване на -1.9 заедно с 0.5 не ми изглежда правилно, може да бъркам ,но не виждам логиката

[ 16 декември 2021 г. 13:00 ] ⁨LechevSpace⁩: std::fmt:: Debug

[ 16 декември 2021 г. 13:01 ] ⁨Todor⁩: едното е от единия обхват, другото е от другия

[ 16 декември 2021 г. 13:02 ] ⁨Todor⁩: По-логично ми изглежда да е стойност закръглена/интерполирана към втория обхват, на -1.9 би отговаряло -0.4х

[ 16 декември 2021 г. 13:02 ] ⁨Todor⁩: -0.45 или нещо такова

[ 16 декември 2021 г. 13:06 ] ⁨Todor⁩: А, или то самото си праща съответното от двата обхвата.

[ 16 декември 2021 г. 13:08 ] ⁨Todor⁩: Да, при другите, които интерполират, с 0.5 и 1.9 би интерполирало. А тук т.е. мисля че трябва value да се интерполира до другия, ще пробвам да видя какво ще даде.

[ 16 декември 2021 г. 13:14 ] ⁨Todor⁩: В случая само /4.0 го преобразува, но пак е същото, ако не е "кръгло" чисто от множеството просто си връща извън него, било -0.475 или -1.9.
  
  [ 16 декември 2021 г. 13:21 ] ⁨Todor⁩: Пуснах с trianglular, странно пак дава value NaN обаче сега като се замисля, този NaN май няма значение.
  ```
[ 16 декември 2021 г. 13:32 ] ⁨Todor⁩: Set: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB value: NaN

```
Ако се вземе стойността като дискретна -0.5, изходът е даден дискретен singleton, т.е. мисля че там трябва да закръгля към най-близката стойност. Да, добре щеше да е и да я записва във value, обаче тя е равна на NB от съответното множество (от тия ламбди функции обаче не съм сигурен как точно да прочета, Хеш/множество,  
output.0[output.0.len()-1] ...
[ 16 декември 2021 г. 13:33 ] ⁨Todor⁩: Не.

[ 16 декември 2021 г. 13:49 ] ⁨Todor⁩: Не знам, но май разбрах откъде идва NaN. От rules.rs, compute( ...)

```
let result_values = set
            .cache
            .borrow()
            .iter()
            .filter_map(|(&key, &value)| {
                if value <= expression_result {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, f32>>();
  ```
[ 16 декември 2021 г. 13:49 ] ⁨Todor⁩: let expression_result = (*self.condition).eval(context);

[ 16 декември 2021 г. 13:50 ] ⁨Todor⁩: Само че после ще го гледам повече.

[ 16 декември 2021 г. 17:25 ] ⁨Todor⁩: За dbg и#[derive(Debug)],  и 
```
impl Debug for InferenceContext<'_>{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>{
		f.debug_struct("context")
         .field("values", &self.values)
         .field("universes", &self.universes)
		 .field("options", &self.options)
         .finish()
	}
} и т.н.

- То добре, но почва каскада от:

error[E0277]: `(dyn for<'r> Fn(&'r Set) -> f32 + 'static)` doesn't implement `Debug`
   --> src\inference.rs:113:32
    |
113 |          .field("defuzz_func", &self.defuzz_func)
    |                                ^^^^^^^^^^^^^^^^^ `(dyn for<'r> Fn(&'r Set) -> f32 + 'static)` cannot be formatted using `{:?}` because it doesn't implement `Debug`
    |
    = help: the trait `Debug` is not implemented for `(dyn for<'r> Fn(&'r Set) -> f32 + 'static)`
    = note: required because of the requirements on the impl of `Debug` for `Box<(dyn for<'r> Fn(&'r Set) -> f32 + 'static)>`
    = note: required for the cast to the object type `dyn Debug`

For more information about this error, try `rustc --explain E0277`.
warning: `fuzzy_logic` (lib) generated 3 warnings
error: could not compile `fuzzy_logic` due to previous error; 3 warnings emitted
```
Трябва на всичко да програмираш ръчно частни случаи по полета с fmt, просто за да им изпише стойността? (поне не зн сега друг вариант). 

Трябва да видя някакъв профайлър, трейсър, без да е нужно да пишеш на всичко ръчно.
(...)

[ 16 декември 2021 г. 17:28 ]LechevSpace⁩: #[derive(Debug, Clone, Copy, PartialEq, Eq) - и още много други

[ 16 декември 2021 г. 17:29 ] ⁨Todor⁩: Debug го включих, не беше достатъчен.

[ 16 декември 2021 г. 17:29 ] LechevSpace⁩: Можеш и ръчно, но попинцип е с някаква причина

[ 16 декември 2021 г. 17:29 ] ⁨Todor⁩: затова го коментирах и минах на impl Debug


(...)


[ 19 декември 2021 г. 23:46 ] ⁨Todor⁩: За сравненето на плаващи зап. в Ръст и епсилон, дето говорихме - гледам че в тестовете на фузито си има такава конст. от Ръст. https://doc.rust-lang.org/std/primitive.f32.html#associatedconstant.EPSILON

#[cfg(test)]
mod test {
(....)

    #[test]
    fn sigmoidal() {
        let steepness = 2.0;
        for i in -100..100 {
            let midpoint = i as f32;
            let f = MembershipFactory::sigmoidal(steepness, midpoint);
            let diff = (0.5 - f(midpoint)).abs();
            assert!(diff <= f32::EPSILON);
        }
    }
}
[ 19 декември 2021 г. 23:56 ] ⁨Todor⁩: За размитата логика - сингълтон функцията ти ли си я е писал? Гледах оригиналното хранилище, там не я виждам.
Забелязвам отново, че в другите функции използват и x за върнатата стойност,, умножават, изваждат от него и т.н. т,е, това тегло на принадлежност зависи и от х, докато в сегашното сингълтон се връща само 1.0 и 0.0 (без значение от x) и то в мн. особен случай на съвпадение.

[ 20 декември 2021 г. 0:14 ] ⁨Todor⁩: Док. разбирам, compute на библиотеката не е направен за сингълтони, а за интерполации. А сингълтона иска дискретни фиксирани стойности -0.5, -0.25 ,... Т.е.  ако е така, мисля че ако се направи една матрица :

[ 20 декември 2021 г. 0:14 ] ⁨Todor⁩: в която Z, PS и т.н. се заменят със съответните 0, 0.25 и т.н.

И просто се адресират чрез класовете, които се разпознаят за x_d и x.

[ 20 декември 2021 г. 0:14 ] ⁨Todor⁩: Съобщение със снимка

[ 20 декември 2021 г. 0:15 ] ⁨Todor⁩: -1.9, -1.0 ==> NB, NS
NB, NS = 0, 1 
defuzz[0][1] = -0.25

[ 20 декември 2021 г. 0:16 ] ⁨Todor⁩: 0, 0.25, 0.5, 0.5, 0.5
-0.25, 0, 0.25, 0.5, 0.5
...
[ 20 декември 2021 г. 10:13 ] ⁨Todor⁩: За сингълтона, при сегашната версия, да проверява типа и ако е сингълтон да смята така. не зн дали няма много да наруши заплетената архитектура за интерполации, cache.borrow, колко да се преработва.

[ 20 декември 2021 г. 14:50 ] ⁨Todor⁩: Може за триъгълниците, според мен имплементацията е това което описвам, иначе като смисъл/приложение - ако актуаторите /АЦП са със съответни стъпки, но това може да се закръгли и без това.

[ 20 декември 2021 г. 14:52 ] ⁨Todor⁩: Иначе - и на мен в началото ми беше странно защо се ползва сингълтон, да връща дискретни стойности, като целта на размитата логика са по-скор плавни.

[ 20 декември 2021 г. 14:55 ] ⁨Lachezar Lechev⁩: това също е част от екперимента. В началото на доклада казват че са стигнали до тези стойности тествайки го
[ 20 декември 2021 г. 14:56 ] ⁨Lachezar Lechev⁩: тъй като работят в куб 4х4х4м може би и движенията не са толкова резки

[ 20 декември 2021 г. 14:57 ] ⁨Lachezar Lechev⁩: аз със сигуроност бих пробвал няколко екперимента като направя логиката от доклада
[ 20 декември 2021 г. 14:57 ] ⁨Lachezar Lechev⁩: даже си мисля вече как да го променя да използва GPS координати или NED

[ 20 декември 2021 г. 14:58 ] ⁨Lachezar Lechev⁩: напр. ако е "близо до точката" (5 м) да използва по-финни джижения (и по-малък pitch), а при големи разстояния да има голям pitch

[ 20 декември 2021 г. 14:58 ] ⁨Lachezar Lechev⁩: съответно да се движи по-бързо

[ 20 декември 2021 г. 14:59 ] ⁨Todor⁩: На мен още не ми е ясно защо е нужна специална "логика" за това, това са траектории/интерполации, обратна връзкаю

[ 20 декември 2021 г. 15:00 ] ⁨Todor⁩: трябва да прочета статията внимателно...

[ 20 декември 2021 г. 15:00 ] ⁨Todor⁩: и това NS, NB, PS ... Този запис по-скоро обърква според мен.

[ 20 декември 2021 г. 15:00 ] ⁨Lachezar Lechev⁩: да

[ 20 декември 2021 г. 15:00 ] ⁨Lachezar Lechev⁩: съгласен съм

[ 20 декември 2021 г. 15:01 ] ⁨Lachezar Lechev⁩: трябва да са някакви стойности тип:

много близо
близо
Средно
Далеч
Много далеч
[ 20 декември 2021 г. 15:02 ] ⁨Lachezar Lechev⁩: и всяко да си има опредена стойност от нас

[ 20 декември 2021 г. 15:04 ] ⁨Todor⁩: Да, и също тези етикети са нужни за индикатори и т.н, а иначе ако са нормализирани стойности от -1, 1 е по-ясно, после само се мащабира (умножава)

[ 20 декември 2021 г. 15:05 ] ⁨Todor⁩: Също запис със знаци за сравнение може да са по-лесни за запомняне и работа за хора, освен онези, които изброи.


```
[ 20 декември 2021 г. 18:00 ] ⁨Todor⁩: Пуснах го на триъгълник ето така (мисля че това емулира сингълтон, "изроден триъгълник" )

    let pitch_output = UniversalSet::new("pitch_output")
            .with_domain(vec![-0.5, -0.25, 0.0, 0.25, 0.5])
                .add_set("NB", MembershipFactory::triangular(-0.5, -0.5, -0.5))
                .add_set("NS", MembershipFactory::triangular(-0.25, -0.25, -0.25))
                .add_set("Z", MembershipFactory::triangular(0.0, 0.0, 0.0))
                .add_set("PS", MembershipFactory::triangular(0.25, 0.25, 0.25))
                .add_set("PB", MembershipFactory::triangular(0.5, 0.5,0.5));
[ 20 декември 2021 г. 18:02 ] ⁨Todor⁩: С -2.0, -1.0 резултатът мисля че излезе коректен, но не в принта който беше направен (там честно казано не го разчитам, повтарят се разни неща). Сложих принтове там където смята Some(...) и понякога поставя None, в rules.rs, compute:

/// Computes the current rule. Returns the fuzzy set as the result.
    pub fn compute(&self, context: &InferenceContext) -> Set {
        let expression_result = (*self.condition).eval(context);
		println!("expression_result={}", expression_result);
[ 20 декември 2021 г. 18:03 ] ⁨Todor⁩: И там се вижда:

expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=1
key, value, expression_result=-0.25,NaN,1
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
[src\inference.rs:98] &result = Set { name: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB
cache:  }
Set: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB
 ### value: NaN
[ 20 декември 2021 г. 18:03 ] ⁨Todor⁩: И в цикъла:

let result_values = set
            .cache
            .borrow()
            .iter()
            .filter_map(|(&key, &value)| {
                //Some((key, value))
                println!("key, value, expression_result={},{},{}", key, value, expression_result);
                if value <= expression_result {
                    Some((key, value))
                } else {
                    None
                }
            })
[ 20 декември 2021 г. 18:04 ] ⁨Todor⁩: Този принт е от цикъла, key,value, ...
[ 20 декември 2021 г. 18:04 ] ⁨Todor⁩: Та там "value" е принадлежността.
[ 20 декември 2021 г. 18:04 ] ⁨Todor⁩: И за -0.25 е 1, за другите е 0.
[ 20 декември 2021 г. 18:04 ] ⁨Todor⁩: Като се направи на -1.9, -1, дава 0.9 за -0.25 и 0.1 за 0 (това май и преди със сингълтона излизаше? ако не греша, имаше нещо такова)
[ 20 декември 2021 г. 18:05 ] ⁨Todor⁩: Първото множество е триъгълно, и затова предполагам че може да е такова не цяло число.
[ 20 декември 2021 г. 18:05 ] ⁨Todor⁩: И след това може да се закръгля, взимаме най-голямото.
[ 20 декември 2021 г. 18:05 ] ⁨Todor⁩: expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=0.9
key, value, expression_result=-0.25,NaN,0.9
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=0.25,NaN,0
//END compute()
expression_result=0.100000024
key, value, expression_result=0,NaN,0.100000024
//END compute()
expression_result=0
key, value, expression_result=-0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
[src\inference.rs:98] &result = Set { name: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB
cache:  }
Set: pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: NB UNION pitch_output: PS UNION pitch_output: Z UNION pitch_output: NS UNION pitch_output: NB UNION pitch_output: NB
 ### value: NaN
[ 20 декември 2021 г. 18:07 ] ⁨Todor⁩: Това value сега не го виждам откъде точно се присвоява, но според мен от такива данни и без него се получава резултатът (няма значение, че е NaN)
[ 20 декември 2021 г. 18:10 ] ⁨Todor⁩: Друг тест:
 let input = vec![
        ("x_dest".into(), -1.0_f32), //-1.9_f32), //2.0_f32),
        ("x".into(), 1.0_f32), //-1.0_f32),  /
    ]
    .into_iter()
    .collect();
    //-1.0, 1.0 = NS, PS ==> NB
[ 20 декември 2021 г. 18:10 ] ⁨Todor⁩: expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.25,NaN,0
//END compute()
expression_result=1
key, value, expression_result=-0.5,NaN,1
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
[ 20 декември 2021 г. 18:11 ] ⁨Todor⁩: =====> key, value, expression_result=-0.5,NaN,1  
//END compute()

NB от изхода е -0.5
[ 20 декември 2021 г. 18:17 ] ⁨Todor⁩: NB, PB (-2, 2) --> NB (-0.5) също е вярно:
   values: {
        "x_dest": -2.0,
        "x": 2.0,
    },
    options: InferenceOptions {
        logic_ops: ZadehOps,
        set_ops: MinMaxOps,
        defuzz_func: "dyn core::ops::function::Fn<(&fuzzy_logic::set::Set,)>+Output = f32 & TypeId { t: 8626939776008802616 }",
    },
}
expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()

*** expression_result=1
===>>>>>>>>> key, value, expression_result=-0.5,NaN,1 <===============

//END compute()
expression_result=0
key, value, expression_result=0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=0,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.25,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
//END compute()
expression_result=0
key, value, expression_result=-0.5,NaN,0
[ 20 декември 2021 г. 18:23 ] ⁨Todor⁩: :Д 

за PB, PB (2,2) --> всичко нули. (отговорът е Z)
Обаче и за др. дето не трябва да е нула дава всичко 0.
[ 20 декември 2021 г. 18:24 ] ⁨Todor⁩: Само че сега виждам друго от таблицата.
[ 20 декември 2021 г. 18:24 ] ⁨Todor⁩: Дето по-горе споменах "за какво им е размита логика".
[ 20 декември 2021 г. 18:24 ] ⁨Todor⁩: Доколкото виждам от таблицата, това е обратна връзка, разликата спрямо целевата стойност.
[ 20 декември 2021 г. 18:26 ] ⁨Todor⁩: x_dest = -1.9 NB, x = -1.0 NS ==> т.е. иска да отиде към x_dest, което е < от x, т.е. NS, т.е. курсът трябва да се коригира още на минус за да стигне NB
[ 20 декември 2021 г. 18:26 ] ⁨Todor⁩: Ако x_dest е PB а x е NB, значи има голямо отклонение, и корекцията трябва да е PB
[ 20 декември 2021 г. 18:26 ] ⁨Todor⁩: +0.5
[ 20 декември 2021 г. 18:26 ] ⁨Todor⁩: и става -1.5
[ 20 декември 2021 г. 18:27 ] ⁨Todor⁩: Пак е отрицателно NB, x_dest ако е PB пак PB + 0.5
```

```
[ 20 декември 2021 г. 18:29 ] ⁨Todor⁩: (Както и ти каза, че трябва да е по-фино като се приближава към границите на куба.)

[ 20 декември 2021 г. 18:49 ] ⁨Lachezar Lechev⁩: Мисля че е по-скоро координатите в куба от колкото границите на самия куб

[ 20 декември 2021 г. 18:51 ] ⁨Lachezar Lechev⁩: Ще видя кога ще се върна пак на Fuzzy проекта, но имам няколко идеи и за API-я, но със сигурност може просто да "копирам" логиката от друга библиотека (като python-ската)
(...)

[ 20 декември 2021 г. 18:52 ] ⁨Lachezar Lechev⁩: Ако решиш да експериментираш можеш да отвориш и pr

[ 20 декември 2021 г. 18:52 ] ⁨Lachezar Lechev⁩: Във Viber не мога да разбера кода

[ 20 декември 2021 г. 18:52 ] ⁨Todor⁩: pr? (може да го запиша и като Issue и т.н.)

[ 20 декември 2021 г. 18:54 ] ⁨Todor⁩: Може ли да сложиш в хранилището линк и към статията, от която е таблицата? (за пълнота и за др. ако погледнат)
(някъде нагоре в хронологията е)

[ 20 декември 2021 г. 18:56 ] ⁨Todor⁩: Кодът сега - само принтове и параметри. Ако се върже към симулация ще си покаже най-добре..

[ 20 декември 2021 г. 18:56 ] ⁨Todor⁩: Дали и как работи.

[ 20 декември 2021 г. 18:57 ] ⁨Todor⁩: От Газебо имаше една Чесна. (Не че на мен сега ми се занимава/предполагам и че ще е сложно да се свърже). Да се програмира да излети, и после да държи някакъв курс като използва тази логика (тези отклонения). https://www.youtube.com/watch?v=PkTrZhPtzXQ

[ 20 декември 2021 г. 18:59 ] ⁨Todor⁩: Да има и някакви ветрове, които да се генерират като шум с елемент на случайност в различни посоки и т.н. (не зн сега как се моделира в Газебото). И самолетът трябва да си коригира курса.

[ 20 декември 2021 г. 19:01 ] ⁨Todor⁩: https://gazebosim.org/tutorials?tut=aerodynamics&cat=physics

[ 20 декември 2021 г. 19:02 ] ⁨Todor⁩: Ветровете може да се симулират и ако изкуствено променяме координатите на самолета, без то "да знае" (ако е възможно), някаква външна сила, ние да си го смятаме. 

И може да се варира и плътността на въздуха, "турбуленции" и т.н.

[ 20 декември 2021 г. 19:03 ] ⁨Todor⁩: Съобщение със снимка

[ 20 декември 2021 г. 20:28 ] ⁨Lachezar Lechev⁩: Има опции за това в настройките на света

[ 20 декември 2021 г. 20:29 ] ⁨Lachezar Lechev⁩: Това ще е и темата на 2рата работилница - за симулации и по-конкретно gazebo

[ 20 декември 2021 г. 20:30 ] ⁨Lachezar Lechev⁩: Още не знам какво ще покрие но ще се променя .world файл и ще се добавят неща в средата

