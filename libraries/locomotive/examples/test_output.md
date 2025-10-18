cargo loco generate scaffold posts id:int title:string! content:text user_id:references:user! created_at:date_time
cargo loco generate scaffold users id:int name:string! email:string^ bio:string