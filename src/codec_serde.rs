#[macro_export]
macro_rules! CGM_codec{
    ( structname:$s_alias:ident,
     rename:{ $(($r_alias:ident,$r_function:ident,$r_type:ty,$r_rename:expr)),*$(,)*},
    optional:{ $(($o_alias:ident,$o_function:ident,$o_type:ty)),*$(,)*},
    rename_optional:{ $(($ro_alias:ident,$ro_function:ident,$ro_type:ty,$ro_rename:expr)),*$(,)*},
    else:{$(($e_alias:ident,$e_function:ident,$e_type:ty)),*$(,)*}
       ) =>{
#[derive(Serialize, Deserialize, Default,Debug, Clone)]
#[serde(default)]
        pub struct $s_alias{
             $(
    #[serde(rename = $r_rename)]
                pub $r_alias:$r_type,)*
            $(
    #[serde(rename = $ro_rename)]
 #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
                pub $ro_alias:Option<Option<$ro_type>>,)*
                            $(
                pub $e_alias:$e_type,)*
            $(
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
                pub $o_alias:Option<Option<$o_type>>),*
        }

        impl $s_alias{
            pub fn serialize_send(a:$s_alias) -> Result<String, serde_json::Error> {
                serde_json::to_string(&a)
            }
            pub fn deserialize_receive(json: &str) -> Result<$s_alias, serde_json::Error> {
                serde_json::from_str(json)
            }
            $( pub fn $e_function(&mut self,s:$e_type)->&mut Self{
                self.$e_alias = s;
                self
            })*
            $( pub fn $r_function(&mut self,s:$r_type)->&mut Self{
                self.$r_alias = s;
                self
            })*
            $( pub fn $o_function(&mut self,s:$o_type)->&mut Self{
                self.$o_alias = Some(Some(s));
                self
            })* 
            $( pub fn $ro_function(&mut self,s:$ro_type)->&mut Self{
                self.$ro_alias = Some(Some(s));
                self
            })* 
        }
    }
}
