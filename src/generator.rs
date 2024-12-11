use either::Either;

use crate::{
    component::ICalComponent,
    values::{
        ICalValue,
        binary::*, date::*, datetime::*, duration::*, geo::*, integer::*, period::*, recur::*,
        text::*,
    },
    property::*
};

#[derive(Debug, PartialEq)]
pub enum GetPropError {
    PropertyMissing,
    WrongValueType,
}

gen_prop_methods!(
    /// 3.7 Calendar Properties
    calscale One Text,
    method   One Text,
    prodid   One Text,
    version  One Text,

    /// 3.8.1 Descriptive Component Properties
    attach      Many Either Text Binary,
    categories  Many TextList,
    class       One  Text,
    comment     Many Text,
    description One  Text,
    geo One     Geo,
    location    One  Text,
    percent_complete One  Integer,
    priority    One  Integer,
    resources   Many TextList,
    status      One  Text,
    summary     One  Text,


    /// 3.8.2 Date and Time Component Properties
    completed One DateTime,
    dtend     One Either DateTime Date,
    due       One Either DateTime Date,
    dtstart   One Either DateTime Date,
    duration  One Duration,
    freebusy  One Period,
    transp    One Text,


    /// 3.8.3 Time Zone Component Properties
    tzid         One Text,
    tzname       One Text,
    tzoffsetfrom One Text,
    tzoffsetto   One Text,
    tzurl        One Text,


    /// 3.8.4 Relationship Component Properties
    attendee      Many Text,
    contact       Many Text,
    organizer     One  Text,
    recurrence_id One  Either DateTime Date,
    related_to    Many Text,
    url           One  Text,
    uid           One  Text,

    /// 3.8.5 Recurrence Component Properties
    exdate Many Either DateTimeList DateList,
    rrule  One  Recur,

    /// 3.8.6 Alarm Component Properties
    action  One Text,
    repeat  One Integer,
    trigger One Either Duration DateTime,

    /// 3.8.7 Change Management Component Properties
    created       One DateTime,
    dtstamp       One DateTime,
    last_modified One DateTime,
    sequence      One Integer,

    /// 3.8.8 Miscellaneous Component Properties
    request_status Many Text,
);

