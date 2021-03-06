/*
    Rust fonksiyonel dil özelliklerini de bünyesinde barındırır.
    Yani fonksiyonları sonradan kullanmak üzere değişkenlere atamak,
    başka fonksiyonlardan fonksiyon döndürmek,
    başka fonksiyonlara fonksiyonları parametre olarak verebilmek gibi
    temel fonksiyonel dil kabiliyetlerine de sahiptir.

    Pattern Matching, enum, clouser ve iterator kavramları Rust'ın öne çıkan fonksiyonel dil yetenekleridir.

    Closures: isimsiz fonksiyonlardır. Parametre olarak fonksiyonlara aktarılabilirler veya bir değişkende tutulabilirler.
    Değişkenlere atanabilmeleri bir yerde tanımlanıp tamamen farklı bir context içerisinde kullanılabilmelerine olanak sağlar.
*/
fn main() {
    /*
        #1 Basit Closure örnekleri ile başlayalım.
    */

    // Örneğin isimsiz bir fonksiyonu bir değişkende tutabilir ve kod akışına göre çağırılmasını sağlayabiliriz
    let div = |x: f32, y: f32| -> f32 {
        if y == 0.0 {
            panic!("SIfıra bölme sorunu")
        }
        x / y
    };

    println!("10/2.4={}", div(10.0, 2.4)); // div değişkenine atanmış fonksiyonu çağırdık

    /*
        Tabii yukarıdaki kullanımın bir fonksiyon çağırımı ile neredeyse aynı olduğunu ifade edebiliriz.
        Ancak closure'ları fonksiyonlara parametre olarak geçebilmek veya döndürebilmek önemli bir avantajdır.
        Şimdi buna bakalım.

        call fonksiyonu generic tanımlanmıştır ve F için Fn trait'i ile ifade edilmiştir. Buna göre f32 tipinden parametre
        alan ve yine f32 türünden değer döndüren closure'lar call fonksiyonuna yollanabilir.

        closure'ları parametre olarak geçerken FnOnce, FnMut ve Fn trait'lerine ihtiyacımız vardır nitekim bir closure bunlardan en az birini uyarlamak zorundadır(Generic kullanımlarda bu önem kazanıyor)
    */
    call(div, 3.2, 9.4);

    /*
        Closure tanımlarken dönen türü belirtmek zorunda değilizdir.
        Rust derleyici bunu tahmin eder. Ancak burada dikkat edilmesi gereken bir durum vardır.
        Aşağıdaki tanımlamaya dikkat edelim.
        do_something türü belli olmayan value isimli bir parametre alıyor ve bunu aynen geriye döndürüyor.
    */
    let do_something = |value| value;
    let summary = do_something(3); // Burada tipi tahmin etti ve artık i32 ile çalışacağı belli oldu
    println!("{}", summary);
    //let other_summary = do_something(3.1415); // Bu satırda ise kod derlenmeyecektir. "expected integer, found floating-point numberrustc(E0308)"
    // Çünkü ilk kullanımla birlikte do_something fonksiyonunun çalışacağı tür i32 olarak belirlenmiştir

    /*
        Game struct'ının closure ile birlikte kullanımı.
        new(constructor)'a bir fonksiyon aktardık. Artık içerideki find_medal fonksiyonu bu fonksiyonu baz alarak çalışacak
    */
    let mut blizard = Game::new(|point| point + 1);
    println!("{:?}", blizard.find_medal(18));
    println!("{:?}", blizard.find_medal(32));

    // blizard.medal_calculator = |p| (p + 10 / 2);
    // println!("{:?}", blizard.find_medal(16));

    /*
        Closure'ları fonksiyonlardan ayıran bir özellik de,
        bulundukları kapsamdaki değişkenlere erişebiliyor olmalarıdır.
        Aynen aşağıdaki örnekte olduğu gibi.
        Tabii bu durumda closure'un çevreden çektiği değişkeni sahiplenmesi söz konusudur ki bu da bellekte bu değişkenler için yer ayırdığı anlamına gelir.
        Performans açısından dikkat edilmesi gereken bir durum.
    */
    let some_number = 10;
    let process_function = |n: i32| n + some_number; // isimsiz fonksiyon içerisinde yukarıda tanımlı (main scope'una dahil) some_number değişkenine erişilmiştir
    let processed = process_function(5);
    println!("{},{}", processed, some_number);

    /*
        Yukarıda closure ile yaptığımız şeyi aşağıdaki gibi yapamayız.
        Derleyici "can't capture dynamic environment in a fn item" şeklinde hata verecektir
    */
    // let another_number=11;
    // fn add(nbr: i32) -> i32 {
    //     nbr + another_number
    // };

    /*
        Bu arada process_function kullanımı ile ilgili olarak şunu da öğrendim gibi.
        Closure'un çevre değişkenleri sahiplenmesi 3 Trait ile mümkün oluyor. Fn, FnMut ve FnOnce
        process_function, some_number'ı sadece okuduğu için Fn Trait'ini uygular.
        Ama kod bloğunda some_number'ı değiştirip kullanmak istersek aşağıdaki gibi bir yol izlememiz gerekir
        ki bu durumda FnMut Trait'i devreye girer.(mut kullanımlarına dikkat)
    */
    let mut some_number2 = 10;
    let mut process_function2 = |n: i32| {
        some_number2 += 1;
        n + some_number2
    };
    let processed2 = process_function2(5);
    println!("{},{}", processed2, some_number2);

    /*
        Bir closure farklı bir thread'e alınırken sahiplendiği verinin de mutlak suretle taşınmasını istersek
        move komutunu kullanabiliriz. Bu durum Concurrency konusunda değer kazanacak.
    */

    /*
        Fonksiyonlardan fonksiyon dönebileceğimizden de bahsetmiştik.
        Aşağıda örnek bir kod yer alıyor.
        get_fn fonksiyonu, parametre olarak gelen Enum türüne göre geriye Fn(i32,i32)->i32 türünden uygun fonksiyonu döndürüyor.
        Eğer toplama işlemi yaptırmak istersek toplama, çarpma yaptırmak istersek de çarpma fonksiyonu gibi...
    */
    let now_what = get_fn(Process::Division);
    println!("Division {}", now_what(16, 4));
    let now_what = get_fn(Process::Extraction);
    println!("Extraction {}", now_what(12, 6));
}

