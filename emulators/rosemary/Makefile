PORTNAME=	rosemary
DISTVERSION=	0.4.1
CATEGORIES=	emulators

MAINTAINER=	teknxs@mailfence.com
COMMENT=	Hybridized translation layer for macOS applications
WWW=		https://github.com/microlift/rosemary

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE

ONLY_FOR_ARCHS=	amd64

LIB_DEPENDS=	libcurl.so:ftp/curl \
		libvulkan.so:graphics/vulkan-loader \
		libwebkitgtk-6.0.so:www/webkit2-gtk \
		libxkbcommon.so:x11/libxkbcommon

USES=		cargo gl localbase openal pkgconfig sdl
USE_GL=		egl opengl
USE_SDL=	sdl3

USE_GITHUB=	yes
GH_ACCOUNT=	microlift
GH_PROJECT=	rosemary
GH_TAGNAME=	v${DISTVERSION}

CARGO_INSTALL=		no
CARGO_TEST=		no
CARGO_BUILD_ARGS=	--bin rosemary

do-install:
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/release/${PORTNAME} \
	${STAGEDIR}${PREFIX}/bin/${PORTNAME}

PLIST_FILES=	bin/rosemary

.include "Makefile.crates"
.include <bsd.port.mk>
