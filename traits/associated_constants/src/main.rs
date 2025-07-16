trait Investment<T> {
    fn amount(&self)->T;

    // fn set_amount(&mut self, newamount: T);

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64>{
    const TAX_RATE: f64 = 0.25; 

    fn tax_bill(&self)-> f64{
        self.amount() *Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income{
    amount: f64,
}

impl Investment<f64> for Income{
    fn amount(&self)->f64 {
        self.amount
    }

    // fn set_amount(&mut self, newamount: f64){
    //     self.amount = newamount;
    // }

    fn double_amount(&mut self){
        self.amount * 2.0;
    }
}

impl Taxable for Income{}

#[derive(Debug)]
struct Bonus{
    amount: f64
}

impl Investment<f64> for Bonus{
    fn amount(&self)->f64 {
        self.amount
    }

    fn double_amount(&mut self) {
        self.amount * 2.0; 
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 12.0;
}

#[derive(Debug)]
struct QualityTime{
    minutes: u32
}

impl Investment<u32> for QualityTime {
    fn amount(&self)->u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes = self.minutes * 2;
    }
}

fn main() {
    let mut income = Income{
        amount: 5000.00
    };

    println!("org {:.2}",income.tax_bill());

    income.double_amount();

    println!("double {:.2}",income.tax_bill());

    let bonus = Bonus{
        amount: 12908.00
    };

    println!("{:.2}", bonus.tax_bill());

    let mut newQ = QualityTime{ minutes: 120};

    newQ.double_amount();

    println!("{:.2?}", newQ);
}
