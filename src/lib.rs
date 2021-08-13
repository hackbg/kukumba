#[macro_export] macro_rules! kukumba {

    (
        $ErrorType:ty,
        $(
            #[$unit:ident]
            $( $($op:ident $desc:literal)+ { $($stmt:stmt;)* } )*
        )*
    ) => {
        $(#[test] fn $unit () -> Result<(), $ErrorType> {
            $(
                $(print!("\n  {} {}", stringify!($op), $desc);)+
                print!(" ");
                $($stmt;);*
            )*;
            Ok(())
        })*
    };

}

#[macro_export] macro_rules! kukumba_harnessed {
    (
        type Error = $Error:ty;
        let $Test:ident: $Harness:ty;
        $( $name:ident { $(
          $op:ident $desc:literal $({$($body:tt)+})?
        )* })*
    ) => {
        #[cfg(test)] fn kukumba_test_setup () {
            // before all
        }
        #[cfg(test)] fn kukumba_test_prepare () -> $Harness {
            // before each
            <$Harness>::new()
        }
        #[cfg(test)] fn kukumba_test_cleanup () {
            // after each
        }
        #[cfg(test)] fn kukumba_test_teardown () {
            // after all
        }
        $(
            #[test] fn $name () -> Result<(), $Error> {
                let mut $Test = kukumba_test_prepare();
                $(
                    print!("\n  {} {}", stringify!($op), $desc);
                    print!(" ");
                    $($($body)+)?;
                )*
                Ok(())
            }
        )*
    };
}
