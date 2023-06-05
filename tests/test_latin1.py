from mtokenizers import encode_latin1, decode_latin1


def test_encode_latin1():
    assert encode_latin1("hello", 8) == [104, 101, 108, 108, 111, 0, 0, 0]


def test_decode_latin1():
    assert decode_latin1([119, 111, 114, 108, 100]) == 'world'
