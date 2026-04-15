
        if check_key_id(&MLKEM512, algorithm_id) {
            return Ok(&MLKEM512);
        } else 

        if check_key_id(&MLKEM768, algorithm_id) {
            return Ok(&MLKEM768);
        } else 

        if check_key_id(&MLKEM1024, algorithm_id) {
            return Ok(&MLKEM1024);
        } 
