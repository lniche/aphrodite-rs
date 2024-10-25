CREATE TABLE "public"."t_user" (
  "user_code" text COLLATE "pg_catalog"."default" NOT NULL,
  "user_no" int8 NOT NULL,
  "username" text COLLATE "pg_catalog"."default",
  "nickname" text COLLATE "pg_catalog"."default",
  "password" text COLLATE "pg_catalog"."default",
  "salt" text COLLATE "pg_catalog"."default",
  "email" text COLLATE "pg_catalog"."default",
  "phone" text COLLATE "pg_catalog"."default" NOT NULL,
  "open_id" text COLLATE "pg_catalog"."default",
  "client_ip" text COLLATE "pg_catalog"."default",
  "login_at" timestamptz(6),
  "login_token" text COLLATE "pg_catalog"."default",
  "id" int8 NOT NULL DEFAULT nextval('t_user_id_seq'::regclass),
  "created_at" timestamptz(6),
  "updated_at" timestamptz(6),
  "deleted_at" timestamptz(6),
  "created_by" text COLLATE "pg_catalog"."default",
  "updated_by" text COLLATE "pg_catalog"."default",
  "version" int4 DEFAULT 1,
  "is_deleted" bool DEFAULT false,
  CONSTRAINT "t_user_pkey" PRIMARY KEY ("id"),
  CONSTRAINT "uni_t_user_user_code" UNIQUE ("user_code"),
  CONSTRAINT "uni_t_user_user_no" UNIQUE ("user_no"),
  CONSTRAINT "uni_t_user_username" UNIQUE ("username")
)
;

ALTER TABLE "public"."t_user" 
  OWNER TO "postgres";

CREATE INDEX "idx_t_user_phone" ON "public"."t_user" USING btree (
  "phone" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "public"."t_user"."user_code" IS '用户编码';

COMMENT ON COLUMN "public"."t_user"."user_no" IS '用户编号';

COMMENT ON COLUMN "public"."t_user"."username" IS '用户名';

COMMENT ON COLUMN "public"."t_user"."nickname" IS '昵称';

COMMENT ON COLUMN "public"."t_user"."password" IS '密码';

COMMENT ON COLUMN "public"."t_user"."salt" IS '盐值';

COMMENT ON COLUMN "public"."t_user"."email" IS '邮箱';

COMMENT ON COLUMN "public"."t_user"."phone" IS '电话';

COMMENT ON COLUMN "public"."t_user"."open_id" IS '微信OpenID';

COMMENT ON COLUMN "public"."t_user"."client_ip" IS '客户端IP';

COMMENT ON COLUMN "public"."t_user"."login_at" IS '登录时间';

COMMENT ON COLUMN "public"."t_user"."login_token" IS '登录令牌';