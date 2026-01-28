pub struct ApprovalGate {
    key_manager: Arc<KeyManager>,
}

pub struct ApprovedSnapshot {
    /// Original IR
    pub ir: IR,
    
    /// Validation metadata
    pub validation: ValidationMetadata,
    
    /// Approval metadata
    pub approval: ApprovalMetadata,
    
    /// Cryptographic hash of (ir + validation)
    pub hash: Hash,
    
    /// Creator signature
    pub signature: Signature,
}

pub struct ApprovalMetadata {
    pub creator_id: CreatorId,
    pub approved_at: Timestamp,
    pub version: SemanticVersion,
    pub tags: Vec<String>,
}

impl ApprovalGate {
    pub fn approve(
        &self,
        validated_ir: ValidatedIR,
        creator_id: CreatorId,
    ) -> Result<ApprovedSnapshot, ApprovalError> {
        // 1. Check creator authorization
        if !self.key_manager.is_authorized(&creator_id) {
            return Err(ApprovalError::Unauthorized);
        }
        
        // 2. Compute cryptographic hash
        let hash = Self::compute_hash(&validated_ir);
        
        // 3. Sign with creator's private key
        let signature = self.key_manager.sign(&hash, &creator_id)?;
        
        // 4. Bundle into immutable snapshot
        Ok(ApprovedSnapshot {
            ir: validated_ir.ir,
            validation: validated_ir.into_metadata(),
            approval: ApprovalMetadata {
                creator_id,
                approved_at: Timestamp::now(),
                version: SemanticVersion::new(1, 0, 0),
                tags: vec![],
            },
            hash,
            signature,
        })
    }
    
    fn compute_hash(validated_ir: &ValidatedIR) -> Hash {
        use sha3::{Sha3_256, Digest};
        let mut hasher = Sha3_256::new();
        hasher.update(&bincode::serialize(&validated_ir).unwrap());
        Hash(hasher.finalize().into())
    }
}