import { Autocomplete, TextField } from "@mui/material";
import { useState } from "react";
import styles from './Header.module.less';

export const Header = () => {
  const [searchValue, setSearchValue] = useState("");
  return (
    <div className={styles.header}>
      <Autocomplete
        freeSolo
        options={[]}
        renderInput={(params) => (
          <TextField id="outlined-basic" label="检索" variant="outlined" {...params} />
        )}
      />
    </div>
  );
};
