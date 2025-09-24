#![no_std]

use core::{fmt, iter};
use core::ops::Fn;
use core::fmt::{Debug, Formatter};

#[doc(hidden)]
pub struct FnFormat<F>(pub F)
    where F: Fn(&mut Formatter<'_>) -> fmt::Result;

impl<F> Debug for FnFormat<F>
    where F: Fn(&mut Formatter<'_>) -> fmt::Result {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

#[doc(hidden)]
/// This trait and implementation is a replacement of stds functions with the same name.
/// In core, they are flagged as unstable without any issue, implying they are not meant for general use.
/// For more information about their use, refer to rust source.
pub trait FormatterExt {
    fn debug_struct_field1_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str, value1: &dyn Debug
    ) -> fmt::Result;

    fn debug_struct_field2_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str, value1: &dyn Debug,
        name2: &str, value2: &dyn Debug,
    ) -> fmt::Result;

    fn debug_struct_field3_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str, value1: &dyn Debug,
        name2: &str, value2: &dyn Debug,
        name3: &str, value3: &dyn Debug,
    ) -> fmt::Result;

    fn debug_struct_field4_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str, value1: &dyn Debug,
        name2: &str, value2: &dyn Debug,
        name3: &str, value3: &dyn Debug,
        name4: &str, value4: &dyn Debug,
    ) -> fmt::Result;

    fn debug_struct_field5_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str, value1: &dyn Debug,
        name2: &str, value2: &dyn Debug,
        name3: &str, value3: &dyn Debug,
        name4: &str, value4: &dyn Debug,
        name5: &str, value5: &dyn Debug,
    ) -> fmt::Result;

    fn debug_struct_fields_finish<'b>(
        &'b mut self,
        name: &str,
        names: &[&str],
        values: &[&dyn Debug],
    ) -> fmt::Result;

    fn debug_tuple_field1_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug
    ) -> fmt::Result;

    fn debug_tuple_field2_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
    ) -> fmt::Result;

    fn debug_tuple_field3_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
        value3: &dyn Debug,
    ) -> fmt::Result;

    fn debug_tuple_field4_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
        value3: &dyn Debug,
        value4: &dyn Debug,
    ) -> fmt::Result;

    fn debug_tuple_field5_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
        value3: &dyn Debug,
        value4: &dyn Debug,
        value5: &dyn Debug,
    ) -> fmt::Result;

    fn debug_tuple_fields_finish<'b>(
        &'b mut self,
        name: &str,
        values: &[&dyn Debug],
    ) -> fmt::Result;
}

impl<'a> FormatterExt for Formatter<'a> {
    fn debug_struct_field1_finish<'b>(&'b mut self, name: &str, name1: &str, value1: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_struct(name);
        builder.field(name1, value1);
        builder.finish()
    }

    fn debug_struct_field2_finish<'b>(&'b mut self, name: &str, name1: &str, value1: &dyn Debug, name2: &str, value2: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_struct(name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.finish()
    }

    fn debug_struct_field3_finish<'b>(&'b mut self, name: &str, name1: &str, value1: &dyn Debug, name2: &str, value2: &dyn Debug, name3: &str, value3: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_struct(name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.field(name3, value3);
        builder.finish()
    }

    fn debug_struct_field4_finish<'b>(&'b mut self, name: &str, name1: &str, value1: &dyn Debug, name2: &str, value2: &dyn Debug, name3: &str, value3: &dyn Debug, name4: &str, value4: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_struct(name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.field(name3, value3);
        builder.field(name4, value4);
        builder.finish()
    }

    fn debug_struct_field5_finish<'b>(&'b mut self, name: &str, name1: &str, value1: &dyn Debug, name2: &str, value2: &dyn Debug, name3: &str, value3: &dyn Debug, name4: &str, value4: &dyn Debug, name5: &str, value5: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_struct(name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.field(name3, value3);
        builder.field(name4, value4);
        builder.field(name5, value5);
        builder.finish()
    }

    fn debug_struct_fields_finish<'b>(&'b mut self, name: &str, names: &[&str], values: &[&dyn Debug]) -> fmt::Result {
        core::assert_eq!(names.len(), values.len());

        let mut builder = self.debug_struct(name);
        for (name, value) in iter::zip(names, values) {
            builder.field(name, value);
        }

        builder.finish()
    }

    fn debug_tuple_field1_finish<'b>(&'b mut self, name: &str, value1: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_tuple(name);
        builder.field(value1);
        builder.finish()
    }

    fn debug_tuple_field2_finish<'b>(&'b mut self, name: &str, value1: &dyn Debug, value2: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_tuple(name);
        builder.field(value1);
        builder.field(value2);
        builder.finish()
    }

    fn debug_tuple_field3_finish<'b>(&'b mut self, name: &str, value1: &dyn Debug, value2: &dyn Debug, value3: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_tuple(name);
        builder.field(value1);
        builder.field(value2);
        builder.field(value3);
        builder.finish()
    }

    fn debug_tuple_field4_finish<'b>(&'b mut self, name: &str, value1: &dyn Debug, value2: &dyn Debug, value3: &dyn Debug, value4: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_tuple(name);
        builder.field(value1);
        builder.field(value2);
        builder.field(value3);
        builder.field(value4);
        builder.finish()
    }

    fn debug_tuple_field5_finish<'b>(&'b mut self, name: &str, value1: &dyn Debug, value2: &dyn Debug, value3: &dyn Debug, value4: &dyn Debug, value5: &dyn Debug) -> fmt::Result {
        let mut builder = self.debug_tuple(name);
        builder.field(value1);
        builder.field(value2);
        builder.field(value3);
        builder.field(value4);
        builder.field(value5);
        builder.finish()
    }

    fn debug_tuple_fields_finish<'b>(&'b mut self, name: &str, values: &[&dyn Debug]) -> fmt::Result {
        let mut builder = self.debug_tuple(name);
        for value in values {
            builder.field(value);
        }
        builder.finish()
    }
}