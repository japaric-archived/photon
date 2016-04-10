extern {
    pub fn HAL_Pin_Mode(pin: i16, mode: u8);
    pub fn HAL_GPIO_Write(pin: i16, value: u8);
    pub fn HAL_Delay_Milliseconds(delay: u32);
}
