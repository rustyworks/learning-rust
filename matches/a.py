from enum import Enum, auto

class IpAddr(Enum):
    IpV4 = auto(),
    IpV6 = auto(),


def matcher(kind: IpAddr)-> IpAddr:
    match kind:
        case IpAddr.IpV4:
            return IpAddr.IpV4
        case IpAddr.IpV6:
            return IpAddr.IpV6


if __name__ == "__main__":
    a = IpAddr.IpV4
    b = IpAddr.IpV6

    print(matcher(a))
    print(matcher(b))
