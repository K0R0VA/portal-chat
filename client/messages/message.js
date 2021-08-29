/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
"use strict";

var $protobuf = require("protobufjs/minimal");

// Common aliases
var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;

// Exported root namespace
var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});

/**
 * MessageType enum.
 * @exports MessageType
 * @enum {number}
 * @property {number} CONTACT=0 CONTACT value
 * @property {number} GROUP=1 GROUP value
 */
$root.MessageType = (function() {
    var valuesById = {}, values = Object.create(valuesById);
    values[valuesById[0] = "CONTACT"] = 0;
    values[valuesById[1] = "GROUP"] = 1;
    return values;
})();

$root.ClientMessage = (function() {

    /**
     * Properties of a ClientMessage.
     * @exports IClientMessage
     * @interface IClientMessage
     * @property {number|null} [senderId] ClientMessage senderId
     * @property {string|null} [text] ClientMessage text
     * @property {Uint8Array|null} [media] ClientMessage media
     * @property {MessageType|null} [requestType] ClientMessage requestType
     * @property {number|null} [recipientId] ClientMessage recipientId
     */

    /**
     * Constructs a new ClientMessage.
     * @exports ClientMessage
     * @classdesc Represents a ClientMessage.
     * @implements IClientMessage
     * @constructor
     * @param {IClientMessage=} [properties] Properties to set
     */
    function ClientMessage(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * ClientMessage senderId.
     * @member {number} senderId
     * @memberof ClientMessage
     * @instance
     */
    ClientMessage.prototype.senderId = 0;

    /**
     * ClientMessage text.
     * @member {string} text
     * @memberof ClientMessage
     * @instance
     */
    ClientMessage.prototype.text = "";

    /**
     * ClientMessage media.
     * @member {Uint8Array|null|undefined} media
     * @memberof ClientMessage
     * @instance
     */
    ClientMessage.prototype.media = null;

    /**
     * ClientMessage requestType.
     * @member {MessageType} requestType
     * @memberof ClientMessage
     * @instance
     */
    ClientMessage.prototype.requestType = 0;

    /**
     * ClientMessage recipientId.
     * @member {number} recipientId
     * @memberof ClientMessage
     * @instance
     */
    ClientMessage.prototype.recipientId = 0;

    // OneOf field names bound to virtual getters and setters
    var $oneOfFields;

    /**
     * ClientMessage _media.
     * @member {"media"|undefined} _media
     * @memberof ClientMessage
     * @instance
     */
    Object.defineProperty(ClientMessage.prototype, "_media", {
        get: $util.oneOfGetter($oneOfFields = ["media"]),
        set: $util.oneOfSetter($oneOfFields)
    });

    /**
     * Creates a new ClientMessage instance using the specified properties.
     * @function create
     * @memberof ClientMessage
     * @static
     * @param {IClientMessage=} [properties] Properties to set
     * @returns {ClientMessage} ClientMessage instance
     */
    ClientMessage.create = function create(properties) {
        return new ClientMessage(properties);
    };

    /**
     * Encodes the specified ClientMessage message. Does not implicitly {@link ClientMessage.verify|verify} messages.
     * @function encode
     * @memberof ClientMessage
     * @static
     * @param {IClientMessage} message ClientMessage message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ClientMessage.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.senderId != null && Object.hasOwnProperty.call(message, "senderId"))
            writer.uint32(/* id 1, wireType 0 =*/8).int32(message.senderId);
        if (message.text != null && Object.hasOwnProperty.call(message, "text"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.text);
        if (message.media != null && Object.hasOwnProperty.call(message, "media"))
            writer.uint32(/* id 3, wireType 2 =*/26).bytes(message.media);
        if (message.requestType != null && Object.hasOwnProperty.call(message, "requestType"))
            writer.uint32(/* id 4, wireType 0 =*/32).int32(message.requestType);
        if (message.recipientId != null && Object.hasOwnProperty.call(message, "recipientId"))
            writer.uint32(/* id 5, wireType 0 =*/40).int32(message.recipientId);
        return writer;
    };

    /**
     * Encodes the specified ClientMessage message, length delimited. Does not implicitly {@link ClientMessage.verify|verify} messages.
     * @function encodeDelimited
     * @memberof ClientMessage
     * @static
     * @param {IClientMessage} message ClientMessage message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ClientMessage.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a ClientMessage message from the specified reader or buffer.
     * @function decode
     * @memberof ClientMessage
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {ClientMessage} ClientMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ClientMessage.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.ClientMessage();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.senderId = reader.int32();
                break;
            case 2:
                message.text = reader.string();
                break;
            case 3:
                message.media = reader.bytes();
                break;
            case 4:
                message.requestType = reader.int32();
                break;
            case 5:
                message.recipientId = reader.int32();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a ClientMessage message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof ClientMessage
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {ClientMessage} ClientMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ClientMessage.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a ClientMessage message.
     * @function verify
     * @memberof ClientMessage
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    ClientMessage.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        var properties = {};
        if (message.senderId != null && message.hasOwnProperty("senderId"))
            if (!$util.isInteger(message.senderId))
                return "senderId: integer expected";
        if (message.text != null && message.hasOwnProperty("text"))
            if (!$util.isString(message.text))
                return "text: string expected";
        if (message.media != null && message.hasOwnProperty("media")) {
            properties._media = 1;
            if (!(message.media && typeof message.media.length === "number" || $util.isString(message.media)))
                return "media: buffer expected";
        }
        if (message.requestType != null && message.hasOwnProperty("requestType"))
            switch (message.requestType) {
            default:
                return "requestType: enum value expected";
            case 0:
            case 1:
                break;
            }
        if (message.recipientId != null && message.hasOwnProperty("recipientId"))
            if (!$util.isInteger(message.recipientId))
                return "recipientId: integer expected";
        return null;
    };

    /**
     * Creates a ClientMessage message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof ClientMessage
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {ClientMessage} ClientMessage
     */
    ClientMessage.fromObject = function fromObject(object) {
        if (object instanceof $root.ClientMessage)
            return object;
        var message = new $root.ClientMessage();
        if (object.senderId != null)
            message.senderId = object.senderId | 0;
        if (object.text != null)
            message.text = String(object.text);
        if (object.media != null)
            if (typeof object.media === "string")
                $util.base64.decode(object.media, message.media = $util.newBuffer($util.base64.length(object.media)), 0);
            else if (object.media.length)
                message.media = object.media;
        switch (object.requestType) {
        case "CONTACT":
        case 0:
            message.requestType = 0;
            break;
        case "GROUP":
        case 1:
            message.requestType = 1;
            break;
        }
        if (object.recipientId != null)
            message.recipientId = object.recipientId | 0;
        return message;
    };

    /**
     * Creates a plain object from a ClientMessage message. Also converts values to other types if specified.
     * @function toObject
     * @memberof ClientMessage
     * @static
     * @param {ClientMessage} message ClientMessage
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    ClientMessage.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.senderId = 0;
            object.text = "";
            object.requestType = options.enums === String ? "CONTACT" : 0;
            object.recipientId = 0;
        }
        if (message.senderId != null && message.hasOwnProperty("senderId"))
            object.senderId = message.senderId;
        if (message.text != null && message.hasOwnProperty("text"))
            object.text = message.text;
        if (message.media != null && message.hasOwnProperty("media")) {
            object.media = options.bytes === String ? $util.base64.encode(message.media, 0, message.media.length) : options.bytes === Array ? Array.prototype.slice.call(message.media) : message.media;
            if (options.oneofs)
                object._media = "media";
        }
        if (message.requestType != null && message.hasOwnProperty("requestType"))
            object.requestType = options.enums === String ? $root.MessageType[message.requestType] : message.requestType;
        if (message.recipientId != null && message.hasOwnProperty("recipientId"))
            object.recipientId = message.recipientId;
        return object;
    };

    /**
     * Converts this ClientMessage to JSON.
     * @function toJSON
     * @memberof ClientMessage
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    ClientMessage.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return ClientMessage;
})();

$root.ServerMessage = (function() {

    /**
     * Properties of a ServerMessage.
     * @exports IServerMessage
     * @interface IServerMessage
     * @property {number|null} [senderId] ServerMessage senderId
     * @property {string|null} [text] ServerMessage text
     * @property {MessageType|null} [requestType] ServerMessage requestType
     */

    /**
     * Constructs a new ServerMessage.
     * @exports ServerMessage
     * @classdesc Represents a ServerMessage.
     * @implements IServerMessage
     * @constructor
     * @param {IServerMessage=} [properties] Properties to set
     */
    function ServerMessage(properties) {
        if (properties)
            for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                if (properties[keys[i]] != null)
                    this[keys[i]] = properties[keys[i]];
    }

    /**
     * ServerMessage senderId.
     * @member {number} senderId
     * @memberof ServerMessage
     * @instance
     */
    ServerMessage.prototype.senderId = 0;

    /**
     * ServerMessage text.
     * @member {string} text
     * @memberof ServerMessage
     * @instance
     */
    ServerMessage.prototype.text = "";

    /**
     * ServerMessage requestType.
     * @member {MessageType} requestType
     * @memberof ServerMessage
     * @instance
     */
    ServerMessage.prototype.requestType = 0;

    /**
     * Creates a new ServerMessage instance using the specified properties.
     * @function create
     * @memberof ServerMessage
     * @static
     * @param {IServerMessage=} [properties] Properties to set
     * @returns {ServerMessage} ServerMessage instance
     */
    ServerMessage.create = function create(properties) {
        return new ServerMessage(properties);
    };

    /**
     * Encodes the specified ServerMessage message. Does not implicitly {@link ServerMessage.verify|verify} messages.
     * @function encode
     * @memberof ServerMessage
     * @static
     * @param {IServerMessage} message ServerMessage message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ServerMessage.encode = function encode(message, writer) {
        if (!writer)
            writer = $Writer.create();
        if (message.senderId != null && Object.hasOwnProperty.call(message, "senderId"))
            writer.uint32(/* id 1, wireType 0 =*/8).int32(message.senderId);
        if (message.text != null && Object.hasOwnProperty.call(message, "text"))
            writer.uint32(/* id 2, wireType 2 =*/18).string(message.text);
        if (message.requestType != null && Object.hasOwnProperty.call(message, "requestType"))
            writer.uint32(/* id 4, wireType 0 =*/32).int32(message.requestType);
        return writer;
    };

    /**
     * Encodes the specified ServerMessage message, length delimited. Does not implicitly {@link ServerMessage.verify|verify} messages.
     * @function encodeDelimited
     * @memberof ServerMessage
     * @static
     * @param {IServerMessage} message ServerMessage message or plain object to encode
     * @param {$protobuf.Writer} [writer] Writer to encode to
     * @returns {$protobuf.Writer} Writer
     */
    ServerMessage.encodeDelimited = function encodeDelimited(message, writer) {
        return this.encode(message, writer).ldelim();
    };

    /**
     * Decodes a ServerMessage message from the specified reader or buffer.
     * @function decode
     * @memberof ServerMessage
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @param {number} [length] Message length if known beforehand
     * @returns {ServerMessage} ServerMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ServerMessage.decode = function decode(reader, length) {
        if (!(reader instanceof $Reader))
            reader = $Reader.create(reader);
        var end = length === undefined ? reader.len : reader.pos + length, message = new $root.ServerMessage();
        while (reader.pos < end) {
            var tag = reader.uint32();
            switch (tag >>> 3) {
            case 1:
                message.senderId = reader.int32();
                break;
            case 2:
                message.text = reader.string();
                break;
            case 4:
                message.requestType = reader.int32();
                break;
            default:
                reader.skipType(tag & 7);
                break;
            }
        }
        return message;
    };

    /**
     * Decodes a ServerMessage message from the specified reader or buffer, length delimited.
     * @function decodeDelimited
     * @memberof ServerMessage
     * @static
     * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
     * @returns {ServerMessage} ServerMessage
     * @throws {Error} If the payload is not a reader or valid buffer
     * @throws {$protobuf.util.ProtocolError} If required fields are missing
     */
    ServerMessage.decodeDelimited = function decodeDelimited(reader) {
        if (!(reader instanceof $Reader))
            reader = new $Reader(reader);
        return this.decode(reader, reader.uint32());
    };

    /**
     * Verifies a ServerMessage message.
     * @function verify
     * @memberof ServerMessage
     * @static
     * @param {Object.<string,*>} message Plain object to verify
     * @returns {string|null} `null` if valid, otherwise the reason why it is not
     */
    ServerMessage.verify = function verify(message) {
        if (typeof message !== "object" || message === null)
            return "object expected";
        if (message.senderId != null && message.hasOwnProperty("senderId"))
            if (!$util.isInteger(message.senderId))
                return "senderId: integer expected";
        if (message.text != null && message.hasOwnProperty("text"))
            if (!$util.isString(message.text))
                return "text: string expected";
        if (message.requestType != null && message.hasOwnProperty("requestType"))
            switch (message.requestType) {
            default:
                return "requestType: enum value expected";
            case 0:
            case 1:
                break;
            }
        return null;
    };

    /**
     * Creates a ServerMessage message from a plain object. Also converts values to their respective internal types.
     * @function fromObject
     * @memberof ServerMessage
     * @static
     * @param {Object.<string,*>} object Plain object
     * @returns {ServerMessage} ServerMessage
     */
    ServerMessage.fromObject = function fromObject(object) {
        if (object instanceof $root.ServerMessage)
            return object;
        var message = new $root.ServerMessage();
        if (object.senderId != null)
            message.senderId = object.senderId | 0;
        if (object.text != null)
            message.text = String(object.text);
        switch (object.requestType) {
        case "CONTACT":
        case 0:
            message.requestType = 0;
            break;
        case "GROUP":
        case 1:
            message.requestType = 1;
            break;
        }
        return message;
    };

    /**
     * Creates a plain object from a ServerMessage message. Also converts values to other types if specified.
     * @function toObject
     * @memberof ServerMessage
     * @static
     * @param {ServerMessage} message ServerMessage
     * @param {$protobuf.IConversionOptions} [options] Conversion options
     * @returns {Object.<string,*>} Plain object
     */
    ServerMessage.toObject = function toObject(message, options) {
        if (!options)
            options = {};
        var object = {};
        if (options.defaults) {
            object.senderId = 0;
            object.text = "";
            object.requestType = options.enums === String ? "CONTACT" : 0;
        }
        if (message.senderId != null && message.hasOwnProperty("senderId"))
            object.senderId = message.senderId;
        if (message.text != null && message.hasOwnProperty("text"))
            object.text = message.text;
        if (message.requestType != null && message.hasOwnProperty("requestType"))
            object.requestType = options.enums === String ? $root.MessageType[message.requestType] : message.requestType;
        return object;
    };

    /**
     * Converts this ServerMessage to JSON.
     * @function toJSON
     * @memberof ServerMessage
     * @instance
     * @returns {Object.<string,*>} JSON object
     */
    ServerMessage.prototype.toJSON = function toJSON() {
        return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
    };

    return ServerMessage;
})();

module.exports = $root;
