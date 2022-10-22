pub const CREATE_MEMO_DATA: &str = "CREATE TABLE IF NOT EXISTS `memo_data`  (
    /* 主键ID */
    `id` binary(36) NOT NULL,
    /* 标题 */
    `title` varchar(255) NOT NULL,
    /* 内容 */
    `content` text NOT NULL DEFAULT '',
    /* 创建时间 */
    `created` int(11) NOT NULL DEFAULT 0,
    /* 更新时间 */
    `updated` int(11) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
);";

pub const CREATE_MEMO_TAG: &str = "CREATE TABLE IF NOT EXISTS `memo_tag` (
    /* 主键ID */
    `id` binary(36) NOT NULL,
    /* 主键ID */
    `name` varchar(255) NOT NULL,
    /* 主键ID */
    `created` int(11) NOT NULL DEFAULT 0,
    /* 主键ID */
    `updated` int(11) NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`)
);";

pub const CREATE_MEMO_TAG_DATA: &str = "CREATE TABLE IF NOT EXISTS `memo_tag_data` (
    /* 主键ID */
    `id` binary(36) NOT NULL, 
    /* 主键ID */
    `tag_id` int(10) NOT NULL, 
    /* 主键ID */
    `memo_id` binary(36) NOT NULL, 
    /* 主键ID */
    `created` int(11) NOT NULL DEFAULT 0, 
    /* 主键ID */
    `updated` int(11) NOT NULL DEFAULT 0, 
    PRIMARY KEY (`id`)
);";
