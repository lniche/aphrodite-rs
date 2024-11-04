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
  "version" int8 DEFAULT 1,
  "is_deleted" bool DEFAULT false,
  CONSTRAINT "t_user_pkey" PRIMARY KEY ("id"),
  CONSTRAINT "uni_t_user_username" UNIQUE ("username"),
  CONSTRAINT "uni_t_user_user_code" UNIQUE ("user_code"),
  CONSTRAINT "uni_t_user_user_no" UNIQUE ("user_no")
)
;

ALTER TABLE "public"."t_user" 
  OWNER TO "postgres";

CREATE INDEX "idx_t_user_phone" ON "public"."t_user" USING btree (
  "phone" COLLATE "pg_catalog"."default" "pg_catalog"."text_ops" ASC NULLS LAST
);

COMMENT ON COLUMN "public"."t_user"."user_code" IS 'User code';

COMMENT ON COLUMN "public"."t_user"."user_no" IS 'User number';

COMMENT ON COLUMN "public"."t_user"."username" IS 'Username';

COMMENT ON COLUMN "public"."t_user"."nickname" IS 'Nickname';

COMMENT ON COLUMN "public"."t_user"."password" IS 'Password';

COMMENT ON COLUMN "public"."t_user"."salt" IS 'Salt';

COMMENT ON COLUMN "public"."t_user"."email" IS 'Email';

COMMENT ON COLUMN "public"."t_user"."phone" IS 'Phone';

COMMENT ON COLUMN "public"."t_user"."open_id" IS 'WeChat OpenID';

COMMENT ON COLUMN "public"."t_user"."client_ip" IS 'Client IP';

COMMENT ON COLUMN "public"."t_user"."login_at" IS 'Login time';

COMMENT ON COLUMN "public"."t_user"."login_token" IS 'Login token';