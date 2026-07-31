// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title Logic
/// @author Anoma Foundation, 2025
/// @notice A library containing type definitions of the resource logic proving system.
/// @custom:security-contact security@anoma.foundation
library Logic {
    /// @notice An enum representing the supported blob deletion criteria.
    enum DeletionCriterion {
        Immediately,
        Never
    }

    /// @notice A struct containing payloads of different kinds.
    /// @param resourcePayload A list of blobs for encoding plaintext info connected to resources.
    /// @param discoveryPayload A list of blobs for encoding data with public keys for discovery.
    /// @param externalPayload A list of blobs for encoding data connected with external calls.
    /// @param applicationPayload A list of blobs for application-specific purposes.
    struct AppData {
        ExpirableBlob[] resourcePayload;
        ExpirableBlob[] discoveryPayload;
        ExpirableBlob[] externalPayload;
        ExpirableBlob[] applicationPayload;
    }

    /// @notice A blob with a deletion criterion attached.
    /// @param deletionCriterion The deletion criterion.
    /// @param blob The bytes-encoded blob data.
    struct ExpirableBlob {
        DeletionCriterion deletionCriterion;
        bytes blob;
    }
}
