import { Autocomplete, Checkbox, TextField } from "@mui/material";
import React, { useEffect, useState } from "react";
import {
  CheckBoxOutlineBlank as CheckBoxOutlineBlankIcon,
  CheckBox as CheckBoxIcon,
} from "@mui/icons-material";
import { invoke } from "@tauri-apps/api";
import { useMergedState } from "@src/hooks";
import { isEqual } from "lodash-es";
import { MemoTag } from "@src/types";

const icon = <CheckBoxOutlineBlankIcon fontSize="small" />;
const checkedIcon = <CheckBoxIcon fontSize="small" />;

interface SelectTagProps {
  value?: any[];
  onChange?: (v: any[]) => void;
}

export const SelectTag: React.FC<SelectTagProps> = (props) => {
  const { value: v, onChange: onValueChange } = props;
  const [tagOptions, setTagOptions] = useState([]);
  const [inputValue, setInputValue] = useState("");
  const [open, setOpen] = useState(false);

  const [value, setValue] = useMergedState([], {
    value: v,
    onChange: onValueChange,
  });

  const fetchTags = async () => {
    try {
      const resp = await invoke("search_memo_tag", {
        params: {
          content: "",
          offset: 0,
          limit: 1000,
        },
      });
      console.log(resp);
      setTagOptions(resp);
    } catch (error) {
      console.log(error);
    }
  };

  useEffect(() => {
    open && fetchTags();
  }, [open]);

  const handleAddTag = async (): Promise<MemoTag> => {
    try {
      const resp = await invoke("insert_memo_tag", {
        params: {
          content: inputValue,
        },
      });
      return resp;
    } catch (error) {
      console.log(error);
      return null as MemoTag;
    }
  };

  return (
    <Autocomplete
      multiple
      options={tagOptions}
      disableCloseOnSelect
      getOptionLabel={(option) => option.name}
      renderOption={(props, option, { selected }) => {
        return (
          <li {...props}>
            <Checkbox
              icon={icon}
              checkedIcon={checkedIcon}
              style={{ marginRight: 8 }}
              checked={selected}
            />
            {option.name}
          </li>
        );
      }}
      noOptionsText="暂无标签，您可以直接输入任意值然后回车添加"
      renderInput={(params) => (
        <TextField {...params} label="标签" placeholder="请输入标签" />
      )}
      value={value}
      onChange={(_e, values) => {
        setValue(values);
      }}
      isOptionEqualToValue={isEqual}
      inputValue={inputValue}
      onInputChange={(_e, v) => setInputValue(v)}
      onKeyDown={async ({ code }) => {
        
        if (code === "Enter") {
          const isExist = tagOptions.find(i => i.name === inputValue);
          if(isExist){
            setValue([isExist,...value]);
            return;
          }
          // 不存在的标签新增
          const t = await handleAddTag();
          if (t) {
            setTagOptions([t, ...tagOptions]);
            setValue([t, ...value]);
          }
        }
      }}
      open={open}
      onOpen={() => {
        setOpen(true);
      }}
      onClose={() => {
        setOpen(false);
      }}
    />
  );
};