// Geriye fonksiyon döndüreceğimiz için impl Fn gibi tanım yaptık (FnMut, FnOnce da söz konusu olabilir tabii)
fn get_fn(process: Process) -> impl Fn(i32, i32) -> i32 {
    // match ile process enum durumlarına bakıyoruz
    // ve uygun bir fonksiyonu geriye döndürüyoruz. Süper değil mi?
    match process {
        Process::Addition => |x, y| x + y,
        Process::Multiplication => |x, y| x * y,
        Process::Division => |x, y| {
            if y == 0 {
                panic!("Sıfıra bölme durumu");
            } else {
                x / y
            }
        },
        Process::Extraction => |x, y| x - y,
    }
}

enum Process {
    Addition,
    Multiplication,
    Extraction,
    Division,
}

fn call<F>(closure: F, a: f32, b: f32)
where
    F: Fn(f32, f32) -> f32,
{
    let result = closure(a, b);
    println!("{}", result);
}

/*
    Closure'ları parametre olarak geçebildiğimizden bahsediyoruz.
    Örneğin bir Struct'ın bir alanını da closure olarak tanımlayabiliriz.

    Game isimli generic struct i32 tipinden değer alıp yine i32 türünden değer döndüren bir fonksiyonu
    medal_calculator alanında taşıyacak şekilde tanımlandı.

    new(constructor) fonksiyon parametre olarak gelen fonksiyonu medal_calculator alanına atıyor.
    find_medal fonksiyonunda ise gelen argüman değeriner göre closure fonksiyonunu çağırıyor.
    Struct'a atanan hesaplama fonksiyonu ne ise (medal_calculator'a atanan fonksiyon) o icra ediliyor.
*/
struct Game<T>
where
    T: Fn(i32) -> i32,
{
    medal_calculator: T,
    current_point: i32,
}

impl<T> Game<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calc: T) -> Game<T> {
        Game {
            medal_calculator: calc,
            current_point: 0,
        }
    }

    fn find_medal(&mut self, arg: i32) -> i32 {
        let value = (self.medal_calculator)(arg);
        self.current_point = value;
        value
    }
}
