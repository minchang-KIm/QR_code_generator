# QR Image Generator 🎨

**프로덕션 레벨의 키워드 기반 QR 코드 이미지 생성기**

키워드를 입력하면 해당 키워드에 맞는 아름다운 배경 이미지를 가져오고, 그 위에 QR 코드를 임베드합니다.
생성된 QR 코드는 **자동으로 검증**되어 100% 읽을 수 있음을 보장합니다.

## ✨ 주요 기능

- 🔍 **키워드 기반 이미지 검색**: Unsplash API를 통한 고품질 이미지 제공
- 🎯 **스마트 QR 임베딩**: 알파 블렌딩을 사용한 자연스러운 QR 코드 배치
- ✅ **자동 검증**: QR 코드 생성 후 자동으로 디코딩하여 읽기 가능 여부 확인
- 🛡️ **프로덕션 레벨**: 에러 핸들링, 로깅, 재시도 로직 포함
- 🎨 **커스터마이징**: 크기, 위치, 투명도 등 다양한 옵션 제공
- 🌐 **폴백 시스템**: API 실패 시 자동으로 대체 이미지 생성

## 📋 요구사항

- Rust 1.70 이상
- 인터넷 연결 (이미지 다운로드용)
- Unsplash API 키 (선택사항, 없으면 폴백 사용)

## 🚀 설치 및 빌드

```bash
# 저장소 클론
git clone <repository-url>
cd QR_code_generator

# 빌드
cargo build --release

# 빌드된 바이너리 실행
./target/release/QR_code_generator --help
```

## 📖 사용법

### 기본 사용

```bash
cargo run -- --keyword "nature" --data "https://example.com"
```

### 전체 옵션

```bash
cargo run -- \
  --keyword "ocean sunset" \
  --data "https://snowball-tree.tistory.com/" \
  --output "my_qr_image.png" \
  --width 1920 \
  --height 1080 \
  --qr-size 0.3 \
  --position bottom-right \
  --opacity 230 \
  --verbose
```

### 옵션 설명

| 옵션 | 짧은 옵션 | 설명 | 기본값 |
|------|-----------|------|--------|
| `--keyword` | `-k` | 배경 이미지 검색 키워드 | (필수) |
| `--data` | `-d` | QR 코드에 인코딩할 데이터 (URL, 텍스트 등) | (필수) |
| `--output` | `-o` | 출력 파일 경로 | `qr_output.png` |
| `--api-key` | | Unsplash API 키 | 환경변수 `UNSPLASH_API_KEY` |
| `--width` | | 이미지 너비 (픽셀) | `1920` |
| `--height` | | 이미지 높이 (픽셀) | `1080` |
| `--qr-size` | | QR 코드 크기 비율 (0.1~0.5) | `0.25` |
| `--position` | | QR 코드 위치 | `bottom-right` |
| `--opacity` | | QR 코드 배경 투명도 (0-255) | `230` |
| `--verbose` | `-v` | 상세 로그 출력 | `false` |

### QR 코드 위치 옵션

- `top-left`: 왼쪽 상단
- `top-right`: 오른쪽 상단
- `bottom-left`: 왼쪽 하단
- `bottom-right`: 오른쪽 하단 (기본값)
- `center`: 중앙

## 🔑 Unsplash API 키 설정

더 나은 품질의 이미지를 위해 Unsplash API 키를 사용하는 것을 권장합니다.

1. [Unsplash Developers](https://unsplash.com/developers)에서 무료 계정 생성
2. 새 애플리케이션 등록
3. Access Key 복사
4. 환경변수 설정:

```bash
export UNSPLASH_API_KEY="your_access_key_here"
```

또는 실행 시 직접 전달:

```bash
cargo run -- --keyword "mountain" --data "https://example.com" --api-key "your_key"
```

## 📝 사용 예시

### 예시 1: 블로그 QR 코드 생성

```bash
cargo run -- \
  -k "coffee laptop" \
  -d "https://snowball-tree.tistory.com/" \
  -o "blog_qr.png"
```

### 예시 2: 제품 페이지 QR 코드

```bash
cargo run -- \
  -k "modern technology" \
  -d "https://shop.example.com/product/123" \
  -o "product_qr.png" \
  --position center \
  --qr-size 0.3
```

### 예시 3: 이벤트 초대 QR 코드

```bash
cargo run -- \
  -k "party celebration" \
  -d "https://event.example.com/invitation" \
  -o "event_qr.png" \
  --width 1080 \
  --height 1080 \
  --position bottom-right
```

### 예시 4: 상세 로그와 함께 생성

```bash
cargo run -- \
  -k "abstract art" \
  -d "https://example.com" \
  -v
```

## 🏗️ 프로젝트 구조

```
src/
├── main.rs              # CLI 엔트리포인트
├── lib.rs               # 라이브러리 인터페이스
├── config.rs            # 설정 관리
├── error.rs             # 에러 타입 정의
├── image_provider.rs    # 이미지 검색/생성 모듈
├── qr_embedder.rs       # QR 코드 임베딩 모듈
└── qr_validator.rs      # QR 코드 검증 모듈
```

## 🔍 검증 프로세스

생성된 모든 QR 코드 이미지는 다음 프로세스를 거칩니다:

1. **QR 코드 생성**: 입력 데이터를 QR 코드로 변환
2. **이미지 임베딩**: 배경 이미지에 QR 코드 오버레이
3. **자동 디코딩**: 생성된 이미지에서 QR 코드 검출 및 디코딩
4. **데이터 검증**: 디코딩된 데이터가 원본 데이터와 일치하는지 확인
5. **재시도 로직**: 실패 시 다양한 이미지 처리 기법 적용 후 재시도 (최대 3회)

검증에 실패한 이미지는 절대 반환되지 않습니다!

## 🛠️ 기술 스택

- **Rust**: 안전하고 빠른 시스템 프로그래밍
- **image**: 이미지 처리
- **qrcode**: QR 코드 생성
- **rqrr**: QR 코드 디코딩 및 검증
- **reqwest**: HTTP 클라이언트
- **clap**: CLI 인자 파싱
- **serde**: JSON 직렬화
- **anyhow/thiserror**: 에러 핸들링
- **log/env_logger**: 로깅

## 🧪 테스트

```bash
# 단위 테스트 실행
cargo test

# 상세 출력과 함께 테스트
cargo test -- --nocapture

# 특정 테스트만 실행
cargo test test_validator
```

## 🚨 트러블슈팅

### "No QR code detected in image"

- QR 코드 크기를 늘려보세요: `--qr-size 0.35`
- 투명도를 낮춰보세요: `--opacity 250`
- 배경이 너무 복잡한 경우 다른 키워드 시도

### "Image download failed"

- 인터넷 연결 확인
- Unsplash API 키 확인
- API 키 없이도 폴백 이미지가 생성됩니다

### "API rate limit exceeded"

- Unsplash 무료 플랜은 시간당 50개 요청 제한
- 잠시 후 다시 시도하거나 API 키 없이 실행

## 📄 라이선스

MIT License

## 🤝 기여

이슈와 풀 리퀘스트를 환영합니다!

## 📞 문의

문제가 발생하면 이슈를 등록해주세요.

---

**Made with ❤️ and Rust**
