# Create-Leptos-App

## 1. Intro

[EN] This template serves as the fundamental setup for implementing Server-Side Rendering (SSR) using server functions within the Leptos framework. Embracing the latest trend, both the front and back ends operate seamlessly as a unified server entity.

This adaptation stems from the todo_app_sqlite_axum example within the Leptos repository. To facilitate future microservices implementation through Docker, a PostgreSQL database was integrated into Docker. Upon completion, only the server-component folder was Dockerized to generate an image, which can be utilized in Kubernetes akin to the database setup.

---

[KR] 이 템플릿은 Leptos로 server functions을 사용하여 SSR를 구현하기 위한 기본 세팅입니다. 최신 프레임워크 추세로 프론트와 백엔드를 1개의 서버로 동시에 작동시킵니다.

이것은 Leptos 예제 중 todo_app_sqlite_axum의 예제를 변형한 것으로, 추후 Docker를 통해 microservices를 쉽게 구현할 수 있도록 DB는 Postges를 도커에 올려 구현했으며, 완성 후에는 sever-component 폴더만 도커로 이미지를 만들어 db와 같이 쿠버네티스 등을 통해 배포하면 됩니다.

## 2. DB Table Name Caution

[EN] The table name in PostgreSQL is 'fmtf'. When adjusting the table name to align with the user's requirements, follow these steps:

1. Open the Dockerfile.dev file in the DB folder and modify ‘fmtf’ in POSTGRES_DB=fmtf to the desired table name.
2. Replace all occurrences of 'fmtf' in the Makefile with the intended table name

---

[KR] PostgreSQL 내 테이블 이름은 'fmtf'로 되어 있습니다. 사용자의 목적에 맞게 테이블 이름을 바꿀시에는

1. DB폴더 내 Dockerfile.dev 파일을 열어 POSTGRES_DB=fmtf에서 'fmtf'를 사용자가 원하는 이름으로 바꾸세요
2. Makefile에서 모든 'fmtf'를 사용자가 의도한 테이블 이름으로 바꿔주세요

## 3-1. Quick Start

```script
make init
```

## 3-2. Quick Stop

```script
make q
```
