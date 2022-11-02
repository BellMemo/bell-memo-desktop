import { Autocomplete, TextField } from "@mui/material";
import { useCallback, useState } from "react";
import styles from "./Header.module.less";
import { invoke } from "@tauri-apps/api";
import { debounce } from "lodash-es";
import { Action } from "./Action";

export const Header = () => {
  const [searchValue, setSearchValue] = useState("");

  const handleSearch = async (v) => {
    const result = await invoke("select_memo_data", {
      params: {
        content: v,
      },
    });
    console.log(result);
  };

  const debounceSearch = useCallback(debounce(handleSearch, 1000), []);

  const handleChange = (v) => {
    setSearchValue(v);
    debounceSearch(v);
  };

  return (
    <div className={styles.header}>
      <Autocomplete
        freeSolo
        options={[]}
        className={styles.search}
        renderInput={(params) => (
          <TextField
            id="outlined-basic"
            label="检索"
            variant="outlined"
            {...params}
            value={searchValue}
            onChange={(e) => {
              handleChange(e.target.value);
            }}
          />
        )}
      />
      <Action />
    </div>
  );
};