macro_rules! gen_prop_methods {
    (
        $(
            $(#[$field_meta:meta])*
            $prop:ident $($count:ident $typ:ident $(($typ1:ident $typ2:ident))?)+,
        )+
    ) => {
        impl ICalComponent {
            $(
                gen_prop_methods!(@prop_methods $(#[$field_meta])* $prop $($count $typ)+);
            )+
        }
    };

    (@prop_methods $(#[$field_meta:meta])* $prop:ident Many Either $typ1:ident $typ2:ident) => {
        paste::paste! {
            //INSERTS a new property
            $(#[$field_meta])*
            fn [<$prop _prop>](&mut self, prop: ICalProperty) -> &mut Self {
                self.insert_prop(gen_prop_methods!(@prop_name $prop), prop)
            }

            //INSERTS a new property with given value and parameters
            $(#[$field_meta])*
            pub fn [<$prop _with_params>](&mut self, value: [<ICal $typ1>], params: ICalParameterMap) -> &mut Self {
                self.[<$prop _prop>](ICalProperty::new(value.into(), params))
            }

            //INSERTS a new property with given value & no parameters
            $(#[$field_meta])*
            pub fn $prop(&mut self, value: [<ICal $typ1>]) -> &mut Self {
                self.[<$prop _prop>](ICalProperty::from_value(value.into()))
            }

            //INSERTS a new property with (non-default) value
            //also adds VALUE=XXX parameter because this type is non default
            $(#[$field_meta])*
            pub fn [<$prop _ $typ2:lower>](&mut self, value: [<ICal $typ2>]) -> &mut Self {
                let mut p = ICalProperty::from_value(value.into());
                p.set_param("VALUE", &p.value.to_value_param().to_string());
                self.[<$prop _prop>](p);
                self
            }

            //Returns mutable ref to all properties
            $(#[$field_meta])*
            pub fn [<get_ $prop _prop>](&mut self) -> Option<&mut Vec<ICalProperty>> {
                self.get_props(gen_prop_methods!(@prop_name $prop))
            }

            //Returns immutable ref to all values of either types
            $(#[$field_meta])*
            pub fn [<get_ $prop _values>](&self) -> Result<Vec<Either<&[<ICal $typ1>], &[<ICal $typ2>]>>, GetPropError> {
                Ok(self.props.get_vec(gen_prop_methods!(@prop_name $prop))
                    .ok_or(GetPropError::PropertyMissing)?
                    .iter()
                    .try_fold(Vec::new(), |mut acc, prop| {
                        acc.push(prop.get_as_either().ok_or(GetPropError::WrongValueType)?);
                        Ok(acc)
                    })?)
            }
        }
    };

    (@prop_methods $(#[$field_meta:meta])* $prop:ident Many $typ:ident) => {
        paste::paste! {
            //INSERTS a new property
            $(#[$field_meta])*
            fn [<$prop _prop>](&mut self, prop: ICalProperty) -> &mut Self {
                self.insert_prop(gen_prop_methods!(@prop_name $prop), prop)
            }

            //INSERTS a new property with given value and parameters
            $(#[$field_meta])*
            pub fn [<$prop _with_params>](&mut self, value: [<ICal $typ>], params: ICalParameterMap) -> &mut Self {
                self.[<$prop _prop>](ICalProperty::new(value.into(), params))
            }

            //INSERTS a new property with given value & no parameters
            $(#[$field_meta])*
            pub fn $prop(&mut self, value: [<ICal $typ>]) -> &mut Self {
                self.[<$prop _prop>](ICalProperty::from_value(value.into()))
            }

            //Returns mutable ref to all properties
            $(#[$field_meta])*
            pub fn [<get_ $prop _prop>](&mut self) -> Option<&mut Vec<ICalProperty>> {
                self.get_props(gen_prop_methods!(@prop_name $prop))
            }

            //Returns immutable ref to all property values
            $(#[$field_meta])*
            pub fn [<get_ $prop _values>](&self) -> Result<Vec<&[<ICal $typ>]>, GetPropError> {
                Ok(self.props.get_vec(gen_prop_methods!(@prop_name $prop))
                    .ok_or(GetPropError::PropertyMissing)?
                    .iter()
                    .try_fold(Vec::new(), |mut acc, prop| {
                        acc.push(prop.get_as().ok_or(GetPropError::WrongValueType)?);
                        Ok(acc)
                    })?)
            }
        }
    };

    (@prop_methods $(#[$field_meta:meta])* $prop:ident One Either $typ1:ident $typ2:ident) => {
        paste::paste! {
            //SETS property to given value and parameters or creates a new one if it doesn't exist
            $(#[$field_meta])*
            pub fn [<$prop _with_params>](&mut self, value: [<ICal $typ1>], params: ICalParameterMap) -> &mut Self {
                let p = ICalProperty::new(value.into(), params);
                self.set_prop(gen_prop_methods!(@prop_name $prop), p)
            }

            //SETS the property's value or creates a new property if it doesn't exist
            $(#[$field_meta])*
            pub fn $prop(&mut self, value: [<ICal $typ1>]) -> &mut Self {
                self.set_prop_value(gen_prop_methods!(@prop_name $prop), value.into())
            }

            //SETS the property's (non-default) value or creates a new property if it doesn't exist
            //also adds VALUE=XXX parameter because this type is non default
            $(#[$field_meta])*
            pub fn [<$prop _ $typ2:lower>](&mut self, value: [<ICal $typ2>]) -> &mut Self {
                let prop_value: ICalValue = value.into();
                let prop_name = gen_prop_methods!(@prop_name $prop);
                let value_param = prop_value.to_value_param().to_string();
                self.set_prop_value(prop_name, prop_value);
                self.set_prop_param(prop_name, "VALUE", value_param);
                self
            }

            //Returns mutable ref to property
            $(#[$field_meta])*
            pub fn [<get_ $prop _prop>](&mut self) -> Option<&mut ICalProperty> {
                self.get_prop(gen_prop_methods!(@prop_name $prop))
            }

            //Returns immutable ref to either value
            $(#[$field_meta])*
            pub fn [<get_ $prop _value>](&self) -> Result<Either<&[<ICal $typ1>], &[<ICal $typ2>]>, GetPropError> {
                Ok(self.props.get(gen_prop_methods!(@prop_name $prop))
                    .ok_or(GetPropError::PropertyMissing)?
                    .get_as_either()
                    .ok_or(GetPropError::WrongValueType)?)
            }
        }
    };

    (@prop_methods $(#[$field_meta:meta])* $prop:ident One $typ:ident) => {
        paste::paste! {
            //SETS property to given value and parameters or creates a new one if it doesn't exist
            $(#[$field_meta])*
            pub fn [<$prop _with_params>](&mut self, value: [<ICal $typ>], params: ICalParameterMap) -> &mut Self {
                let p = ICalProperty::new(value.into(), params);
                self.set_prop(gen_prop_methods!(@prop_name $prop), p)
            }

            //SETS the property's value or creates a new property if it doesn't exist
            $(#[$field_meta])*
            pub fn $prop(&mut self, value: [<ICal $typ>]) -> &mut Self {
                self.set_prop_value(gen_prop_methods!(@prop_name $prop), value.into())
            }

            //Returns mutable ref to property
            $(#[$field_meta])*
            pub fn [<get_ $prop _prop>](&mut self) -> Option<&mut ICalProperty> {
                self.get_prop(gen_prop_methods!(@prop_name $prop))
            }

            //Returns immutable ref to value
            $(#[$field_meta])*
            pub fn [<get_ $prop _value>](&self) -> Result<&[<ICal $typ>], GetPropError> {
                Ok(self.props.get(gen_prop_methods!(@prop_name $prop))
                    .ok_or(GetPropError::PropertyMissing)?
                    .get_as()
                    .ok_or(GetPropError::WrongValueType)?)
            }
        }
    };

    (@prop_name $prop:ident) => {
        paste::paste! {
            &stringify!([<$prop:upper>]).replace("_", "-")
        }
    };
}

pub(crate) use gen_prop_methods;

impl ICalComponent {
    pub fn uid_random(&mut self) -> &mut Self {
        let uuid = uuid7::uuid7();
        self.uid(uuid.to_string())
    }
}

// -- RDate --
pub enum RDateValue<'a> {
    DateTimeList(&'a ICalDateTimeList),
    DateList(&'a ICalDateList),
    PeriodList(&'a ICalPeriodList)
}

impl ICalComponent {
    //SETS the property's value or creates a new property if it doesn't exist
    pub fn rdate(&mut self, value: ICalDateTimeList) -> &mut Self {
        self.set_prop_value("RDATE", value.into())
    }

    fn rdate_non_default(&mut self, value: ICalValue) -> &mut Self {
        let value_param = value.to_value_param().to_string();
        self.set_prop_value("RDATE", value);
        self.set_prop_param("RDATE", "VALUE", value_param);
        self
    }

    //SETS the property's (non-default) value or creates a new property if it doesn't exist
    //also adds VALUE=DATE parameter because this type is non default
    pub fn rdate_datelist(&mut self, value: ICalDateList) -> &mut Self {
        self.rdate_non_default(value.into())
    }

    //SETS the property's (non-default) value or creates a new property if it doesn't exist
    //also adds VALUE=PERIOD parameter because this type is non default
    pub fn rdate_periodlist(&mut self, value: ICalPeriodList) -> &mut Self {
        self.rdate_non_default(value.into())
    }

    //Returns immutable ref to value
    pub fn get_rdate(&self) -> Result<RDateValue, GetPropError> {
        let prop = self.props.get("RDATE").ok_or(GetPropError::PropertyMissing)?;
        Ok(match &prop.value {
            ICalValue::DateList(v) => RDateValue::DateList(&v),
            ICalValue::DateTimeList(v) => RDateValue::DateTimeList(&v),
            ICalValue::PeriodList(v) => RDateValue::PeriodList(&v),
            _ => return Err(GetPropError::WrongValueType)
        })
    }
}
