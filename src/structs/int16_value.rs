#[derive(Clone, Default, Debug)]
pub struct Int16Value {
    _value: Option<i16>,
}
impl Int16Value {
    pub(crate) fn _get_value_string(&self) -> String {
        self._value.unwrap_or_default().to_string()
    }

    pub(crate) fn _set_value(&mut self, value: i16) -> &mut Int16Value {
        self._value = Some(value);
        self
    }

    pub(crate) fn _set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int16Value {
        self._set_value(value.into().parse::<i16>().unwrap())
    }

    pub(crate) fn _has_value(&self) -> bool {
        self._value.is_some()
    }

    pub(crate) fn _get_hash_string(&self) -> String {
        self._value
            .map_or_else(|| String::from("empty!!"), |v| v.to_string())
    }
}
