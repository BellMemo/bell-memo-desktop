import { Autocomplete, TextField } from "@mui/material";
import { useState } from "react";
import styles from "./Header.module.less";
import { Log } from "@src/util/log";

export const Header = () => {
  const [searchValue, setSearchValue] = useState("");

  const handleSearch = () => {
    Log.info('test');
  };

  return (
    <div className={styles.header}>
      <Autocomplete
        freeSolo
        options={[]}
        renderInput={(params) => (
          <TextField
            id="outlined-basic"
            label="检索"
            variant="outlined"
            {...params}
            value={searchValue}
            onChange={(e) => {
              setSearchValue(e.target.value);
              handleSearch();
            }}
          />
        )}
      />
    </div>
  );
};
