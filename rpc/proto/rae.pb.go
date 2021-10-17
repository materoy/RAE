// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.17.3
// source: rpc/proto/rae.proto

package RAE

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Request struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id             int32    `protobuf:"varint,1,opt,name=id,proto3" json:"id,omitempty"`
	Name           string   `protobuf:"bytes,2,opt,name=name,proto3" json:"name,omitempty"`
	Executable     []byte   `protobuf:"bytes,3,opt,name=executable,proto3" json:"executable,omitempty"`
	ExecuteCommand string   `protobuf:"bytes,4,opt,name=executeCommand,proto3" json:"executeCommand,omitempty"`
	Data           string   `protobuf:"bytes,5,opt,name=data,proto3" json:"data,omitempty"`
	Path           string   `protobuf:"bytes,6,opt,name=path,proto3" json:"path,omitempty"`
	Argv           []string `protobuf:"bytes,7,rep,name=argv,proto3" json:"argv,omitempty"`
	Envv           []string `protobuf:"bytes,8,rep,name=envv,proto3" json:"envv,omitempty"`
}

func (x *Request) Reset() {
	*x = Request{}
	if protoimpl.UnsafeEnabled {
		mi := &file_rpc_proto_rae_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Request) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Request) ProtoMessage() {}

func (x *Request) ProtoReflect() protoreflect.Message {
	mi := &file_rpc_proto_rae_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Request.ProtoReflect.Descriptor instead.
func (*Request) Descriptor() ([]byte, []int) {
	return file_rpc_proto_rae_proto_rawDescGZIP(), []int{0}
}

func (x *Request) GetId() int32 {
	if x != nil {
		return x.Id
	}
	return 0
}

func (x *Request) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *Request) GetExecutable() []byte {
	if x != nil {
		return x.Executable
	}
	return nil
}

func (x *Request) GetExecuteCommand() string {
	if x != nil {
		return x.ExecuteCommand
	}
	return ""
}

func (x *Request) GetData() string {
	if x != nil {
		return x.Data
	}
	return ""
}

func (x *Request) GetPath() string {
	if x != nil {
		return x.Path
	}
	return ""
}

func (x *Request) GetArgv() []string {
	if x != nil {
		return x.Argv
	}
	return nil
}

func (x *Request) GetEnvv() []string {
	if x != nil {
		return x.Envv
	}
	return nil
}

type Response struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Result string `protobuf:"bytes,1,opt,name=result,proto3" json:"result,omitempty"`
}

func (x *Response) Reset() {
	*x = Response{}
	if protoimpl.UnsafeEnabled {
		mi := &file_rpc_proto_rae_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Response) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Response) ProtoMessage() {}

func (x *Response) ProtoReflect() protoreflect.Message {
	mi := &file_rpc_proto_rae_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Response.ProtoReflect.Descriptor instead.
func (*Response) Descriptor() ([]byte, []int) {
	return file_rpc_proto_rae_proto_rawDescGZIP(), []int{1}
}

func (x *Response) GetResult() string {
	if x != nil {
		return x.Result
	}
	return ""
}

var File_rpc_proto_rae_proto protoreflect.FileDescriptor

var file_rpc_proto_rae_proto_rawDesc = []byte{
	0x0a, 0x13, 0x72, 0x70, 0x63, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x72, 0x61, 0x65, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x22,
	0xc5, 0x01, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69,
	0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x02, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e,
	0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
	0x1e, 0x0a, 0x0a, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x0c, 0x52, 0x0a, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x12,
	0x26, 0x0a, 0x0e, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
	0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65,
	0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18,
	0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x12, 0x0a, 0x04, 0x70,
	0x61, 0x74, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
	0x12, 0x0a, 0x04, 0x61, 0x72, 0x67, 0x76, 0x18, 0x07, 0x20, 0x03, 0x28, 0x09, 0x52, 0x04, 0x61,
	0x72, 0x67, 0x76, 0x12, 0x12, 0x0a, 0x04, 0x65, 0x6e, 0x76, 0x76, 0x18, 0x08, 0x20, 0x03, 0x28,
	0x09, 0x52, 0x04, 0x65, 0x6e, 0x76, 0x76, 0x22, 0x22, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x32, 0x49, 0x0a, 0x0d, 0x53,
	0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x38, 0x0a, 0x0d,
	0x46, 0x65, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x11, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x1a, 0x12, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x30, 0x01, 0x42, 0x16, 0x5a, 0x14, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x72, 0x6d, 0x67, 0x65, 0x6e, 0x2f, 0x52, 0x41, 0x45, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_rpc_proto_rae_proto_rawDescOnce sync.Once
	file_rpc_proto_rae_proto_rawDescData = file_rpc_proto_rae_proto_rawDesc
)

func file_rpc_proto_rae_proto_rawDescGZIP() []byte {
	file_rpc_proto_rae_proto_rawDescOnce.Do(func() {
		file_rpc_proto_rae_proto_rawDescData = protoimpl.X.CompressGZIP(file_rpc_proto_rae_proto_rawDescData)
	})
	return file_rpc_proto_rae_proto_rawDescData
}

var file_rpc_proto_rae_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_rpc_proto_rae_proto_goTypes = []interface{}{
	(*Request)(nil),  // 0: protobuf.Request
	(*Response)(nil), // 1: protobuf.Response
}
var file_rpc_proto_rae_proto_depIdxs = []int32{
	0, // 0: protobuf.StreamService.FetchResponse:input_type -> protobuf.Request
	1, // 1: protobuf.StreamService.FetchResponse:output_type -> protobuf.Response
	1, // [1:2] is the sub-list for method output_type
	0, // [0:1] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_rpc_proto_rae_proto_init() }
func file_rpc_proto_rae_proto_init() {
	if File_rpc_proto_rae_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_rpc_proto_rae_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Request); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_rpc_proto_rae_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Response); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_rpc_proto_rae_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_rpc_proto_rae_proto_goTypes,
		DependencyIndexes: file_rpc_proto_rae_proto_depIdxs,
		MessageInfos:      file_rpc_proto_rae_proto_msgTypes,
	}.Build()
	File_rpc_proto_rae_proto = out.File
	file_rpc_proto_rae_proto_rawDesc = nil
	file_rpc_proto_rae_proto_goTypes = nil
	file_rpc_proto_rae_proto_depIdxs = nil
}
