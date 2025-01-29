# mfhx官网后端

# Hifuufantasy Club API 文档



# API 文档进度

## 任务情况
- [x] 获取全部成员信息接口
- [x] 获取成员信息数量接口
- [x] 获取成员信息接口
- [ ] 获取帖子数量接口
- [ ] 获取特定帖子的详细信息接口

## **接口列表**

### 1. 获取成员信息

**API 请求**：

```
GET https://hifuufantasy.club/api/member/<id>
```

**请求示例**：

```
GET https://hifuufantasy.club/api/member/1
```

**返回示例**：

```json
{
    "name": "柒灵子",
    "position": "主催",
    "avatar": "https://hifuufantasy.club/icu_avatar/qilingzi.png",
    "introduction": "这个人是社团的主催，浙江省湖州人，十七岁是男娘。"
}
```

**字段说明**：

- `name`: 成员的名字
- `position`: 成员的职务
- `avatar`: 成员的头像 URL
- `introduction`: 成员的个人介绍

------

### 2. 获取全部成员信息

**API 请求**：

```
GET https://hifuufantasy.club/api/icu_all
```

**返回示例**：

```json
[
    {
        "name": "柒灵子",
        "position": "主催",
        "avatar": 	"https://hifuufantasy.club/icu_avatar/qilingzi.png"
    }
]
```

**字段说明**：

- `name`:  成员的名字
- `position`:  成员的职务
- `avatar`:  成员的头像 URL

------

### 3. 获取成员信息数量

**API 请求**：

```
GET https://hifuufantasy.club/api/post_number
```

**返回示例**：

```json
{
    "number": 5
}
```

**字段说明**：

- `post_count`: 当前社团中的信息数量
