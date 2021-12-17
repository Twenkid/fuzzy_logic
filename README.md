
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
