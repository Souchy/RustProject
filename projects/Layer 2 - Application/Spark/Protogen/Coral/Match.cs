// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: Match.proto
// </auto-generated>
#pragma warning disable 1591, 0612, 3021, 8981
#region Designer generated code

using pb = global::Google.Protobuf;
using pbc = global::Google.Protobuf.Collections;
using pbr = global::Google.Protobuf.Reflection;
using scg = global::System.Collections.Generic;
namespace Models {

  /// <summary>Holder for reflection information generated from Match.proto</summary>
  public static partial class MatchReflection {

    #region Descriptor
    /// <summary>File descriptor for Match.proto</summary>
    public static pbr::FileDescriptor Descriptor {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static MatchReflection() {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "CgtNYXRjaC5wcm90bxIGbW9kZWxzIlUKBU1hdGNoEgoKAmlkGAEgASgJEg0K",
            "BXF1ZXVlGAIgASgFEhEKCWdhbWVfcG9ydBgDIAEoBRINCgV0b2tlbhgEIAEo",
            "CRIPCgdwbGF5ZXJzGAUgAygJYgZwcm90bzM="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::Models.Match), global::Models.Match.Parser, new[]{ "Id", "Queue", "GamePort", "Token", "Players" }, null, null, null, null)
          }));
    }
    #endregion

  }
  #region Messages
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class Match : pb::IMessage<Match>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<Match> _parser = new pb::MessageParser<Match>(() => new Match());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<Match> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Models.MatchReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public Match() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public Match(Match other) : this() {
      id_ = other.id_;
      queue_ = other.queue_;
      gamePort_ = other.gamePort_;
      token_ = other.token_;
      players_ = other.players_.Clone();
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public Match Clone() {
      return new Match(this);
    }

    /// <summary>Field number for the "id" field.</summary>
    public const int IdFieldNumber = 1;
    private string id_ = "";
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Id {
      get { return id_; }
      set {
        id_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "queue" field.</summary>
    public const int QueueFieldNumber = 2;
    private int queue_;
    /// <summary>
    /// Type of match
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int Queue {
      get { return queue_; }
      set {
        queue_ = value;
      }
    }

    /// <summary>Field number for the "game_port" field.</summary>
    public const int GamePortFieldNumber = 3;
    private int gamePort_;
    /// <summary>
    /// Game server
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int GamePort {
      get { return gamePort_; }
      set {
        gamePort_ = value;
      }
    }

    /// <summary>Field number for the "token" field.</summary>
    public const int TokenFieldNumber = 4;
    private string token_ = "";
    /// <summary>
    /// Access token to the game on the game server
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public string Token {
      get { return token_; }
      set {
        token_ = pb::ProtoPreconditions.CheckNotNull(value, "value");
      }
    }

    /// <summary>Field number for the "players" field.</summary>
    public const int PlayersFieldNumber = 5;
    private static readonly pb::FieldCodec<string> _repeated_players_codec
        = pb::FieldCodec.ForString(42);
    private readonly pbc::RepeatedField<string> players_ = new pbc::RepeatedField<string>();
    /// <summary>
    /// List of players in the match
    /// </summary>
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public pbc::RepeatedField<string> Players {
      get { return players_; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as Match);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(Match other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (Id != other.Id) return false;
      if (Queue != other.Queue) return false;
      if (GamePort != other.GamePort) return false;
      if (Token != other.Token) return false;
      if(!players_.Equals(other.players_)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (Id.Length != 0) hash ^= Id.GetHashCode();
      if (Queue != 0) hash ^= Queue.GetHashCode();
      if (GamePort != 0) hash ^= GamePort.GetHashCode();
      if (Token.Length != 0) hash ^= Token.GetHashCode();
      hash ^= players_.GetHashCode();
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (Id.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Id);
      }
      if (Queue != 0) {
        output.WriteRawTag(16);
        output.WriteInt32(Queue);
      }
      if (GamePort != 0) {
        output.WriteRawTag(24);
        output.WriteInt32(GamePort);
      }
      if (Token.Length != 0) {
        output.WriteRawTag(34);
        output.WriteString(Token);
      }
      players_.WriteTo(output, _repeated_players_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (Id.Length != 0) {
        output.WriteRawTag(10);
        output.WriteString(Id);
      }
      if (Queue != 0) {
        output.WriteRawTag(16);
        output.WriteInt32(Queue);
      }
      if (GamePort != 0) {
        output.WriteRawTag(24);
        output.WriteInt32(GamePort);
      }
      if (Token.Length != 0) {
        output.WriteRawTag(34);
        output.WriteString(Token);
      }
      players_.WriteTo(ref output, _repeated_players_codec);
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (Id.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Id);
      }
      if (Queue != 0) {
        size += 1 + pb::CodedOutputStream.ComputeInt32Size(Queue);
      }
      if (GamePort != 0) {
        size += 1 + pb::CodedOutputStream.ComputeInt32Size(GamePort);
      }
      if (Token.Length != 0) {
        size += 1 + pb::CodedOutputStream.ComputeStringSize(Token);
      }
      size += players_.CalculateSize(_repeated_players_codec);
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(Match other) {
      if (other == null) {
        return;
      }
      if (other.Id.Length != 0) {
        Id = other.Id;
      }
      if (other.Queue != 0) {
        Queue = other.Queue;
      }
      if (other.GamePort != 0) {
        GamePort = other.GamePort;
      }
      if (other.Token.Length != 0) {
        Token = other.Token;
      }
      players_.Add(other.players_);
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 10: {
            Id = input.ReadString();
            break;
          }
          case 16: {
            Queue = input.ReadInt32();
            break;
          }
          case 24: {
            GamePort = input.ReadInt32();
            break;
          }
          case 34: {
            Token = input.ReadString();
            break;
          }
          case 42: {
            players_.AddEntriesFrom(input, _repeated_players_codec);
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 10: {
            Id = input.ReadString();
            break;
          }
          case 16: {
            Queue = input.ReadInt32();
            break;
          }
          case 24: {
            GamePort = input.ReadInt32();
            break;
          }
          case 34: {
            Token = input.ReadString();
            break;
          }
          case 42: {
            players_.AddEntriesFrom(ref input, _repeated_players_codec);
            break;
          }
        }
      }
    }
    #endif

  }

  #endregion

}

#endregion Designer generated code
